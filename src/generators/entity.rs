// src/generators/entity
use colored::*;
use convert_case::{Case, Casing};
use std::fs;
use std::path::Path;

/// Définit les types de relations possibles entre les modèles
#[derive(Debug, Clone)]
enum RelationType {
    HasOne(String),
    HasMany(String),
    BelongsTo(String),
}

/// Configuration de validation pour un champ
#[derive(Debug, Clone)]
pub struct FieldValidation {
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    min: Option<String>,
    max: Option<String>,
    unique: bool,
    email: bool,
    url: bool,
}

impl Default for FieldValidation {
    fn default() -> Self {
        Self {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
            min: None,
            max: None,
            unique: false,
            email: false,
            url: false,
        }
    }
}

/// Liste des types valides pour les champs du modèle
const VALID_TYPES: [&str; 25] = [
    // Types de base
    "String",
    "i32",
    "i64",
    "f32",
    "f64",
    "bool",
    "DateTime<Utc>",
    "Vec<String>",
    "Option<String>",
    "u32",
    "u64",
    "usize",
    // Types personnalisés
    "Email",
    "Phone",
    "Url",
    "Password",
    "Json",
    "Money",
    "Uuid",
    "Slug",
    // Types relationnels
    "HasOne",
    "HasMany",
    "BelongsTo",
    // Types enum
    "Status",
    "Role",
];

/// Template pour la génération des modèles
const ENTITY_TEMPLATE: &str = r#"use serde::{Deserialize, Serialize};
use async_graphql::{{SimpleObject, InputObject, Enum}};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::entities::base_entity::BaseEntity;
{imports}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {name} {
    #[serde(flatten)]
    base: BaseEntity,

    #[serde(skip_serializing_if = "Option::is_none")]
    deleted_at: Option<DateTime<Utc>>,

    // Custom fields with validation
{fields}

    // Relations
{relations}
}

