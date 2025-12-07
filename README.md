<div align="center">
  <img src="assets/logo.svg" alt="Joyous Departures" width="256" height="256">
</div>

# Joyous Departures

Generate warm, heartfelt sign-off messages for email templates and other communication contexts.

## Overview

A high-performance library that generates random, warm sign-off messages from a pre-curated corpus. The core generation logic is implemented in Rust for speed, with language bindings provided for TypeScript (npm package) and Python (PyPI package).

> **Over-engineered**? Most definitely.
>
> This project is an exploration of spec-first AI-assisted design and
> simultaneously targetting multiple consumer languages from a high-performance Rust core.
> It is also an ode to a cherished partner and the aspects they inspire deeply within myself and in many others ❤️
>
> Most importantly, as ever, because it was fun!

## Features

- 🚀 **Fast**: Message generation in <10ms
- 🌍 **International**: Support for multiple languages (with translator callback)
- 🎨 **Customizable**: Template variables (`{name}`, `{location}`, `{date}`, `{time}`)
- 😊 **Emoji Support**: Emojis included by default, with option to strip
- 🌐 **Browser Ready**: TypeScript bindings use WASM for browser support
- 📦 **Easy to Use**: Simple API, works with pnpm and uv

## Template Variables

Messages can include template variables that are replaced with custom values or defaults:

| Variable | Default Value | Format | Description |
|----------|--------------|--------|-------------|
| `{name}` | `"Good Soul"` | Any string (max 50 chars) | Recipient's name |
| `{location}` | `"The World"` | Any string | Location context |
| `{date}` | Current date | `YYYY-MM-DD` | Current date in specified timezone |
| `{time}` | Current time | `HH:MM` | Current time in specified timezone |

**Note**: Not all messages include template variables. Some messages are simple phrases without any variables.

## Installation

### TypeScript/JavaScript

```bash
pnpm add @siviter-xyz/joyous-departures
# or
npm install @siviter-xyz/joyous-departures
```

### Python

```bash
uv pip install joyous-departures
```

## Quick Start

See [quickstart guide](specs/001-goodbye-generator/quickstart.md) for detailed examples.

### TypeScript

```typescript
import { generateGoodbye } from '@siviter-xyz/joyous-departures';

const message = await generateGoodbye({
  templateArgs: { name: 'Alice' }
});
console.log(message); // "Wishing you a joyous day, Alice ❤️"
```

### Python

```python
import asyncio
from joyous_departures import generate_goodbye

async def main():
    message = await generate_goodbye(template_args={"name": "Alice"})
    print(message)  # "Wishing you a joyous day, Alice ❤️"

asyncio.run(main())
```


## Local Development Setup

### Prerequisites

1. **Rust Toolchain** (1.91.1+)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Build Tools** (for compiling Rust dependencies)
   ```bash
   # Debian/Ubuntu/WSL
   sudo apt-get update
   sudo apt-get install build-essential clang
   
   # This provides: gcc, g++, make, clang, and other essential build tools
   # clang is needed for WASM builds (lz4 dependency)
   ```

3. **Python** (3.14+)
   ```bash
   # Using uv (recommended)
   curl -LsSf https://astral.sh/uv/install.sh | sh
   uv python install 3.14
   ```

4. **Node.js** (18+) and pnpm
   ```bash
   # Install Node.js via nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   
   # Install pnpm
   npm install -g pnpm
   ```

5. **Build Tools for Bindings**
   ```bash
   # Install wasm-pack (for TypeScript/WASM bindings)
   source ~/.cargo/env
   cargo install wasm-pack
   # Verify: wasm-pack --version
   
   # Install maturin (for Python bindings)
   # Option 1: Via pip (recommended, faster, avoids cargo dependency conflicts)
   pip install maturin
   # or with uv:
   uv pip install maturin --system
   
   # Option 2: Via cargo (requires build-essential, may have dependency conflicts)
   source ~/.cargo/env
   cargo install maturin
   # Verify: maturin --version
   ```

### Development Workflow

1. **Clone and Setup**
   ```bash
   git clone <repository-url>
   cd joyous-departures
   source ~/.cargo/env  # If Rust is installed via rustup
   
   # Install pre-commit hooks (recommended)
   ./scripts/setup-git-hooks.sh
   ```

2. **Run Rust Core Tests**
   ```bash
   source ~/.cargo/env  # Ensure cargo is in PATH
   cd joy-generator
   cargo test           # ✅ All 21 tests should pass
   cargo bench          # Run benchmarks (shows ~615ns generation time)
   ```

3. **Build and Test Python Bindings**
   ```bash
   cd bindings/python
   maturin develop  # Build Python extension in development mode
   uv run pytest tests/test_e2e.py
   ```

