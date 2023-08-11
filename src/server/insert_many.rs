use std::sync::Arc;

use axum::{extract::State, Json};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
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
    documents: Vec<Document>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    inserted_ids: Value,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    tracing::info!("{payload:?}");

    let result = state
        .mongo()
        .database(&payload.database)
        .collection(&payload.collection)
        .insert_many(payload.documents, None)
        .await?;

    let inserted_ids = result
        .inserted_ids
        .into_values()
        .map(normalize::bson_to_json)
        .collect();

    let response = Response {
        inserted_ids: Value::Array(inserted_ids),
    };

    tracing::info!("{response:?}");

    Ok(Json(response))
}
