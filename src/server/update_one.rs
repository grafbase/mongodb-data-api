use std::sync::Arc;

use axum::{extract::State, Json};
use mongodb::{bson::Document, options::UpdateOptions};
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
    update: Document,
    upsert: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    matched_count: u64,
    modified_count: u64,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    tracing::info!("{payload:?}");

    let options = UpdateOptions::builder().upsert(payload.upsert).build();

    let result = state
        .mongo()
        .database(&payload.database)
        .collection::<Document>(&payload.collection)
        .update_one(payload.filter, payload.update, Some(options))
        .await?;

    let response = Response {
        matched_count: result.matched_count,
        modified_count: result.modified_count,
    };

    tracing::info!("{response:?}");

    Ok(Json(response))
}
