// src/generators/frontend.rs
use colored::*;
use std::process::Command;

pub fn create_structure(project_name: &str) -> Result<(), String> {
    println!("{}", "Creating Remix project...".yellow());

    let frontend_path = format!("{}/frontend", project_name);

    // Création du projet Remix avec Deno
    let status = Command::new("deno")
        .args([
            "run",
            "-A",
            "npm:create-remix@latest",
            &frontend_path,
            "--no-install",
            "--no-git-init",
        ])
        .status()
        .expect("Failed to execute deno command");

    if !status.success() {
        return Err("Failed to create Remix project".to_string());
    }

    println!("\n{}", "Frontend created successfully!".green());
    println!("{}", "To start the development server:".blue());
    println!("  cd {}/frontend", project_name);
    println!("  deno task dev");

    Ok(())
}
