# Joy Goodbye Constitution

## Core Principles

### I. Performance-First Architecture (NON-NEGOTIABLE)

**MUST**: Core generation logic MUST be implemented in Rust for performance. All performance-critical operations (corpus loading, template replacement, emoji handling) MUST be in the Rust core.

**MUST**: Message generation MUST complete in <10ms from function call to returned string (measured on standard hardware).

**MUST**: Library MUST handle at least 1000 concurrent function calls per second without performance degradation.

**MUST**: Memory footprint MUST be <10MB total (corpus + runtime).

**MUST NOT**: Use LLMs or runtime generation - all messages MUST be pre-curated and deterministic.

**Rationale**: Performance is a core value proposition. Rust provides the speed needed for high-throughput scenarios while maintaining safety.

---

### II. Test-First Development (NON-NEGOTIABLE)

**MUST**: All code MUST be written using Test-Driven Development (TDD):
1. Write tests first
2. Ensure tests fail
3. Implement functionality
4. Verify tests pass
5. Refactor if needed

**MUST**: Rust code MUST include unit tests using `cargo test`. All modules MUST have corresponding test files in `tests/unit/`.

**MUST**: Python bindings MUST include E2E tests using pytest in `bindings/python/tests/test_e2e.py`.

**MUST**: TypeScript bindings MUST include E2E tests using Vitest in `bindings/typescript/tests/e2e.test.ts`.

**MUST**: All test suites MUST verify:
- Functionality (correct output)
- Randomness (different results on each call)
- Performance benchmarks (generation speed)

**MUST**: Benchmark results MUST be included in README.md and MUST demonstrate <10ms generation time.

**MUST NOT**: Commit code without corresponding tests (except infrastructure/setup tasks).

**Rationale**: Testing ensures correctness, performance, and prevents regressions. Benchmarks validate performance requirements.

---

### III. Code Quality Standards

**MUST**: Use latest stable versions:
- Rust 1.91.1+
- TypeScript 5.9.3+
- Python 3.14+

**MUST**: All Rust code MUST:
- Pass `cargo clippy` with no warnings
- Be formatted with `cargo fmt`
- Follow Rust naming conventions
- Include doc comments for public APIs
- Use `Result<T, E>` for error handling (no panics in public API)

**MUST**: All TypeScript code MUST:
- Pass ESLint with strict rules
- Be formatted with Prettier
- Include JSDoc comments for public APIs
- Provide complete type definitions (.d.ts files)
- Use ESLint for all linting (no other linters)

**MUST**: All Python code MUST:
- Pass `ruff` linting with no critical warnings
- Pass `ty` type checking
- Be formatted with black
- Include docstrings for all public functions
- Follow PEP 8 style guidelines

**MUST**: All code MUST be thread-safe. Rust core MUST use `Arc` for shared read-only data, avoid mutable shared state.

**MUST NOT**: Include TODO, FIXME, or temporary comments in committed code unless explicitly approved.

**Rationale**: High code quality ensures maintainability, reduces bugs, and improves developer experience.

---

### IV. User Experience Consistency

**MUST**: API MUST be consistent across all language bindings (TypeScript, Python, Rust core).

**MUST**: All bindings MUST:
- Use the same option names (language_code, templateArgs, use_emojis, timezone)
- Provide the same default values ("en-GB", "Good Soul", "The World", "Europe/London")
- Handle errors consistently (clear error messages, sensible fallbacks)

**MUST**: Error messages MUST be:
- Clear and actionable
- Include context (what failed, why it failed)
- Provide guidance on how to fix the issue

**MUST**: TypeScript bindings MUST:
- Support both Node.js and browsers (via WASM)
- Provide complete type definitions
- Be importable as `import { generateGoodbye } from '@siviter-xyz/joy-goodbye'`

**MUST**: Python bindings MUST:
- Be installable via `uv pip install joy-goodbye`
- Provide async support where needed
- Follow Python naming conventions (snake_case)

**MUST NOT**: Break API compatibility without a major version bump (semantic versioning).

**Rationale**: Consistent UX across languages reduces cognitive load and makes the library easier to use.

---

### V. Documentation Standards

**MUST**: All public APIs MUST include:
- Function/type documentation
- Parameter descriptions
- Return value descriptions
- Usage examples
- Error conditions

**MUST**: README.md MUST include:
- Installation instructions (pnpm for TypeScript, uv for Python)
- Basic usage examples
- Performance benchmarks
- API reference links

