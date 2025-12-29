# Feature Specification: Native Dual-Language Library Architecture (v2.0)

**Feature Branch**: `003-native-dual-lib`  
**Created**: 2025-12-29  
**Status**: Draft  
**Input**: User description: "Move the whole specification for the project into markdown specification that converts to PYTHON AND TS NATIVE (all platforms) libs. Remove WASM binding. Ensure unit tests correctly apply to both uv and pnpm libs."

## Clarifications

### Session 2025-12-29

- Q: What corpus embedding strategy should be used? → A: Code-gen from text file to native constants (highly performant embedding in both Python and TypeScript)
- Q: What language should the code generator be written in? → A: Node.js/TypeScript script (uses existing toolchain, simple maintenance)
- Q: Should the code generator deduplicate the corpus? → A: Yes, deduplicate at generation time (unique messages only in output)
- Q: Should v2.0 support multiple corpus languages? → A: Single language for v2.0 (en-GB only), extensible design for future languages
- Q: When should the code generator run? → A: Pre-commit (generated files committed to repo, visible in PRs)

## Background

The current architecture uses a Rust core with WASM bindings for TypeScript and PyO3/maturin for Python. The WASM binding has persistent issues in Cloudflare Workers environments (URL initialization failures, empty error objects). This specification defines a new major version (v2.0) that:

1. Removes the WASM dependency entirely
2. Establishes a markdown-based specification as the single source of truth
3. Generates native implementations for both Python and TypeScript
4. Ensures universal platform compatibility (including Cloudflare Workers)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Platform-Agnostic Message Generation (Priority: P1)

A developer wants to generate warm, heartfelt sign-off messages in their application. They need the library to work reliably across all JavaScript/TypeScript runtimes (Node.js, Deno, Bun, Cloudflare Workers, browsers) and Python environments without any WASM-related initialization issues.

**Why this priority**: This is the core functionality. If message generation doesn't work universally, the library fails its primary purpose. The WASM failures on Cloudflare Workers are the direct motivation for this redesign.

**Independent Test**: Can be fully tested by importing the library and calling the generate function in any supported environment. Delivers working message generation without environment-specific initialization.

**Acceptance Scenarios**:

1. **Given** a TypeScript/JavaScript environment (Node.js, Deno, Bun, browser, Cloudflare Workers), **When** a developer imports and calls `generateGoodbye()`, **Then** a valid goodbye message is returned without any WASM loading or initialization
2. **Given** a Python environment (3.10+), **When** a developer imports and calls `generate_goodbye()`, **Then** a valid goodbye message is returned using pure Python implementation
3. **Given** any supported environment, **When** the library is imported, **Then** no async initialization or URL-based loading is required
4. **Given** a Cloudflare Workers environment, **When** the package is deployed and invoked, **Then** it works without any "Invalid URL string" or empty error object failures

---

### User Story 2 - Consistent API Across Languages (Priority: P1)

A developer uses both Python and TypeScript in their projects. They want the same API semantics across both implementations so they can switch between languages without learning new patterns.

**Why this priority**: API consistency ensures developer productivity and reduces cognitive load. It also simplifies documentation and examples.

**Independent Test**: Can be fully tested by comparing API signatures and behaviors between Python and TypeScript implementations. Delivers consistent developer experience.

**Acceptance Scenarios**:

1. **Given** the TypeScript function `generateGoodbye(options)`, **When** compared to Python's `generate_goodbye(options)`, **Then** they have equivalent parameter names (with language-appropriate casing conventions)
2. **Given** template variables `{name}`, `{location}`, `{date}`, `{time}`, **When** used in either language, **Then** they are replaced identically
3. **Given** the `stripEmojis` option, **When** set to true in either language, **Then** the output contains no emoji characters
4. **Given** edge case inputs (empty strings, special characters), **When** processed by either language, **Then** the behavior is identical

---

### User Story 3 - Specification-Driven Development (Priority: P2)

A maintainer wants to evolve the library by updating a single markdown specification file. Changes to the specification should be reflected in both Python and TypeScript implementations, ensuring they stay synchronized.

