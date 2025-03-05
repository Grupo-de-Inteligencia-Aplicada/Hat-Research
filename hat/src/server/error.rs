use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: StatusCode,
    pub errors: Vec<Cow<'static, str>>,
}

impl ApiError {
    pub fn new<S: Into<Cow<'static, str>>>(code: StatusCode, description: S) -> Self {
        Self {
            code,
            errors: vec![description.into()],
        }
    }
    pub fn bad_request<S: Into<Cow<'static, str>>>(description: S) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            errors: vec![description.into()],
        }
    }
    pub fn internal_error<S: Into<Cow<'static, str>>>(description: S) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: vec![description.into()],
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let mut errors: Vec<serde_json::Value> = Vec::with_capacity(self.errors.len());
        for error in self.errors {
            errors.push(json!({
                "description": error
            }));
        }
        (
            self.code,
            Json(json!({
                "errors": errors,
            })),
        )
            .into_response()
    }
}

pub trait MapErrToApi<T> {
    fn map_err_to_api(self, code: StatusCode) -> Result<T, ApiError>;
}

impl<T, E: Display> MapErrToApi<T> for Result<T, E> {
    fn map_err_to_api(self, code: StatusCode) -> Result<T, ApiError> {
        self.map_err(|e| ApiError {
            code,
            errors: vec![format!("{e}").into()],
        })
    }
}

pub trait RaiseInternalError<T> {
    fn raise_internal_error(self, user_message: Option<&str>) -> Result<T, ApiError>;
}

impl<T, E: Debug> RaiseInternalError<T> for Result<T, E> {
    fn raise_internal_error(self, user_message: Option<&str>) -> Result<T, ApiError> {
        self.map_err(|internal| {
            error!("Internal error: {internal:?}");
            match user_message {
                Some(m) => ApiError::internal_error(m.to_owned()),
                None => ApiError::internal_error("Falha ao processar requisição"),
            }
        })
    }
}
