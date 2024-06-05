use std::fs;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    agent::{create_agent, delete_agent, read_agent, update_agent},
    check::check_task,
    handler::health_checker_handler,
    model,
    task::{create_task, read_task, read_tasks},
};

use tower_http::validate_request::ValidateRequestHeaderLayer;

pub fn create_router(bearer_token: String) -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/agent", post(create_agent))
        .route("/api/agent/:id", get(read_agent))
        .route("/api/agent/:id", patch(update_agent))
        .route("/api/agent/:id", delete(delete_agent))
        .route("/api/task", post(create_task))
        .route("/api/task", get(read_tasks))
        .route("/api/task/:id", get(read_task))
        .route("/api/task/:id/check", post(check_task))
        .route_layer(ValidateRequestHeaderLayer::bearer(&bearer_token))

    // .route(
    //     "/api/todos",
    //     post(create_todo_handler).get(todos_list_handler),
    // )
    // .route(
    //     "/api/todos/:id",
    //     get(get_todo_handler)
    //         .patch(edit_todo_handler)
    //         .delete(delete_todo_handler),
    // )
    // .route(
    //     "/api/ranking",
    //     get(get_ranking_handler)
    // )
    // .with_state(db)
}
