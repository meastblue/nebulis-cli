pub fn get_package_json_template(project_name: &str) -> String {
    format!(
        r#"{{
  "name": "{}_frontend",
  "private": true,
  "sideEffects": false,
  "type": "module",
  "scripts": {{
    "build": "remix build",
    "dev": "remix dev",
    "start": "remix-serve build/index.js",
    "typecheck": "tsc"
  }},
  "dependencies": {{
    "@remix-run/css-bundle": "^2.5.0",
    "@remix-run/deno": "^2.5.0",
    "@remix-run/react": "^2.5.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "urql": "^4.0.0"
  }},
  "devDependencies": {{
    "@remix-run/dev": "^2.5.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "typescript": "^5.1.0"
  }},
  "engines": {{
    "node": ">=18.0.0"
  }}
}}"#,
        project_name
    )
}
