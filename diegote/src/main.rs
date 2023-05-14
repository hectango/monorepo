use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use config::{Config, Environment};
use serde::{Deserialize, Serialize};
use settings::Settings;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use tracing::info;

mod cloudflare;
mod handlers;
mod settings;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("reading dotenv");
    let config_reader = Config::builder()
        .add_source(Environment::default())
        .build()
        .expect("creating config reader");
    let settings: Settings = config_reader
        .try_deserialize()
        .expect("deserializing settings");
    // initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting up");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.db_url)
        .await
        .expect("postgres db pool");

    let context = AppContext { db_pool };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/videos", get(handlers::get_videos))
        .with_state(context);

    // run our app with hyper, listening globally on port 3000
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //axum::serve(listener, app).await.unwrap();

    let addr = format!("0.0.0.0:{}", settings.port).parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
