use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::{header, request::Parts, Response, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::core::{
    models::{Claims, ErrorResponse},
    utils::decode_jwt,
};

pub struct Authorized<Claims>(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Authorized<Claims>
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(header::AUTHORIZATION);
        match auth_header {
            Some(value) => {
                let bearer_token_result = value.to_str();

                if let Ok(bearer_token) = bearer_token_result {
                    let scheme = bearer_token.splitn(2, " ").next().unwrap_or_default();
                    if scheme != "Bearer" {
                        let status = StatusCode::UNAUTHORIZED;
                        let payload = ErrorResponse::new(
                            vec!["Invalid authorization scheme".to_string()],
                            status,
                        );
                        return Err((status, Json(payload)).into_response());
                    }
                    let token = bearer_token
                        .splitn(2, " ")
                        .skip(1)
                        .next()
                        .unwrap_or_default();

                    let decoded = decode_jwt(token);

                    match decoded {
                        Ok(claims) => Ok(Authorized(claims)),
                        Err(e) => {
                            let status = StatusCode::UNAUTHORIZED;
                            let payload = ErrorResponse::new(vec![e.to_string()], status);

                            Err((status, Json(payload)).into_response())
                        }
                    }
                } else {
                    let status = StatusCode::INTERNAL_SERVER_ERROR;
                    let message = "Error converting authorization value to string".to_string();
                    let payload = ErrorResponse::new(vec![message], status);
                    Err((status, Json(payload)).into_response())
                }
            }
            None => {
                let status = StatusCode::UNAUTHORIZED;

                let payload =
                    ErrorResponse::new(vec!["Missing authorization header".to_string()], status);
                Err((status, Json(payload)).into_response())
            }
        }
    }
}
