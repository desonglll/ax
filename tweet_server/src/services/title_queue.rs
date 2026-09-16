//! Background AI titling: posts published without a title are queued and a
//! worker asks the model for a short title, writing it back when it arrives.

use sqlx::PgPool;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use uuid::Uuid;

use super::ai::OpenAiClient;

/// Starts the worker. Returns `None` (feature disabled) when no API key is set.
pub fn spawn(pool: PgPool) -> Option<UnboundedSender<Uuid>> {
    let Some(client) = OpenAiClient::from_env() else {
        tracing::info!("OPENAI_API_KEY not set; AI title generation disabled");
        return None;
    };
    let (tx, mut rx) = unbounded_channel::<Uuid>();

    // Catch up on posts that were left untitled (e.g. while the key was missing).
    let backlog_tx = tx.clone();
    let backlog_pool = pool.clone();
    tokio::spawn(async move {
        match sqlx::query_scalar!("select id from posts where title = ''")
            .fetch_all(&backlog_pool)
            .await
        {
            Ok(ids) => {
                tracing::info!(count = ids.len(), "queued untitled posts for AI titling");
                for id in ids {
                    let _ = backlog_tx.send(id);
                }
            }
            Err(e) => tracing::error!("could not scan for untitled posts: {e}"),
        }
    });

    tokio::spawn(async move {
        while let Some(post_id) = rx.recv().await {
            if let Err(e) = generate_title(&pool, &client, post_id).await {
                tracing::warn!(%post_id, "title generation failed: {e}");
            }
        }
    });
    Some(tx)
}

async fn generate_title(pool: &PgPool, client: &OpenAiClient, post_id: Uuid) -> Result<(), String> {
    let post = sqlx::query!("select title, content from posts where id = $1", post_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
    let Some(post) = post else {
        return Ok(()); // deleted before we got to it
    };
    if !post.title.trim().is_empty() {
        return Ok(());
    }

    let prompt = format!(
        "Write a short, catchy title (10 words or fewer) for this micro-blog post. \
         Reply with the title only, no quotes or prefix.\n\n{}",
        post.content
    );
    let title = client.complete(&prompt, 50).await?;
    let title: String = title.trim_matches('"').chars().take(120).collect();

    sqlx::query!(
        "update posts set title = $1, updated_at = now() where id = $2 and title = ''",
        title,
        post_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;
    tracing::info!(%post_id, %title, "AI title applied");
    Ok(())
}
