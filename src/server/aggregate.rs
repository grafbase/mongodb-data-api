use std::sync::Arc;

use axum::{extract::State, Json};
use futures::TryStreamExt;
use mongodb::bson::Document;
use serde::Deserialize;
use serde_json::Value;

use crate::{error::Error, server::normalize};

use super::AppState;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[allow(unused)]
    data_source: String,
    database: String,
    collection: String,
    pipeline: Vec<Document>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Value>, Error> {
    tracing::info!("{payload:?}");

    let document_count = payload.pipeline.len();

    let mut cursor = state
        .mongo()
        .database(&payload.database)
        .collection::<Document>(&payload.collection)
        .aggregate(payload.pipeline, None)
        .await?;

    let mut response = Vec::with_capacity(document_count);

    while let Some(document) = cursor.try_next().await? {
        response.push(Value::Object(normalize::document_to_json(document)));
    }

    tracing::info!("{response:?}");

    Ok(Json(Value::Array(response)))
}
