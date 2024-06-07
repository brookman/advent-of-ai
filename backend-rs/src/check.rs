use axum::{extract::{Path, Query}, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    agent::AgentInDb, error::{AppError, DtoValidationError}, task::{AgentToken, TaskInDb}, traits::{CrudModel, DtoValidator}
};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTaskRequestDto {
    pub solution: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckTaskResponseDto {
    pub correct: bool,
}

impl DtoValidator for CheckTaskRequestDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.solution.len() > 32768 {
            return Err(DtoValidationError(
                "solution too long (must be <=32768)".into(),
            ));
        }
        Ok(())
    }
}

pub async fn check_task(
    Extension(pool): Extension<SqlitePool>,
    Path((agent_id, task_id)): Path<(Uuid,Uuid)>,
    token: Query<AgentToken>,
    Json(dto): Json<CheckTaskRequestDto>,
) -> Result<Json<CheckTaskResponseDto>, AppError> {
    let agent = AgentInDb::read(&pool, agent_id).await?;
    if agent.token != token.token {
        return Err(AppError::Unauthorized);
    }
    dto.validate()?;
    let task = TaskInDb::read(&pool, task_id).await?;
    let correct = task.solution == dto.solution;
    Ok(Json(CheckTaskResponseDto { correct }))
}
