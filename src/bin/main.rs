use controller::get_info_handler;
use controller::login_handler;
use ntex_askama_test::*;
use askama::Template;
use diesel::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use axum::{
    routing::{get, post},
    Router,
};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/login", get(login_handler))
        // `POST /users` goes to `create_user`
        .route("/info", post(get_info_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
