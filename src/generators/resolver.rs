// src/generators/resolver.rs
use colored::*;
use convert_case::{Case, Casing};
use std::fs;
use std::path::Path;

pub fn execute(name: &str) -> Result<(), String> {
    println!("{} {}", "Generating resolver:".blue(), name);

    if !Path::new("backend").exists() {
        return Err("Not in a Nebulis project directory".into());
    }

    let resolver_path = format!("backend/src/graphql/resolvers/{}.rs", name.to_lowercase());

    // Créer les répertoires nécessaires
    fs::create_dir_all("backend/src/graphql/resolvers")
        .map_err(|e| format!("Failed to create resolvers directory: {}", e))?;

    // Générer le contenu du resolver
    let resolver_content = generate_resolver_content(name)?;
    fs::write(&resolver_path, resolver_content)
        .map_err(|e| format!("Failed to write resolver file: {}", e))?;

    // Mettre à jour le mod.rs
    update_resolvers_mod(name)?;

    println!("{} Generated files:", "✓".green());
    println!("  - {}", resolver_path);
    Ok(())
}

fn generate_resolver_content(name: &str) -> Result<String, String> {
    let pascal_name = name.to_case(Case::Pascal);
    let snake_name = name.to_lowercase();

    Ok(format!(
        r#"use async_graphql::{{Context, Object, Result, ID}};
use crate::entities::{snake_name}::{pascal_name};
use crate::services::{snake_name}::{pascal_name}Service;

#[derive(Default)]
pub struct {pascal_name};

#[Object]
impl {pascal_name} {{
    async fn get_{snake_name}(&self, ctx: &Context<'_>, id: ID) -> Result<Option<{pascal_name}>> {{
        let service = {pascal_name}Service::new(ctx.data()?);
        service.find_by_id(&id).await
    }}

    async fn list_{snake_name}s(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>
    ) -> Result<Vec<{pascal_name}>> {{
        let service = {pascal_name}Service::new(ctx.data()?);
        service.find_all(limit, offset).await
    }}

    async fn create_{snake_name}(
        &self,
        ctx: &Context<'_>,
        input: {pascal_name}Input
    ) -> Result<{pascal_name}> {{
        let service = {pascal_name}Service::new(ctx.data()?);
        service.create(input).await
    }}

    async fn update_{snake_name}(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: {pascal_name}Input
    ) -> Result<{pascal_name}> {{
        let service = {pascal_name}Service::new(ctx.data()?);
        service.update(&id, input).await
    }}

    async fn delete_{snake_name}(
        &self,
        ctx: &Context<'_>,
        id: ID
    ) -> Result<bool> {{
        let service = {pascal_name}Service::new(ctx.data()?);
        service.delete(&id).await
    }}
}}
"#,
        pascal_name = pascal_name,
        snake_name = snake_name
    ))
}

fn update_resolvers_mod(name: &str) -> Result<(), String> {
    let mod_path = "backend/src/graphql/resolvers/mod.rs";
    let mut content = fs::read_to_string(mod_path).unwrap_or_else(|_| String::new());

    let module_name = name.to_lowercase();
    if !content.contains(&format!("pub mod {};", module_name)) {
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("pub mod {};\n", module_name));
        content.push_str(&format!("pub use {}::*;\n", module_name));
    }

    fs::write(mod_path, content).map_err(|e| format!("Failed to update mod.rs: {}", e))?;

    Ok(())
}