impl {name} {
    pub fn new() -> Self {
        Self {
            base: BaseEntity::new(),
            deleted_at: None,
{field_inits}
{relation_inits}
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn mark_as_deleted(&mut self) {
        self.deleted_at = Some(Utc::now());
    }

    pub fn restore(&mut self) {
        self.deleted_at = None;
    }
}

#[derive(InputObject, Validate)]
#[serde(rename_all = "camelCase")]
pub struct {name}Input {
{input_fields}
}

#[derive(InputObject)]
#[serde(rename_all = "camelCase")]
pub struct {name}Filter {
    pub include_deleted: Option<bool>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    // Custom field filters
{filter_fields}
    // Relation filters
{relation_filters}
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum {name}SortField {
    CreatedAt,
    UpdatedAt,
{sort_fields}
}

#[derive(InputObject)]
pub struct {name}Sort {
    pub field: {name}SortField,
    pub order: SortOrder,
}

#[derive(InputObject)]
pub struct {name}Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub cursor: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}"#;

// src/generators/entity (suite)

/// Point d'entrée principal pour la génération d'un modèle
pub fn execute(name: &str, fields: &[String]) -> Result<(), String> {
    println!("{} {}", "Generating entity:".blue(), name);

    if !Path::new("backend").exists() {
        return Err("Not in a Nebulis project directory".into());
    }

    let (fields, relations) = parse_fields_and_relations(fields)?;

    let entity_path = format!("backend/src/entities/{}.rs", name.to_lowercase());
    let entity_content = generate_entity_content(name, &fields, &relations)?;
    fs::write(&entity_path, entity_content)
        .map_err(|e| format!("Failed to write entity file: {}", e))?;

    update_entities_mod(name, &relations)?;
    update_graphql_mod(name)?;

    println!("{} Generated files:", "✓".green());
    println!("  - {}", entity_path);
    Ok(())
}

/// Parse les champs et les relations à partir des définitions fournies
/// Parse les champs et les relations à partir des définitions fournies
fn parse_fields_and_relations(
    fields: &[String],
) -> Result<(Vec<(String, String, FieldValidation)>, Vec<RelationType>), String> {
    let mut parsed_fields = Vec::new();
    let mut relations = Vec::new();

    // Séparer la chaîne en champs individuels
    for field_group in fields {
        let field_definitions: Vec<&str> = field_group.split(',').collect();

        for field_def in field_definitions {
            if field_def.contains("->") {
                let parts: Vec<&str> = field_def.split("->").collect();
                if parts.len() != 2 {
                    return Err(format!("Invalid relation format: {}", field_def));
                }

                let relation_type = match parts[0].trim() {
                    "hasOne" => RelationType::HasOne(parts[1].trim().to_string()),
                    "hasMany" => RelationType::HasMany(parts[1].trim().to_string()),
                    "belongsTo" => RelationType::BelongsTo(parts[1].trim().to_string()),
                    _ => return Err(format!("Invalid relation type: {}", parts[0])),
                };
                relations.push(relation_type);
            } else {
                let field_parts: Vec<&str> = field_def.split(':').collect();
                if field_parts.len() != 2 {
                    return Err(format!("Invalid field format: {}. Expected format: name:type|validation1 validation2", field_def));
                }

                let name = field_parts[0].trim().to_string();
                let type_and_validations: Vec<&str> = field_parts[1].split('|').collect();

                if type_and_validations.is_empty() {
                    return Err(format!("Missing type for field: {}", name));
                }

                let field_type = type_and_validations[0].trim().to_string();
                validate_field_type(&field_type)?;

                let mut validation = FieldValidation::default();
                if type_and_validations.len() > 1 {
                    let validation_rules = type_and_validations[1]
                        .split(|c| c == ' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();

                    parse_validations(&mut validation, &validation_rules)?;
                }

                parsed_fields.push((name, field_type, validation));
            }
        }
    }

    Ok((parsed_fields, relations))
}

/// Valide le type d'un champ
fn parse_validations(validation: &mut FieldValidation, rules: &[&str]) -> Result<(), String> {
    for rule in rules {
        let rule = rule.trim();
        if rule.is_empty() {
            continue;
        }

        // Parse une règle avec sa valeur optionnelle
        let (rule_name, rule_value) = if let Some(idx) = rule.find('=') {
            let (name, value) = rule.split_at(idx);
            (name.trim(), Some(value[1..].trim()))
        } else {
            (rule, None)
        };

        match rule_name {
            "required" => validation.required = true,
            "unique" => validation.unique = true,
            "email" => validation.email = true,
            "url" => validation.url = true,
            "minLength" | "min_length" => {
                if let Some(value) = rule_value {
                    let length = value
                        .parse::<usize>()
                        .map_err(|_| format!("Invalid minLength value: {}", value))?;
                    validation.min_length = Some(length);
                } else {
                    return Err("minLength requires a value".into());
                }
            }
            "maxLength" | "max_length" => {
                if let Some(value) = rule_value {
                    let length = value
                        .parse::<usize>()
                        .map_err(|_| format!("Invalid maxLength value: {}", value))?;
                    validation.max_length = Some(length);
                } else {
                    return Err("maxLength requires a value".into());
                }
            }
            "min" => {
                if let Some(value) = rule_value {
                    validation.min = Some(value.to_string());
                } else {
                    return Err("min requires a value".into());
                }
            }
            "max" => {
                if let Some(value) = rule_value {
                    validation.max = Some(value.to_string());
                } else {
                    return Err("max requires a value".into());
                }
            }
            "pattern" => {
                if let Some(value) = rule_value {
                    validation.pattern = Some(value.to_string());
                } else {
                    return Err("pattern requires a value".into());
                }
            }
            _ => return Err(format!("Unknown validation rule: {}", rule_name)),
        }
    }
    Ok(())
}
/// Valide le type d'un champ
fn validate_field_type(field_type: &str) -> Result<(), String> {
    let valid_types = [
        // Types de base
        "String",
        "i32",
        "i64",
        "f32",
        "f64",
        "bool",
        "DateTime",
        "Vec<String>",
        "Option<String>",
        "u32",
        "u64",
        "usize",
        // Types personnalisés
        "Email",
        "Phone",
        "Url",
        "Password",
        "Json",
        "Money",
        "Uuid",
        "Slug",
        // Types relationnels
        "HasOne",
        "HasMany",
        "BelongsTo",
        // Types enum
        "Status",
        "Role",
    ];

    if !valid_types.contains(&field_type) {
        return Err(format!(
            "Invalid type: {}. Valid types are: {}",
            field_type,
            valid_types.join(", ")
        ));
    }
    Ok(())
}

// src/generators/entity (suite et fin)

/// Génère le contenu du fichier modèle
fn generate_entity_content(
    name: &str,
    fields: &[(String, String, FieldValidation)],
    relations: &[RelationType],
) -> Result<String, String> {
    let struct_name = name.to_case(Case::Pascal);

    // Génère les imports pour les relations
    let imports = generate_relation_imports(relations);

    // Génère les champs avec leurs validations
    let fields_def = fields
        .iter()
        .map(|(name, type_, validation)| generate_field_with_validation(name, type_, validation))
        .collect::<Vec<_>>()
        .join(",\n\n");

    // Génère les relations
    let relations_def = relations
        .iter()
        .map(generate_relation_field)
        .collect::<Vec<_>>()
        .join(",\n");

    // Génère les initialisations de champs
    let field_inits = fields
        .iter()
        .map(|(name, _, _)| format!("            {}: {}::default()", name, name))
        .collect::<Vec<_>>()
        .join(",\n");

    // Génère les initialisations de relations
    let relation_inits = relations
        .iter()
        .map(generate_relation_init)
        .collect::<Vec<_>>()
        .join(",\n");

    // Génère les champs d'input
    let input_fields = fields
        .iter()
        .map(|(name, type_, validation)| generate_input_field(name, type_, validation))
        .collect::<Vec<_>>()
        .join(",\n");

    // Génère les champs de filtre
    let filter_fields = generate_filter_fields(fields);

    // Génère les filtres de relation
    let relation_filters = generate_relation_filters(relations);

    // Génère les champs de tri
    let sort_fields = fields
        .iter()
        .map(|(name, _, _)| format!("    {}", name.to_case(Case::Pascal)))
        .collect::<Vec<_>>()
        .join(",\n");

    let content = ENTITY_TEMPLATE
        .replace("{imports}", &imports)
        .replace("{name}", &struct_name)
        .replace("{fields}", &fields_def)
        .replace("{relations}", &relations_def)
        .replace("{field_inits}", &field_inits)
        .replace("{relation_inits}", &relation_inits)
        .replace("{input_fields}", &input_fields)
        .replace("{filter_fields}", &filter_fields)
        .replace("{relation_filters}", &relation_filters)
        .replace("{sort_fields}", &sort_fields);

    Ok(content)
}

/// Génère les imports pour les relations
fn generate_relation_imports(relations: &[RelationType]) -> String {
    let mut imports = Vec::new();
    for relation in relations {
        match relation {
            RelationType::HasOne(target)
            | RelationType::HasMany(target)
            | RelationType::BelongsTo(target) => {
                imports.push(format!(
                    "use crate::entities::{}::{};",
                    target.to_lowercase(),
                    target
                ));
            }
        }
    }
    imports.join("\n")
}

/// Génère un champ avec ses validations
fn generate_field_with_validation(name: &str, type_: &str, validation: &FieldValidation) -> String {
    let mut validations = Vec::new();

    if validation.required {
        validations.push("#[validate(required)]".to_string());
    }
    if validation.unique {
        validations.push("#[validate(custom = \"validate_unique\")]".to_string());
    }
    if validation.email {
        validations.push("#[validate(email)]".to_string());
    }
    if validation.url {
        validations.push("#[validate(url)]".to_string());
    }
    if let Some(min) = &validation.min {
        let validation = format!("#[validate(range(min = \"{}\"))]", min);
        validations.push(validation);
    }
    if let Some(max) = &validation.max {
        let validation = format!("#[validate(range(max = \"{}\"))]", max);
        validations.push(validation);
    }
    if let Some(min_length) = validation.min_length {
        let validation = format!("#[validate(length(min = {}))]", min_length);
        validations.push(validation);
    }
    if let Some(max_length) = validation.max_length {
        let validation = format!("#[validate(length(max = {}))]", max_length);
        validations.push(validation);
    }
    if let Some(pattern) = &validation.pattern {
        let validation = format!("#[validate(regex(path = \"{}\"))]", pattern);
        validations.push(validation);
    }

    let validations_str = validations.join("\n    ");
    format!("    {}\n    pub {}: {}", validations_str, name, type_)
}

/// Génère le champ pour une relation
fn generate_relation_field(relation: &RelationType) -> String {
    match relation {
        RelationType::HasOne(target) => {
            format!("    pub {}: Option<{}>", target.to_lowercase(), target)
        }
        RelationType::HasMany(target) => {
            format!("    pub {}: Vec<{}>", target.to_lowercase(), target)
        }
        RelationType::BelongsTo(target) => {
            format!(
                "    pub {}_id: ID,\n    pub {}: Option<{}>",
                target.to_lowercase(),
                target.to_lowercase(),
                target
            )
        }
    }
}

/// Génère l'initialisation pour une relation
fn generate_relation_init(relation: &RelationType) -> String {
    match relation {
        RelationType::HasOne(target) => {
            format!("            {}: None", target.to_lowercase())
        }
        RelationType::HasMany(target) => {
            format!("            {}: Vec::new()", target.to_lowercase())
        }
        RelationType::BelongsTo(target) => {
            format!(
                "            {}_id: ID::default(),\n            {}: None",
                target.to_lowercase(),
                target.to_lowercase()
            )
        }
    }
}

/// Génère un champ d'input avec ses validations
fn generate_input_field(name: &str, type_: &str, validation: &FieldValidation) -> String {
    let mut validations = Vec::new();
    if validation.required {
        validations.push("    #[validate(required)]");
    }
    let validations_str = validations.join("\n");
    format!("    {}\n    pub {}: {}", validations_str, name, type_)
}

/// Génère les champs de filtre
fn generate_filter_fields(fields: &[(String, String, FieldValidation)]) -> String {
    fields
        .iter()
        .map(|(name, type_, _)| match type_.as_str() {
            "String" => format!("    pub {}_contains: Option<String>", name),
            "i32" | "i64" => format!(
                "    pub {}_min: Option<{}>,\n    pub {}_max: Option<{}>",
                name, type_, name, type_
            ),
            "bool" => format!("    pub {}: Option<bool>", name),
            _ => format!("    pub {}: Option<{}>", name, type_),
        })
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Génère les filtres pour les relations
fn generate_relation_filters(relations: &[RelationType]) -> String {
    relations
        .iter()
        .map(|relation| match relation {
            RelationType::HasOne(target) | RelationType::BelongsTo(target) => {
                format!("    pub has_{}: Option<bool>", target.to_lowercase())
            }
            RelationType::HasMany(target) => {
                format!(
                    "    pub has_{}: Option<bool>,\n    pub {}_count_min: Option<i32>,\n    pub {}_count_max: Option<i32>",
                    target.to_lowercase(),
                    target.to_lowercase(),
                    target.to_lowercase()
                )
            }
        })
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Met à jour le fichier mod.rs des entités
fn update_entities_mod(name: &str, relations: &[RelationType]) -> Result<(), String> {
    let mod_path = "backend/src/entities/mod.rs";
    let mut content = fs::read_to_string(mod_path).unwrap_or_else(|_| String::new());

    let module_name = name.to_lowercase();
    let pascal_name = name.to_case(Case::Pascal);

    if !content.contains(&format!("pub mod {};", module_name)) {
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("pub mod {};\n", module_name));
        content.push_str(&format!("pub use {}::{};\n", module_name, pascal_name));

        for relation in relations {
            match relation {
                RelationType::HasOne(target)
                | RelationType::HasMany(target)
                | RelationType::BelongsTo(target) => {
                    if !content.contains(&format!("pub mod {};", target.to_lowercase())) {
                        content.push_str(&format!("pub mod {};\n", target.to_lowercase()));
                        content.push_str(&format!(
                            "pub use {}::{};\n",
                            target.to_lowercase(),
                            target
                        ));
                    }
                }
            }
        }
    }

    fs::write(mod_path, content).map_err(|e| format!("Failed to update entities/mod.rs: {}", e))?;

    Ok(())
}

/// Met à jour le fichier mod.rs des types GraphQL
fn update_graphql_mod(name: &str) -> Result<(), String> {
    let mod_path = "backend/src/graphql/types/mod.rs";
    let mut content = fs::read_to_string(mod_path).unwrap_or_else(|_| String::new());

    let module_name = name.to_lowercase();
    if !content.contains(&format!("pub mod {};", module_name)) {
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("pub mod {};\n", module_name));
        content.push_str(&format!("pub use {}::*;\n", module_name));
    }

    fs::write(mod_path, content)
        .map_err(|e| format!("Failed to update graphql/types/mod.rs: {}", e))?;

    Ok(())
}
