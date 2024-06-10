use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{
    error::{AppError, DtoValidationError},
    traits::DtoValidator,
};

#[derive(FromRow, Debug)]
pub struct AgentInDb {
    pub id: Uuid,
    pub token: Uuid,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDto {
    pub id: Uuid,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreateDto {
    pub name: String,
}

impl DtoValidator for AgentCreateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.name.len() > 64 {
            return Err(DtoValidationError("name too long (must be <=64)".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreatedDto {
    pub id: Uuid,
    pub token: Uuid,
}

pub async fn create_agent(
    Extension(pool): Extension<SqlitePool>,
    Json(dto): Json<AgentCreateDto>,
) -> Result<Json<AgentCreatedDto>, AppError> {
    dto.validate()?;

    let model = AgentInDb::new(dto.name);
    model.create(&pool).await?;

    Ok(Json(AgentCreatedDto {
        id: model.id,
        token: model.token,
    }))
}

pub async fn read_agent(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<AgentDto>, AppError> {
    let model = AgentInDb::read(&pool, id).await?;

    Ok(Json(AgentDto {
        id,
        name: model.name,
    }))
}

pub async fn read_all_agents(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<AgentDto>>, AppError> {
    let model = AgentInDb::read_all(&pool).await?;
    let result = model
        .iter()
        .map(|dto| AgentDto {
            id: dto.id,
            name: dto.name.clone(),
        })
        .collect();
    Ok(Json(result))
}

impl AgentInDb {
    pub fn new(name: String) -> Self {
        let id = Uuid::now_v7();
        let token = Uuid::now_v7();
        Self { id, token, name }
    }

    pub async fn create(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(
            r#"INSERT INTO agent (id, token, name) VALUES (?, ?, ?);"#,
            self.id,
            self.token,
            self.name,
        )
        .execute(conn.as_mut())
        .await?;

        Ok(())
    }

    pub async fn read(pool: &SqlitePool, id: Uuid) -> anyhow::Result<Self> {
        let mut conn = pool.acquire().await?;
        let model = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", token as "token: uuid::Uuid", name FROM agent WHERE id = ?;"#,
            id,
        )
        .fetch_one(conn.as_mut())
        .await ?;
        Ok(model)
    }

    // pub async fn read_by_token(pool: &SqlitePool, id: Uuid, token: Uuid) -> anyhow::Result<Self> {
    //     let mut conn = pool.acquire().await?;
    //     let model = sqlx::query_as!(
    //         Self,
    //         r#"SELECT id as "id: uuid::Uuid", token as "token: uuid::Uuid", name FROM agent WHERE id = ? AND token = ?;"#,
    //         id,
    //         token,
    //     )
    //     .fetch_one(conn.as_mut())
    //     .await ?;
    //     Ok(model)
    // }

    pub async fn read_all(pool: &SqlitePool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.acquire().await?;
        let models = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", token as "token: uuid::Uuid", name FROM agent;"#,
        )
        .fetch_all(conn.as_mut())
        .await?;
        Ok(models)
    }

    // pub async fn update(&self, pool: &SqlitePool) -> anyhow::Result<()> {
    //     let mut conn = pool.acquire().await?;
    //     sqlx::query!(
    //         r#"UPDATE agent SET token = ?, name = ? WHERE id = ?;"#,
    //         self.token,
    //         self.name,
    //         self.id,
    //     )
    //     .execute(conn.as_mut())
    //     .await?;

    //     Ok(())
    // }

    // pub async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
    //     let mut conn = pool.acquire().await?;
    //     sqlx::query!(r#"DELETE FROM agent WHERE id = ?;"#, self.id,)
    //         .execute(conn.as_mut())
    //         .await?;

    //     Ok(())
    // }
}
