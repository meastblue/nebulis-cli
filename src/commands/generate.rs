// src/commands/generate.rs
use crate::generators::{migration, entity, resolver};

pub fn execute_entity(name: &str, fields: &[String]) -> Result<(), String> {
    entity::execute(name, fields)
}

pub fn execute_migration(name: &str) -> Result<(), String> {
    migration::execute(name)
}

pub fn execute_resolver(name: &str) -> Result<(), String> {
    resolver::execute(name)
}
