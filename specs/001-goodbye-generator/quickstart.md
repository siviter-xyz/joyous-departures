# Quickstart Guide: Goodbye Message Generator

**Date**: 2025-01-27  
**Feature**: 001-goodbye-generator

## Overview

This guide helps you get started with the Goodbye Message Generator library in your preferred language (TypeScript or Python).

## TypeScript/JavaScript Quickstart

### Installation

```bash
# Using pnpm (recommended)
pnpm add @siviter-xyz/joy-goodbye

# Using npm
npm install @siviter-xyz/joy-goodbye
```

### Basic Usage

```typescript
import { generateGoodbye } from '@siviter-xyz/joy-goodbye';

// Generate a message with defaults
const message = generateGoodbye();
console.log(message);
// Output: "Wishing you a liberated day, Good Soul❤️"
```

### Customization

```typescript
// Custom name
const message = generateGoodbye({
  templateArgs: { name: "Alice" }
});
// Output: "Wishing you a liberated day, Alice❤️"

// Without emojis
const message = generateGoodbye({
  use_emojis: false
});
// Output: "Wishing you a liberated day, Good Soul"

// Different language (requires translator)
const message = generateGoodbye({
  language_code: "fr-FR",
  translator: (lang, msg) => translateToFrench(msg)
});
```

### Integration Example

```typescript
// email-template.ts
import { generateGoodbye } from '@siviter-xyz/joy-goodbye';

export function generateEmailBody(recipientName: string): string {
  const body = `
    Dear ${recipientName},
    
    Thank you for your inquiry. We'll get back to you soon.
    
    ${generateGoodbye({ templateArgs: { name: recipientName } })}
  `;
  return body;
}
```

## Python Quickstart

### Installation

```bash
# Using uv (recommended)
uv pip install joy-goodbye

# Using pip
pip install joy-goodbye
```

### Basic Usage

```python
import joy_goodbye

# Generate a message with defaults
message = joy_goodbye.generate_goodbye()
print(message)
# Output: "Wishing you a liberated day, Good Soul❤️"
```

### Customization

```python
# Custom name
message = joy_goodbye.generate_goodbye(
    template_args={"name": "Alice"}
)
# Output: "Wishing you a liberated day, Alice❤️"

# Without emojis
message = joy_goodbye.generate_goodbye(use_emojis=False)
# Output: "Wishing you a liberated day, Good Soul"

# Different language (requires translator)
def translate_to_french(lang: str, msg: str) -> str:
    return translate_sync(msg, target_lang=lang)

message = joy_goodbye.generate_goodbye(
    language_code="fr-FR",
    translator=translate_to_french
)
```

### Integration Example

```python
# email_template.py
import joy_goodbye

def generate_email_body(recipient_name: str) -> str:
    body = f"""
    Dear {recipient_name},
    
    Thank you for your inquiry. We'll get back to you soon.
    
    {joy_goodbye.generate_goodbye(template_args={"name": recipient_name})}
    """
    return body
```

## Common Use Cases

### Email Signatures

```typescript
// Add to email template
const signature = generateGoodbye({
  templateArgs: { name: recipientName },
  use_emojis: false  // Professional emails
});
```

### Chat Messages

```python
# Add to chat bot responses
goodbye = joy_goodbye.generate_goodbye(
    template_args={"name": user.display_name},
    use_emojis=True  # Casual context
)
```

### Multi-language Support

```typescript
// With translation service
const message = generateGoodbye({
  language_code: user.preferredLanguage,
  translator: async (lang, msg) => {
    const response = await fetch('/api/translate', {
      method: 'POST',
      body: JSON.stringify({ text: msg, target: lang })
    });
    return response.json().translatedText;
  }
});
```

## Performance Tips

- The library loads the message corpus on first use (one-time cost ~50ms)
- Subsequent calls are fast (<10ms)
- The library is thread-safe and can be called concurrently
- Consider pre-warming by calling the function once at application startup

## Troubleshooting

### Installation Issues

**TypeScript**: Ensure Node.js 18+ is installed
```bash
node --version  # Should be 18.0.0 or higher
```

**Python**: Ensure Python 3.8+ is installed
```bash
python --version  # Should be 3.8.0 or higher
```

### Runtime Errors

**CorpusLoadError**: The message corpus failed to load. This is a critical error and the library will not function. Please file an issue.

**InvalidLanguageCodeError**: The provided language code format is invalid. Use ISO 639 format (e.g., "en-GB", "fr-FR").

## Next Steps

- Read the full API documentation in `contracts/`
- Check out the data model in `data-model.md`
- Review the implementation plan in `plan.md`

