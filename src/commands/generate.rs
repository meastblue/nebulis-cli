// src/commands/generate.rs
use crate::generators::{migration, model, resolver};

pub fn execute_model(name: &str, fields: &[String]) -> Result<(), String> {
    model::execute(name, fields)
}

pub fn execute_migration(name: &str) -> Result<(), String> {
    migration::execute(name)
}

pub fn execute_resolver(name: &str) -> Result<(), String> {
    resolver::execute(name)
}
