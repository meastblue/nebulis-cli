// src/commands/db.rs
use crate::generators::migration;
use colored::*;

pub fn execute_list() -> Result<(), String> {
    println!("{}", "Available migrations:".blue());
    migration::list_migrations()?;
    Ok(())
}

pub fn execute_migrate() -> Result<(), String> {
    println!("{}", "Running migrations...".blue());
    migration::execute_pending_migrations()?;
    println!("{} Migrations completed", "✓".green());
    Ok(())
}

pub fn execute_rollback(steps: Option<u32>) -> Result<(), String> {
    let steps = steps.unwrap_or(1);
    println!(
        "{}",
        format!("Rolling back {} migration(s)...", steps).blue()
    );
    migration::rollback_migrations(steps)?;
    println!("{} Rollback completed", "✓".green());
    Ok(())
}
