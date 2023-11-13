use std::net::SocketAddr;
use axum::{Router, http::StatusCode, Json};
use serde_json::{Value, json};
use tokio;

#[tokio::main]
async fn main() {   

    let api =  Router::new().fallback(error_handler);

    let app: Router  = Router::new().nest("/api", api); //router
    //address
    let address =  SocketAddr::from(([127,0,0,1], 8080));

    //bind server with address
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
    
    
}


async fn error_handler() -> (StatusCode, Json<Value>){
    let body = json!({
        "status":404,
        "message":"Not Found"
    });

    (StatusCode::NOT_FOUND,Json(body))
}


async fn root_rote ()-> (StatusCode, Json<Value>){
    let body =  json!({
        "status":"ok",
        "message":"hello world"
    });

    (StatusCode::OK,Json(body))
}