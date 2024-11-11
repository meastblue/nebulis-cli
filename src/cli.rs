use crate::commands::{generate, new};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nebulis")]
#[command(about = "Nebulis - A Full Stack Rust/Remix Project Generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new Nebulis project")]
    New {
        #[arg(help = "Name of the project")]
        name: String,
    },
    #[command(about = "Generate project components")]
    Generate {
        #[arg(help = "Type (model, migration, resolver)")]
        component_type: String,
        #[arg(help = "Name of the component")]
        name: String,
    },
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::New { name } => new::execute(name),
            Commands::Generate {
                component_type,
                name,
            } => generate::execute(component_type, name),
        }
    }
}
