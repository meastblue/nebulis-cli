# Nebulis 🚀

[![Crates.io](https://img.shields.io/crates/v/nebulis.svg)](https://crates.io/crates/nebulis)
[![Downloads](https://img.shields.io/crates/d/nebulis.svg)](https://crates.io/crates/nebulis)
[![License](https://img.shields.io/crates/l/nebulis.svg)](https://github.com/yourusername/nebulis/blob/master/LICENSE)


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

You can install Nebulis using Cargo:

```bash
cargo install nebulis
```

Or download pre-built binaries from [GitHub Releases](https://github.com/yourusername/nebulis/releases).

...

### From Source
```bash
cargo install --git https://github.com/meastblue/nebulis-cli.git
```

### From Releases
Download the latest binary from the [releases page](https://github.com/meastblue/nebulis-cli/releases).

## Usage

### Create a new project
```bash
nebulis new my-project
```

### Generate components
```bash
nebulis generate entity User
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
git clone https://github.com/meastblue/nebulis-cli.git
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