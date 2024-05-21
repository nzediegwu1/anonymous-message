use crate::core::{error_transformer::error_transformer, traits::TransformValidationErrors};
use validator::ValidationErrors;

pub struct LoginValidationError;
impl TransformValidationErrors for LoginValidationError {
    fn new() -> Self {
        LoginValidationError
    }
    fn transform_errors(&self, errors: ValidationErrors) -> Vec<String> {
        error_transformer(errors)
    }
}
