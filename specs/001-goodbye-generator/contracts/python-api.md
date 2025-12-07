# Python API Contract

**Package**: `joyous-departures`  
**Version**: 1.0.0  
**Date**: 2025-01-27

## Installation

```bash
uv pip install joyous-departures
# or
pip install joyous-departures
```

## Type Definitions

### GoodbyeOptions

```python
from typing import Optional, Callable, Dict, Awaitable

class GoodbyeOptions:
    """
    Options for customizing goodbye message generation
    
    Attributes:
        language_code: ISO 639 language code (e.g., "en-GB", "en-US", "fr-FR")
                      Defaults to "en-GB"
        template_args: Dictionary of template variable replacements
                      Supports "name" (defaults to "Good Soul"), "location" (defaults to "The World"),
                      "date" (defaults to current date), "time" (defaults to current time)
        use_emojis: Whether to include emojis in the message
                   Defaults to True
        timezone: IANA timezone identifier (e.g., "Europe/London", "America/New_York")
                 Used for default values of {date} and {time} template variables
                 Defaults to "Europe/London"
        translator: Optional async translator callback for custom language translation
                   Called when language_code is not natively supported
                   Signature: async (language_code: str, message: str) -> str
    """
    language_code: Optional[str] = None
    template_args: Optional[Dict[str, str]] = None
    use_emojis: bool = True
    timezone: Optional[str] = None
    translator: Optional[Callable[[str, str], Awaitable[str]]] = None
```

## Function Signature

### generate_goodbye

```python
async def generate_goodbye(
    language_code: Optional[str] = None,
    template_args: Optional[Dict[str, str]] = None,
    use_emojis: bool = True,
    timezone: Optional[str] = None,
    translator: Optional[Callable[[str, str], Awaitable[str]]] = None
) -> str:
    """
    Generates a random warm sign-off message
    
    Args:
        language_code: ISO 639 language code (defaults to "en-GB")
        template_args: Dictionary of template variable replacements
                      Supports "name" (defaults to "Good Soul"), "location" (defaults to "The World"),
                      "date" (defaults to current date), "time" (defaults to current time)
        use_emojis: Whether to include emojis (defaults to True)
        timezone: IANA timezone identifier (defaults to "Europe/London")
        translator: Optional async translator callback for custom languages
    
    Returns:
        A string containing the generated goodbye message
    
    Raises:
        CorpusLoadError: If the message corpus cannot be loaded
        InvalidLanguageCodeError: If language_code format is invalid
    
    Example:
        >>> import asyncio
        >>> import joyous_departures
        >>> 
        >>> # Basic usage
        >>> message = asyncio.run(joyous_departures.generate_goodbye())
        >>> print(message)
        "Wishing you a liberated day, Good Soul❤️"
        >>> 
        >>> # With custom name
        >>> message = asyncio.run(joyous_departures.generate_goodbye(
        ...     template_args={"name": "Alice"}
        ... ))
        >>> print(message)
        "Wishing you a liberated day, Alice❤️"
        >>> 
        >>> # With location and timezone
        >>> message = asyncio.run(joyous_departures.generate_goodbye(
        ...     template_args={"location": "Paris"},
        ...     timezone="Europe/Paris"
        ... ))
        >>> 
        >>> # Without emojis
        >>> message = asyncio.run(joyous_departures.generate_goodbye(use_emojis=False))
        >>> print(message)
        "Wishing you a liberated day, Good Soul"
    """
```

## Error Handling

### CorpusLoadError

Raised when the message corpus cannot be loaded or decompressed.

```python
class CorpusLoadError(Exception):
    """Raised when corpus loading fails"""
    pass
```

### InvalidLanguageCodeError

Raised when an invalid language code format is provided.

```python
class InvalidLanguageCodeError(Exception):
    """Raised when language code format is invalid"""
    pass
```

## Examples

### Basic Usage

```python
import asyncio
import joyous_departures

message = asyncio.run(joyous_departures.generate_goodbye())
print(message)
# "May your path be filled with light, Good Soul✨"
```

### Custom Name

```python
message = asyncio.run(joyous_departures.generate_goodbye(
    template_args={"name": "Alice"}
))
print(message)
# "Wishing you a liberated day, Alice❤️"
```

### With Location and Timezone

```python
message = asyncio.run(joyous_departures.generate_goodbye(
    template_args={"name": "Alice", "location": "New York"},
    timezone="America/New_York"
))
# {date} and {time} will use current date/time in New York timezone
```

### No Emojis

```python
message = asyncio.run(joyous_departures.generate_goodbye(use_emojis=False))
print(message)
# "Wishing you a liberated day, Good Soul"
```

### Custom Language with Translator

```python
async def translate_message(lang: str, msg: str) -> str:
    # Call your translation service
    response = await translate_api(msg, target_lang=lang)
    return response

message = asyncio.run(joyous_departures.generate_goodbye(
    language_code="es-ES",
    translator=translate_message
))
```

### Using as Dictionary (Alternative API)

```python
message = joyous_departures.generate_goodbye(
    language_code="en-US",
    template_args={"name": "Bob"},
    use_emojis=True
)
```

## Performance

- Typical execution time: <10ms
- Thread-safe: Yes (can be called concurrently)
- Memory usage: <10MB (includes corpus)

## Platform Support

- Python 3.14+
- Platforms: Linux, macOS, Windows
- Requires: CPython (not PyPy or other implementations initially)

