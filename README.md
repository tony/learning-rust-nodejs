# learning-rust-nodejs

A Neon project demonstrating Rust-Node.js interoperability. This project uses [Neon](https://neon-bindings.com/) to build native Node.js modules in Rust.

## Prerequisites

- Node.js (>= 10.x)
- Rust (>= 1.85.0)
- pnpm

## Project Structure

```
.
├── lib/
│   └── index.js        # Node.js entry point
├── native/
│   ├── Cargo.toml      # Rust project configuration
│   ├── build.rs        # Rust build script
│   └── src/
│       └── lib.rs      # Rust native module implementation
├── package.json        # Node.js project configuration
└── README.md
```

## Installation

Install dependencies and build the native module:

```bash
pnpm install
```

## Development

### Using npm scripts (recommended)

The root `package.json` acts as a convenient Makefile, wrapping both Node.js and Rust commands:

Development build:
```bash
npm run build:dev
```

Production build:
```bash
npm run build
```

Run tests (runs Rust tests, then Node.js test):
```bash
npm test
```

Lint code:
```bash
npm run lint
```

Fix linting issues:
```bash
npm run lint:fix
```

Check Rust code without building:
```bash
npm run check
```

### Using cargo and npm directly

You can also use the tools directly for more control:

**Rust commands** (run from native/ directory):

| Command                  | Description                  |
|--------------------------|------------------------------|
| `cargo build`            | Development build            |
| `cargo build --release`  | Production build             |
| `cargo test`             | Run Rust tests               |
| `cargo check`            | Check code without building  |
| `cargo clippy`           | Lint Rust code               |
| `cargo fmt`              | Format Rust code             |

First navigate to the native directory:
```bash
cd native
```

**Node.js commands** (run from project root):
```bash
neon build
node lib/index.js
```

- `neon build` - Build native module
- `node lib/index.js` - Run the example

## Testing

The project includes:
- Rust unit tests (currently none defined)
- Node.js integration test (runs the module and verifies output)

Run all tests:
```bash
npm test
```

## Building for Production

```bash
npm run build
```

This creates an optimized release build of the native module.

## How it Works

1. The Rust code in `native/src/lib.rs` defines functions that can be called from JavaScript
2. Neon provides the bridge between Rust and Node.js, handling:
   - Type conversions between Rust and JavaScript
   - Memory management
   - Function registration
3. The built native module is loaded by `lib/index.js` and can be used like any other Node.js module

## Example Usage

The current implementation exports a simple `hello` function:

```javascript
const addon = require('./native');
console.log(addon.hello()); // outputs: "hello node"
```

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Ensure you have the correct Rust version:
   ```bash
   rustc --version  # Should be 1.85.0 or later
   ```

2. Clean and rebuild:
   ```bash
   npm run prebuild  # Cleans Rust build artifacts
   npm run build:dev
   ```

### Platform-specific Issues

Neon builds native modules that are platform-specific. If you're sharing this project:
- Add `native/target/` to `.gitignore`
- Add `native/index.node` to `.gitignore`
- Each developer/deployment needs to build locally

## License

UNLICENSED (private use only)
