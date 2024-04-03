use validator::ValidationErrors;

pub trait TransformValidationErrors {
    fn new() -> Self;
    fn transform_errors(&self, errors: ValidationErrors) -> Vec<String>;
}
