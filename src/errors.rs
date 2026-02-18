use std::fmt::Debug;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError {
    message: String,
    status: StatusCode,
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError {
            message: msg.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError {
            status: StatusCode::BAD_REQUEST,
            message: msg.into(),
        }
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        AppError {
            status: StatusCode::FORBIDDEN,
            message: msg.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError {
            status: StatusCode::NOT_FOUND,
            message: msg.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}
