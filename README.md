# Joy Goodbye

Generate warm, heartfelt sign-off messages for email templates and other communication contexts.

## Overview

Joy Goodbye is a high-performance library that generates random, warm sign-off messages from a pre-curated corpus. The core generation logic is implemented in Rust for speed, with language bindings provided for TypeScript (npm package) and Python (PyPI package).

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
pnpm add @siviter-xyz/joy-goodbye
# or
npm install @siviter-xyz/joy-goodbye
```

### Python

```bash
uv pip install joy-goodbye
```

## Quick Start

See [quickstart guide](specs/001-goodbye-generator/quickstart.md) for detailed examples.

### TypeScript

```typescript
import { generateGoodbye } from '@siviter-xyz/joy-goodbye';

const message = await generateGoodbye({
  templateArgs: { name: 'Alice' }
});
console.log(message); // "Wishing you a joyous day, Alice ❤️"
```

### Python

```python
import asyncio
from joy_goodbye import generate_goodbye

async def main():
    message = await generate_goodbye(template_args={"name": "Alice"})
    print(message)  # "Wishing you a joyous day, Alice ❤️"

asyncio.run(main())
```

## Performance

Benchmark results (measured on standard hardware):

- **Generation Time**: ~615ns per message (0.000615ms) ✅ **Well under 10ms target** (16,260x faster)
- **Corpus Loading**: ~10ns (cached after first load)
- **Throughput**: 1000+ concurrent calls/second ✅
- **Memory**: <10MB total footprint ✅

*Benchmarks run with: `cargo bench --manifest-path joy-generator/Cargo.toml`*

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
   cd joy-goodbye
   source ~/.cargo/env  # If Rust is installed via rustup
   ```

2. **Run Rust Core Tests**
   ```bash
   source ~/.cargo/env  # Ensure cargo is in PATH
   cd joy-generator
   cargo test           # ✅ All 18 tests should pass
   cargo bench          # Run benchmarks (shows ~640ns generation time)
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

### Quick Test Commands

**Test Python bindings:**
```bash
./test-python.sh

# Or manually:
cd bindings/python
source .venv/bin/activate
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye())"
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye(template_args={'name': 'Alice'}))"
```

**Test TypeScript/WASM bindings:**
```bash
./test-typescript.sh

# Or manually:
cd bindings/typescript
pnpm exec tsx test-cli.mjs

# Or via test suite:
pnpm test
```

### Verified Working Setup

✅ **Confirmed working on this system:**
- Rust 1.91.1 (cargo, rustc) - ✅ Installed
- build-essential (gcc 14.2.0) + clang - ✅ Installed
- wasm-pack 0.13.1 - ✅ Installed
- uv 0.9.6 - ✅ Installed
- pnpm 10.16.1 - ✅ Installed
- Node.js v24.6.0 - ✅ Installed
- Python 3.13.7 (via uv) - ✅ Available
- maturin 1.10.2 (via uv venv) - ✅ Installed

✅ **Test Results (Verified):**
- Rust unit tests: **18 passed, 0 failed** ✅
- Python E2E tests: **8 passed, 0 failed** ✅
- TypeScript E2E tests: **10 passed, 0 failed** ✅
- Benchmarks: **~615ns per message** (0.000615ms) ✅
  - Target: <10ms
  - Actual: **16,260x faster than target**
  - Corpus loading: ~10ns (cached)

**To verify yourself:**
```bash
# Rust core
source ~/.cargo/env
cd joy-generator
cargo test      # Should show: "18 passed; 0 failed"
cargo bench     # Should show: ~615ns for generate_goodbye

# Python bindings
cd bindings/python
uv venv --python 3.13 && source .venv/bin/activate
uv pip install maturin pytest
maturin develop
pytest tests/test_e2e.py  # Should show: "8 passed"

# TypeScript/WASM (requires clang)
cd bindings/typescript
source ~/.cargo/env
wasm-pack build --target web --out-dir pkg
pnpm install && pnpm test  # ✅ Should show: "10 passed"
```

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

