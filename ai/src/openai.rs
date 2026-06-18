use async_trait::async_trait;
use reqwest::Client;
use crate::AiService;
use crate::models::{ChatCompletionRequest, ChatCompletionResponse};

pub struct OpenAiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAiClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }
}

#[async_trait]
impl AiService for OpenAiClient {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String> {
        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!(
                "API returned error status {}: {}",
                status,
                error_text
            ));
        }

        let result = response
            .json::<ChatCompletionResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Message, ChatCompletionRequest};
    use std::net::TcpListener;
    use std::io::{Read, Write};
    use std::thread;

    #[tokio::test]
    async fn test_openai_chat_completion_success() {
        // Start a mock TCP server on an ephemeral port
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        // Spawn mock server handler thread
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).unwrap();

            // Return a mock OpenAI JSON response
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\
                \"id\": \"chatcmpl-123\",\
                \"object\": \"chat.completion\",\
                \"created\": 1677858227,\
                \"model\": \"gpt-4o\",\
                \"choices\": [{\
                    \"index\": 0,\
                    \"message\": {\
                        \"role\": \"assistant\",\
                        \"content\": \"Hello! How can I help you today?\"\
                    },\
                    \"finish_reason\": \"stop\"\
                }],\
                \"usage\": {\
                    \"prompt_tokens\": 9,\
                    \"completion_tokens\": 12,\
                    \"total_tokens\": 21\
                }\
            }";
            stream.write_all(response.as_bytes()).unwrap();
        });

        // Instantiate OpenAiClient targeting the mock server
        let client = OpenAiClient::new(
            "mock-api-key".to_string(),
            Some(format!("http://127.0.0.1:{}", port))
        );

        let request = ChatCompletionRequest {
            model: "gpt-4o".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hi".to_string(),
            }],
            temperature: Some(0.7),
            max_tokens: Some(100),
        };

        let result = client.chat_completion(request).await.unwrap();

        assert_eq!(result.id, "chatcmpl-123");
        assert_eq!(result.choices[0].message.content, "Hello! How can I help you today?");
        assert_eq!(result.choices[0].message.role, "assistant");
    }
}
