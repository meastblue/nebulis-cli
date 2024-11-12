# Nebulis 🚀

Nebulis is a CLI tool for bootstrapping full-stack applications with a Rust backend (Axum + GraphQL + SurrealDB) and a Remix frontend (Deno 2).

## Features

- 🦀 **Rust Backend**
  - Axum web framework
  - GraphQL with async-graphql
  - SurrealDB database
  - Modular architecture
  - Migration system

- 🎭 **Remix Frontend**
  - Deno 2 runtime
  - TypeScript support
  - Tailwind CSS
  - Ready-to-use project structure

- 🐳 **Docker Integration**
  - SurrealDB container
  - Development environment
  - Easy deployment

## Installation

### From Source
```bash
cargo install --git https://github.com/yourusername/nebulis.git
```

### From Releases
Download the latest binary from the [releases page](https://github.com/yourusername/nebulis/releases).

## Usage

### Create a new project
```bash
nebulis new my-project
```

### Generate components
```bash
nebulis generate model User
nebulis generate migration CreateUsers
nebulis generate resolver UserResolver
```

### Project Structure
```
my-project/
├── backend/
│   ├── src/
│   │   ├── db/
│   │   ├── entities/
│   │   ├── graphql/
│   │   ├── repositories/
│   │   ├── services/
│   │   └── utils/
│   └── Cargo.toml
├── frontend/
│   ├── app/
│   ├── public/
│   └── package.json
└── docker-compose.yml
```

## Development

### Requirements
- Rust
- Deno
- Docker
- Git

### Building from source
```bash
git clone https://github.com/yourusername/nebulis.git
cd nebulis
cargo build --release
```

### Running tests
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.