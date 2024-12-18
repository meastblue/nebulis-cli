// src/generators/backend.rs
use crate::utils::fs as fs_utils;
use std::fs;

pub fn create_structure(project_name: &str) {
    let backend_path = format!("{}/backend", project_name);
    fs::create_dir_all(&backend_path).unwrap();

    create_directories(&backend_path);
    create_cargo_toml(&backend_path, project_name);
    create_source_files(&backend_path);
}

fn create_directories(backend_path: &str) {
    let directories = vec![
        "src/graphql/mutations",
        "src/graphql/queries",
        "src/graphql/types",
        "src/graphql/scalars",
        "src/migrations",
        "src/entities",
        "src/repositories",
        "src/services",
        "src/utils",
    ];

    fs_utils::create_directories(backend_path, &directories);
}

fn create_cargo_toml(path: &str, project_name: &str) {
    let content = format!(
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
    );

    fs::write(format!("{}/Cargo.toml", path), content)
        .unwrap_or_else(|_| panic!("Failed to create Cargo.toml"));
}

fn create_source_files(path: &str) {
    let main_rs = r#"mod db;
mod graphql;
mod entities;
mod repositories;
mod services;
mod utils;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    // Run server
    server::run()
    .await?;
}"#;

    let server_rs = r#"
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQLHandler;
use axum::{
    routing::get,
    Router,
};

struct Server;

impl Server {
    async fn run() {
        //Environment variables
        let host = env::var("SERVER_HOST")?;
        let port = env::var("SERVER_PORT")?.parse::<u16>()?;

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
        axum::serve(listener, app).await?;
    }
}"#;

    fs::write(format!("{}/src/main.rs", path), main_rs)
        .unwrap_or_else(|_| panic!("Failed to create main.rs"));
    fs::write(format!("{}/src/server.rs", path), server_rs)
        .unwrap_or_else(|_| panic!("Failed to create server.rs"));

    create_mod_files(path);
}

fn create_mod_files(base_path: &str) {
    let mod_paths = [
        "src/migrations",
        "src/graphql",
        "src/entities",
        "src/repositories",
        "src/services",
        "src/utils",
    ];

    for path in mod_paths {
        fs::write(
            format!("{}/{}/mod.rs", base_path, path),
            "// Generated by Nebulis CLI\n",
        )
        .unwrap_or_else(|_| panic!("Failed to create mod.rs in {}", path));
    }
}
