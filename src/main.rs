use std::net::SocketAddr;
use axum::{
    routing::get,
    response::{Json, IntoResponse},
    Router, extract::{Query, Path},
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    msg: String
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(welcome_handler))
    .route("/ping", get(ping_handler))
    .route("/user/:id", get(user_handler))
    .route("/users", get(users_handler));

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

#[derive(Deserialize, Debug)]
struct UserParams {
    limit: Option<String>,
    offset: Option<String>,
    name: Option<String>,
}

async fn user_handler(Path(id): Path<String>) -> impl IntoResponse {
    let user = User {
        id: id.parse::<u64>().unwrap(),
        name: String::from("sam hanna"),
        email: String::from("sam_hanna@gmail.com")
    };

    Json(user)
}

async fn users_handler(Query(params): Query<UserParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("");
    let limit = params.limit.as_deref().unwrap_or("10");
    let offset = params.offset.as_deref().unwrap_or("0");

    println!("{} {} {}", name, limit, offset);

    let msg = Msg { msg: String::from("No users found!") };
    Json(msg)
}
