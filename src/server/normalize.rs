use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use mongodb::bson::{Bson, Document};
use serde_json::{json, Map, Value};

pub fn document_to_json(document: Document) -> Map<String, Value> {
    let mut result: Map<String, Value> = Map::new();

    for (key, value) in document {
        result.insert(key, bson_to_json(value));
    }

    result
}

pub fn bson_to_json(bson: Bson) -> Value {
    match bson {
        Bson::Double(value) => Value::from(value),
        Bson::String(value) => Value::String(value),
        Bson::Array(value) => Value::Array(value.into_iter().map(bson_to_json).collect()),
        Bson::Document(value) => Value::Object(document_to_json(value)),
        Bson::Boolean(value) => Value::Bool(value),
        Bson::Null | Bson::Undefined => Value::Null,
        Bson::Int32(value) => Value::from(value),
        Bson::Int64(value) => Value::from(value),
        Bson::Timestamp(value) => json!({
            "T": value.time,
        }),
        Bson::Binary(value) => Value::String(STANDARD_NO_PAD.encode(value.bytes)),
        Bson::ObjectId(value) => Value::String(value.to_hex()),
        Bson::DateTime(value) => Value::String(value.try_to_rfc3339_string().unwrap()),
        Bson::Decimal128(value) => Value::String(value.to_string()),
        _ => todo!("Not supporting this type today"),
    }
}
