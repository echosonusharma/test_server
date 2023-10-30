use std::net::SocketAddr;
use axum::{
    routing::get,
    response::{Json, IntoResponse},
    Router,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    msg: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(welcome_handler))
    .route("/ping", get(ping_handler));
    
    let addr = SocketAddr::from(([127,0,0,1], 9000));

    println!("server is running on: http://{addr}");

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn welcome_handler() -> impl IntoResponse {
    let welcome_msg = Msg { msg: String::from("Welcome to the server 😄!") };
    Json(welcome_msg)
}

async fn ping_handler() -> impl IntoResponse {
    let ping_msg = Msg { msg: String::from("pong 🏓!") };
    Json(ping_msg)
}
