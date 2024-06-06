// use axum::{extract::Path, Json};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// use crate::{
//     error::{AppError, DtoValidationError},
//     traits::{CrudModel, DtoValidator},
// };

// #[allow(non_snake_case)]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CheckTaskRequestDto {
//     pub solution: String,
// }

// #[allow(non_snake_case)]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CheckTaskResponseDto {
//     pub correct: bool,
// }

// impl DtoValidator for CheckTaskRequestDto {
//     fn validate(&self) -> Result<(), DtoValidationError> {
//         if self.solution.len() > 32768 {
//             return Err(DtoValidationError(
//                 "solution too long (must be <=32768)".into(),
//             ));
//         }
//         Ok(())
//     }
// }

// pub async fn check_task(
//     Path(id): Path<Uuid>,
//     Json(dto): Json<CheckTaskRequestDto>,
// ) -> Result<Json<CheckTaskResponseDto>, AppError> {
//     dto.validate()?;

//     let (_, model) = TaskModel::read(id).await?;

//     let correct = match model.task_type {
//         TaskType::SimpleTask {
//             description: _,
//             solution,
//         } => solution == dto.solution,
//         TaskType::AdventOfCodePartOne {
//             description: _,
//             input: _,
//             solution,
//         } => solution == dto.solution,
//         TaskType::AdventOfCodePartTwo {
//             description: _,
//             input: _,
//             solution,
//         } => solution == dto.solution,
//     };

//     Ok(Json(CheckTaskResponseDto { correct }))
// }
