use std::path::PathBuf;

use sqlx::{
    query::{Map, Query},
    sqlite::SqliteArguments,
    Sqlite, SqlitePool,
};
use tokio::fs::{self};
use uuid::Uuid;

use crate::error::{CrudError, DtoValidationError};

pub trait CrudModel<T>
where
    T: serde::Serialize,
    T: for<'a> serde::Deserialize<'a>,
{
    fn model_name() -> &'static str;

    async fn create(model: T) -> Result<(Uuid, T), CrudError> {
        let id = Uuid::now_v7();
        let path = Self::get_file_path(id).await?;
        if fs::try_exists(&path).await.map_err(|_| CrudError::IO)? {
            return Err(CrudError::DuplicateId(id));
        }
        let json = serde_json::to_string(&model).map_err(|_| CrudError::Serializiation)?;
        fs::write(path, json).await.map_err(|_| CrudError::IO)?;
        Ok((id, model))
    }

    async fn read(id: Uuid) -> Result<(Uuid, T), CrudError> {
        let path = Self::get_file_path(id).await?;
        if !fs::try_exists(&path).await.map_err(|_| CrudError::IO)? {
            return Err(CrudError::UnknownId(id));
        }
        let json = fs::read_to_string(path).await.map_err(|_| CrudError::IO)?;
        let model: T = serde_json::from_str(&json).map_err(|_| CrudError::Serializiation)?;
        Ok((id, model))
    }

    async fn list() -> Result<Vec<(Uuid, T)>, CrudError> {
        let model_dir: PathBuf = PathBuf::from("./data").join(Self::model_name());
        let mut stream = fs::read_dir(model_dir).await.map_err(|_| CrudError::IO)?;

        let mut ids = Vec::new();

        while let Some(child) = stream.next_entry().await.map_err(|_| CrudError::IO)? {
            if child.metadata().await.map_err(|_| CrudError::IO)?.is_file()
                && child.file_name().to_string_lossy().ends_with(".json")
            {
                // get the stem of the file name without the extension
                let id = Uuid::parse_str(
                    &child
                        .file_name()
                        .to_string_lossy()
                        .replace(".json", "")
                        .to_string(),
                )
                .map_err(|_| CrudError::IO)?;

                let json = fs::read_to_string(child.path())
                    .await
                    .map_err(|_| CrudError::IO)?;
                let model: T =
                    serde_json::from_str(&json).map_err(|_| CrudError::Serializiation)?;

                ids.push((id, model));
            }
        }
        Ok(ids)
    }

    async fn update(id_model: (Uuid, T)) -> Result<(Uuid, T), CrudError> {
        let (id, model) = id_model;
        let path = Self::get_file_path(id).await?;
        if !fs::try_exists(&path).await.map_err(|_| CrudError::IO)? {
            return Err(CrudError::UnknownId(id));
        }
        let json = serde_json::to_string(&model).map_err(|_| CrudError::Serializiation)?;
        fs::write(path, json).await.map_err(|_| CrudError::IO)?;
        Ok((id, model))
    }

    async fn delete(id: Uuid) -> Result<(), CrudError> {
        let path = Self::get_file_path(id).await?;
        if !fs::try_exists(&path).await.map_err(|_| CrudError::IO)? {
            return Err(CrudError::UnknownId(id));
        }
        fs::remove_file(path).await.map_err(|_| CrudError::IO)?;
        Ok(())
    }

    async fn get_file_path(id: Uuid) -> Result<PathBuf, CrudError> {
        let model_dir = PathBuf::from("./data").join(Self::model_name());
        fs::create_dir_all(&model_dir)
            .await
            .map_err(|_| CrudError::IO)?;
        Ok(model_dir.join(format!("{}.json", id)))
    }
}

pub trait DtoValidator {
    fn validate(&self) -> Result<(), DtoValidationError>;
}
