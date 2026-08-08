# learning-rust-nodejs

A Neon project demonstrating Rust-Node.js interoperability. This project uses [Neon](https://neon-bindings.com/) 1.x to build native Node.js modules in Rust.

Neon 1.x is built on [Node-API](https://nodejs.org/api/n-api.html), so the compiled addon links no V8 C++ symbols and stays loadable across Node major versions without a rebuild.

## Prerequisites

- Node.js (>= 22.13)
- Rust (>= 1.85.0)
- pnpm

## Project Structure

```
.
├── lib/
│   └── index.js        # Node.js entry point
├── native/
│   ├── Cargo.toml      # Rust project configuration
│   └── src/
│       ├── lib.rs      # Rust native module implementation
│       └── logic.rs    # Pure Rust logic (unit tested)
├── package.json        # Node.js project configuration
└── README.md
```

## Installation

Install dependencies:

```bash
pnpm install
```

Then build the native module:

```bash
pnpm run build:dev
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
pnpm run build:dev
node lib/index.js
```

- `pnpm run build:dev` - Build native module
- `node lib/index.js` - Run the example

The build scripts wrap [`cargo-cp-artifact`](https://www.npmjs.com/package/cargo-cp-artifact), which runs `cargo build` and copies the resulting `cdylib` to `native/index.node`.

## Testing

The project includes:
- Rust unit tests in `native/src/logic.rs`
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
2. `#[neon::main]` marks the module entry point; it registers the addon with Node-API and exports each function via `cx.export_function`
3. Neon provides the bridge between Rust and Node.js, handling:
   - Type conversions between Rust and JavaScript
   - Memory management
   - Function registration
4. The built native module is loaded by `lib/index.js` and can be used like any other Node.js module

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
   cd native && cargo clean
   ```

   ```bash
   npm run build:dev
   ```

### Platform-specific Issues

Neon builds native modules that are platform-specific. If you're sharing this project:
- Add `native/target/` to `.gitignore`
- Add `native/index.node` to `.gitignore`
- Each developer/deployment needs to build locally

## License

UNLICENSED (private use only)
