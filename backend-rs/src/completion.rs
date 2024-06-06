use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct CompletionInDb {
    pub id: Uuid,
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub completion_time: Option<DateTime<Utc>>,
    pub best_time_in_ms: Option<i64>,
}

impl CompletionInDb {
    pub fn new(task_id: Uuid, agent_id: Uuid) -> Self {
        let id = Uuid::now_v7();
        let start_time = Utc::now();

        Self {
            id,
            task_id,
            agent_id,
            start_time,
            completion_time: None,
            best_time_in_ms: None,
        }
    }

    pub fn complete(&mut self) {
        let completion_time = Utc::now();
        let duration = (completion_time - self.start_time).num_milliseconds();

        let best_time_in_ms = self
            .best_time_in_ms
            .map(|best_time_in_ms| duration.min(best_time_in_ms))
            .unwrap_or_else(|| duration);

        self.completion_time = Some(completion_time);
        self.best_time_in_ms = Some(best_time_in_ms);
    }

    async fn create(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(
            r#"INSERT INTO completion (id, task_id, agent_id, start_time, completion_time, best_time_in_ms) VALUES (?, ?, ?, ?, ?, ?);"#,
            self.id,
            self.task_id,
            self.agent_id,
            self.start_time,
            self.completion_time,
            self.best_time_in_ms,
        )
        .execute(conn.as_mut())
        .await?;

        Ok(())
    }

    async fn read(pool: &SqlitePool, id: Uuid) -> anyhow::Result<Self> {
        let mut conn = pool.acquire().await?;
        let model = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", task_id as "task_id: uuid::Uuid", agent_id as "agent_id: uuid::Uuid", start_time as "start_time: chrono::DateTime<Utc>", completion_time as "completion_time: chrono::DateTime<Utc>", best_time_in_ms FROM completion WHERE id = ?;"#,
            id,
        )
        .fetch_one(conn.as_mut())
        .await ?;
        Ok(model)
    }

    async fn read_by(
        pool: &SqlitePool,
        task_id: Uuid,
        agent_id: Uuid,
    ) -> anyhow::Result<Option<Self>> {
        let mut conn = pool.acquire().await?;
        let model = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", task_id as "task_id: uuid::Uuid", agent_id as "agent_id: uuid::Uuid", start_time as "start_time: chrono::DateTime<Utc>", completion_time as "completion_time: chrono::DateTime<Utc>", best_time_in_ms FROM completion WHERE task_id = ? AND agent_id = ?;"#,
            task_id,
            agent_id,
        )
        .fetch_optional(conn.as_mut())
        .await?;
        Ok(model)
    }

    async fn read_all(pool: &SqlitePool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.acquire().await?;
        let models = sqlx::query_as!(
            Self,
            r#"SELECT id as "id: uuid::Uuid", task_id as "task_id: uuid::Uuid", agent_id as "agent_id: uuid::Uuid", start_time as "start_time: chrono::DateTime<Utc>", completion_time as "completion_time: chrono::DateTime<Utc>", best_time_in_ms FROM completion;"#,
        )
        .fetch_all(conn.as_mut())
        .await?;
        Ok(models)
    }

    async fn update(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(
            r#"UPDATE completion SET task_id = ?, agent_id = ?, start_time = ?, completion_time = ?, best_time_in_ms = ? WHERE id = ?;"#,
            self.task_id,
            self.agent_id,
            self.start_time,
            self.completion_time,
            self.best_time_in_ms,
            self.id,
        )
        .execute(conn.as_mut())
        .await?;

        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query!(r#"DELETE FROM completion WHERE id = ?;"#, self.id,)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
}
