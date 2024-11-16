// src/templates/backend.rs
pub fn get_cargo_toml_template(project_name: &str) -> String {
    format!(
        r#"[package]
name = "{}_backend"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {{ version = "1.0", features = ["full"] }}
axum = {{ version = "0.7", features = ["macros"] }}
async-graphql = {{ version = "7.0", features = ["chrono"] }}
async-graphql-axum = "7.0"
surrealdb = {{ version = "1.0.0" }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tower = "0.4"
tower-http = {{ version = "0.5", features = ["cors"] }}
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}
thiserror = "1.0"
chrono = {{ version = "0.4", features = ["serde"] }}
validator = {{ version = "0.16", features = ["derive"] }}"#,
        project_name
    )
}

pub fn get_main_rs_template() -> String {
    r#"mod db;
mod graphql;
mod entities;
mod repositories;
mod services;
mod utils;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQLHandler;
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Build our GraphQL schema
    let schema = Schema::new(
        graphql::QueryRoot::default(),
        graphql::MutationRoot::default(),
        EmptySubscription,
    );

    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Nebulis Backend API" }))
        .route("/graphql",
            get(GraphQLHandler::new(schema.clone()))
            .post(GraphQLHandler::new(schema))
        );

    // Run our application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}"#
    .to_string()
}

pub fn get_env_template() -> String {
    r#"# Database Configuration
DATABASE_URL="ws://localhost:8000"
DB_USER="root"
DB_PASS="root"
DB_NAMESPACE="test"
DB_DATABASE="test"

# Server Configuration
PORT=3000
HOST="0.0.0.0"

# Environment
RUST_ENV="development"
"#
    .to_string()
}
