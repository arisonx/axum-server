use std::net::SocketAddr;
use tokio;
use super::routes;

#[tokio::main]
pub async fn run_server() {

    //local address
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    //bind server with address
    axum::Server::bind(&address)
        .serve(routes::setup_routes().into_make_service())
        .await
        .unwrap();                                                   
}
