use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    agent::{create_agent, read_agent, read_all_agents}, check::check_task, completion::delete_all_completions, health_check::health_checker_handler, task::{create_task, delete_task, read_all_tasks, read_task}
};

use tower_http::validate_request::ValidateRequestHeaderLayer;

pub fn create_router(bearer_token: &str) -> Router {
    // let db = model::todo_db();

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/agent", post(create_agent))
        .route("/api/agent", get(read_all_agents))
        .route("/api/agent/:id", get(read_agent))
        .route("/api/agent/:agentId/task", get(read_all_tasks))
        .route("/api/agent/:agent_id/task/:task_id", get(read_task))
        .route("/api/agent/:agent_id/task/:task_id/check", post(check_task))

        .route("/api/admin/task", post(create_task))
        .route("/api/admin/task/:id", delete(delete_task))
        .route("/api/admin/completion", delete(delete_all_completions))
        
        .route_layer(ValidateRequestHeaderLayer::bearer(bearer_token))
}
