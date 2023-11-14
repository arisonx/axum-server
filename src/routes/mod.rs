use super::errors;
use axum::{http::StatusCode, routing::get, Json, Router, response::Html};
use serde_json::{json, Value};

pub fn setup_routes() -> Router {
    let app: Router = Router::new()
        .fallback(errors::api_fallback())
        .route("/", get(root))
        .route("/html", get(|| async {Html("Hello <strong>World!!</strong>")}))
        ;
    return app;
}

async fn root() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status":200,
        "message":"Ok"
    });

    (StatusCode::OK, Json(body))
}  
