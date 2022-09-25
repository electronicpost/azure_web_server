use std::{env, net::SocketAddr};

use axum::{Router, routing::post, Json, Server, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[tokio::main]

async fn main() {
    let app = Router::new().route("/api/contact", post(contact));

    let port = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = env::var(port)
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[derive(Deserialize)]

pub struct Contact {
    pub name: String,
    pub mail: String,
    pub message: String,
}

async fn contact(Json(payload): Json<Contact>) -> impl IntoResponse {
    println!("{}, {}, {}", payload.name, payload.mail, payload.message);
    (StatusCode::OK, Json("{ status: 200 }"))
}
