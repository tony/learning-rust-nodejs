# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Neon project that demonstrates Rust-Node.js interoperability. Neon allows you to write native Node.js modules in Rust.

## Architecture

The project has a hybrid structure:
- **Node.js side**: Entry point at `lib/index.js` that loads the native addon
- **Rust side**: Native module in `native/` directory containing Rust code compiled to a Node.js addon
- **Build system**: Uses `neon-build` to compile Rust code into a native Node.js module

Key components:
- `native/src/lib.rs`: Main Rust module that exports functions to Node.js using Neon macros
- `native/Cargo.toml`: Rust project configuration specifying Neon dependencies
- `lib/index.js`: JavaScript entry point that requires the compiled native module

## Common Commands

### Building the project
```bash
# Install dependencies and build the native module
pnpm install

# Or build the native module explicitly
neon build
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
# Run the main script to test the native module
node lib/index.js
```

## Development Workflow

1. Rust code changes: Edit files in `native/src/`
2. Rebuild: Run `neon build` or `pnpm install` from the project root
3. Test: Run `node lib/index.js` or integrate the module in your Node.js application

The Neon build system handles the complexity of cross-compilation and binding generation between Rust and Node.js.

## Package Management

This project uses pnpm with corepack for package management. The package manager version is pinned in `package.json` using the `packageManager` field. Corepack ensures everyone uses the same version of pnpm.