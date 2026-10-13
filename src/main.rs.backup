use axum::{
    extract::State,
    http::{header, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;

const API_URL: &str = "https://api-cdn.thehive.ai/api/v3/chat/completions";
const MODEL: &str = "deepseek-ai/deepseek-v4.1-flash";

#[derive(Clone)]
struct AppState {
    client: Client,
    api_key: String,
}

#[derive(Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Serialize)]
struct ChatResponse {
    answer: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(error) = run().await {
        eprintln!("Server error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("HIVE_API_KEY")
        .map_err(|_| "HIVE_API_KEY is missing from the environment")?;

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(10000);

    let state = AppState {
        client: Client::new(),
        api_key,
    };

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("https://shrondajeanine79-gif.github.io"),
            HeaderValue::from_static("https://shrondajeanineco.com"),
            HeaderValue::from_static("https://www.shrondajeanineco.com"),
            HeaderValue::from_static("http://localhost:5500"),
            HeaderValue::from_static("http://127.0.0.1:5500"),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/chat", post(chat))
        .layer(cors)
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("==================================");
    println!("       SHRonda STUDIO AI");
    println!("==================================");
    println!("Web server listening on port {port}");
    println!("Health check: /health");
    println!("AI endpoint:  /api/chat");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "Shronda Studio AI is running."
}

async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    let prompt = request.prompt.trim();

    if prompt.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Please enter a prompt.".to_string(),
            }),
        ));
    }

    if prompt.chars().count() > 10000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Your prompt is too long. Please keep it under 10,000 characters."
                    .to_string(),
            }),
        ));
    }

    let response = state
        .client
        .post(API_URL)
        .bearer_auth(&state.api_key)
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
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: "Could not connect to the AI service. Please try again.".to_string(),
                }),
            )
        })?;

    let status = response.status();

    if !status.is_success() {
        eprintln!("Hive API returned HTTP status {status}");

        return Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: "The AI service returned an error. Please try again later.".to_string(),
            }),
        ));
    }

    let result: serde_json::Value = response.json().await.map_err(|_| {
        (
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: "Could not read the AI service response.".to_string(),
            }),
        )
    })?;

    let answer = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: "The AI service returned an unexpected response.".to_string(),
                }),
            )
        })?;

    Ok(Json(ChatResponse {
        answer: answer.to_string(),
    }))
}