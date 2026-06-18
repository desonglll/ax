use ai::openai::OpenAiClient;
use ai::AiService;
use ai::models::{ChatCompletionRequest, Message};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load .env file from the workspace root or current directory
    dotenv().ok();

    println!("====================================================");
    println!("       AI Client Connection Test Tool");
    println!("====================================================");

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => {
            if key.is_empty() {
                eprintln!("Error: OPENAI_API_KEY is defined but empty.");
                return;
            }
            key
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY is not defined in environment or .env file.");
            println!("Please check your .env file or environment variables.");
            return;
        }
    };

    let base_url = env::var("OPENAI_API_BASE").ok();

    println!("✓ API Key loaded successfully.");
    if let Some(ref url) = base_url {
        println!("✓ Custom Base URL: {}", url);
    } else {
        println!("✓ Base URL: Default (https://api.openai.com/v1)");
    }

    let client = match base_url {
        Some(url) => OpenAiClient::new_with_base_url(api_key, url),
        None => OpenAiClient::new(api_key),
    };

    // Prompt selection: can specify a model from environment or fallback to standard
    let model = env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
    println!("✓ Targeting Model: {}", model);
    println!("----------------------------------------------------");
    println!("Sending chat completion request...");

    let request = ChatCompletionRequest {
        model,
        messages: vec![Message {
            role: "user".to_string(),
            content: "Please reply with exactly: 'AI Connection Test Successful!'".to_string(),
        }],
        temperature: Some(0.7),
        max_tokens: Some(50),
    };

    match client.chat_completion(request).await {
        Ok(response) => {
            println!("\n🎉 SUCCESS!");
            println!("- Request ID: {}", response.id);
            println!("- Response Model: {}", response.model);
            if let Some(choice) = response.choices.first() {
                println!("- Content Received:\n  \"{}\"", choice.message.content.trim());
            } else {
                println!("- Received empty choices array.");
            }
        }
        Err(err) => {
            eprintln!("\n❌ ERROR OCCURRED!");
            eprintln!("- Error Details: {}", err);
            eprintln!("Please check your API key validity, base URL endpoint, and internet connection.");
        }
    }
    println!("====================================================");
}