**MUST**: API contracts MUST be documented in `specs/001-goodbye-generator/contracts/` for:
- TypeScript API
- Python API
- Rust core API

**MUST**: Quickstart guide MUST be validated - all examples MUST work as documented.

**MUST NOT**: Include outdated or incorrect examples in documentation.

**Rationale**: Good documentation enables adoption and reduces support burden.

---

### VI. Package Management & Distribution

**MUST**: Use specified package managers:
- pnpm for TypeScript/JavaScript (not npm)
- uv for Python (not pip)

**MUST**: Package versioning MUST be controlled via git tags using semantic versioning (MAJOR.MINOR.PATCH).

**MUST**: CI/CD MUST automate:
- Testing on multiple platforms (Linux, macOS, Windows)
- Building all packages
- Publishing to npm and PyPI (when credentials are configured)

**MUST**: Package sizes MUST be reasonable:
- npm package: <5MB
- PyPI package: <10MB

**MUST NOT**: Publish packages manually - all publishing MUST be automated via GitHub Actions.

**Rationale**: Consistent tooling and automation reduces errors and improves reliability.

---

### VII. Corpus Management

**MUST**: Message corpus MUST be stored in repository as multiline text file (`corpus/en-GB.txt`), versioned alongside code.

**MUST**: Corpus MUST contain exactly 360 unique messages in en-GB (initial version).

**MUST**: Corpus MUST be:
- Compressed at build time (using lz4)
- Embedded in Rust binary using `include_bytes!`
- Decompressed at runtime on first use
- Cached in memory (Arc) for thread-safe access

**MUST**: Messages MUST be:
- Warm, heartfelt, whimsical sign-off phrases
- Between 10-100 characters (excluding template variable placeholders, including emojis)
- Mix of messages with and without template variables
- Include emojis where appropriate

**MUST NOT**: Generate messages at runtime or use external services for message generation.

**Rationale**: Version-controlled corpus ensures reproducibility and allows curation improvements over time.

---

## Additional Constraints

### Technology Stack Requirements

**MUST**: Use specified technologies:
- Rust: PyO3 (Python bindings), wasm-pack (WASM compilation), lz4 (compression), chrono-tz (timezone)
- TypeScript: Vitest (testing), wasm-pack output
- Python: maturin (building), pytest (testing)

**MUST NOT**: Add dependencies without justification. Prefer standard library or well-established crates.

### Security Requirements

**MUST**: Validate all inputs (language codes, timezone identifiers, template variable names).

**MUST**: Use safe string handling (Rust's type system prevents common vulnerabilities).

**MUST NOT**: Access file system or network (except via optional translator callback provided by user).

### Performance Standards

**MUST**: Meet all performance goals:
- Generation: <10ms
- Throughput: 1000+ concurrent calls/second
- Memory: <10MB total

**MUST**: Include benchmarks in CI/CD to catch performance regressions.

---

## Development Workflow

### Code Review Requirements

**MUST**: All code changes MUST be reviewed before merging.

**MUST**: Reviews MUST verify:
- Tests are included and passing
- Performance benchmarks are met
- Code quality standards are followed
- Documentation is updated

### Quality Gates

**MUST**: CI/CD MUST block merges if:
- Tests fail
- Benchmarks regress (>10ms generation time)
- Code quality checks fail (clippy, lint, format)
- Documentation examples are broken

**MUST**: All commits MUST pass:
- `cargo test` (Rust)
- `cargo clippy` (Rust)
- `cargo fmt --check` (Rust)
- `pnpm test` (TypeScript)
- `pnpm lint` (TypeScript - ESLint)
- `uv run pytest` (Python)
- `uv run ruff check` (Python)
- `uv run ty` (Python - type checking)

---

## Governance

**MUST**: This constitution supersedes all other practices and preferences. Any deviation MUST be:
1. Documented with justification
2. Approved explicitly
3. Include a migration plan if breaking changes

**MUST**: All PRs and reviews MUST verify compliance with constitution principles.

**MUST**: Complexity MUST be justified. Simpler solutions are preferred unless complexity provides clear value.

**MUST**: Amendments to this constitution require:
1. Documentation of the change
2. Approval from project maintainers
3. Update to version number and date

**Version**: 1.0.0 | **Ratified**: 2025-01-27 | **Last Amended**: 2025-01-27
