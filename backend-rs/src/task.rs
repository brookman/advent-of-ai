use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{
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

impl DtoValidator for TaskCreateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.name.len() > 64 {
            return Err(DtoValidationError("name too long (must be <=64)".into()));
        }
        if self.solution.len() > 4096 {
            return Err(DtoValidationError(
                "solution too long (must be <=4096)".into(),
            ));
        }

        match &self.taskType {
            TaskTypeDto::SimpleTask { description } => {
                if description.len() > 4096 {
                    return Err(DtoValidationError(
                        "description too long (must be <=4096)".into(),
                    ));
                }
            }
            TaskTypeDto::AdventOfCodePartOne { description, input } => {
                if description.len() > 4096 {
                    return Err(DtoValidationError(
                        "description too long (must be <=4096)".into(),
                    ));
                }
                if input.len() > 32768 {
                    return Err(DtoValidationError(
                        "input too long (must be <=32768)".into(),
                    ));
                }
            }
            TaskTypeDto::AdventOfCodePartTwo { description, input } => {
                if description.len() > 4096 {
                    return Err(DtoValidationError(
                        "description too long (must be <=4096)".into(),
                    ));
                }
                if input.len() > 32768 {
                    return Err(DtoValidationError(
                        "input too long (must be <=32768)".into(),
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
    Path(id): Path<Uuid>,
) -> Result<Json<TaskDto>, AppError> {
    let model = TaskInDb::read(&pool, id).await?;

    let task_type = serde_json::from_str(&model.task_json).unwrap();

    Ok(Json(TaskDto {
        name: model.name,
        taskType: task_type,
    }))
}

pub async fn read_all_tasks(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<TasksDto>>, AppError> {
    let models = TaskInDb::read_all(&pool).await?;

    let tasks = models
        .into_iter()
        .map(|model| TasksDto {
            id: model.id,
            name: model.name,
        })
        .collect();

    Ok(Json(tasks))
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

    pub async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(r#"DELETE FROM task WHERE id = ?;"#, self.id,)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
}
