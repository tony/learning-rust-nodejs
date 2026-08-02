# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Neon 1.x project that demonstrates Rust-Node.js interoperability. Neon allows you to write native Node.js modules in Rust. Neon 1.x targets Node-API, so the addon links no V8 C++ symbols and remains loadable across Node major versions.

## Architecture

The project has a hybrid structure:
- **Node.js side**: Entry point at `lib/index.js` that loads the native addon
- **Rust side**: Native module in `native/` directory containing Rust code compiled to a Node.js addon
- **Build system**: `cargo build` produces a `cdylib`; `cargo-cp-artifact` copies it to `native/index.node`. There is no build script and no C++ toolchain.

Key components:
- `native/src/lib.rs`: Main Rust module; `#[neon::main]` registers the addon and exports functions to Node.js
- `native/src/logic.rs`: Pure Rust logic, unit tested with `cargo test`
- `native/Cargo.toml`: Rust project configuration specifying Neon dependencies
- `lib/index.js`: JavaScript entry point that requires the compiled native module

## Common Commands

### Building the project
```bash
# Install dependencies
pnpm install
```

```bash
# Build the native module (debug)
pnpm run build:dev
```

```bash
# Build the native module (release)
pnpm run build
```

### Rust development
```bash
# Navigate to the native directory for Rust-specific commands
cd native

# Check code for errors without building
cargo check

# Build the Rust project
cargo build

# Format Rust code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Testing
```bash
# Run Rust unit tests, lints, then the Node.js smoke test
pnpm test
```

```bash
# Run the main script to test the native module
node lib/index.js
```

## Development Workflow

1. Rust code changes: Edit files in `native/src/`
2. Rebuild: Run `pnpm run build:dev` from the project root
3. Test: Run `node lib/index.js` or integrate the module in your Node.js application

Neon handles the type conversions and binding generation between Rust and Node.js; Node-API keeps the resulting binary ABI-stable across Node releases.

## Package Management

This project uses pnpm with corepack for package management. The package manager version is pinned in `package.json` using the `packageManager` field. Corepack ensures everyone uses the same version of pnpm.