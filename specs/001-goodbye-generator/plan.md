# Implementation Plan: Goodbye Message Generator

**Branch**: `001-goodbye-generator` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-goodbye-generator/spec.md`

## Summary

This feature implements a high-performance Rust library that generates warm, heartfelt sign-off messages for email templates and other communication contexts. The core generation logic is implemented in Rust for speed, with language bindings provided for TypeScript (npm package) and Python (PyPI package). Messages are randomly selected from a pre-curated, compressed corpus of phrases, support template variable replacement (e.g., `{name}`), include emojis by default (with option to strip), and support internationalization through an optional translator callback.

## Technical Context

**Language/Version**: Rust 1.91.1+, TypeScript 5.9.3+, Python 3.14+  
**Primary Dependencies**: 
- Rust: `pyo3` (for Python bindings), `wasm-pack` (for WASM compilation for TypeScript/JavaScript), compression library (e.g., `flate2` or `lz4`), `chrono-tz` (for timezone support)
- TypeScript: Node.js 18+, TypeScript 5.9.3+, Vitest for testing
- Python: uv for package management, maturin for building Python extensions from Rust

**Storage**: Message corpus stored in repository as multiline text file (`corpus/en-GB.txt`), then compressed and embedded in Rust binary/library at build time  
**Testing**: 
- Rust: `cargo test` with unit and integration tests, `cargo bench` for benchmarks
- TypeScript: Vitest for E2E tests and benchmarks
- Python: pytest for E2E tests and benchmarks
- All tests must verify: functionality, randomness (different results each call), performance benchmarks

**Target Platform**: 
- Rust core: Linux, macOS, Windows (native binaries)
- TypeScript: Node.js 18+ and browsers (WASM)
- Python: CPython 3.14+ (native extension module)

**Project Type**: Multi-language library (Rust workspace with multiple crates)  
**Performance Goals**: 
- Message generation: <10ms from function call to returned string
- Support 1000+ concurrent calls per second
- Minimal memory footprint (<10MB for corpus + runtime)

**Constraints**: 
- Deterministic generation (no LLMs at runtime)
- Must compile to native binaries for both Python and Node.js
- Package size should be reasonable (<5MB for npm package, <10MB for PyPI package)
- Thread-safe for concurrent access

**Scale/Scope**: 
- Initial corpus: 360 unique messages in en-GB
- Support for template variables: `name` (default "Good Soul"), `location` (default "The World"), `date` (default current date), `time` (default current time)
- Timezone support: defaults to "Europe/London", configurable via options
- Messages may or may not contain template variables (not required)
- Support for 5+ languages in core corpus (expandable via async translator callback)
- Single function API (simple, focused interface)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Constitution Compliance**: Constitution file exists at `.specify/memory/constitution.md`. All principles are being followed:
- ✅ Performance-First Architecture: Rust core, <10ms target, 1000+ concurrent calls
- ✅ Test-First Development: TDD required, unit + E2E + benchmark tests
- ✅ Code Quality Standards: ruff + ty (Python), ESLint (TypeScript), cargo clippy (Rust)
- ✅ User Experience Consistency: Consistent APIs, @siviter-xyz/joyous-departures package name
- ✅ Package Management: pnpm (TypeScript), uv (Python)
- ✅ Corpus Management: Version-controlled text file, 360 messages

## Project Structure

### Documentation (this feature)

```text
specs/001-goodbye-generator/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
joyous-departures/
├── Cargo.toml                    # Workspace manifest
├── corpus/                      # Message corpus source files
│   └── en-GB.txt                # 360 messages, one per line
├── joy-generator/                # Core Rust library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs               # Public API
│   │   ├── generate.rs          # Core generation logic
│   │   ├── corpus.rs            # Message corpus management
│   │   ├── template.rs          # Template variable replacement
│   │   ├── emoji.rs             # Emoji handling/stripping
│   │   ├── datetime.rs          # Date/time handling with timezone
│   │   └── options.rs           # Options struct definitions
│   ├── benches/                  # Benchmark tests
│   │   └── generate_bench.rs
│   └── tests/
│       ├── integration/
│       └── unit/
│
├── bindings/
│   ├── python/                  # Python bindings (PyO3)
│   │   ├── Cargo.toml
│   │   ├── pyproject.toml       # Python package config
│   │   ├── src/
│   │   │   └── lib.rs           # PyO3 bindings
│   │   └── tests/
│   │       └── test_e2e.py      # E2E tests with benchmarks
│   │
│   └── typescript/              # TypeScript/JavaScript bindings (WASM)
│       ├── Cargo.toml
│       ├── package.json          # npm package config (pnpm)
│       ├── tsconfig.json
│       ├── src/
│       │   ├── index.ts          # TypeScript wrapper
│       │   └── index.d.ts        # Type definitions
│       ├── tests/
│       │   └── e2e.test.ts      # E2E tests with benchmarks (Vitest)
│       └── pkg/                  # Generated WASM package
│
├── .github/
│   └── workflows/
│       ├── ci.yml               # CI for testing
│       ├── version.yml          # Automated versioning from git tags
│       ├── publish-npm.yml      # Publish to npm
│       └── publish-pypi.yml     # Publish to PyPI
│
├── scripts/
│   ├── build.sh                 # Build all packages
│   ├── test.sh                  # Run all tests
│   ├── bench.sh                 # Run benchmarks
│   ├── version.sh               # Semantic versioning script
│   └── publish.sh               # Publish to registries
│
└── README.md                     # Includes benchmark results
```

**Structure Decision**: Using a Rust workspace with separate crates for the core library and each language binding. This allows:
- Independent versioning and publishing of each package
- Clear separation of concerns
- Efficient compilation (shared dependencies)
- Easy addition of future language bindings
- Corpus stored as versioned text file in repository, embedded at build time
- WASM compilation for browser support in TypeScript bindings
- Semantic versioning controlled via git tags, automated via GitHub Actions

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations identified. The multi-crate workspace structure is necessary to support multiple language bindings while maintaining a clean separation between the core logic and platform-specific code.
