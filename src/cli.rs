use clap::{Parser, Subcommand};
use colored::*;

#[derive(Subcommand)]
pub enum GenerateType {
    #[command(about = "Generate a new entity")]
    Entity {
        #[arg(help = "Name of the entity")]
        name: String,
        #[arg(
            long, 
            help = "Fields in format: name:type|validation email:type|validation", 
            num_args = 1..,
            value_delimiter = ' '
        )]
        fields: Vec<String>,
    },
    #[command(about = "Generate a new migration")]
    Migration {
        #[arg(help = "Name of the migration")]
        name: String,
    },
    #[command(about = "Generate a new resolver")]
    Resolver {
        #[arg(help = "Name of the resolver")]
        name: String,
    },
}

#[derive(Subcommand)]
pub enum DbCommand {
    #[command(about = "List all migrations")]
    List,

    #[command(about = "Run pending migrations")]
    Migrate,

    #[command(about = "Rollback migrations")]
    Rollback {
        #[arg(
            short,
            long,
            help = "Number of migrations to rollback",
            default_value = "1"
        )]
        steps: u32,
    },
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
        #[command(subcommand)]
        type_: GenerateType,
    },
    #[command(about = "Database operations")]
    Db {
        #[command(subcommand)]
        command: DbCommand,
    },
}

#[derive(Parser)]
#[command(name = "nebulis")]
#[command(about = "A Full Stack Rust/Remix Project Generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::New { name } => {
                crate::commands::new::execute(name);
            }
            Commands::Generate { type_ } => {
                let result = match type_ {
                    GenerateType::Entity { name, fields } => {
                        crate::generators::entity::execute(name, fields)
                    }
                    GenerateType::Migration { name } => crate::generators::migration::execute(name),
                    GenerateType::Resolver { name } => crate::generators::resolver::execute(name),
                };

                if let Err(e) = result {
                    eprintln!("{} {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            }
            Commands::Db { command } => {
                let result = match command {
                    DbCommand::List => crate::commands::db::execute_list(),
                    DbCommand::Migrate => crate::commands::db::execute_migrate(),
                    DbCommand::Rollback { steps } => {
                        crate::commands::db::execute_rollback(Some(*steps))
                    }
                };

                if let Err(e) = result {
                    eprintln!("{} {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }
    }
}
