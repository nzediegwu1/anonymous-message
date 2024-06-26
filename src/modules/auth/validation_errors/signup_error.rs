use crate::core::{error_transformer::error_transformer, traits::TransformValidationErrors};
use validator::ValidationErrors;

pub struct SignupValidationError;
impl TransformValidationErrors for SignupValidationError {
    fn new() -> Self {
        SignupValidationError
    }
    fn transform_errors(&self, errors: ValidationErrors) -> Vec<String> {
        error_transformer(errors)
    }
}
