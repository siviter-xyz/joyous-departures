# Data Model: Goodbye Message Generator

**Date**: 2025-01-27  
**Feature**: 001-goodbye-generator

## Core Entities

### GoodbyeMessage

A generated sign-off message string with template variables replaced and optional emojis.

**Properties**:
- `content: String` - The final message text
- `original_template: String` - The original template before variable replacement
- `language_code: String` - Language of the message (ISO 639 format, e.g., "en-GB")
- `has_emojis: bool` - Whether the original message contained emojis

**Constraints**:
- Length: 10-100 characters (excluding template variables)
- May or may not contain template variable placeholders (not required)
- Must be a valid UTF-8 string

### MessageTemplate

A template from the corpus before variable replacement.

**Properties**:
- `template: String` - Template string with optional placeholders (e.g., "Wishing you a liberated day, {name}❤️" or "Until we meet again")
- `language_code: String` - Language code (default: "en-GB")
- `template_variables: Vec<String>` - List of variable names used in template (e.g., ["name"], ["name", "location"], or [])
- `has_emojis: bool` - Whether template contains emojis
- `emoji_positions: Vec<usize>` - Optional: positions of emojis for efficient stripping

**Constraints**:
- Template must be valid UTF-8
- Variable names must match pattern: `{variable_name}` where `variable_name` is alphanumeric + underscore
- Template variables are optional - messages may have zero or more variables
- Supported variables: `name`, `location`, `date`, `time`

### MessageCorpus

The collection of all message templates.

**Properties**:
- `messages: Vec<MessageTemplate>` - All available message templates
- `compressed_data: Vec<u8>` - Compressed representation of messages (embedded at build time)
- `language_index: HashMap<String, Vec<usize>>` - Index mapping language codes to message indices
- `initialized: bool` - Whether corpus has been decompressed and loaded
- `source_file: String` - Path to source corpus file in repository (e.g., "corpus/en-GB.txt")

**Constraints**:
- Must contain 360 messages in default language (en-GB)
- Corpus stored in repository as multiline text file, versioned alongside code
- All messages must be valid
- Compression must be reversible (lossless)

### CoreGoodbyeOptions

Options structure for Rust core library (without translator callback).

**Properties**:
- `language_code: Option<String>` - ISO 639 language code (default: "en-GB")
- `template_args: HashMap<String, String>` - Map of variable names to replacement values
  - Key: variable name (e.g., "name", "location", "date", "time")
  - Value: replacement text (e.g., "Alice", "Paris", "2025-01-27", "14:30")
- `use_emojis: bool` - Whether to include emojis (default: true)
- `timezone: Option<String>` - IANA timezone identifier (default: "Europe/London")
  - Used for default values of `{date}` and `{time}` when not provided in template_args

**Constraints**:
- `language_code` must be valid ISO 639 format if provided
- `template_args` keys must be valid variable names (alphanumeric + underscore)
- `template_args` values must be valid UTF-8 strings
- `timezone` must be valid IANA timezone identifier if provided
- Default template variable values:
  - `name`: "Good Soul" (if not provided)
  - `location`: "The World" (if not provided)
  - `date`: Current date in specified timezone (if not provided)
  - `time`: Current time in specified timezone (if not provided)

### GoodbyeOptions (Language Bindings)

Extended options structure for TypeScript/Python (includes translator callback).

**Properties** (extends CoreGoodbyeOptions):
- All properties from `CoreGoodbyeOptions`
- `translator: Option<Callable>` - Optional async translator function
  - Signature: `async (language_code: String, message: String) -> String` (TypeScript/Python)
  - Called when language_code is not natively supported
  - Returns translated message or original if translation fails
  - Must be awaited in language bindings before calling Rust core

**Constraints**:
- Translator callback is optional
- Translator is async (handled by language bindings, not Rust core)
- If translator returns empty string or throws error, fallback to default language
- Translator is not passed to Rust core (handled by language binding wrapper)

## Data Flow

### Message Generation Flow

