# Technical Plan: Native Dual-Language Library Architecture (v2.0)

**Feature**: Native Dual-Language Library  
**Spec**: [spec.md](./spec.md)  
**Status**: Planning  
**Created**: 2025-12-29  
**Branch**: `003-native-dual-lib`

## Overview

This plan outlines the implementation of v2.0 of the joyous-departures library, replacing the Rust/WASM architecture with pure native implementations in TypeScript and Python. The goal is to eliminate Cloudflare Workers compatibility issues while maintaining performance and API compatibility.

## Technical Context

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Corpus Embedding | Code-gen to native constants | Zero runtime I/O, maximum portability |
| Generator Language | Node.js/TypeScript | Existing toolchain, simple maintenance |
| Deduplication | At generation time | Reduces bundle size |
| Multi-language | en-GB only for v2.0 | Keep scope manageable, extensible design |
| Generated Files | Committed to repo | Visible in PRs, no CI build step |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    corpus/en-GB.txt                         │
│                   (Human-editable source)                   │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              scripts/generate-corpus.ts                      │
│           (Node.js code generator, pre-commit)              │
└──────────────────┬─────────────────────┬────────────────────┘
                   │                     │
                   ▼                     ▼
┌─────────────────────────────┐ ┌─────────────────────────────┐
│ packages/typescript/src/    │ │ packages/python/src/        │
│ corpus.generated.ts         │ │ joyous_departures/corpus.py │
│                             │ │                             │
│ export const CORPUS =       │ │ CORPUS: tuple[str, ...] =   │
│   [...] as const;           │ │   (...)                     │
└─────────────────────────────┘ └─────────────────────────────┘
```

## Implementation Phases

### Phase 1: Repository Reorganization

**Goal**: Restructure the repository for the new dual-package architecture.

**Tasks**:

1. **Create new directory structure**
   ```
   /
   ├── corpus/                    # Existing (keep)
   │   └── en-GB.txt
   ├── packages/
   │   ├── typescript/            # New
   │   │   ├── src/
   │   │   │   ├── index.ts
   │   │   │   └── corpus.generated.ts
   │   │   ├── tests/
   │   │   ├── package.json
   │   │   └── tsconfig.json
   │   └── python/                # New
   │       ├── src/joyous_departures/
   │       │   ├── __init__.py
   │       │   └── corpus.py
   │       ├── tests/
   │       └── pyproject.toml
   ├── scripts/
   │   └── generate-corpus.ts     # New
   └── specs/                     # Existing (keep)
   ```

2. **Archive Rust/WASM artifacts** (do not delete immediately)
   - Move `joy-generator/` to `_archive/joy-generator/`
   - Move `bindings/` to `_archive/bindings/`
   - Move `Cargo.toml`, `Cargo.lock` to `_archive/`

3. **Update root configuration**
   - Update `.gitignore` for new structure
   - Update `README.md` with new installation instructions

**Deliverables**: New directory structure, archived old code

---

### Phase 2: Code Generator Implementation

**Goal**: Create the TypeScript script that generates corpus constants for both languages.

**Implementation**:

```typescript
// scripts/generate-corpus.ts
import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, '..');

function parseCorpus(content: string): string[] {
  return content
    .split('\n')
    .map(line => line.trim())
    .filter(line => line && !line.startsWith('#'))
    .filter((msg, idx, arr) => arr.indexOf(msg) === idx); // Deduplicate
}

function generateTypeScript(messages: string[]): string {
  const escaped = messages.map(m => JSON.stringify(m));
  return `// Auto-generated from corpus/en-GB.txt - DO NOT EDIT
// Generated: ${new Date().toISOString()}
// Message count: ${messages.length}

export const CORPUS = [
${escaped.map(m => `  ${m},`).join('\n')}
] as const;

export type Message = typeof CORPUS[number];
`;
}

function generatePython(messages: string[]): string {
  const escaped = messages.map(m => JSON.stringify(m));
  return `# Auto-generated from corpus/en-GB.txt - DO NOT EDIT
# Generated: ${new Date().toISOString()}
# Message count: ${messages.length}

CORPUS: tuple[str, ...] = (
${escaped.map(m => `    ${m},`).join('\n')}
)
`;
}

// Main execution
const corpusPath = join(ROOT, 'corpus', 'en-GB.txt');
const content = readFileSync(corpusPath, 'utf-8');
const messages = parseCorpus(content);

console.log(`Parsed ${messages.length} unique messages`);

