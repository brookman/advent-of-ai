use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, DtoValidationError},
    traits::{CrudModel, DtoValidator},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskModel {
    pub name: String,
    pub task_type: TaskType,
}

impl CrudModel<TaskModel> for TaskModel {
    fn model_name() -> &'static str {
        "task"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    SimpleTask {
        description: String,
        solution: String,
    },
    AdventOfCodePartOne {
        description: String,
        input: String,
        solution: String,
    },
    AdventOfCodePartTwo {
        description: String,
        input: String,
        solution: String,
    },
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreateDto {
    pub name: String,
    pub taskType: TaskTypeCreateDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskTypeCreateDto {
    SimpleTask {
        description: String,
        solution: String,
    },
    AdventOfCodePartOne {
        description: String,
        input: String,
        solution: String,
    },
    AdventOfCodePartTwo {
        description: String,
        input: String,
        solution: String,
    },
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskReadDto {
    pub id: Uuid,
    pub name: String,
    pub taskType: TaskTypeReadDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskTypeReadDto {
    SimpleTask { description: String },
    AdventOfCodePartOne { description: String, input: String },
    AdventOfCodePartTwo { description: String, input: String },
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TasksReadDto {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}

impl DtoValidator for TaskCreateDto {
    fn validate(&self) -> Result<(), DtoValidationError> {
        if self.name.len() > 64 {
            return Err(DtoValidationError("name too long (must be <=64)".into()));
        }

        match &self.taskType {
            TaskTypeCreateDto::SimpleTask {
                description,
                solution,
            } => {
                if description.len() > 4096 {
                    return Err(DtoValidationError(
                        "description too long (must be <=4096)".into(),
                    ));
                }
                if solution.len() > 4096 {
                    return Err(DtoValidationError(
                        "solution too long (must be <=4096)".into(),
                    ));
                }
            }
            TaskTypeCreateDto::AdventOfCodePartOne {
                description,
                input,
                solution,
            } => {
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
                if solution.len() > 4096 {
                    return Err(DtoValidationError(
                        "solution too long (must be <=4096)".into(),
                    ));
                }
            }
            TaskTypeCreateDto::AdventOfCodePartTwo {
                description,
                input,
                solution,
            } => {
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
                if solution.len() > 4096 {
                    return Err(DtoValidationError(
                        "solution too long (must be <=4096)".into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AgentCreatedDto {
//     pub id: Uuid,
//     pub token: Uuid,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AgentDeleteDto {
//     pub token: Uuid,
// }

// #[allow(non_snake_case)]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct AgentUpdateDto {
//     pub token: Uuid,
//     pub name: Option<String>,
//     pub usedModelsAndApis: Option<Vec<String>>,
// }

// impl DtoValidator for AgentUpdateDto {
//     fn validate(&self) -> Result<(), DtoValidationError> {
//         if let Some(name) = &self.name {
//             if name.len() > 64 {
//                 return Err(DtoValidationError("name too long (must be <=64)".into()));
//             }
//         }
//         if let Some(used_models_and_apis) = &self.usedModelsAndApis {
//             if used_models_and_apis.len() > 64 {
//                 return Err(DtoValidationError(
//                     "sedModelsAndApis too long (must be <=64)".into(),
//                 ));
//             }
//             if used_models_and_apis.iter().any(|m| m.len() > 64) {
//                 return Err(DtoValidationError(
//                     "entrty in sedModelsAndApis too long (each must be <=64)".into(),
//                 ));
//             }
//         }
//         Ok(())
//     }
// }

pub async fn create_task(Json(dto): Json<TaskCreateDto>) -> Result<Json<Uuid>, AppError> {
    dto.validate()?;

    let model = TaskModel {
        name: dto.name,
        task_type: match dto.taskType {
            TaskTypeCreateDto::SimpleTask {
                description,
                solution,
            } => TaskType::SimpleTask {
                description,
                solution,
            },
            TaskTypeCreateDto::AdventOfCodePartOne {
                description,
                input,
                solution,
            } => TaskType::AdventOfCodePartOne {
                description,
                input,
                solution,
            },
            TaskTypeCreateDto::AdventOfCodePartTwo {
                description,
                input,
                solution,
            } => TaskType::AdventOfCodePartTwo {
                description,
                input,
                solution,
            },
        },
    };

    let (id, _) = TaskModel::create(model).await?;

    Ok(Json(id))
}

pub async fn read_task(Path(id): Path<Uuid>) -> Result<Json<TaskReadDto>, AppError> {
    let (id, model) = TaskModel::read(id).await?;

    Ok(Json(TaskReadDto {
        id,
        name: model.name,
        taskType: match model.task_type {
            TaskType::SimpleTask {
                description,
                solution: _,
            } => TaskTypeReadDto::SimpleTask { description },
            TaskType::AdventOfCodePartOne {
                description,
                input,
                solution: _,
            } => TaskTypeReadDto::AdventOfCodePartOne { description, input },
            TaskType::AdventOfCodePartTwo {
                description,
                input,
                solution: _,
            } => TaskTypeReadDto::AdventOfCodePartTwo { description, input },
        },
    }))
}

pub async fn read_tasks() -> Result<Json<Vec<TasksReadDto>>, AppError> {
    let tasks = TaskModel::list().await?;

    let mut dtos = tasks
        .iter()
        .map(|(id, model)| TasksReadDto {
            id: *id,
            name: model.name.clone(),
            completed: false,
        })
        .collect::<Vec<_>>();

    dtos.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(Json(dtos))
}

// pub async fn delete_agent(
//     Path(id): Path<Uuid>,
//     Json(dto): Json<AgentDeleteDto>,
// ) -> Result<(), AppError> {
//     let (id, model) = AgentModel::read(id).await?;

//     if dto.token != model.token {
//         return Err(AppError::Unauthorized);
//     }

//     AgentModel::delete(id).await?;

//     Ok(())
// }

// pub async fn update_agent(
//     Path(id): Path<Uuid>,
//     Json(dto): Json<AgentUpdateDto>,
// ) -> Result<Json<AgentDto>, AppError> {
//     dto.validate()?;

//     let (id, mut model) = AgentModel::read(id).await?;

//     if dto.token != model.token {
//         return Err(AppError::Unauthorized);
//     }

//     model.name = dto.name.unwrap_or(model.name);
//     model.used_models_and_apis = dto.usedModelsAndApis.unwrap_or(model.used_models_and_apis);

//     let (id, model) = AgentModel::update((id, model)).await?;

//     Ok(Json(AgentDto {
//         id,
//         name: model.name,
//         usedModelsAndApis: model.used_models_and_apis,
//     }))
// }
