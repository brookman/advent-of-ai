use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, DtoValidationError},
    traits::{CrudModel, DtoValidator},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentModel {
    pub token: Uuid,
    pub name: String,
    pub used_models_and_apis: Vec<String>,
}

impl CrudModel<AgentModel> for AgentModel {
    fn model_name() -> &'static str {
        "agent"
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDto {
    pub id: Uuid,
    pub name: String,
    pub usedModelsAndApis: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreateDto {
    pub name: String,
    pub usedModelsAndApis: Vec<String>,
}

impl DtoValidator for AgentCreateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.name.len() > 64 {
            return Err(DtoValidationError("name too long (must be <=64)".into()));
        }
        if self.usedModelsAndApis.len() > 64 {
            return Err(DtoValidationError(
                "sedModelsAndApis too long (must be <=64)".into(),
            ));
        }
        if self.usedModelsAndApis.iter().any(|m| m.len() > 64) {
            return Err(DtoValidationError(
                "entrty in sedModelsAndApis too long (each must be <=64)".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreatedDto {
    pub id: Uuid,
    pub token: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDeleteDto {
    pub token: Uuid,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentUpdateDto {
    pub token: Uuid,
    pub name: Option<String>,
    pub usedModelsAndApis: Option<Vec<String>>,
}

impl DtoValidator for AgentUpdateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if let Some(name) = &self.name {
            if name.len() > 64 {
                return Err(DtoValidationError("name too long (must be <=64)".into()));
            }
        }
        if let Some(used_models_and_apis) = &self.usedModelsAndApis {
            if used_models_and_apis.len() > 64 {
                return Err(DtoValidationError(
                    "sedModelsAndApis too long (must be <=64)".into(),
                ));
            }
            if used_models_and_apis.iter().any(|m| m.len() > 64) {
                return Err(DtoValidationError(
                    "entrty in sedModelsAndApis too long (each must be <=64)".into(),
                ));
            }
        }
        Ok(())
    }
}

pub async fn create_agent(
    Json(dto): Json<AgentCreateDto>,
) -> Result<Json<AgentCreatedDto>, AppError> {
    dto.validate()?;

    let token = Uuid::now_v7();
    let model = AgentModel {
        token: token.clone(),
        name: dto.name,
        used_models_and_apis: dto.usedModelsAndApis,
    };

    let (id, _) = AgentModel::create(model).await?;

    Ok(Json(AgentCreatedDto { id, token: token }))
}

pub async fn read_agent(Path(id): Path<Uuid>) -> Result<Json<AgentDto>, AppError> {
    let (id, model) = AgentModel::read(id).await?;

    Ok(Json(AgentDto {
        id,
        name: model.name,
        usedModelsAndApis: model.used_models_and_apis,
    }))
}

pub async fn delete_agent(
    Path(id): Path<Uuid>,
    Json(dto): Json<AgentDeleteDto>,
) -> Result<(), AppError> {
    let (id, model) = AgentModel::read(id).await?;

    if dto.token != model.token {
        return Err(AppError::Unauthorized);
    }

    AgentModel::delete(id).await?;

    Ok(())
}

pub async fn update_agent(
    Path(id): Path<Uuid>,
    Json(dto): Json<AgentUpdateDto>,
) -> Result<Json<AgentDto>, AppError> {
    dto.validate()?;

    let (id, mut model) = AgentModel::read(id).await?;

    if dto.token != model.token {
        return Err(AppError::Unauthorized);
    }

    model.name = dto.name.unwrap_or(model.name);
    model.used_models_and_apis = dto.usedModelsAndApis.unwrap_or(model.used_models_and_apis);

    let (id, model) = AgentModel::update((id, model)).await?;

    Ok(Json(AgentDto {
        id,
        name: model.name,
        usedModelsAndApis: model.used_models_and_apis,
    }))
}