**Why this priority**: Reduces maintenance burden and prevents drift between implementations. The specification becomes the authoritative source of truth.

**Independent Test**: Can be fully tested by modifying the specification and verifying both implementations update accordingly. Delivers maintainable architecture.

**Acceptance Scenarios**:

1. **Given** a markdown specification file defining the message corpus, **When** a new message is added, **Then** both Python and TypeScript tests validate the message is available
2. **Given** a markdown specification defining API contracts, **When** the specification is updated, **Then** tests fail until implementations are updated
3. **Given** the specification defines default values for template variables, **When** either implementation is used without providing values, **Then** the same defaults are applied

---

### User Story 4 - Easy Installation Without Build Tools (Priority: P2)

A developer wants to install the library using standard package managers (pnpm/npm for TypeScript, uv/pip for Python) without needing Rust, wasm-pack, clang, or other build toolchains.

**Why this priority**: Simplifies adoption and removes friction for new users. The current WASM build requirements are a barrier to entry.

**Independent Test**: Can be fully tested by installing from fresh environments without build tools. Delivers zero-dependency installation experience.

**Acceptance Scenarios**:

1. **Given** a clean Node.js environment, **When** running `pnpm add @siviter-xyz/joyous-departures`, **Then** installation succeeds without native compilation
2. **Given** a clean Python environment, **When** running `uv pip install joyous-departures`, **Then** installation succeeds without native compilation
3. **Given** either package installed, **When** imported, **Then** no post-installation initialization or compilation is required

---

### User Story 5 - Unified Test Suite (Priority: P2)

A developer contributing to the project wants confidence that their changes work correctly in both languages. A unified test specification should generate equivalent test cases for both Python and TypeScript.

**Why this priority**: Ensures feature parity and prevents regressions across implementations.

**Independent Test**: Can be fully tested by running both test suites and comparing coverage reports. Delivers confidence in cross-language consistency.

**Acceptance Scenarios**:

1. **Given** a test case defined in the specification, **When** the test suites run, **Then** equivalent tests execute in both Python (pytest) and TypeScript (vitest/jest)
2. **Given** the corpus of messages, **When** tests run, **Then** 100% of messages are validated in both implementations
3. **Given** a failing test in one language, **When** the same scenario is tested in the other language, **Then** the same failure is detected (if the bug exists in the specification)

---

### Edge Cases

- What happens when the message corpus is empty?
- How does the system handle Unicode characters and emojis across different platforms?
- What happens when template variables contain reserved regex characters?
- How does timezone handling work when `{date}` and `{time}` are used without explicit timezone?
- What happens when the random selection produces duplicates in rapid succession?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST generate goodbye messages from a predefined corpus without WASM or native dependencies
- **FR-002**: System MUST support template variable substitution for `{name}`, `{location}`, `{date}`, `{time}`
- **FR-003**: System MUST provide default values for template variables: name="Good Soul", location="The World"
- **FR-004**: System MUST support emoji stripping via configuration option
- **FR-005**: System MUST work synchronously (no async initialization required)
- **FR-006**: TypeScript implementation MUST be pure ESM with CommonJS compatibility via package.json exports
- **FR-007**: Python implementation MUST be pure Python (no Cython, no native extensions)
- **FR-008**: Both implementations MUST use the same corpus source file (`corpus/*.txt`), with a code generator producing native constants at build time
- **FR-009**: *(Deferred to v2.1)* Seeding support for deterministic output is not included in v2.0. Randomness uses language-native random functions without explicit seed control
- **FR-010**: System MUST maintain backward-compatible public API from v1.x (function signatures remain the same)
- **FR-011**: Build artifacts MUST be publishable to npm and PyPI without native compilation during installation

### Key Entities

