use crate::generators::{backend, docker, frontend};
use crate::utils::print;
use colored::*;

pub fn execute(name: &str) {
    print::header("Creating Nebulis Full Stack Project");

    // Create project root
    println!("\n{}", "Creating project structure...".blue());
    std::fs::create_dir_all(name).unwrap_or_else(|_| {
        eprintln!("Failed to create project directory");
        std::process::exit(1);
    });

    // Initialize Git at root level only
    if let Err(e) = init_git(name) {
        println!("{} {}", "Warning: Failed to initialize git:".yellow(), e);
    }

    // Generate backend
    println!("\n{}", "Generating Rust backend...".blue());
    backend::create_structure(name);

    // Generate frontend
    println!("\n{}", "Generating Remix frontend...".blue());
    if let Err(e) = frontend::create_structure(name) {
        eprintln!("\n{}: {}", "Error creating frontend".red(), e);
        std::process::exit(1);
    }

    // Setup Docker
    println!("\n{}", "Setting up Docker environment...".blue());
    docker::create_structure(name);

    print::success("\nNebulis project created successfully! 🚀");
    print::next_steps(name);
}

fn init_git(path: &str) -> Result<(), String> {
    use std::process::Command;

    Command::new("git")
        .args(["init"])
        .current_dir(path)
        .status()
        .map_err(|e| e.to_string())?;

    // Create .gitignore
    std::fs::write(
        format!("{}/.gitignore", path),
        r#"# IDE
.idea/
.vscode/
*.swp
*.swo
.DS_Store

# Rust Backend
backend/target/
backend/Cargo.lock
backend/debug/
backend/**/*.rs.bk
backend/.env
backend/logs/

# Logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# Frontend - Remix with Deno
frontend/node_modules/
frontend/build/
frontend/.cache/
frontend/functions/\[\[path\]\].js
frontend/functions/\[\[path\]\].js.map
frontend/functions/metafile.*
frontend/public/build
frontend/.env*
!frontend/.env.example

# Database
database/
data/

# Temporary files
*.tmp
*.temp
.DS_Store
Thumbs.db

# Environment variables
.env
.env.*
!.env.example

# Docker
docker-compose.override.yml
"#,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
