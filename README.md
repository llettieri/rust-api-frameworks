# rust-api-frameworks

A curated collection and exploration of Rust web API frameworks, with code samples, benchmarks, and hands-on developer
notes.

## Overview

This project explores popular Rust web API frameworks through real-world experiments. The goal is to compare their
ergonomics, performance, and productivity, offering a solid reference for backend and API developers evaluating Rust for
their next project.

## Frameworks Covered

- [Actix-web](https://actix.rs/) – High-performance, actor-based web framework
- [Axum](https://docs.rs/axum/latest/axum/) – Modular, tower-based HTTP server framework

## Features

- Side-by-side comparison of framework setup and idioms
- Real-world use cases: CRUD APIs, authentication, OpenAPI/Swagger integration
- Code quality and ergonomics notes
- API documentation generation examples (utoipa)
- Tips for Dockerization, testing, and async best practices

## Getting Started

### Project Structure

| Directory                   | Description               |
|-----------------------------|---------------------------|
| `vehicle-manager-actix-web` | Actix-web example project |
| `vehicle-manager-axum`      | Axum example project      |

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/llettieri/rust-api-frameworks.git
   cd rust-api-frameworks
   ```

2. Pick a framework folder and run examples:
   ```bash
   cd vehicle-manager-actix-web
   cargo run
   ```

## Contributing

Ideas, improvements, and examples are welcome! Open an issue or submit a pull request.

## License

MIT

