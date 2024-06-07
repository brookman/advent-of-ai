use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{
    agent::AgentInDb,
    completion::{self, CompletionInDb},
    error::{AppError, DtoValidationError},
    traits::DtoValidator,
};

#[derive(FromRow, Debug)]
pub struct TaskInDb {
    pub id: Uuid,
    pub name: String,
    pub task_json: String,
    pub solution: String,
}

#[derive(FromRow, Debug)]
pub struct TaskWithCompletionInDb {
    pub id: Uuid,
    pub name: String,
    pub task_json: String,
    pub solution: String,
    pub completion_id: Option<Uuid>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDto {
    pub name: String,
    pub taskType: TaskTypeDto,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksDto {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
    pub time: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreateDto {
    pub name: String,
    pub taskType: TaskTypeDto,
    pub solution: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskTypeDto {
    SimpleTask { description: String },
    AdventOfCodePartOne { description: String, input: String },
    AdventOfCodePartTwo { description: String, input: String },
}

#[derive(Deserialize)]
pub struct AgentToken {
    pub token: Uuid,
}

impl DtoValidator for TaskCreateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.name.len() > 64 {
            return Err(DtoValidationError("name too long (must be <=64)".into()));
        }
        if self.solution.len() > 32768 {
            return Err(DtoValidationError(
                "solution too long (must be <=32768)".into(),
            ));
        }

        match &self.taskType {
            TaskTypeDto::SimpleTask { description } => {
                if description.len() > 32768 {
                    return Err(DtoValidationError(
                        "description too long (must be <=32768)".into(),
                    ));
                }
            }
            TaskTypeDto::AdventOfCodePartOne { description, input } => {
                if description.len() > 32768 {
                    return Err(DtoValidationError(
                        "description too long (must be <=32768)".into(),
                    ));
                }
                if input.len() > 65536 {
                    return Err(DtoValidationError(
                        "input too long (must be <=65536)".into(),
                    ));
                }
            }
            TaskTypeDto::AdventOfCodePartTwo { description, input } => {
                if description.len() > 32768 {
                    return Err(DtoValidationError(
                        "description too long (must be <=32768)".into(),
                    ));
                }
                if input.len() > 65536 {
                    return Err(DtoValidationError(
                        "input too long (must be <=65536)".into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

pub async fn create_task(
    Extension(pool): Extension<SqlitePool>,
    Json(dto): Json<TaskCreateDto>,
) -> Result<Json<Uuid>, AppError> {
    dto.validate()?;

    let task_json = serde_json::to_string(&dto.taskType).unwrap();
    let model = TaskInDb::new(dto.name, task_json, dto.solution);
    model.create(&pool).await?;

    Ok(Json(model.id))
}

pub async fn read_task(
    Extension(pool): Extension<SqlitePool>,
    Path((agent_id, task_id)): Path<(Uuid, Uuid)>,
    token: Query<AgentToken>,
) -> Result<Json<TaskDto>, AppError> {
    let agent = AgentInDb::read(&pool, agent_id).await?;
    if agent.token != token.token {
        return Err(AppError::Unauthorized);
    }

    let model = TaskInDb::read(&pool, task_id).await?;
    let task_type = serde_json::from_str(&model.task_json).unwrap();

    if let Some(completion) = &mut CompletionInDb::read_by(&pool, task_id, agent_id).await? {
        completion.start_time = Utc::now();
        completion.completion_time = None;
        completion.update(&pool).await?;
    } else {
        let completion = CompletionInDb::new(task_id, agent_id);
        completion.create(&pool).await?;
    }

    Ok(Json(TaskDto {
        name: model.name,
        taskType: task_type,
    }))
}

pub async fn read_all_tasks(
    Extension(pool): Extension<SqlitePool>,
    Path(agent_id): Path<Uuid>,
    token: Query<AgentToken>,
) -> Result<Json<Vec<TasksDto>>, AppError> {
    let agent = AgentInDb::read(&pool, agent_id).await?;
    if agent.token != token.token {
        return Err(AppError::Unauthorized);
    }

    let models = TaskInDb::read_all_with_completion(&pool, agent_id).await?;
    let mut dtos = vec![];

    for model in models {
        let mut dto = TasksDto {
            id: model.id,
            name: model.name,
            completed: model.completion_id.is_some(),
            time: None,
        };

        if let Some(completion_id) = model.completion_id {
            let completion = completion::CompletionInDb::read(&pool, completion_id).await?;
            dto.time = Some(completion.completion_time.unwrap().timestamp());
        }

        dtos.push(dto);
    }

    Ok(Json(dtos))
}

pub async fn delete_task(
    Extension(pool): Extension<SqlitePool>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Uuid>, AppError> {
    let model = TaskInDb::read(&pool, task_id).await?;
    model.delete(&pool).await?;

    Ok(Json(model.id))
}

impl TaskInDb {
    pub fn new(name: String, task_json: String, solution: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            task_json,
            solution,
        }
    }

    pub async fn create(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(
            r#"INSERT INTO task (id, name, task_json, solution) VALUES (?, ?, ?, ?);"#,
            self.id,
            self.name,
            self.task_json,
            self.solution,
        )
        .execute(conn.as_mut())
        .await?;

        Ok(())
    }

    pub async fn read(pool: &SqlitePool, id: Uuid) -> anyhow::Result<Self> {
        let mut conn = pool.acquire().await?;
        let model = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", name, task_json, solution FROM task WHERE id = ?;"#,
            id,
        )
        .fetch_one(conn.as_mut())
        .await?;
        Ok(model)
    }

    pub async fn read_all(pool: &SqlitePool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.acquire().await?;
        let models = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", name, task_json, solution FROM task;"#,
        )
        .fetch_all(conn.as_mut())
        .await?;
        Ok(models)
    }

    pub async fn read_all_with_completion(
        pool: &SqlitePool,
        agent_id: Uuid,
    ) -> anyhow::Result<Vec<TaskWithCompletionInDb>> {
        let mut conn = pool.acquire().await?;
        let models = sqlx::query_as!(
            TaskWithCompletionInDb,
            r#"SELECT task.id as "id: uuid::Uuid", task.name, task.task_json, task.solution, completion.id as "completion_id: uuid::Uuid" FROM task LEFT JOIN completion ON task.id = completion.task_id AND completion.agent_id = ? AND completion.completion_time NOT NULL;"#,
            agent_id,
        )
        .fetch_all(conn.as_mut())
        .await?;
        Ok(models)
    }

    pub async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(r#"DELETE FROM task WHERE id = ?;"#, self.id)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
}
