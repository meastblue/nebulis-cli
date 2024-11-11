use colored::*;

pub fn header(message: &str) {
    println!("\n{}", "=".repeat(50).blue());
    println!("{}", message.blue().bold());
    println!("{}", "=".repeat(50).blue());
}

pub fn success(message: &str) {
    println!("{}", message.green().bold());
}

pub fn next_steps(project_name: &str) {
    println!("\n{}", "Next steps:".bold());
    println!("  cd {}", project_name);
    println!("  docker-compose up -d     # Start SurrealDB");
    println!("  cd backend && cargo run  # Start backend server");
    println!("  cd ../frontend && deno task dev  # Start Remix dev server");
    println!("\n{}", "Happy coding! 🚀".green());
}
