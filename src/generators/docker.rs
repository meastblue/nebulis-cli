// src/generators/docker.rs
use std::fs;

pub fn create_structure(project_name: &str) {
    create_docker_compose(project_name);
}

fn create_docker_compose(project_name: &str) {
    let content = r#"version: '3.8'
services:
  surrealdb:
    image: surrealdb/surrealdb:latest
    container_name: {{project_name}}_surrealdb
    ports:
      - "8000:8000"
    volumes:
      - ./database:/data
    command: start --user root --pass root file:/data/database.db
"#
    .replace("{{project_name}}", project_name);

    fs::write(format!("{}/docker-compose.yml", project_name), content).unwrap();
}
