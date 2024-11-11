// src/generators/frontend.rs
use colored::*;
use std::process::Command;

pub fn create_structure(project_name: &str) -> Result<(), String> {
    println!("{}", "Creating Remix project...".yellow());

    let frontend_path = format!("{}/frontend", project_name);

    // Utiliser la commande npx pour être sûr d'avoir la dernière version
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
        // Plan B: Essayer avec une autre méthode si la première échoue
        println!("{}", "Trying alternative method...".yellow());

        let status = Command::new("deno")
            .args([
                "run",
                "-A",
                "-r",
                "https://create-remix.run",
                &frontend_path,
                "--no-install",
                "--no-git-init",
            ])
            .status()
            .expect("Failed to execute deno command");

        if !status.success() {
            return Err("Failed to create Remix project".to_string());
        }
    }

    // Afficher des informations utiles après la création
    println!("\n{}", "Frontend created successfully!".green());
    println!("{}", "To start the development server:".blue());
    println!("  cd {}/frontend", project_name);
    println!("  deno task dev");

    Ok(())
}
