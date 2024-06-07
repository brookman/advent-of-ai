use crate::error::DtoValidationError;

pub trait DtoValidator {
    fn validate(&self) -> Result<(), DtoValidationError>;
}
