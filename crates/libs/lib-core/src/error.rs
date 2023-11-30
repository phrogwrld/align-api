use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("{0}")]
	Unauthorized(String),
	#[error("{0}")]
	BadRequest(String),
	#[error("{0}")]
	Forbidden(String),
	#[error("{0}")]
	NotFound(String),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let (status_code, error_message) = match self {
			Error::Unauthorized(err) => (StatusCode::UNAUTHORIZED, err),
			Error::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
			Error::Forbidden(err) => (StatusCode::FORBIDDEN, err),
			Error::NotFound(err) => (StatusCode::NOT_FOUND, err),
			_ => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
		};

		let body = Json(json!({
		  "error": error_message,
		}));

		(status_code, body).into_response()
	}
}
