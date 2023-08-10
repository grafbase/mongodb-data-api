use serde_json::{Map, Value};

pub fn ejson_to_json(ejson: Map<String, Value>) -> Map<String, Value> {
    let mut result: Map<String, Value> = Map::new();

    for (key, value) in ejson {
        match value {
            Value::Object(mut object) if object.contains_key("$oid") => {
                let value = object.remove("$oid").unwrap();
                result.insert(key, value);
            }
            Value::Object(object) => {
                let value = ejson_to_json(object);
                result.insert(key, Value::Object(value));
            }
            value => {
                result.insert(key, value);
            }
        }
    }

    result
}
