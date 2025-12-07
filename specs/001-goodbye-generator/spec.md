# Feature Specification: Goodbye Message Generator

**Feature Branch**: `001-goodbye-generator`  
**Created**: 2025-01-27  
**Status**: Draft  
**Input**: User description: "Dual Python (uv) and TypeScript (pnpm) code API interface that generates short signoff messages for adding to the end of email templates etc. The generation should be done in Rust for speed and provide the relevant bindings to TypeScript/python to wrap."

## Clarifications

### Session 2025-01-27

- Q: When the message corpus is empty or corrupted and cannot be loaded, what should the system return? → A: Return a hardcoded fallback message: "Wishing you a joyous day, {name} ❤️"
- Q: When template replacement fails (e.g., malformed template syntax), what should the system do? → A: Return a fallback message without templates: "Wishing you a joyous day❤️"
- Q: How should the system handle very long custom names and error fallbacks? → A: Rust core should raise exceptions/errors for all error conditions (corpus load failure, template replacement failure, validation failures). Python/TypeScript bindings should catch these errors and implement fallback logic (truncate names to 50 chars, return fallback messages). Error handling is the responsibility of the language bindings, not the Rust core.
- Q: When an invalid language code format is provided, what should the system do? → A: Python/TypeScript bindings MUST validate language code format (ISO 639-1 or ISO 639-1 with region) and fall back to "en-GB" if invalid before calling Rust core. Rust core MUST raise InvalidLanguageCodeError if it receives an invalid code (defensive check).
- Q: When an invalid timezone identifier is provided, what should the system do? → A: Python/TypeScript bindings MUST validate IANA timezone identifier format and fall back to "Europe/London" if invalid before calling Rust core. Rust core MUST raise an error if it receives an invalid timezone (defensive validation).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Basic Goodbye Message (Priority: P1)

A developer wants to generate a warm, heartfelt sign-off message for use in email templates or other communication contexts. They import the library in their preferred language (TypeScript or Python) and call a simple function to get a randomly selected message.

**Why this priority**: This is the core functionality that delivers the primary value. Without this, the library has no purpose.

**Independent Test**: Can be fully tested by importing the library, calling the generate function with no arguments, and receiving a valid goodbye message string. This delivers immediate value as a working library.

**Acceptance Scenarios**:

1. **Given** a developer has installed the TypeScript package, **When** they call `generateGoodbye()` with no arguments, **Then** they receive a string containing a warm sign-off message with default name "Good Soul" and emojis included
2. **Given** a developer has installed the Python package, **When** they call `generate_goodbye()` with no arguments, **Then** they receive a string containing a warm sign-off message with default name "Good Soul" and emojis included
3. **Given** a developer calls the function, **When** they receive a message, **Then** the message contains template placeholders (like `{name}`) that are replaced with actual values

---

### User Story 2 - Customize Message Generation (Priority: P1)

A developer wants to customize the generated message by providing options such as a custom name, disabling emojis, or specifying a language code for internationalization.

**Why this priority**: Customization is essential for practical use. Users need to personalize messages and control emoji usage based on context.

**Independent Test**: Can be fully tested by calling the function with various option combinations and verifying the output matches the specified options. This delivers value by enabling practical customization.

**Acceptance Scenarios**:

1. **Given** a developer calls the function with `{ templateArgs: { name: "Alice" } }`, **When** the message is generated, **Then** all `{name}` placeholders are replaced with "Alice"
2. **Given** a developer calls the function with `{ use_emojis: false }`, **When** the message is generated, **Then** the message contains no emoji characters
3. **Given** a developer calls the function with `{ language_code: "en-US" }`, **When** the message is generated, **Then** the system attempts to provide a message in the specified language (or falls back to default if translation unavailable)
4. **Given** a developer calls the function with `{ templateArgs: { name: undefined } }` or omits the name option, **When** the message is generated, **Then** the default name "Good Soul" is used
5. **Given** a developer calls the function with `{ templateArgs: { location: "Paris" } }`, **When** the message is generated, **Then** all `{location}` placeholders are replaced with "Paris"
6. **Given** a developer calls the function without specifying location, **When** the message is generated, **Then** the default location "The World" is used for `{location}` placeholders
7. **Given** a developer calls the function with `{ timezone: "America/New_York" }`, **When** the message is generated, **Then** `{date}` and `{time}` placeholders use the specified timezone
8. **Given** a developer calls the function without specifying timezone, **When** the message is generated, **Then** the default timezone "Europe/London" is used for date/time placeholders
9. **Given** a message template does not contain any template variables, **When** the message is generated, **Then** the message is returned as-is without requiring any template variables

---

### User Story 3 - Use External Translator for Internationalization (Priority: P2) ⏸️ DEFERRED

**Status**: ⚠️ **DEFERRED** - Implementation deferred for later. Interface is defined in API contracts for future compatibility.

