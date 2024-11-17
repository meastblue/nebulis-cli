use chrono::Utc;
use colored::*;
use convert_case::{Case, Casing};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Structure pour stocker les informations du modèle
#[derive(Debug)]
struct EntityInfo {
    fields: Vec<(String, String, FieldValidation)>,
    relations: Vec<RelationType>,
}

#[derive(Debug, Clone)]
struct FieldValidation {
    required: bool,
    unique: bool,
    email: bool,
    url: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    min: Option<String>,
    max: Option<String>,
}

impl Default for FieldValidation {
    fn default() -> Self {
        Self {
            required: false,
            unique: false,
            email: false,
            url: false,
            min_length: None,
            max_length: None,
            pattern: None,
            min: None,
            max: None,
        }
    }
}

#[derive(Debug, Clone)]
enum RelationType {
    HasOne(String),
    HasMany(String),
    BelongsTo(String),
}

#[derive(Debug)]
enum MigrationOperation {
    CreateTable(String),
    AddColumn(String, String, String),    // table, column, type
    RemoveColumn(String, String),         // table, column
    RenameColumn(String, String, String), // table, old_name, new_name
    AddIndex(String, Vec<String>),        // table, columns
    AddRelation(String, String, String),  // from_table, to_table, type
}

impl EntityInfo {
    fn parse_from_file(entity_path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(entity_path)
            .map_err(|e| format!("Could not read entity file: {}", e))?;

        let mut fields = Vec::new();
        let mut relations = Vec::new();

        // Regex pour capturer les validations et les champs
        let field_regex =
            Regex::new(r"(?m)(?s)#\[validate\((.*?)\)\].*?pub\s+(\w+)\s*:\s*(\w+)").unwrap();

        for cap in field_regex.captures_iter(&content) {
            let validations = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let field_name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let field_type = cap.get(3).map(|m| m.as_str()).unwrap_or("");

            // Ignorer les champs système
            if ["id", "created_at", "updated_at", "deleted_at"].contains(&field_name) {
                continue;
            }

            let mut validation = FieldValidation::default();

            // Parser les validations
            for val in validations.split(',').map(str::trim) {
                match val {
                    "required" => validation.required = true,
                    "unique" => validation.unique = true,
                    "email" => validation.email = true,
                    "url" => validation.url = true,
                    _ => {}
                }
            }

            fields.push((field_name.to_string(), field_type.to_string(), validation));
        }

        Ok(EntityInfo { fields, relations })
    }
}

impl MigrationOperation {
    fn from_name(name: &str) -> Option<Self> {
        let name = name.to_case(Case::Snake);

        match name.split('_').collect::<Vec<&str>>().as_slice() {
            // Formats explicites
            ["create", table] => Some(Self::CreateTable(table.to_string())),
            ["add", field, "to", table] => Some(Self::AddColumn(
                table.to_string(),
                field.to_string(),
                "String".to_string(),
            )),
            ["remove", field, "from", table] => {
                Some(Self::RemoveColumn(table.to_string(), field.to_string()))
            }
            ["rename", old_name, "to", new_name, "on", table] => Some(Self::RenameColumn(
                table.to_string(),
                old_name.to_string(),
                new_name.to_string(),
            )),
            ["add", "index", "on", table, "fields", fields @ ..] => Some(Self::AddIndex(
                table.to_string(),
                fields.iter().map(|s| s.to_string()).collect(),
            )),
            ["add", "relation", from_table, "to", to_table] => Some(Self::AddRelation(
                from_table.to_string(),
                to_table.to_string(),
                "BelongsTo".to_string(),
            )),
            // Format simple - juste le nom de la table
            [table] => {
                // Si le nom est au singulier, on le convertit en pluriel
                let table_name = if table.ends_with('s') {
                    table.to_string()
                } else {
                    format!("{}s", table)
                };
                Some(Self::CreateTable(table_name))
            }
            _ => None,
        }
    }

