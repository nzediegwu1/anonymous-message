use crate::core::{
    models::{ApiError, ErrorResponse},
    traits::TransformValidationErrors,
};
use std::vec;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};
use regex::Regex;
use validator::Validate;

pub struct ValidatedBody<J, TVE>(pub J, pub TVE);

#[async_trait]
impl<S, T, TVE> FromRequest<S> for ValidatedBody<T, TVE>
where
    S: Send + Sync,
    T: Validate + 'static,
    TVE: TransformValidationErrors + Send,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let transform_validation_errors: TVE = TVE::new();
        let value_result = Json::<T>::from_request(req, _state).await;

        match value_result {
            Ok(value) => {
                let validation_result = value.validate();
                if validation_result.is_err() {
                    let errors = validation_result.unwrap_err();
                    let error_messages = transform_validation_errors.transform_errors(errors);

                    let error = ApiError::BadRequest {
                        errors: error_messages,
                    };
                    return Err(error.into_response());
                }
                Ok(Self(value.0, transform_validation_errors))
            }
            Err(rejection) => {
                let (status, message) = (rejection.status(), rejection.body_text());

                let pattern = r": (.*?) at line";
                let regex = Regex::new(pattern).unwrap();
                let result: String = match regex.captures(&message) {
                    Some(value) => value.get(1).unwrap().as_str().to_string(),
                    None => message,
                };
                let payload = ErrorResponse::new(vec![result.to_string()], status);

                Err((status, Json(payload)).into_response())
            }
        }
    }
}