A developer wants to provide their own translation function (e.g., using an LLM service) to translate messages into languages not natively supported by the library.

**Why this priority**: This extends functionality beyond the core corpus, enabling users to support any language through external services. Lower priority than core functionality but important for international use cases.

**Independent Test**: Can be fully tested by providing a translator callback function and verifying it is called with the appropriate parameters when a non-default language is requested. This delivers value by enabling extensibility.

**Note**: The async translator callback interface is defined in the TypeScript and Python API contracts (`contracts/typescript-api.md`, `contracts/python-api.md`) for future implementation. Implementation tasks are deferred.

**Acceptance Scenarios**:

1. **Given** a developer provides an async `translator` callback function and calls with `{ language_code: "fr-FR" }`, **When** the message is generated, **Then** the translator function is awaited with the language code and English message, and the returned translation is used
2. **Given** a developer provides a translator that returns an empty string or throws an error, **When** the message is generated, **Then** the system falls back to the default English message
3. **Given** a developer calls the function with a language code but no translator, **When** the language is not natively supported, **Then** the system uses the default English message

---

### User Story 4 - Install and Use from Package Registries (Priority: P2)

A developer wants to install the library from npm (for TypeScript) or PyPI (for Python) using standard package managers and use it in their project.

**Why this priority**: Distribution and ease of installation are critical for adoption. Without published packages, the library cannot be easily used.

**Independent Test**: Can be fully tested by installing the package from the registry and successfully importing/using it in a new project. This delivers value by making the library accessible to users.

**Acceptance Scenarios**:

1. **Given** a developer runs `pnpm add @siviter-xyz/joyous-departures` (or equivalent npm command), **When** the package installs successfully, **Then** they can import and use the library in their TypeScript/JavaScript project
2. **Given** a developer runs `uv pip install joyous-departures` (or equivalent pip command), **When** the package installs successfully, **Then** they can import and use the library in their Python project
3. **Given** a developer installs the package, **When** they check the package version, **Then** it displays a semantic version number

---

### Edge Cases

- What happens when the message corpus is empty or corrupted? Rust core MUST raise CorpusLoadError. Python/TypeScript bindings MUST catch this error and return fallback message: "Wishing you a joyous day, {name} ❤️" (with template variables replaced using defaults).
- How does the system handle invalid language codes? Python/TypeScript bindings MUST validate language code format (ISO 639-1 or ISO 639-1 with region) and fall back to "en-GB" if invalid before calling Rust core. Rust core MUST raise InvalidLanguageCodeError if it receives an invalid code (defensive validation).
- How does the system handle invalid timezone identifiers? Python/TypeScript bindings MUST validate IANA timezone identifier format and fall back to "Europe/London" if invalid before calling Rust core. Rust core MUST raise an error if it receives an invalid timezone (defensive validation).
- What happens when template replacement fails (e.g., malformed template)? Rust core MUST raise TemplateVariableError. Python/TypeScript bindings MUST catch this error and return fallback message: "Wishing you a joyous day❤️" (no template variables).
- How does the system handle very long custom names? Python/TypeScript bindings MUST truncate names to 50 characters maximum before passing to Rust core. Rust core does not perform validation - it receives pre-validated inputs from bindings.
- What happens when the translator callback takes too long or hangs? System should implement a timeout mechanism or document expected behavior.
- How does the system handle concurrent calls from multiple threads/processes? System should be thread-safe and handle concurrent access correctly.
- What happens when emoji stripping encounters edge cases (e.g., emojis in the middle of words)? System should use a robust emoji detection and removal mechanism.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a function that generates a random sign-off message from a pre-curated corpus of warm, heartfelt phrases
- **FR-002**: System MUST support template variable replacement (e.g., `{name}`, `{location}`, `{date}`, `{time}`) in generated messages when present in the template
- **FR-003**: System MUST default the name template variable to "Good Soul" when not provided
- **FR-004**: System MUST default the location template variable to "The World" when not provided
- **FR-005**: System MUST default the date template variable to the current date (respecting timezone) when not provided
- **FR-006**: System MUST default the time template variable to the current time (respecting timezone) when not provided
- **FR-007**: System MUST default the timezone to "Europe/London" when not specified
- **FR-008**: System MUST NOT require template variables in messages - messages may or may not contain variables
- **FR-009**: System MUST include emojis in messages by default
- **FR-010**: System MUST provide an option to strip emojis from generated messages
- **FR-011**: System MUST default the language code to "en-GB" when not specified
- **FR-012**: System MUST accept an optional async translator callback function for custom language translation
- **FR-013**: System MUST provide TypeScript/JavaScript bindings (WASM-based for browser support) that can be imported from a package registry (npm)
- **FR-014**: System MUST provide Python bindings that can be imported from a package registry (PyPI)
- **FR-015**: System MUST implement the core message generation logic in Rust for performance
- **FR-016**: System MUST store the message corpus in the repository as a multiline text file, versioned alongside the code
- **FR-017**: System MUST generate messages deterministically (no LLM or runtime generation)
- **FR-018**: System MUST provide TypeScript type definitions (`.d.ts` files) for the JavaScript bindings
- **FR-019**: System MUST handle missing or invalid options gracefully with sensible defaults
- **FR-025**: Rust core MUST raise exceptions/errors (CorpusLoadError, InvalidLanguageCodeError, TemplateVariableError, InvalidTimezoneError) for error conditions. Python/TypeScript bindings MUST catch these errors and implement fallback logic (return fallback messages, truncate inputs, apply defaults).
- **FR-026**: Python/TypeScript bindings MUST validate and sanitize inputs before calling Rust core (e.g., truncate names to 50 characters, validate language codes, validate timezone identifiers). Bindings MUST fall back to defaults ("en-GB", "Europe/London") for invalid inputs before calling Rust core.
- **FR-020**: System MUST support the translator callback only in the high-level language bindings (TypeScript/Python), not in the Rust core
- **FR-021**: System MUST include unit tests in Rust using `cargo test`
- **FR-022**: System MUST include E2E tests in Python using pytest and TypeScript using Vitest
- **FR-023**: System MUST include benchmark tests that measure generation speed and produce results for inclusion in README
- **FR-024**: System MUST verify that each call produces different results (randomness testing)

