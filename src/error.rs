use axum::{http::StatusCode, response::IntoResponse};

pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        );

        response.into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
