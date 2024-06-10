use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    agent::{create_agent, read_agent, read_all_agents},
    auth::ValidateUserOrAdminTokenLayer,
    check::check_task,
    completion::delete_all_completions,
    health_check::health_checker_handler,
    task::{create_task, delete_task, patch_task, read_all_tasks, read_all_tasks_admin, read_task},
};

use tower_http::validate_request::ValidateRequestHeaderLayer;

pub fn create_router(user_token: &str, admin_token: &str) -> Router {
    let api_route = Router::new().nest(
        "/api",
        create_public_router()
            .merge(create_user_router(&user_token, &admin_token))
            .nest("/admin", create_admin_router(&admin_token)),
    );
    api_route
}

fn create_public_router() -> Router {
    Router::new().route("/health", get(health_checker_handler))
}

fn create_user_router(user_token: &str, admin_token: &str) -> Router {
    Router::new()
        .route("/agent", post(create_agent))
        .route("/agent", get(read_all_agents))
        .route("/agent/:id", get(read_agent))
        .route("/agent/:agentId/task", get(read_all_tasks))
        .route("/agent/:agent_id/task/:task_id", get(read_task))
        .route("/agent/:agent_id/task/:task_id/check", post(check_task))
        .layer(ValidateUserOrAdminTokenLayer::new(user_token, admin_token))
}

fn create_admin_router(user_token: &str) -> Router {
    Router::new()
        .route("/task", get(read_all_tasks_admin))
        .route("/task", post(create_task))
        .route("/task/:task_id", delete(delete_task))
        .route("/task/:task_id", patch(patch_task))
        .route("/completion", delete(delete_all_completions))
        .route_layer(ValidateRequestHeaderLayer::bearer(user_token))
}