### Key Entities *(include if feature involves data)*

- **GoodbyeMessage**: A string containing a warm sign-off phrase with optional template variables and emojis. Examples: "Wishing you a liberated day, {name}❤️", "May your path be filled with light, {name}✨", "Until we meet again". Messages may or may not contain template variables.
- **MessageCorpus**: A collection of pre-curated goodbye messages stored in the repository as a multiline text file, then compressed and embedded in the Rust binary. Contains messages in the default language (English) with template variables and emoji annotations. Initial corpus contains 360 messages.
- **GoodbyeOptions**: Configuration object containing optional parameters: language_code (string, default "en-GB"), templateArgs (object with name, location, date, time, and other template variables), use_emojis (boolean, default true), timezone (string, default "Europe/London"), translator (async callback function, optional)
- **CoreGoodbyeOptions**: Simplified options structure for the Rust core library, excluding the translator callback which is handled by language bindings. Includes timezone for date/time variable defaults.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can generate a goodbye message in under 10 milliseconds from function call to returned string (measured on standard hardware)
- **SC-002**: The TypeScript package can be installed and imported in a new project within 30 seconds using standard package manager commands
- **SC-003**: The Python package can be installed and imported in a new project within 30 seconds using standard package manager commands
- **SC-004**: 100% of generated messages contain valid template replacements (no unprocessed placeholders when name is provided)
- **SC-005**: When emoji stripping is enabled, 100% of emoji characters are removed from output messages
- **SC-006**: The message corpus contains 360 unique goodbye messages in the default language (en-GB)
- **SC-007**: The Rust core library compiles to a binary/library that can be linked by both Python and TypeScript bindings
- **SC-008**: CI/CD pipeline successfully publishes packages to npm and PyPI when a release tag is created (⏸️ **DEFERRED** - pending credentials configuration)
- **SC-009**: All generated messages are between 10 and 100 characters in length (excluding template variable placeholders, but including emojis and all other characters)
- **SC-010**: The library handles at least 1000 concurrent function calls per second without performance degradation

## Assumptions

- Users have Rust toolchain 1.91.1+ installed for building the core library (or CI/CD handles this)
- Users have pnpm installed for TypeScript package management
- Users have uv installed for Python package management
- The message corpus will be curated manually and stored in the repository as a multiline text file, versioned alongside the code
- Translation services (if used via translator callback) are provided by the user/external service
- Package registries (npm, PyPI) are accessible and user has appropriate credentials for publishing
- The library targets modern Node.js versions (18+) and Python versions (3.14+)
- The Rust core will use standard FFI mechanisms (PyO3 for Python, WASM for TypeScript/JavaScript for browser support)
- Package versioning is controlled via git tags using semantic versioning, automated via GitHub Actions workflows

## Dependencies

- Rust toolchain 1.91.1+ and Cargo for core library development
- PyO3 for Python bindings
- wasm-pack or similar for WASM compilation (TypeScript/JavaScript bindings for browser support)
- Package managers: pnpm for TypeScript, uv for Python
- CI/CD service (GitHub Actions, free for public repos)
- Access to npm and PyPI for package publishing
- Semantic versioning tool (cargo-semver-checks or similar) for version management

## Out of Scope

- Automatic message corpus generation or curation (manual process)
- Built-in translation services (users provide translator callback if needed)
- Web UI or graphical interface for message generation
- Integration with specific email clients or platforms
- Message history or caching functionality
- User authentication or access control
- Analytics or usage tracking within the library
