use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(messages: Vec<String>, code: StatusCode) -> Self {
        ErrorResponse {
            errors: messages,
            message: match code {
                StatusCode::UNAUTHORIZED => "Unauthorized".to_string(),
                StatusCode::FORBIDDEN => "Forbidden".to_string(),
                StatusCode::NOT_FOUND => "Not Found".to_string(),
                StatusCode::BAD_REQUEST => "Bad Request".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY => "Unprocessable Entity".to_string(),
                StatusCode::CONFLICT => "Conflict".to_string(),
                _ => "Internal Server Error".to_string(),
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("user may not perform this action")]
    Forbidden,

    #[error("request path or resource not found")]
    NotFound(String),

    #[error("error in the request body")]
    BadRequest { errors: Vec<String> },

    #[error("an error occurred with the database")]
    Database(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    InternalServer(String),

    #[error("a conflict occurred, eg: data already exists")]
    Conflict(String),

    #[error("invalid password or authorization token")]
    Unauthorized(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound(..) => StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Database(_) | Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        match &self {
            Self::BadRequest { errors } => {
                let code = self.status_code();
                return (code, Json(ErrorResponse::new(errors.to_vec(), code))).into_response();
            }
            Self::NotFound(ref error) => {
                let code = self.status_code();
                return (
                    code,
                    Json(ErrorResponse::new(vec![error.to_string()], code)),
                )
                    .into_response();
            }
            Self::Unauthorized(ref error) => {
                let code = self.status_code();
                return (
                    code,
                    Json(ErrorResponse::new(vec![error.to_string()], code)),
                )
                    .into_response();
            }
            Self::Database(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                // log::error!("SQLx error: {:?}", e);
                let code = self.status_code();
                return (code, Json(ErrorResponse::new(vec![e.to_string()], code))).into_response();
            }

            Self::InternalServer(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                // log::error!("Generic error: {:?}", e);
                let code = self.status_code();
                return (code, Json(ErrorResponse::new(vec![e.to_string()], code))).into_response();
            }

            Self::Conflict(ref e) => {
                let code = self.status_code();
                return (code, Json(ErrorResponse::new(vec![e.to_string()], code))).into_response();
            }

            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
