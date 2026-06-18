use std::env;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use sqlx::PgPool;
use ai::openai::OpenAiClient;
use ai::models::{ChatCompletionRequest, Message};
use ai::AiService;
use uuid::Uuid;
use crate::infra::log::Log;

pub struct QueueWorker {
    db: PgPool,
    receiver: UnboundedReceiver<Uuid>,
}

impl QueueWorker {
    pub fn new(db: PgPool, receiver: UnboundedReceiver<Uuid>) -> Self {
        Self { db, receiver }
    }

    pub async fn run(mut self) {
        Log::info("AI Title Completion Queue Worker started.".to_string());
        while let Some(post_id) = self.receiver.recv().await {
            Log::info(format!("Queue Worker: Processing post_id={}", post_id));
            if let Err(e) = self.process_post(post_id).await {
                Log::error(format!("Queue Worker: Error processing post_id={}: {}", post_id, e));
            }
        }
        Log::info("AI Title Completion Queue Worker stopped.".to_string());
    }

    async fn process_post(&self, post_id: Uuid) -> Result<(), String> {
        // 1. Fetch post
        let post = sqlx::query!(
            "SELECT id, title, content FROM posts WHERE id = $1",
            post_id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| format!("Database query error: {}", e))?;

        let post = match post {
            Some(p) => p,
            None => {
                Log::info(format!("Queue Worker: Post not found, skipping. id={}", post_id));
                return Ok(());
            }
        };

        // 2. Check if title is empty/whitespace
        if !post.title.trim().is_empty() {
            Log::info(format!("Queue Worker: Post id={} already has a title, skipping.", post_id));
            return Ok(());
        }

        // 3. Initialize AI client
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) if !key.trim().is_empty() => key,
            _ => {
                return Err("OPENAI_API_KEY is not configured or empty".to_string());
            }
        };

        let base_url = env::var("OPENAI_API_BASE").ok().filter(|s| !s.trim().is_empty());
        let client = match base_url {
            Some(url) => OpenAiClient::new_with_base_url(api_key, url),
            None => OpenAiClient::new(api_key),
        };

        let model = env::var("OPENAI_MODEL")
            .or_else(|_| env::var("MODEL"))
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let prompt = format!(
            "Generate an extremely short, concise, and catchy title for the following micro-blog post content. \
             The title must be strictly 10 words or less. Do NOT wrap the title in quotes or add any prefix like 'Title:' or explanations. \
             Just return the title text itself.\n\n\
             Content:\n{}",
            post.content
        );

        let req = ChatCompletionRequest {
            model,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.7),
            max_tokens: Some(50),
        };

        Log::info(format!("Queue Worker: Calling AI API to generate title for post_id={}...", post_id));
        let response = client.chat_completion(req).await?;

        let generated_title = response.choices.first()
            .and_then(|choice| Some(choice.message.content.trim().to_string()))
            .unwrap_or_default();

        if generated_title.is_empty() {
            return Err("AI returned an empty title".to_string());
        }

        Log::info(format!("Queue Worker: Generated title for post_id={}: \"{}\"", post_id, generated_title));

        // 4. Update the post title in database
        sqlx::query!(
            "UPDATE posts SET title = $1, updated_at = NOW() WHERE id = $2",
            generated_title,
            post_id
        )
        .execute(&self.db)
        .await
        .map_err(|e| format!("Failed to update post title: {}", e))?;

        Log::info(format!("Queue Worker: Successfully updated title for post_id={}", post_id));
        Ok(())
    }
}

pub async fn scan_and_enqueue_empty_titles(db: &PgPool, sender: &UnboundedSender<Uuid>) {
    Log::info("Scanning database for posts with empty titles...".to_string());
    let posts = sqlx::query!(
        "SELECT id FROM posts WHERE title = '' OR title IS NULL"
    )
    .fetch_all(db)
    .await;

    match posts {
        Ok(rows) => {
            let count = rows.len();
            for row in rows {
                let _ = sender.send(row.id);
            }
            Log::info(format!("Scan completed. Enqueued {} posts for title completion.", count));
        }
        Err(e) => {
            Log::error(format!("Failed to scan existing posts for empty titles: {}", e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::db::get_db_pool;
    use std::net::TcpListener;
    use std::io::{Read, Write};
    use std::thread;

    #[actix_rt::test]
    async fn test_queue_worker_process_post() {
        // Initialize db pool
        let db = get_db_pool().await;

        // Insert a temporary post with an empty title
        let post_content = "This is a test post content for AI title completion queue.";
        let post_row = sqlx::query!(
            "INSERT INTO posts (title, content, user_id, user_name)
             VALUES ($1, $2, $3, $4)
             RETURNING id",
            "",
            post_content,
            1,
            "test_user"
        )
        .fetch_one(&db)
        .await
        .unwrap();

        // Start a mock TCP server to mock OpenAI API response
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).unwrap();

            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\
                \"id\": \"chatcmpl-test\",\
                \"object\": \"chat.completion\",\
                \"created\": 1677858227,\
                \"model\": \"gpt-4o-mini\",\
                \"choices\": [{\
                    \"index\": 0,\
                    \"message\": {\
                        \"role\": \"assistant\",\
                        \"content\": \"AI Generated Catchy Title\"\
                    },\
                    \"finish_reason\": \"stop\"\
                }],\
                \"usage\": {\
                    \"prompt_tokens\": 20,\
                    \"completion_tokens\": 5,\
                    \"total_tokens\": 25\
                }\
            }";
            stream.write_all(response.as_bytes()).unwrap();
        });

        // Backup existing env vars to restore later
        let old_api_key = env::var("OPENAI_API_KEY").ok();
        let old_api_base = env::var("OPENAI_API_BASE").ok();

        // Set env vars to target the mock server
        env::set_var("OPENAI_API_KEY", "test-mock-key");
        env::set_var("OPENAI_API_BASE", format!("http://127.0.0.1:{}", port));

        // Create the worker and process the post
        let (_tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let worker = QueueWorker::new(db.clone(), rx);
        let res = worker.process_post(post_row.id).await;

        // Restore env vars
        if let Some(key) = old_api_key {
            env::set_var("OPENAI_API_KEY", key);
        } else {
            env::remove_var("OPENAI_API_KEY");
        }
        if let Some(base) = old_api_base {
            env::set_var("OPENAI_API_BASE", base);
        } else {
            env::remove_var("OPENAI_API_BASE");
        }

        // Assert process succeeded
        assert!(res.is_ok(), "Expected process_post to succeed, got: {:?}", res);

        // Fetch updated post and assert title is completed
        let updated_post = sqlx::query!(
            "SELECT title FROM posts WHERE id = $1",
            post_row.id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(updated_post.title, "AI Generated Catchy Title");

        // Clean up
        sqlx::query!("DELETE FROM posts WHERE id = $1", post_row.id)
            .execute(&db)
            .await
            .unwrap();
    }
}
