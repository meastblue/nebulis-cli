// src/generators/docker.rs
use std::fs;

pub fn create_structure(project_name: &str) {
    let content = format!(
        r#"version: '3.8'

services:
  surrealdb:
    image: surrealdb/surrealdb:latest
    container_name: {}_db
    ports:
      - "8000:8000"
    volumes:
      - ./database:/data
    command: start --user root --pass root file:/data/database.db
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

networks:
  default:
    name: {}_network"#,
        project_name, project_name
    );

    fs::write(format!("{}/docker-compose.yml", project_name), content)
        .unwrap_or_else(|_| panic!("Failed to create docker-compose.yml"));
}
