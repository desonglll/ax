pub mod models;
pub mod openai;

use async_trait::async_trait;
use models::{ChatCompletionRequest, ChatCompletionResponse};

#[async_trait]
pub trait AiService: Send + Sync {
    /// Send a chat completion request to the AI service.
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String>;
}