    fn up_sql(&self) -> String {
        match self {
            Self::CreateTable(name) => {
                let entity_path = format!(
                    "backend/src/entities/{}.rs",
                    name.trim_end_matches('s').to_lowercase()
                );

                let entity_info =
                    EntityInfo::parse_from_file(&entity_path).unwrap_or_else(|_| EntityInfo {
                        fields: Vec::new(),
                        relations: Vec::new(),
                    });

                let table_name = name.to_lowercase();
                let mut sections = Vec::new();
                let mut seen_fields = std::collections::HashSet::new();
                let mut seen_indexes = std::collections::HashSet::new();

                // Table definition
                sections.push(format!("DEFINE TABLE {table_name} SCHEMAFULL;"));
                sections.push("".to_string());

                // Base fields section
                sections.push("// Champs système".to_string());
                sections.push(format!("DEFINE FIELD id ON {table_name} TYPE ulid;"));
                seen_fields.insert("id".to_string());
                sections.push("".to_string());

                // Entity fields section
                sections.push("// Champs de l'entité".to_string());
                for (field_name, field_type, _) in &entity_info.fields {
                    if !seen_fields.insert(field_name.clone()) {
                        continue; // Skip if field already defined
                    }

                    let surql_type = match field_type.as_str() {
                        "String" => "string",
                        "Email" => "string",
                        "Phone" => "string",
                        "i32" | "i64" => "int",
                        "f32" | "f64" => "float",
                        "bool" => "bool",
                        "DateTime" => "datetime",
                        _ => "string",
                    };

                    sections.push(format!(
                        "DEFINE FIELD {field_name} ON {table_name} TYPE {surql_type};"
                    ));
                }
                sections.push("".to_string());

                // Timestamps section
                sections.push("// Champs de timestamps".to_string());
                for field in ["created_at", "updated_at", "deleted_at"] {
                    if !seen_fields.insert(field.to_string()) {
                        continue;
                    }
                    sections.push(format!(
                        "DEFINE FIELD {field} ON {table_name} TYPE datetime \
                         VALUE $before OR time::now();"
                    ));
                }
                sections.push("".to_string());

                // Indexes section
                sections.push("// Indexes".to_string());

                // Add id index
                let id_index = format!("idx_{table_name}_id");
                if seen_indexes.insert(id_index.clone()) {
                    sections.push(format!(
                        "DEFINE INDEX {id_index} ON {table_name} FIELDS id;"
                    ));
                }

                // Timestamps indexes
                for field in ["created", "updated", "deleted"] {
                    let index_name = format!("idx_{table_name}_{field}");
                    if seen_indexes.insert(index_name.clone()) {
                        sections.push(format!(
                            "DEFINE INDEX {index_name} ON {table_name} FIELDS {field}_at;"
                        ));
                    }
                }

                sections.join("\n")
            }
            Self::AddColumn(table, column, type_) => format!(
                "DEFINE FIELD {} ON {} TYPE {};",
                column.to_lowercase(),
                table.to_lowercase(),
                type_.to_lowercase()
            ),
            Self::RemoveColumn(table, column) => format!(
                "REMOVE FIELD {} ON {};",
                column.to_lowercase(),
                table.to_lowercase()
            ),
            Self::RenameColumn(table, old_name, new_name) => format!(
                "DEFINE FIELD {new} ON {table} TYPE string;\n\
                 UPDATE {table} SET {new} = {old};\n\
                 REMOVE FIELD {old} ON {table};",
                table = table.to_lowercase(),
                old = old_name.to_lowercase(),
                new = new_name.to_lowercase()
            ),
            Self::AddIndex(table, columns) => format!(
                "DEFINE INDEX idx_{table}_{columns} ON {table} FIELDS {column_list};",
                table = table.to_lowercase(),
                columns = columns.join("_").to_lowercase(),
                column_list = columns
                    .iter()
                    .map(|c| c.to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::AddRelation(from_table, to_table, _) => format!(
                "DEFINE FIELD {to}_id ON {from} TYPE record({to});\n\
                 DEFINE INDEX idx_{from}_{to} ON {from} FIELDS {to}_id;",
                from = from_table.to_lowercase(),
                to = to_table.to_lowercase()
            ),
        }
    }

    fn down_sql(&self) -> String {
        match self {
            Self::CreateTable(name) => format!("REMOVE TABLE {};", name.to_lowercase()),
            Self::AddColumn(table, column, _) => format!(
                "REMOVE FIELD {} ON {};",
                column.to_lowercase(),
                table.to_lowercase()
            ),
            Self::RemoveColumn(table, column) => format!(
                "DEFINE FIELD {} ON {} TYPE string;",
                column.to_lowercase(),
                table.to_lowercase()
            ),
            Self::RenameColumn(table, old_name, new_name) => format!(
                "DEFINE FIELD {old} ON {table} TYPE string;\n\
                 UPDATE {table} SET {old} = {new};\n\
                 REMOVE FIELD {new} ON {table};",
                table = table.to_lowercase(),
                old = old_name.to_lowercase(),
                new = new_name.to_lowercase()
            ),
            Self::AddIndex(table, columns) => format!(
                "REMOVE INDEX idx_{table}_{columns} ON {table};",
                table = table.to_lowercase(),
                columns = columns.join("_").to_lowercase()
            ),
            Self::AddRelation(from_table, to_table, _) => format!(
                "REMOVE FIELD {to}_id ON {from};\n\
                 REMOVE INDEX idx_{from}_{to} ON {from};",
                from = from_table.to_lowercase(),
                to = to_table.to_lowercase()
            ),
        }
    }
}

pub fn execute(name: &str) -> Result<(), String> {
    println!("{} {}", "Generating migration:".blue(), name);

    let operation = MigrationOperation::from_name(name)
        .ok_or_else(|| format!("Invalid migration name format: {}", name))?;

    let filename = format!("{}", name.to_case(Case::Snake));

    generate_migration_files(&filename, &operation)?;
    update_migrations_mod(&filename)?;

    println!("{} Migration files generated:", "✓".green());
    println!("  - database/schema/{}.up.surql", filename);
    println!("  - database/schema/{}.down.surql", filename);
    println!("  - backend/src/migrations/{}.rs", filename);

    Ok(())
}

pub fn execute_pending_migrations() -> Result<(), String> {
    let migrations_dir = "backend/src/migrations";
    if !Path::new(migrations_dir).exists() {
        return Err("No migrations directory found".into());
    }

    // TODO: Implémenter la logique d'exécution des migrations
    Ok(())
}

pub fn list_migrations() -> Result<(), String> {
    let migrations_dir = "backend/src/migrations";
    if !Path::new(migrations_dir).exists() {
        return Err("No migrations directory found".into());
    }

    let entries = fs::read_dir(migrations_dir)
        .map_err(|e| format!("Failed to read migrations directory: {}", e))?;

    let mut migrations = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if file_name != "mod.rs" && file_name.ends_with(".rs") {
                migrations.push(file_name.to_string());
            }
        }
    }