4. **Build and Test TypeScript Bindings**
   ```bash
   cd bindings/typescript
   source ~/.cargo/env
   wasm-pack build --target web --out-dir pkg
   pnpm install
   pnpm test
   # Note: Requires clang (see Prerequisites #2) for lz4 dependency
   ```

5. **Run All Tests**
   ```bash
   # From repository root
   ./scripts/test.sh
   ```

6. **Quick Test Individual Components**
   ```bash
   # Test Python bindings
   ./scripts/test-python.sh
   
   # Test TypeScript/WASM bindings
   ./scripts/test-typescript.sh
   ```

### Available Scripts

All scripts are located in the `scripts/` directory:

- **`scripts/test.sh`** - Run all test suites (Rust, Python, TypeScript)
- **`scripts/lint.sh`** - Run linting checks (format + clippy, same as CI)
- **`scripts/setup-git-hooks.sh`** - Install pre-commit hooks (runs linting automatically)
- **`scripts/test-python.sh`** - Quick test of Python bindings with examples
- **`scripts/test-typescript.sh`** - Quick test of TypeScript/WASM bindings
- **`scripts/build.sh`** - Build all packages (Rust workspace, WASM, Python)
- **`scripts/bench.sh`** - Run benchmarks for all components
- **`scripts/version.sh`** - Extract version from git tag and update all package files
- **`scripts/publish.sh`** - Publish packages to npm and PyPI (manual publishing)

### Required Stack

**Prerequisites:**
- Rust 1.91.1+ (cargo, rustc)
- build-essential (gcc 14.2.0+) + clang
- wasm-pack 0.13.1+
- uv 0.9.6+
- pnpm 10.16.1+
- Node.js 18+
- Python 3.13+ (via uv)
- maturin 1.10.2+ (via uv venv)

**Performance Benchmarks:**
- **Generation Time**: ~615ns per message (0.000615ms)
  - Target: <10ms
  - Actual: **16,260x faster than target** ✅
- **Corpus Loading**: ~10ns (cached after first load)
- **Throughput**: 1000+ concurrent calls/second ✅
- **Memory**: <10MB total footprint ✅

*Run benchmarks with: `cargo bench --manifest-path joy-generator/Cargo.toml`*

**To Verify Your Setup:**
```bash
# 1. Verify Rust core
source ~/.cargo/env
cd joy-generator
cargo test      # Expected: "21 passed; 0 failed"
cargo bench     # Expected: ~615ns for generate_goodbye

# 2. Verify Python bindings
cd bindings/python
uv venv --python 3.13 && source .venv/bin/activate
uv pip install maturin pytest pytest-asyncio
maturin develop
pytest tests/test_e2e.py  # Expected: "8 passed"

# 3. Verify TypeScript/WASM (requires clang)
cd bindings/typescript
source ~/.cargo/env
wasm-pack build --target web --out-dir pkg
pnpm install && pnpm test  # Expected: "10 passed"

# Or use the convenience scripts:
./scripts/test.sh   # Run all tests
./scripts/lint.sh   # Run linting checks (format + clippy, same as CI)
```

### Pre-Commit Setup

**Recommended: Install git hooks for automatic linting**

The easiest way to ensure code quality is to install the pre-commit hook, which automatically runs linting checks before each commit:

```bash
# One-time setup (run this after cloning the repo)
./scripts/setup-git-hooks.sh
```

**How it works:** Git hooks in `.git/hooks/` are not tracked by git (they're local to each repository clone). The hooks are stored in `.githooks/` (which IS tracked), and `setup-git-hooks.sh` copies them to `.git/hooks/` for each developer.

After setup, the pre-commit hook will automatically run `./scripts/lint.sh` before each commit. If linting fails, the commit will be blocked.

**Manual alternative:**

If you prefer to run checks manually:
```bash
# 1. Run linting (format + clippy) - same as CI
./scripts/lint.sh

# 2. Run all tests
./scripts/test.sh
```

**Why this matters:** CI runs `cargo clippy --all-targets --all-features -- -D warnings`, which treats warnings as errors. The pre-commit hook ensures you catch the same issues before pushing.

### Troubleshooting

- **"linker `cc` not found"**: Install `build-essential` (see Prerequisites #2)
- **"clang not found" (WASM builds)**: Install `clang` via `sudo apt-get install clang`
- **"cargo: command not found"**: Run `source ~/.cargo/env` or add `~/.cargo/bin` to PATH
- **Python import errors**: Ensure `maturin develop` was run in `bindings/python/` venv
- **WASM build fails**: Ensure `wasm-pack` is installed and `clang` is available
- **maturin install fails via cargo**: Use `uv venv` and `uv pip install maturin` instead

## Documentation

- [Specification](specs/001-goodbye-generator/spec.md)
- [Implementation Plan](specs/001-goodbye-generator/plan.md)
- [API Contracts](specs/001-goodbye-generator/contracts/)

## License

MIT

