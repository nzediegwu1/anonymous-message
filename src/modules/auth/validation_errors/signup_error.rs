use crate::core::traits::TransformValidationErrors;
use std::borrow::Cow;
use validator::ValidationErrors;

pub struct SignupValidationError;
impl TransformValidationErrors for SignupValidationError {
    fn new() -> Self {
        SignupValidationError
    }
    fn transform_errors(&self, errors: ValidationErrors) -> Vec<String> {
        let mut messages = Vec::new();

        for (field, field_errors) in errors.field_errors() {
            for error in field_errors {
                match error.code {
                    Cow::Borrowed("length") => {
                        let min = error
                            .params
                            .get("min")
                            .and_then(|min| min.as_u64())
                            .unwrap_or(0);

                        messages.push(format!("{}: minimum length is {} characters.", field, min));
                    }
                    Cow::Borrowed("email") => {
                        let email = error
                            .params
                            .get("value")
                            .and_then(|min| Some(min.to_string()))
                            .unwrap();

                        messages.push(format!("{}: {} is not a valid email.", field, email));
                    }
                    _ => {
                        messages.push(format!(
                            "{}: {}",
                            field,
                            error.message.clone().unwrap_or_default()
                        ));
                    }
                }
            }
        }

        messages
    }
}
