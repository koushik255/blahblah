use serde_json::{Value};
use axum::{ Json};


pub async fn receive(Json(payload): Json<Value>) {
    println!("Your payload");
    println!("{:?}", serde_json::to_string_pretty(&payload));
}