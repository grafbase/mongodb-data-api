use std::sync::Arc;

use axum::{extract::State, Json};
use mongodb::{bson::Document, options::FindOneOptions};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    projection: Document,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    document: Value,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, Error> {
    let options = FindOneOptions::builder()
        .projection(Some(payload.projection))
        .build();

    let document = state
        .mongo()
        .database(&payload.database)
        .collection::<Value>(&payload.collection)
        .find_one(payload.filter, Some(options))
        .await?
        .unwrap_or(Value::Null);

    Ok(Json(Response { document }))
}
