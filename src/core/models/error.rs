use axum::{
    body::Body,
    http::{header::WWW_AUTHENTICATE, HeaderMap, HeaderValue, StatusCode},
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
                _ => "Internal Server Error".to_string(),
            },
        }
    }
}
#[derive(Debug)]
pub struct ErrorMessage {
    message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("authentication required")]
    Unauthorized(ErrorMessage),

    #[error("user may not perform this action")]
    Forbidden,

    #[error("request path or resource not found")]
    NotFound(ErrorMessage),

    #[error("error in the request body")]
    BadRequest { errors: Vec<String> },

    #[error("an error occurred with the database")]
    Database(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    InternalServer(#[from] anyhow::Error),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(..) => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound(..) => StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Database(_) | Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            Self::Unauthorized(_) => {
                return (
                    self.status_code(),
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string(),
                )
                    .into_response();
            }

            Self::Database(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                // log::error!("SQLx error: {:?}", e);
            }

            Self::InternalServer(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                // log::error!("Generic error: {:?}", e);
            }

            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
