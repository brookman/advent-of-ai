use axum::{response::IntoResponse, Json};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "SWEX Camp 2024 - Advent of AI: Meta hackathon backend";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
