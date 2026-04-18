use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
	Database(sqlx::Error),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
	code: &'static str,
	message: String,
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		match self {
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
		Self::Database(value)
	}
}
