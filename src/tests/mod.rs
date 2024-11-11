// src/tests/mod.rs
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

pub mod utils;

#[test]
fn test_new_command() {
    // Créer un répertoire temporaire pour les tests
    let temp = assert_fs::TempDir::new().unwrap();
    let project_name = "test_project";
    let project_path = temp.child(project_name);

    // Exécuter la commande new
    let output = Command::new(env!("CARGO_BIN_EXE_nebulis-cli"))
        .args(["new", project_name])
        .current_dir(temp.path())
        .output()
        .unwrap();

    // Vérifier que la commande s'est bien exécutée
    assert!(output.status.success());

    // Vérifier la structure du projet
    project_path
        .child("backend/Cargo.toml")
        .assert(predicate::path::exists());
    project_path
        .child("frontend/package.json")
        .assert(predicate::path::exists());
    project_path
        .child("docker-compose.yml")
        .assert(predicate::path::exists());

    // Clean up
    temp.close().unwrap();
}
