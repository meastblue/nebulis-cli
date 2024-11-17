use crate::generators::{backend, docker, frontend};
use crate::utils::print;
use colored::*;
use std::fs;
use std::process::Command;

pub fn execute(name: &str) {
    print::header("Creating Nebulis Full Stack Project");

    // Create project root
    println!("\n{}", "Creating project structure...".blue());
    std::fs::create_dir_all(name).unwrap_or_else(|_| {
        eprintln!("Failed to create project directory");
        std::process::exit(1);
    });

    // Initialize Git at root level only
    init_git(name);

    // Initialize Env file at root level only
    init_env(name);

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

fn init_git(path: &str) {
    match Command::new("git")
        .args(["init"])
        .current_dir(path)
        .status()
    {
        Ok(_) => {
            println!("{}", "Git repository initialized".green());
            create_gitignore(path);
        }
        Err(_) => {
            println!(
                "{}",
                "Warning: Failed to initialize git repository".yellow()
            );
        }
    }
}

fn create_gitignore(path: &str) {
    let gitignore_content = r#"# Dependencies
node_modules/
/target/

# Build
dist/
build/

# Environment variables
.env
.env.*
!.env.example

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Database
database/
"#;

    if let Err(e) = std::fs::write(format!("{}/.gitignore", path), gitignore_content) {
        println!("{} {}", "Warning: Failed to create .gitignore:".yellow(), e);
    }
}

pub fn init_env(path: &str) {
    let content = format!(
        r#"#Database environement
DB_HOST=127.0.0.1
DB_PORT=8000
DB_USER=root
DB_PASSWORD=root
DB_NAME={name}

#Backend environement
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

#Frontend environement
APP_HOST=127.0.0.1
APP_PORT=3000"#,
        name = path
    );

    fs::write(format!("{}/.env", path), content)
        .unwrap_or_else(|_| panic!("Failed to create .evn file"));
}
