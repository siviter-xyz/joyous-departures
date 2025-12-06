# Research: Goodbye Message Generator

**Date**: 2025-01-27\
**Feature**: 001-goodbye-generator

## Technology Choices

### Rust FFI Bindings for TypeScript/Node.js

**Decision**: Use `wasm-pack` for WASM compilation (WebAssembly)

**Rationale**:

* WASM enables browser support, which is a key requirement
* Good performance with modern WASM runtimes
* Cross-platform compatibility (works in Node.js and browsers)
* Strong TypeScript support with automatic `.d.ts` generation
* Active maintenance and good documentation
* Single build target for both Node.js and browser environments

**Alternatives Considered**:

* `napi-rs`: Better performance for Node.js-only, but doesn't support browsers
* `superffi`: Promising but newer, less mature ecosystem
* `neon`: Older, less actively maintained

### Rust FFI Bindings for Python

**Decision**: Use `pyo3` with `maturin` for Python bindings

**Rationale**:

* `pyo3` is the de facto standard for Rust-Python interop
* `maturin` simplifies building and publishing Python packages from Rust
* Excellent performance (native extension modules)
* Strong type safety and error handling
* Well-documented and actively maintained

**Alternatives Considered**:

* `cffi`: Would require writing C bindings manually, more error-prone
* `ctypes`: Lower-level, more manual work required

### Message Corpus Storage

**Decision**: Store messages as compressed static data embedded in binary

**Rationale**:

* Fast access (no file I/O)
* Small binary size with compression
* Deterministic (no external dependencies)
* Works across all platforms

**Implementation Approach**:

* Store corpus in repository as multiline text file (`corpus/en-GB.txt`) with 360 messages, one per line
* Version corpus alongside code in git
* At build time, read corpus file, compress using `flate2` or `lz4` (lz4 for faster decompression)
* Embed compressed corpus in binary using `include_bytes!`
* Decompress at library initialization or on first use
* Store as array of structs with metadata (language, emoji presence, template variables)

**Alternatives Considered**:

* External JSON/YAML files: Requires file I/O, deployment complexity
* Database: Overkill for static data, adds dependencies
* Runtime generation: Violates requirement for deterministic, pre-generated messages
* Binary-only storage: Loses version control and human readability

### Template Variable Replacement

**Decision**: Use simple string replacement with `{variable_name}` syntax

**Rationale**:

* Simple and intuitive (Mustache-like syntax)
* Easy to implement and test
* No external dependencies needed
* Fast performance

**Implementation**:

* Use Rust's `str::replace` for simple cases
* Support multiple variables per message
* Validate variable names (alphanumeric + underscore only)
* Default values for missing variables

**Alternatives Considered**:

* Full Mustache library: Overkill for simple use case, adds dependency
* Regex-based: More complex, potential performance overhead
* Custom parser: Unnecessary complexity

### Emoji Handling

**Decision**: Use Unicode emoji detection and removal library

**Rationale**:

* Accurate emoji detection (handles all Unicode emoji ranges)
* Handles emoji sequences (skin tones, flags, etc.)
* Well-tested libraries available

**Implementation**:

* Use `unicode-emoji` crate or similar for detection
* Strip emojis in post-processing step when `use_emojis: false`
* Preserve emoji metadata in corpus for faster processing

**Alternatives Considered**:

* Regex-based: Less accurate, misses edge cases
* Manual emoji list: Hard to maintain, incomplete

### Internationalization

**Decision**: Core corpus in English, translator callback for other languages

**Rationale**:

* Keeps core library simple and focused
* Allows users to choose their translation method (LLM, API, etc.)
* No hard dependency on translation services
* Can add native language support later without breaking changes

**Implementation**:

* Core `generate_goodbye` function accepts `language_code`
* If language\_code != "en-GB" and no native support, call translator callback
* Translator callback signature: `async fn(language_code: &str, message: &str) -> String` (async for TypeScript/Python bindings)
* Rust core doesn't handle async - language bindings handle async translator calls
* Fallback to English if translator fails or not provided

