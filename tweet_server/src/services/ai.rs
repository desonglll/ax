//! Minimal OpenAI-compatible chat client, used to auto-title posts.
//!
//! Configured entirely from the environment:
//! - `OPENAI_API_KEY`  — required; when absent the feature is disabled.
//! - `OPENAI_API_BASE` — optional, default `https://api.openai.com/v1`
//!   (any OpenAI-compatible endpoint works).
//! - `OPENAI_MODEL`    — optional, default `gpt-4o-mini`.

use serde::{Deserialize, Serialize};

pub struct OpenAiClient {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: [Message<'a>; 1],
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

impl OpenAiClient {
    /// `None` when `OPENAI_API_KEY` is not set.
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .ok()
            .filter(|k| !k.trim().is_empty())?;
        let base_url = std::env::var("OPENAI_API_BASE")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
        Some(Self {
            http: reqwest::Client::new(),
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
        })
    }

    /// Sends a single user prompt and returns the assistant's trimmed reply.
    pub async fn complete(&self, prompt: &str, max_tokens: u32) -> Result<String, String> {
        let request = ChatRequest {
            model: &self.model,
            messages: [Message {
                role: "user",
                content: prompt,
            }],
            temperature: 0.7,
            max_tokens,
        };
        let response = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API returned {status}: {text}"));
        }
        let parsed: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("unexpected response: {e}"))?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content.trim().to_string())
            .filter(|c| !c.is_empty())
            .ok_or_else(|| "empty completion".to_string())
    }
}
