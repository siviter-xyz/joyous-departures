# Quickstart Guide: joyous-departures v2.0

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
# or
pip install joyous-departures
```

## Basic Usage

### TypeScript

```typescript
import { generateGoodbye } from '@siviter-xyz/joyous-departures';

// Simple usage (synchronous!)
const message = generateGoodbye();
console.log(message);
// => "Wishing you a joyous day, Good Soul ❤️"

// With name
const personalMessage = generateGoodbye({
  templateArgs: { name: 'Alice' }
});
console.log(personalMessage);
// => "May your path be filled with joy, Alice 🌻"
```

### Python

```python
from joyous_departures import generate_goodbye_sync, generate_goodbye
import asyncio

# Synchronous usage
message = generate_goodbye_sync()
print(message)
# => "Wishing you a joyous day, Good Soul ❤️"

# With name
message = generate_goodbye_sync(template_args={"name": "Alice"})
print(message)
# => "May your path be filled with joy, Alice 🌻"

# Async usage (for async codebases)
async def main():
    message = await generate_goodbye(template_args={"name": "Alice"})
    print(message)

asyncio.run(main())
```

## Options

### Template Variables

Both implementations support these template variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `{name}` | "Good Soul" | Recipient's name |
| `{location}` | "The World" | Location context |
| `{date}` | Current date | Date in YYYY-MM-DD format |
| `{time}` | Current time | Time in HH:MM format |

```typescript
// TypeScript
const message = generateGoodbye({
  templateArgs: {
    name: 'Alice',
    location: 'London',
  }
});
// => "Sending warmth from London, Alice 💙"
```

```python
# Python
message = generate_goodbye_sync(
    template_args={
        "name": "Alice",
        "location": "London",
    }
)
# => "Sending warmth from London, Alice 💙"
```

### Emoji Stripping

Remove emojis from the output:

```typescript
// TypeScript
const message = generateGoodbye({ stripEmojis: true });
// => "Wishing you a joyous day, Good Soul"
```

```python
# Python
message = generate_goodbye_sync(strip_emojis=True)
# => "Wishing you a joyous day, Good Soul"
```

### Timezone

Specify timezone for `{date}` and `{time}` variables:

```typescript
// TypeScript
const message = generateGoodbye({
  timezone: 'America/New_York'
});
```

```python
# Python
message = generate_goodbye_sync(timezone="America/New_York")
```

### Custom Translation

Use a translator callback for custom translations:

```typescript
// TypeScript (async)
import { generateGoodbyeAsync } from '@siviter-xyz/joyous-departures';

const message = await generateGoodbyeAsync({
  translator: async (msg) => {
    // Your translation logic
    return translateToFrench(msg);
  }
});
```

```python
# Python (async)
async def translate_to_french(message: str) -> str:
    # Your translation logic
    return await call_translation_api(message, "fr")

message = await generate_goodbye(translator=translate_to_french)
```

## Migration from v1.x

### Key Changes

1. **Synchronous by default**: `generateGoodbye()` is now synchronous in TypeScript (no `await` needed!)
2. **No WASM**: No more initialization issues in Cloudflare Workers
3. **Pure Python**: No Rust compilation required

### TypeScript Migration

```typescript
// v1.x (async, WASM)
const message = await generateGoodbye({ templateArgs: { name: 'Alice' } });

// v2.0 (sync, native)
const message = generateGoodbye({ templateArgs: { name: 'Alice' } });

// v2.0 with translator (async)
const message = await generateGoodbyeAsync({
  templateArgs: { name: 'Alice' },
  translator: myTranslator,
});
```

### Python Migration

```python
# v1.x (required Rust/PyO3)
# pip install joyous-departures  # Needed Rust toolchain

# v2.0 (pure Python)
# pip install joyous-departures  # No build tools needed!

# API remains the same
message = await generate_goodbye(template_args={"name": "Alice"})
```

## Platform Support

v2.0 works everywhere:

- ✅ Node.js
- ✅ Deno
- ✅ Bun
- ✅ Browsers
- ✅ **Cloudflare Workers** (no more WASM issues!)
- ✅ Python 3.10+
- ✅ PyPy