**Future Enhancement**:

* Add native support for common languages (Spanish, French, German, etc.)
* Store translations in compressed corpus
* Use translator callback only for unsupported languages

### Compression Library

**Decision**: Use `lz4` for message corpus compression

**Rationale**:

* Fast decompression (important for performance goal of <10ms)
* Good compression ratio for text data
* Small dependency footprint
* Widely used and well-tested

**Alternatives Considered**:

* `flate2` (gzip): Better compression, slower decompression
* `zstd`: Good balance but larger dependency
* `brotli`: Excellent compression, slower decompression

### CI/CD and Publishing

**Decision**: Use GitHub Actions for CI/CD

**Rationale**:

* Free for public repositories
* Excellent integration with GitHub
* Easy to set up workflows for multiple platforms
* Good documentation and community support

**Publishing Strategy**:

* Versioning: Controlled via git tags using semantic versioning (e.g., `v1.0.0`)
* Use `cargo-semver-checks` or similar tool to validate semantic versioning
* Automated versioning script (`scripts/version.sh`) extracts version from git tag
* npm: Use `pnpm publish` in GitHub Actions workflow, triggered by version tags
* PyPI: Use `maturin publish` in GitHub Actions workflow, triggered by version tags
* Automated testing before publishing
* Build for multiple platforms (Linux, macOS, Windows)
* GitHub Actions workflow (`version.yml`) automates the versioning and publishing process

**Alternatives Considered**:

* GitLab CI/CD: Also free, but GitHub Actions more widely used
* CircleCI/Travis CI: Free tiers more limited
* Manual publishing: Error-prone, not scalable

## Performance Considerations

### Message Generation Speed

* Target: <10ms from function call to returned string
* Strategies:
  * Pre-decompress corpus at library initialization (one-time cost)
  * Use efficient random number generation (fast RNG, not cryptographically secure)
  * Minimize allocations (reuse string buffers where possible)
  * Cache compiled templates if needed

### Memory Usage

* Target: <10MB total (corpus + runtime)
* Strategies:
  * Compress corpus (expect 50-70% reduction)
  * Use string interning for common values
  * Lazy loading if corpus is very large (not expected for initial 50 messages)

### Concurrent Access

* Requirement: Thread-safe for 1000+ concurrent calls/second
* Strategies:
  * Use `Arc` for shared corpus data (read-only after initialization)
  * Use thread-local RNG or atomic counter for randomness
  * Avoid locks in hot path (corpus is read-only)

## Testing Strategy

### Unit Tests

* Test message generation with various option combinations
* Test template replacement with edge cases (missing variables, special characters)
* Test emoji stripping accuracy
* Test error handling (invalid language codes, translator failures)

### Integration Tests

* Test Python bindings end-to-end
* Test TypeScript bindings end-to-end
* Test cross-platform compatibility
* Test performance benchmarks

### Contract Tests

* Verify API contracts match specification
* Verify type definitions are correct
* Verify error messages are clear

## Security Considerations

* No user input validation needed (all inputs are developer-controlled)
* No network access (except via optional translator callback)
* No file system access (corpus embedded in binary)
* Safe string handling (Rust's type system prevents common vulnerabilities)

## Open Questions Resolved

1. **Q**: Should we use WASM or native bindings for TypeScript?\
   **A**: Native bindings (napi-rs) for better performance, since browser support is not required.

2. **Q**: How should we handle message corpus updates?\
   **A**: New corpus versions require library updates and republishing. Consider versioning corpus separately in future.

3. **Q**: Should translator callback be async?\
   **A**: No, keep it synchronous for simplicity. Users can wrap async translators if needed.

4. **Q**: How many messages should be in initial corpus?\
   **A**: Generate 360 messages initially. Will expand/curate going forward.
