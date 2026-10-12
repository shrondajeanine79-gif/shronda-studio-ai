use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};

const API_URL: &str = "https://api-cdn.thehive.ai/api/v3/chat/completions";
const MODEL: &str = "deepseek-ai/deepseek-v4.1-flash";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(error) = run().await {
        eprintln!("\nError: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================");
    println!("       SHRonda STUDIO AI");
    println!("==================================");
    println!("Hive DeepSeek assistant");
    println!("Type a prompt, or type exit to quit.\n");

    let api_key = env::var("HIVE_API_KEY")
        .map_err(|_| "HIVE_API_KEY is missing from your .env file")?;

    let client = Client::new();

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt)?;

        let prompt = prompt.trim();

        if prompt.eq_ignore_ascii_case("exit")
            || prompt.eq_ignore_ascii_case("quit")
        {
            println!("Shronda Studio AI: Goodbye!");
            break;
        }

        if prompt.is_empty() {
            continue;
        }

        match ask_ai(&client, &api_key, prompt).await {
            Ok(answer) => {
                println!("\nShronda Studio AI:\n{answer}\n");
            }
            Err(error) => {
                eprintln!("\nRequest failed: {error}\n");
            }
        }
    }

    Ok(())
}

async fn ask_ai(
    client: &Client,
    api_key: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post(API_URL)
        .bearer_auth(api_key)
        .json(&json!({
            "model": MODEL,
            "messages": [
                {
                    "role": "system",
                    "content": "You are the AI assistant for Shronda Studio AI. Help with content creation, writing, brainstorming, brand development, and organizing ideas. Follow the user's instructions and provide complete, useful answers."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "stream": false,
            "max_completion_tokens": 2000
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

    Ok(answer.to_string())
}