# Data Model: Native Dual-Language Library (v2.0)

**Feature**: Native Dual-Language Library  
**Spec**: [spec.md](./spec.md)  
**Plan**: [plan.md](./plan.md)

## Entities

### Message

A single goodbye message from the corpus.

| Field | Type | Description |
|-------|------|-------------|
| text | string | The message text, may contain template variables |

**Template Variables**: `{name}`, `{location}`, `{date}`, `{time}`

**Constraints**:
- Non-empty string
- UTF-8 encoded
- May contain emojis
- Max length: 500 characters

---

### Corpus

The complete collection of messages, embedded as a constant at build time.

**TypeScript**:
```typescript
export const CORPUS: readonly string[] = [...] as const;
```

**Python**:
```python
CORPUS: tuple[str, ...] = (...)
```

**Constraints**:
- Immutable (readonly/tuple)
- No duplicates
- At least 1 message
- Derived from `corpus/en-GB.txt`

---

### TemplateArgs

User-provided values for template variable substitution.

| Field | Type | Default | Constraints |
|-------|------|---------|-------------|
| name | string? | "Good Soul" | Max 50 chars |
| location | string? | "The World" | Max 100 chars |
| date | string? | Current date | Format: YYYY-MM-DD |
| time | string? | Current time | Format: HH:MM |

**TypeScript**:
```typescript
interface TemplateArgs {
  name?: string;
  location?: string;
  date?: string;
  time?: string;
}
```

**Python**:
```python
class TemplateArgs(TypedDict, total=False):
    name: str
    location: str
    date: str
    time: str
```

---

### GeneratorOptions

Configuration options for the `generateGoodbye` function.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| templateArgs | TemplateArgs? | {} | Template variable values |
| stripEmojis | boolean | false | Remove emojis from output |
| timezone | string | "Europe/London" | IANA timezone for date/time |
| translator | function? | undefined | Async translation callback |

**TypeScript**:
```typescript
interface GoodbyeOptions {
  templateArgs?: TemplateArgs;
  stripEmojis?: boolean;
  timezone?: string;
  translator?: (message: string) => Promise<string>;
}
```

**Python**:
```python
# Parameters passed directly to function (keyword-only)
async def generate_goodbye(
    *,
    template_args: dict[str, str] | None = None,
    strip_emojis: bool = False,
    timezone: str = "Europe/London",
    translator: Callable[[str], Awaitable[str]] | None = None,
) -> str: ...
```

---

## State Transitions

This library is stateless. Each call to `generateGoodbye()` is independent.

```
┌─────────────────┐
│  Call function  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Select random   │
│ message from    │
│ CORPUS          │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Substitute      │
│ template vars   │
│ with defaults   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Strip emojis    │
│ (if enabled)    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Call translator │
│ (if provided)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Return string   │
└─────────────────┘
```

## Validation Rules

### Input Validation

| Field | Rule | Action on Violation |
|-------|------|---------------------|
| templateArgs.name | Max 50 chars | Truncate |
| templateArgs.location | Max 100 chars | Truncate |
| templateArgs.date | Max 20 chars | Truncate |
| templateArgs.time | Max 10 chars | Truncate |
| timezone | Valid IANA identifier | Default to "Europe/London" |

### Output Guarantees

- Always returns a non-empty string
- No unsubstituted template variables (defaults applied)
- UTF-8 encoded
- If `stripEmojis: true`, no emoji characters in output