    migrations.sort();
    for migration in migrations {
        println!("  - {}", migration.trim_end_matches(".rs"));
    }

    Ok(())
}

pub fn rollback_migrations(_steps: u32) -> Result<(), String> {
    let migrations_dir = "backend/src/migrations";
    if !Path::new(migrations_dir).exists() {
        return Err("No migrations directory found".into());
    }

    // TODO: Implémenter la logique de rollback
    Ok(())
}

fn generate_migration_files(filename: &str, operation: &MigrationOperation) -> Result<(), String> {
    // Créer les répertoires nécessaires
    fs::create_dir_all("database/schema")
        .map_err(|e| format!("Failed to create schema directory: {}", e))?;
    fs::create_dir_all("backend/src/migrations")
        .map_err(|e| format!("Failed to create migrations directory: {}", e))?;

    // Générer les fichiers SQL
    fs::write(
        format!("database/schema/{}.up.surql", filename),
        operation.up_sql(),
    )
    .map_err(|e| format!("Failed to write up migration: {}", e))?;

    fs::write(
        format!("database/schema/{}.down.surql", filename),
        operation.down_sql(),
    )
    .map_err(|e| format!("Failed to write down migration: {}", e))?;

    // Générer le fichier de migration Rust
    let migration_content = generate_migration_rust(filename)?;
    fs::write(
        format!("backend/src/migrations/{}.rs", filename),
        migration_content,
    )
    .map_err(|e| format!("Failed to write migration file: {}", e))?;

    Ok(())
}

fn generate_migration_rust(filename: &str) -> Result<String, String> {
    let class_name = filename.to_case(Case::Pascal);
    Ok(format!(
        r#"use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::db::migrations::Migration;
use std::fs;

pub struct {class_name};

#[async_trait]
impl Migration for {class_name} {{
    fn version(&self) -> &str {{
        "{version}"
    }}

    fn name(&self) -> &str {{
        "{name}"
    }}

    async fn up(&self, db: &Surreal<Client>) -> Result<(), Box<dyn std::error::Error>> {{
        let schema = fs::read_to_string("database/schema/{filename}.up.surql")?;
        db.query(schema).await?;
        Ok(())
    }}

    async fn down(&self, db: &Surreal<Client>) -> Result<(), Box<dyn std::error::Error>> {{
        let schema = fs::read_to_string("database/schema/{filename}.down.surql")?;
        db.query(schema).await?;
        Ok(())
    }}
}}
"#,
        class_name = class_name,
        version = Utc::now().format("%Y%m%d%H%M%S").to_string(),
        name = filename,
        filename = filename
    ))
}

fn update_migrations_mod(filename: &str) -> Result<(), String> {
    let mod_path = "backend/src/migrations/mod.rs";
    let mut content = fs::read_to_string(mod_path).unwrap_or_else(|_| String::new());

    if !content.contains(&format!("pub mod {};", filename)) {
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("pub mod {};\n", filename));
        content.push_str(&format!("pub use {}::*;\n", filename));
    }

    fs::write(mod_path, content).map_err(|e| format!("Failed to update mod.rs: {}", e))?;

    Ok(())
}
