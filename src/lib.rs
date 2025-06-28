use serde_json::{Value};
use axum::{ http::{StatusCode,}, response::IntoResponse, Json};

pub struct BadRequest(String);

impl IntoResponse for BadRequest {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

// into response to control the http ye
pub async fn receive(Json(payload): Json<Value>) -> Result<String, BadRequest>{

    if is_json(&payload) {
        Err(BadRequest("Nothing to print".to_string()))
    } else {
        println!("Your payload");
        println!("{:?}", serde_json::to_string_pretty(&payload));
        Ok("recieved and printed JSON".to_string())
    }
}

fn is_json(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _=> false
    }
}