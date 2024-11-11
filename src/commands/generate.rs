// src/commands/generate.rs
use crate::generators;
use colored::*;

pub fn execute(component_type: &str, name: &str) {
    println!(
        "\n{}",
        format!("Generating {}: {}", component_type, name).blue()
    );

    match component_type {
        "model" => generators::backend::generate_model(name),
        "migration" => generators::backend::generate_migration(name),
        "resolver" => generators::backend::generate_resolver(name),
        _ => println!(
            "{}",
            format!("Unknown component type: {}", component_type).red()
        ),
    }
}