- **Message Corpus**: Collection of goodbye messages stored in text files (`corpus/*.txt`), code-generated into native constants (`const CORPUS` in TypeScript, `CORPUS: tuple[str, ...]` in Python) at build time for maximum performance and portability
- **Template Variable**: Placeholder in message text (`{name}`, `{location}`, `{date}`, `{time}`) replaced at runtime
- **Generator Options**: Configuration object controlling message generation (templateArgs, stripEmojis, language, translator)
- **Specification File**: Markdown document defining corpus, API contracts, and test cases

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Library works in Cloudflare Workers without any initialization errors (verified via integration test)
- **SC-002**: Installation completes in under 30 seconds on typical broadband connection (no compilation step)
- **SC-003**: Both implementations pass identical test suites covering all corpus messages and API behaviors
- **SC-004**: Message generation completes in under 10ms (maintained from v1.x performance target)
- **SC-005**: Package size is under 100KB for TypeScript, under 50KB for Python (significantly smaller than WASM)
- **SC-006**: Zero runtime dependencies beyond standard library for both implementations

## Out of Scope

- Translation/internationalization beyond the existing translator callback pattern
- New message categories or corpus expansion (maintain existing corpus)
- Breaking changes to public API signatures
- Web component or React hook wrappers
- CLI tool for message generation
- Rust core library maintenance (will be deprecated)

## Assumptions

- The existing message corpus is correct and should be preserved
- The random selection algorithm can be implemented equivalently in both languages
- Date/time formatting will use each language's standard library (slight formatting differences acceptable)
- The translator callback pattern is language-appropriate (callbacks in TS, callables in Python)
- Build processes for npm and PyPI remain separate but follow similar patterns
- Semantic versioning continues: this is a MAJOR version bump (v2.0.0)
- The Rust core and WASM bindings will be removed from the repository in v2.0

## Dependencies

- Access to npm registry for TypeScript package publishing
- Access to PyPI for Python package publishing
- Existing test infrastructure (vitest for TS, pytest for Python)
- GitHub Actions for CI/CD

## Technical Considerations

### Repository Reorganization

The repository structure should be reorganized to:

1. Remove Rust/WASM build artifacts and configuration
2. Create separate package directories for Python and TypeScript
3. Establish a shared specification directory

**Proposed Structure**:

```
/
├── corpus/                    # Existing corpus directory (keep)
│   └── en-GB.txt              # Message corpus source
├── specs/                     # Existing specs directory (keep)
│   └── 003-native-dual-lib/   # This feature specification
├── packages/
│   ├── typescript/            # TypeScript/JavaScript implementation
│   │   ├── src/
│   │   ├── tests/
│   │   └── package.json
│   └── python/                # Python implementation
│       ├── src/joyous_departures/
│       ├── tests/
│       └── pyproject.toml
├── scripts/                   # Build and test scripts
└── README.md
```

### Corpus Format & Code Generation

The message corpus is stored as human-readable text files (`corpus/en-GB.txt`) and code-generated into native language constants at build time:

**Source Format**: Plain text file, one message per line, with optional comments (lines starting with `#`)

**Generated Output**:
- **TypeScript**: `export const CORPUS: readonly string[] = ["msg1", "msg2", ...] as const;`
- **Python**: `CORPUS: tuple[str, ...] = ("msg1", "msg2", ...)`

**Benefits**:
- Zero runtime file I/O (corpus is embedded in the module)
- Maximum portability (no dynamic imports, fetch, or URL resolution)
- Tree-shakeable in TypeScript bundlers
- Immutable data structures in both languages
- Smallest possible package size

**Code Generator**: Node.js/TypeScript script (`scripts/generate-corpus.ts`) that:
1. Reads `corpus/en-GB.txt` (single language for v2.0, extensible for future languages)
2. Parses messages (skipping comments and empty lines)
3. Deduplicates messages (unique messages only)
4. Generates `packages/typescript/src/corpus.generated.ts`
5. Generates `packages/python/src/joyous_departures/corpus.py`

**Future Extensibility**: The generator architecture supports adding new language files (`corpus/es-ES.txt`, etc.) in future versions. The `translator` callback remains available for runtime translation.

**Execution Timing**: Pre-commit hook runs the generator; generated files are committed to the repository. This ensures:
- Generated code is visible in code reviews
- No build step required during CI or installation
- Source and generated files stay in sync

### Test Generation

Tests should be definable in a format that generates equivalent test cases:

- Core test cases defined in JSON/YAML
- Test runners in each language read shared test definitions
- Language-specific test runners execute native assertions