// Generate TypeScript
const tsOutput = generateTypeScript(messages);
const tsPath = join(ROOT, 'packages', 'typescript', 'src', 'corpus.generated.ts');
writeFileSync(tsPath, tsOutput);
console.log(`Generated: ${tsPath}`);

// Generate Python
const pyOutput = generatePython(messages);
const pyPath = join(ROOT, 'packages', 'python', 'src', 'joyous_departures', 'corpus.py');
writeFileSync(pyPath, pyOutput);
console.log(`Generated: ${pyPath}`);
```

**Pre-commit Hook**: Update `.githooks/pre-commit` to run generator when `corpus/*.txt` changes.

**Deliverables**: `scripts/generate-corpus.ts`, pre-commit hook integration

---

### Phase 3: TypeScript Native Library

**Goal**: Implement pure TypeScript version of the library.

**Files**:

1. **`packages/typescript/src/corpus.generated.ts`** - Generated (Phase 2)

2. **`packages/typescript/src/index.ts`** - Main implementation:
   ```typescript
   import { CORPUS } from './corpus.generated.js';
   
   export interface TemplateArgs {
     name?: string;
     location?: string;
     date?: string;
     time?: string;
   }
   
   export interface GoodbyeOptions {
     templateArgs?: TemplateArgs;
     stripEmojis?: boolean;
     timezone?: string;
     translator?: (message: string) => Promise<string>;
   }
   
   export function generateGoodbye(options: GoodbyeOptions = {}): string {
     // Random message selection
     const message = CORPUS[Math.floor(Math.random() * CORPUS.length)];
     
     // Template substitution
     let result = substituteTemplates(message, options.templateArgs, options.timezone);
     
     // Emoji stripping
     if (options.stripEmojis) {
       result = stripEmojis(result);
     }
     
     return result;
   }
   ```

3. **`packages/typescript/package.json`**:
   ```json
   {
     "name": "@siviter-xyz/joyous-departures",
     "version": "2.0.0",
     "type": "module",
     "exports": {
       ".": {
         "import": "./dist/index.js",
         "require": "./dist/index.cjs"
       }
     },
     "files": ["dist"],
     "scripts": {
       "build": "tsup src/index.ts --format esm,cjs --dts",
       "test": "vitest"
     }
   }
   ```

**Key Changes from v1.x**:
- `generateGoodbye()` is now **synchronous** (no async initialization)
- No WASM dependency
- Zero runtime dependencies

**Deliverables**: Complete TypeScript package with tests

---

### Phase 4: Python Native Library

**Goal**: Implement pure Python version of the library.

**Files**:

1. **`packages/python/src/joyous_departures/corpus.py`** - Generated (Phase 2)

2. **`packages/python/src/joyous_departures/__init__.py`**:
   ```python
   """Joyous Departures - Generate warm, heartfelt sign-off messages"""
   
   import random
   import re
   from datetime import datetime
   from typing import Optional, Callable, Awaitable
   from zoneinfo import ZoneInfo
   
   from .corpus import CORPUS
   
   __version__ = "2.0.0"
   __all__ = ["generate_goodbye", "generate_goodbye_sync"]
   
   
   def generate_goodbye_sync(
       *,
       template_args: Optional[dict[str, str]] = None,
       strip_emojis: bool = False,
       timezone: str = "Europe/London",
   ) -> str:
       """Generate a goodbye message (synchronous)."""
       message = random.choice(CORPUS)
       result = _substitute_templates(message, template_args, timezone)
       if strip_emojis:
           result = _strip_emojis(result)
       return result
   
   
   async def generate_goodbye(
       *,
       template_args: Optional[dict[str, str]] = None,
       strip_emojis: bool = False,
       timezone: str = "Europe/London",
       translator: Optional[Callable[[str], Awaitable[str]]] = None,
   ) -> str:
       """Generate a goodbye message (async, for compatibility)."""
       result = generate_goodbye_sync(
           template_args=template_args,
           strip_emojis=strip_emojis,
           timezone=timezone,
       )
       if translator:
           result = await translator(result)
       return result
   ```

3. **`packages/python/pyproject.toml`**:
   ```toml
   [build-system]
   requires = ["hatchling"]
   build-backend = "hatchling.build"
   
   [project]
   name = "joyous-departures"
   version = "2.0.0"
   description = "Generate warm, heartfelt goodbye messages"
   requires-python = ">=3.10"
   license = { text = "MIT" }
   dependencies = []
   ```

**Key Changes from v1.x**:
- No Rust/PyO3 dependency
- Pure Python implementation
- Works with any Python >=3.10

**Deliverables**: Complete Python package with tests

---

### Phase 5: Test Suite

**Goal**: Implement comprehensive tests for both implementations.

**Shared Test Cases** (`specs/003-native-dual-lib/test-cases.json`):
```json
{
  "corpus_validation": {
    "description": "All corpus messages should be valid",
    "cases": [
      {"test": "corpus_not_empty"},
      {"test": "all_messages_non_empty"},
      {"test": "no_duplicate_messages"}
    ]
  },
  "template_substitution": {
    "cases": [
      {"input": {"name": "Alice"}, "contains": "Alice"},
      {"input": {"location": "London"}, "contains": "London"},
      {"input": {}, "not_contains": "{name}"}
    ]
  },
  "emoji_stripping": {
    "cases": [
      {"stripEmojis": true, "regex_not_match": "[\\u{1F600}-\\u{1F64F}]"},
      {"stripEmojis": false, "may_contain_emoji": true}
    ]
  }
}
```

**TypeScript Tests** (`packages/typescript/tests/`):
- `corpus.test.ts` - Corpus validation
- `generator.test.ts` - Generator function tests
- `templates.test.ts` - Template substitution tests

**Python Tests** (`packages/python/tests/`):
- `test_corpus.py` - Corpus validation
- `test_generator.py` - Generator function tests
- `test_templates.py` - Template substitution tests

**Deliverables**: Test suites with >90% coverage for both implementations

---

### Phase 6: CI/CD Updates

**Goal**: Update GitHub Actions workflows for new architecture.

**Workflows**:

1. **`.github/workflows/ci.yml`** - Continuous Integration
   ```yaml
   jobs:
     generate:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: actions/setup-node@v4
         - run: npx tsx scripts/generate-corpus.ts
         - run: git diff --exit-code  # Fail if generated files differ
   
     typescript:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: pnpm/action-setup@v2
         - run: cd packages/typescript && pnpm install && pnpm test
   
     python:
       runs-on: ubuntu-latest
       strategy:
         matrix:
           python-version: ["3.10", "3.11", "3.12", "3.13"]
       steps:
         - uses: actions/checkout@v4
         - uses: astral-sh/setup-uv@v4
         - run: cd packages/python && uv run pytest
   
     cloudflare-workers:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: cloudflare/wrangler-action@v3
         - run: cd packages/typescript && pnpm build
         - run: wrangler deploy --dry-run  # Validate Workers compatibility
   ```

2. **`.github/workflows/publish.yml`** - Package Publishing
   - Triggered by version tags
   - Publishes to npm and PyPI

**Deliverables**: Updated CI/CD workflows

---

### Phase 7: Documentation & Migration Guide

**Goal**: Update documentation for v2.0 release.

**Files**:
1. **`README.md`** - Updated installation and usage
2. **`CHANGELOG.md`** - v2.0.0 release notes
3. **`MIGRATION.md`** - v1.x → v2.0 migration guide

**Key Documentation Points**:
- Breaking change: `generateGoodbye()` is now synchronous in TypeScript
- Installation is simpler (no build tools required)
- API remains compatible (same function signatures)
- Cloudflare Workers now fully supported

**Deliverables**: Complete documentation update

---

## Success Criteria Validation

| Criteria | Validation Method |
|----------|-------------------|
| SC-001: Cloudflare Workers | Integration test in CI |
| SC-002: <30s installation | Measure in clean environment |
| SC-003: Identical tests | Shared test cases, both pass |
| SC-004: <10ms generation | Benchmark in both languages |
| SC-005: Package size | `du -h` on built packages |
| SC-006: Zero dependencies | Check package.json, pyproject.toml |

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| API breaking changes | Maintain same function signatures, add tests |
| Random behavior differences | Use shared test cases with known seeds |
| Date/time formatting differences | Document minor differences, use standard formats |
| Bundle size regression | Measure and compare to v1.x WASM bundle |

## Timeline Estimate

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: Repo Reorganization | 1 day | - |
| Phase 2: Code Generator | 1 day | Phase 1 |
| Phase 3: TypeScript Library | 2 days | Phase 2 |
| Phase 4: Python Library | 2 days | Phase 2 |
| Phase 5: Test Suite | 2 days | Phases 3, 4 |
| Phase 6: CI/CD | 1 day | Phase 5 |
| Phase 7: Documentation | 1 day | All phases |

**Total**: ~10 days

## Next Steps

1. Create task breakdown with `/speckit.tasks`
2. Begin Phase 1: Repository Reorganization
3. Implement code generator (Phase 2)
4. Parallel implementation of TypeScript and Python libraries (Phases 3-4)

