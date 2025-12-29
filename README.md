<div align="center">
  <img src="https://github.com/siviter-xyz/joyous-departures/raw/main/assets/logo.svg" alt="Joyous Departures" width="256" height="256">
</div>

# Joyous Departures

[![npm version](https://img.shields.io/npm/v/@siviter-xyz/joyous-departures.svg)](https://www.npmjs.com/package/@siviter-xyz/joyous-departures)
[![PyPI version](https://img.shields.io/pypi/v/joyous-departures.svg)](https://pypi.org/project/joyous-departures/)
[![CI](https://github.com/siviter-xyz/joyous-departures/actions/workflows/ci.yml/badge.svg)](https://github.com/siviter-xyz/joyous-departures/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Generate warm, heartfelt sign-off messages for email templates and other communication contexts.

## Overview

A library that generates random, warm sign-off messages from a curated corpus of 90 unique phrases. Available as native implementations for both TypeScript (npm) and Python (PyPI) with zero external dependencies.

> **v2.0** - Now with pure native implementations! No more WASM, no native compilation required.
> Works seamlessly in Cloudflare Workers, browsers, Node.js, and all Python environments.

## Features

- 🚀 **Fast**: Message generation in <1ms (synchronous)
- 🌍 **Universal**: Works everywhere - Cloudflare Workers, browsers, Node.js, Python
- 🎨 **Customizable**: Template variables (`{name}`, `{location}`, `{date}`, `{time}`)
- 😊 **Emoji Support**: Emojis included by default, with option to strip
- 📦 **Zero Dependencies**: Pure TypeScript and Python - no native modules
- 🔄 **Backward Compatible**: v1.x API fully supported

## Template Variables

Messages can include template variables that are replaced with custom values or defaults:

| Variable | Default Value | Format | Description |
|----------|--------------|--------|-------------|
| `{name}` | `"Good Soul"` | Any string (max 50 chars) | Recipient's name |
| `{location}` | `"The World"` | Any string (max 100 chars) | Location context |
| `{date}` | Current date | `YYYY-MM-DD` | Current date in specified timezone |
| `{time}` | Current time | `HH:MM` | Current time in specified timezone |

## Installation

### TypeScript/JavaScript

```bash
pnpm add @siviter-xyz/joyous-departures
# or
npm install @siviter-xyz/joyous-departures
```

### Python

```bash
pip install joyous-departures
# or
uv pip install joyous-departures
```

## Quick Start

### TypeScript

```typescript
import { generateGoodbye, generateGoodbyeSync } from '@siviter-xyz/joyous-departures';

// Async (v1.x compatible)
const message = await generateGoodbye({
  templateArgs: { name: 'Alice' }
});
console.log(message); // "Wishing you a joyous day, Alice ❤️"

// Sync (new in v2.0)
const syncMessage = generateGoodbyeSync({
  templateArgs: { name: 'Bob' },
  stripEmojis: true
});
console.log(syncMessage); // "Wishing you a joyous day, Bob"
```

### Python

```python
import asyncio
from joyous_departures import generate_goodbye, generate_goodbye_sync

# Async (v1.x compatible)
async def main():
    message = await generate_goodbye(template_args={"name": "Alice"})
    print(message)  # "Wishing you a joyous day, Alice ❤️"

asyncio.run(main())

# Sync (new in v2.0)
message = generate_goodbye_sync(template_args={"name": "Bob"}, strip_emojis=True)
print(message)  # "Wishing you a joyous day, Bob"
```

### Cloudflare Workers

```typescript
import { generateGoodbyeSync } from '@siviter-xyz/joyous-departures';

export default {
  async fetch(request: Request): Promise<Response> {
    const message = generateGoodbyeSync({ templateArgs: { name: 'World' } });
    return new Response(message);
  }
};
```

## API Reference

### TypeScript

```typescript
// Async function (v1.x compatible)
function generateGoodbye(options?: GoodbyeOptions): Promise<string>;

// Sync function (new in v2.0)
function generateGoodbyeSync(options?: GoodbyeOptions): string;

interface GoodbyeOptions {
  templateArgs?: {
    name?: string;      // Max 50 chars, default: "Good Soul"
    location?: string;  // Max 100 chars, default: "The World"
    date?: string;      // Format: YYYY-MM-DD
    time?: string;      // Format: HH:MM
  };
  use_emojis?: boolean;   // v1.x compat, default: true
  stripEmojis?: boolean;  // v2.0, default: false
  timezone?: string;      // IANA timezone, default: "Europe/London"
  translator?: (lang: string, msg: string) => Promise<string>;
}
```

### Python

```python
# Async function (v1.x compatible)
async def generate_goodbye(
    template_args: dict[str, str] | None = None,
    use_emojis: bool = True,
    strip_emojis: bool = False,
    timezone: str = "Europe/London",
    translator: Callable[[str, str], Awaitable[str]] | None = None,
) -> str: ...

# Sync function (new in v2.0)
def generate_goodbye_sync(
    template_args: dict[str, str] | None = None,
    use_emojis: bool = True,
    strip_emojis: bool = False,
    timezone: str = "Europe/London",
) -> str: ...
```

## Local Development

### Prerequisites

- **Node.js 18+** and pnpm (for TypeScript)
- **Python 3.10+** and uv (for Python)

### Setup

```bash
git clone https://github.com/siviter-xyz/joyous-departures.git
cd joyous-departures

# TypeScript
cd packages/typescript
pnpm install
pnpm test

# Python
cd packages/python
uv venv --python 3.13
source .venv/bin/activate
uv pip install -e ".[dev]"
pytest tests/ -v
```

### Project Structure

```
joyous-departures/
├── packages/
│   ├── typescript/          # npm package
│   │   ├── src/
│   │   │   ├── index.ts     # Main library
│   │   │   └── corpus.generated.ts
│   │   └── tests/
│   └── python/              # PyPI package
│       ├── src/joyous_departures/
│       │   ├── __init__.py  # Main library
│       │   └── corpus.py
│       └── tests/
├── corpus/
│   └── en-GB.txt            # Source message corpus
├── scripts/
│   └── generate-corpus.ts   # Corpus code generator
├── examples/
│   ├── ts-usage/            # TypeScript example
│   └── py-usage/            # Python example
└── _archive/                # Archived v1.x Rust/WASM code
```

### Scripts

- **`scripts/generate-corpus.ts`** - Generate corpus constants from text files
- **`scripts/lint.sh`** - Run linting checks

### Corpus Management

The corpus is stored in `corpus/en-GB.txt` and code-generated into native constants:

```bash
# Regenerate corpus constants after editing corpus/en-GB.txt
npx tsx scripts/generate-corpus.ts
```

## Version Management

The project uses [semantic-release](https://github.com/semantic-release/semantic-release) for automated versioning:

- **`feat:`** → minor version bump (1.0.0 → 1.1.0)
- **`fix:`** → patch version bump (1.0.0 → 1.0.1)
- **`feat!:`** or **`BREAKING CHANGE:`** → major version bump (1.0.0 → 2.0.0)

## Migration from v1.x

v2.0 is fully backward compatible. Your existing code will work without changes.

**New in v2.0:**
- `generateGoodbyeSync()` / `generate_goodbye_sync()` - synchronous variants
- `stripEmojis` option (in addition to `use_emojis`)
- No WASM, no native compilation - works everywhere
- Cloudflare Workers compatible

## Documentation

- [Specification](specs/003-native-dual-lib/spec.md)
- [Implementation Plan](specs/003-native-dual-lib/plan.md)

## License

MIT