```
1. User calls generateGoodbye(options)
   ↓
2. Language binding validates and normalizes options
   ↓
3. If translator needed and provided:
   - Generate message in default language
   - Call translator callback
   - Use translated message
   ↓
4. If translator needed and provided:
   - Generate message in default language
   - Await translator callback
   - Use translated message
   ↓
5. Convert GoodbyeOptions to CoreGoodbyeOptions
   ↓
6. Rust core: generate_goodbye(core_options)
   ↓
7. Load corpus (if not already loaded)
   ↓
8. Filter messages by language_code
   ↓
9. Randomly select message template
   ↓
10. Replace template variables with values from template_args:
    - Use provided values from template_args
    - Apply defaults: name="Good Soul", location="The World"
    - Generate date/time from current time in specified timezone if not provided
   ↓
11. If use_emojis == false: strip emojis
   ↓
12. Return final message string
```

### Corpus Loading Flow

```
1. Build time: Read corpus/en-GB.txt from repository
   ↓
2. Build time: Compress corpus using lz4
   ↓
3. Build time: Embed compressed corpus in binary using include_bytes!
   ↓
4. Runtime: Library initialization (first call or explicit init)
   ↓
5. Runtime: Load compressed corpus data (embedded in binary)
   ↓
6. Runtime: Decompress using lz4
   ↓
7. Runtime: Parse into Vec<MessageTemplate>
   ↓
8. Runtime: Build language_index for fast lookup
   ↓
9. Runtime: Mark corpus as initialized
   ↓
10. Runtime: Cache in memory (Arc for thread-safety)
```

## State Management

### Corpus State

- **Uninitialized**: Corpus not loaded, first access triggers loading
- **Initialized**: Corpus loaded and indexed, ready for queries
- **Error**: Loading failed, fallback to default message

### Thread Safety

- Corpus data: `Arc<MessageCorpus>` (read-only after initialization, safe for concurrent access)
- RNG: Thread-local or atomic counter (no shared mutable state)
- Options: Immutable structs (passed by value)

## Validation Rules

### Language Code Validation
- Format: ISO 639-1 (2 letters) or ISO 639-1 with region (e.g., "en-GB")
- Case-insensitive (normalize to lowercase)
- Invalid codes fall back to "en-GB"

### Template Variable Validation
- Variable names: alphanumeric + underscore only
- Must start with letter or underscore
- Maximum length: 50 characters
- Supported variables: `name`, `location`, `date`, `time`
- Default values:
  - `name`: "Good Soul"
  - `location`: "The World"
  - `date`: Current date in specified timezone (format: YYYY-MM-DD)
  - `time`: Current time in specified timezone (format: HH:MM)
- Reserved names: None (future-proofing, but only supported variables are processed)

### Message Template Validation
- Must contain at least one `{variable}` placeholder
- All placeholders must have valid variable names
- No nested placeholders (e.g., `{{name}}` is invalid)
- Template must be valid UTF-8

## Error Handling

### Error Types

1. **CorpusLoadError**: Failed to load or decompress corpus
   - Fallback: Return hardcoded default message

2. **InvalidLanguageCode**: Language code format invalid
   - Fallback: Use "en-GB"

3. **TemplateVariableError**: Invalid variable name or missing required variable
   - Behavior: Leave placeholder as-is or use default value

4. **TranslatorError**: Translator callback failed
   - Fallback: Use original message in default language

### Error Reporting

- Rust: Use `Result<T, Error>` types
- TypeScript: Throw exceptions with clear error messages
- Python: Raise exceptions with descriptive messages

## Future Extensibility

### Additional Template Variables
- Current: `name` (defaults to "Good Soul"), `location` (defaults to "The World"), `date` (defaults to current date), `time` (defaults to current time)
- All template variables are optional - messages may have zero or more
- Design: HashMap allows arbitrary variable names, but only supported variables are processed with defaults

### Additional Languages
- Current: English (en-GB) in core corpus
- Future: Add native support for Spanish, French, German, etc.
- Design: Language index allows easy addition of new languages

### Message Metadata
- Future: Add tags/categories (e.g., "formal", "casual", "poetic")
- Future: Add sentiment scores or tone indicators
- Design: Extend MessageTemplate struct with optional metadata field

