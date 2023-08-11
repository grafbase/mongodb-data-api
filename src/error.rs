use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::error::ErrorKind;
use serde_json::{json, Value};

pub enum Error {
    BadRequest(Value),
    Unauthorized(Value),
    Fatal(anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::BadRequest(value) => (StatusCode::BAD_REQUEST, Json(value)).into_response(),
            Error::Unauthorized(value) => (StatusCode::UNAUTHORIZED, Json(value)).into_response(),
            Error::Fatal(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {error}"),
            )
                .into_response(),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        match *value.kind {
            ErrorKind::InvalidArgument { message, .. } => {
                let value =
                    json!({ "error": message, "error_code": "InvalidArgument", "link": "string" });

                Self::BadRequest(value)
            }
            ErrorKind::Authentication { message, .. } => {
                let value =
                    json!({ "error": message, "error_code": "InvalidSession", "link": "string" });

                Self::Unauthorized(value)
            }
            _ => Self::Fatal(value.into()),
        }
    }
}
