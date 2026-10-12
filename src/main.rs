use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("HIVE_API_KEY")
        .map_err(|_| "HIVE_API_KEY is missing from your .env file")?;

    let client = Client::new();

    let response = client
        .post("https://api-cdn.thehive.ai/api/v3/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "deepseek-ai/deepseek-v4.1-flash",
            "messages": [
                {
                    "role": "user",
                    "content": "Introduce yourself as the AI assistant powering Shronda Studio AI. Keep it to three sentences."
                }
            ],
            "stream": false,
            "max_completion_tokens": 200
        }))
        .send()
        .await?;

    let status = response.status();

    if !status.is_success() {
        return Err(format!("Hive API returned HTTP status {status}").into());
    }

    let result: Value = response.json().await?;

    let answer = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("The API response did not contain the expected answer")?;

    println!("\n--- Shronda Studio AI ---\n");
    println!("{answer}");

    Ok(())
}