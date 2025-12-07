# TypeScript API Contract

**Package**: `@siviter-xyz/joyous-departures`  
**Version**: 1.0.0  
**Date**: 2025-01-27

## Installation

```bash
pnpm add @siviter-xyz/joyous-departures
# or
npm install @siviter-xyz/joyous-departures
```

## Type Definitions

### GoodbyeOptions

```typescript
interface GoodbyeOptions {
  /**
   * ISO 639 language code (e.g., "en-GB", "en-US", "fr-FR")
   * Defaults to "en-GB"
   */
  language_code?: string;

  /**
   * Template variable replacements
   */
  templateArgs?: GoodbyeTemplateArgs;

  /**
   * Whether to include emojis in the message
   * Defaults to true
   */
  use_emojis?: boolean;

  /**
   * IANA timezone identifier (e.g., "Europe/London", "America/New_York")
   * Used for default values of {date} and {time} template variables
   * Defaults to "Europe/London"
   */
  timezone?: string;

  /**
   * Optional async translator callback for custom language translation
   * Called when language_code is not natively supported
   * 
   * @param language_code - The requested language code
   * @param message - The message to translate
   * @returns Promise resolving to translated message string, or original message if translation fails
   */
  translator?: (language_code: string, message: string) => Promise<string>;
}
```

### GoodbyeTemplateArgs

```typescript
interface GoodbyeTemplateArgs {
  /**
   * Name to replace {name} placeholder
   * Defaults to "Good Soul" if not provided
   */
  name?: string;

  /**
   * Location to replace {location} placeholder
   * Defaults to "The World" if not provided
   */
  location?: string;

  /**
   * Date to replace {date} placeholder (format: YYYY-MM-DD)
   * Defaults to current date in specified timezone if not provided
   */
  date?: string;

  /**
   * Time to replace {time} placeholder (format: HH:MM)
   * Defaults to current time in specified timezone if not provided
   */
  time?: string;

  /**
   * Additional template variables (future extensibility)
   */
  [key: string]: string | undefined;
}
```

## Function Signature

### generateGoodbye

```typescript
/**
 * Generates a random warm sign-off message
 * 
 * @param options - Optional configuration object
 * @returns Promise resolving to a string containing the generated goodbye message
 * 
 * @example
 * ```typescript
 * import { generateGoodbye } from '@siviter-xyz/joyous-departures';
 * 
 * // Basic usage
 * const message = await generateGoodbye();
 * // "Wishing you a liberated day, Good Soul❤️"
 * 
 * // With custom name
 * const message = await generateGoodbye({ templateArgs: { name: "Alice" } });
 * // "Wishing you a liberated day, Alice❤️"
 * 
 * // With location and timezone
 * const message = await generateGoodbye({
 *   templateArgs: { location: "Paris" },
 *   timezone: "Europe/Paris"
 * });
 * 
 * // Without emojis
 * const message = await generateGoodbye({ use_emojis: false });
 * // "Wishing you a liberated day, Good Soul"
 * 
 * // With async translator
 * const message = await generateGoodbye({
 *   language_code: "fr-FR",
 *   translator: async (lang, msg) => await translateToFrench(msg)
 * });
 * ```
 */
function generateGoodbye(options?: GoodbyeOptions): Promise<string>;
```

## Error Handling

The function may throw the following errors:

### CorpusLoadError

Thrown when the message corpus cannot be loaded or decompressed.

```typescript
class CorpusLoadError extends Error {
  constructor(message: string);
}
```

### InvalidLanguageCodeError

Thrown when an invalid language code format is provided (though this should be handled gracefully with fallback).

```typescript
class InvalidLanguageCodeError extends Error {
  constructor(languageCode: string);
}
```

## Examples

### Basic Usage

```typescript
import { generateGoodbye } from '@siviter-xyz/joyous-departures';

const message = generateGoodbye();
console.log(message);
// "May your path be filled with light, Good Soul✨"
```

### Custom Name

```typescript
const message = generateGoodbye({
  templateArgs: {
    name: "Alice"
  }
});
console.log(message);
// "Wishing you a liberated day, Alice❤️"
```

### No Emojis

```typescript
const message = generateGoodbye({
  use_emojis: false
});
console.log(message);
// "Wishing you a liberated day, Good Soul"
```

### Custom Language with Translator

```typescript
async function translateMessage(lang: string, msg: string): Promise<string> {
  // Call your translation service
  const response = await fetch('https://api.translate.com', {
    method: 'POST',
    body: JSON.stringify({ text: msg, target: lang })
  });
  return response.json().translatedText;
}

const message = await generateGoodbye({
  language_code: "es-ES",
  translator: translateMessage
});
```

### With Date, Time, and Timezone

```typescript
const message = await generateGoodbye({
  templateArgs: {
    name: "Alice",
    location: "New York"
  },
  timezone: "America/New_York"
  // {date} and {time} will use current date/time in New York timezone
});
```

## Performance

- Typical execution time: <10ms
- Thread-safe: Yes (can be called concurrently)
- Memory usage: <10MB (includes corpus)

## Platform Support

- Node.js 18+
- TypeScript 5.9.3+
- Browsers (via WASM)
- Platforms: Linux, macOS, Windows

