use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
	Database(sqlx::Error),
	Validation(String),
	NotFound(String),
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
	#[schema(example = "internal_server_error")]
	pub code: &'static str,
	#[schema(example = "database error: connection refused")]
	pub message: String,
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		match self {
			Self::Validation(message) => (
				StatusCode::BAD_REQUEST,
				Json(ErrorResponse {
					code: "validation_error",
					message,
				}),
			)
				.into_response(),
			Self::NotFound(message) => (
				StatusCode::NOT_FOUND,
				Json(ErrorResponse {
					code: "not_found",
					message,
				}),
			)
				.into_response(),
			Self::Database(error) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ErrorResponse {
					code: "internal_server_error",
					message: format!("database error: {error}"),
				}),
			)
				.into_response(),
		}
	}
}

impl From<sqlx::Error> for AppError {
	fn from(value: sqlx::Error) -> Self {
		match value {
			sqlx::Error::RowNotFound => Self::NotFound("resource not found".to_string()),
			other => Self::Database(other),
		}
	}
}
