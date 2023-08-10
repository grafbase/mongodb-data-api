use std::sync::Arc;

use axum::{extract::State, Json};
use futures::stream::TryStreamExt;
use mongodb::{bson::Document, options::FindOptions};
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
    filter: Document,
    projection: Document,
    sort: Option<Document>,
    limit: Option<i64>,
    skip: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    documents: Value,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    tracing::info!("{payload:?}");

    let options = FindOptions::builder()
        .projection(Some(payload.projection))
        .sort(payload.sort)
        .limit(payload.limit)
        .skip(payload.skip)
        .build();

    let mut documents = Vec::new();

    let mut cursor = state
        .mongo()
        .database(&payload.database)
        .collection::<Document>(&payload.collection)
        .find(payload.filter, Some(options))
        .await?;

    while let Some(document) = cursor.try_next().await? {
        documents.push(Value::Object(normalize::document_to_json(document)));
    }

    let response = Response {
        documents: Value::Array(documents),
    };

    tracing::info!("{response:?}");

    Ok(Json(response))
}
