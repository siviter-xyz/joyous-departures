# Joyous Departures

[![npm version](https://img.shields.io/npm/v/@siviter-xyz/joyous-departures.svg)](https://www.npmjs.com/package/@siviter-xyz/joyous-departures)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Generate warm, heartfelt sign-off messages for email templates and other communication contexts.

## Installation

```bash
npm install @siviter-xyz/joyous-departures
# or
pnpm add @siviter-xyz/joyous-departures
```

## Quick Start

```typescript
import { generateGoodbye, generateGoodbyeSync } from '@siviter-xyz/joyous-departures';

// Async
const message = await generateGoodbye({
  templateArgs: { name: 'Alice' }
});
console.log(message); // "Wishing you a joyous day, Alice ❤️"

// Sync
const syncMessage = generateGoodbyeSync({
  templateArgs: { name: 'Bob' },
  stripEmojis: true
});
console.log(syncMessage); // "Wishing you a joyous day, Bob"
```

## Features

- 🚀 **Fast** - Synchronous generation in <1ms
- 🌍 **Universal** - Works in Node.js, browsers, Cloudflare Workers
- 🎨 **Customizable** - Template variables (`{name}`, `{location}`, `{date}`, `{time}`)
- 😊 **Emoji Support** - Included by default, with option to strip
- 📦 **Zero Dependencies** - Pure TypeScript, no native modules

## API

### `generateGoodbye(options?): Promise<string>`

Async function for generating messages. Supports optional translator callback.

### `generateGoodbyeSync(options?): string`

Synchronous function for generating messages.

### Options

```typescript
interface GoodbyeOptions {
  templateArgs?: {
    name?: string;      // Max 50 chars, default: "Good Soul"
    location?: string;  // Max 100 chars, default: "The World"
    date?: string;      // Format: YYYY-MM-DD
    time?: string;      // Format: HH:MM
  };
  use_emojis?: boolean;   // Default: true
  stripEmojis?: boolean;  // Default: false
  timezone?: string;      // IANA timezone, default: "Europe/London"
  translator?: (lang: string, msg: string) => Promise<string>;
}
```

## Template Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `{name}` | "Good Soul" | Recipient's name (max 50 chars) |
| `{location}` | "The World" | Location context (max 100 chars) |
| `{date}` | Current date | Format: YYYY-MM-DD |
| `{time}` | Current time | Format: HH:MM |

## License

MIT

## Links

- [GitHub Repository](https://github.com/siviter-xyz/joyous-departures)
- [Python Package (PyPI)](https://pypi.org/project/joyous-departures/)

