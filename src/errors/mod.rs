
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status":404,
        "message":"Not Found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}  
  
   