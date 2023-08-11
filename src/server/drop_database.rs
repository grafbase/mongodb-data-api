use super::AppState;
use crate::error::Error;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[allow(unused)]
    data_source: String,
    database: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    dropped_database: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    tracing::info!("{payload:?}");

    state.mongo().database(&payload.database).drop(None).await?;

    let response = Response {
        dropped_database: payload.database,
    };

    tracing::info!("{response:?}");

    Ok(Json(response))
}
