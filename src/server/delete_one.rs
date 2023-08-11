use std::sync::Arc;

use axum::{extract::State, Json};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::AppState;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[allow(unused)]
    data_source: String,
    database: String,
    collection: String,
    filter: Document,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    deleted_count: u64,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    tracing::info!("{payload:?}");

    let result = state
        .mongo()
        .database(&payload.database)
        .collection::<Document>(&payload.collection)
        .delete_one(payload.filter, None)
        .await?;

    let response = Response {
        deleted_count: result.deleted_count,
    };

    tracing::info!("{response:?}");

    Ok(Json(response))
}
