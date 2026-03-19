# CLAUDE.md

## Project Overview

**WRPT** is a Rust CLI tool for deploying and managing Docker Compose stacks on Portainer. It supports both manual usage and CI/CD pipeline integration. Published on [crates.io](https://crates.io/crates/wrpt) and [Docker Hub](https://hub.docker.com/r/wahl/wrpt).

- **Language:** Rust (Edition 2021)
- **Version:** 0.6.3
- **License:** MIT

## Quick Reference Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test --verbose

# Lint
cargo clippy --verbose

# Format
cargo fmt --all --verbose

# Full CI check (matches GitHub Actions)
cargo build --verbose && cargo test --verbose && cargo clippy --verbose && cargo fmt --all --verbose
```

## Project Structure

```
src/
├── main.rs                    # Entry point (minimal)
└── commands/
    ├── mod.rs                 # Command dispatch/routing, CliContext initialization
    ├── wrpt.rs                # CLI args struct, logger init, global args
    ├── consts.rs              # API endpoint path constants
    ├── error.rs               # CliError enum (Config, Api, Io, Http)
    ├── helpers.rs             # Shared utilities (CliContext, HTTP client, table formatting, env parsing)
    ├── stacks/                # Stack management (deploy, remove, list, start, stop, resource-control)
    │   ├── args/              # clap argument definitions
    │   ├── handlers/          # Business logic
    │   └── models/            # Data structures
    ├── endpoints/             # Endpoint listing
    │   ├── args/ handlers/ models/
    ├── teams/                 # Team listing
    │   ├── args/ handlers/ models/
    └── users/                 # User listing
        ├── args/ handlers/ models/
```

## Architecture

Each command domain follows **args → handlers → models**:
- **args/**: CLI argument definitions using `clap::Args` and `clap::Subcommand`
- **handlers/**: Business logic and API calls
- **models/**: Data structures for API requests/responses (with Serde)

Shared utilities live in `helpers.rs` (HTTP client factory, URL construction, table formatting, env file parsing, API response handling).

## Code Conventions

- **Naming:** snake_case for modules/functions, PascalCase for structs/enums, UPPER_SNAKE_CASE for constants
- **Error handling:** Custom `CliError` enum (`Config`, `Api`, `Io`, `Http`) with `Result<T, CliError>` propagation via `?` operator
- **Shared context:** `CliContext` struct holds the reusable HTTP client (with 30s timeout) and base URL, passed to all handlers
- **HTTP:** Centralized `create_client()` in helpers; custom headers for Portainer auth (`x-api-key`)
- **Constants:** Compile-time string formatting via `const_format` crate for API paths
- **Output:** `prettytable-rs` for ASCII table display; `simplelog` with Paris for colored logging
- **Global args:** URL (`-l`/`PORTAINER_URL`), access token (`-A`/`PORTAINER_ACCESS_TOKEN`), `--insecure`, verbosity (`-v`), quiet (`-q`), color control

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` (4.x) | CLI argument parsing with derive macros |
| `reqwest` | HTTP client for Portainer API |
| `serde` / `serde_json` | JSON serialization |
| `prettytable-rs` | ASCII table output |
| `simplelog` / `log` | Logging |
| `chrono` | Date/time handling |
| `const_format` | Compile-time string formatting |

## CI/CD

Three GitHub Actions workflows in `.github/workflows/`:
- **tests.yml**: Build, test, clippy, fmt on push/PR
- **release.yml**: Manual dispatch → Cocogitto SemVer bump → changelog → GitHub release → crates.io publish
- **docker.yml**: Multi-platform Docker build (amd64/arm64) → Docker Hub

## Release Process

Uses [Cocogitto](https://docs.cocogitto.io/) (`cog.toml`) with conventional commits. Pre-bump hooks run test, clippy, and fmt. Post-bump hooks push and publish to crates.io.

## Docker

Multi-stage Dockerfile in `docker/Dockerfile`: Rust build → Debian 12 slim runtime with OpenSSL and Docker Compose.

## Important Notes

- Conventional commits are required (feat:, fix:, docs:, refactor:, etc.)
- Branch whitelist for releases: `main` only
- No `.rustfmt.toml` or `clippy.toml` overrides — uses default Rust toolchain settings
- `.env` files are gitignored; the tool supports parsing them at runtime
