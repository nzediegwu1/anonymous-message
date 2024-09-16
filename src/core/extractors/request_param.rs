use axum::{
    async_trait,
    body::Body,
    extract::{self, rejection::PathRejection, FromRequestParts},
    http::{request::Parts, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;

use crate::core::models::ErrorResponse;

pub struct CustomPath<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for CustomPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Response<Body>;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params = extract::Path::<T>::from_request_parts(parts, _state).await;
        match params {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (code, error_response) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let status = StatusCode::BAD_REQUEST;

                        let kind = inner.body_text();
                        (status, ErrorResponse::new(vec![kind.to_string()], status))
                    }
                    PathRejection::MissingPathParams(error) => {
                        let code = StatusCode::INTERNAL_SERVER_ERROR;
                        (code, ErrorResponse::new(vec![error.to_string()], code))
                    }
                    _ => {
                        let code = StatusCode::INTERNAL_SERVER_ERROR;
                        (
                            code,
                            ErrorResponse::new(
                                vec![format!("Unhandled path rejection: {rejection}")],
                                code,
                            ),
                        )
                    }
                };
                Err((code, Json(error_response)).into_response())
            }
        }
    }
}
