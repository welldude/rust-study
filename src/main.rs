use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::env;

#[derive(Deserialize)]
struct UserPayload {
    name: String, 
    email: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    println!("Server running on port 8000");
    axum::serve(listener, app).await.unwrap();
}

// Endpoint Handlers
// test endpoint
async fn root() -> &'static str {
    "Welcome to the User Management API!"
}

// GET ALL
async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .map(|u| (StatusCode::CREATED, Json(u)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
