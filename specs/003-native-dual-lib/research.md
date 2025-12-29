# Research: Native Dual-Language Library (v2.0)

**Feature**: Native Dual-Language Library  
**Spec**: [spec.md](./spec.md)  
**Plan**: [plan.md](./plan.md)

## Background

The v1.x architecture used Rust core with WASM bindings for TypeScript and PyO3/maturin for Python. This caused persistent issues in Cloudflare Workers due to WASM initialization problems (Invalid URL string errors, `import.meta.url` not supported).

## Decision: Remove WASM Entirely

### Rationale

1. **Cloudflare Workers Compatibility**: The WASM binding uses `import.meta.url` for module resolution, which is not supported in Cloudflare Workers. Various workarounds (wasm_module bindings, explicit initialization) were attempted but proved unreliable.

2. **Complexity vs. Benefit**: The Rust core was impressive (~615ns generation time) but overkill for string manipulation on a ~400 message corpus. Native implementations can achieve <10ms easily.

3. **Installation Friction**: Users needed Rust toolchain, wasm-pack, and clang to build from source. This created a high barrier to entry.

### Alternatives Considered

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| Fix WASM bindings | Keeps Rust core | Complex, unreliable in Workers | ❌ Rejected |
| Pure native implementations | Simple, universal compatibility | Slight perf regression (still fast) | ✅ Selected |
| Hybrid (WASM + native fallback) | Best of both | Double maintenance burden | ❌ Rejected |

## Decision: Code-Gen from Text File

### Rationale

1. **Zero Runtime I/O**: Embedding corpus as code constants eliminates file reads, fetch calls, and URL resolution—all sources of environment-specific issues.

2. **Maximum Portability**: Generated code uses only language primitives (const arrays, tuples). Works in every environment.

3. **Tree-Shaking**: TypeScript bundlers can optimize unused exports when corpus is a const.

4. **Human-Readable Source**: Maintainers edit plain text files, not JSON or TypeScript.

### Alternatives Considered

| Approach | Pros | Cons | Decision |
|----------|------|------|----------|
| JSON import | Simple | Requires bundler JSON support, runtime parsing | ❌ Rejected |
| Fetch at runtime | Lazy loading | Network dependency, async init | ❌ Rejected |
| Code-gen to constants | Zero I/O, fast, portable | Slightly larger source files | ✅ Selected |
| Manual string literal | No build step | Hard to maintain | ❌ Rejected |

## Decision: Node.js/TypeScript Generator

### Rationale

1. **Existing Toolchain**: Developers already have Node.js for the TypeScript package.

2. **Simplicity**: String manipulation at this scale doesn't need Rust's performance.

3. **Maintainability**: TypeScript is more accessible to contributors than Rust.

### Alternatives Considered

| Language | Pros | Cons | Decision |
|----------|------|------|----------|
| Node.js/TypeScript | Existing toolchain, simple | Slightly slower than Rust | ✅ Selected |
| Python | Also available | Adds Python as build dependency for TS | ❌ Rejected |
| Rust | Maximum performance | Overkill, adds build complexity | ❌ Rejected |
| Shell script | Minimal dependencies | Hard to maintain, poor error handling | ❌ Rejected |

## Decision: Deduplicate at Generation Time

### Rationale

The source corpus (`corpus/en-GB.txt`) contains duplicates (~360 lines but many repeated). Deduplication at generation time:

1. Reduces bundle size
2. Ensures equal probability for each unique message
3. Keeps source file as-is (can be cleaned later)

## Decision: Single Language (en-GB) for v2.0

### Rationale

1. **Scope Management**: Multi-language support adds complexity (bundle size, API design for language selection).

2. **Existing Pattern**: The `translator` callback already provides runtime translation capability.

3. **Extensible**: Generator architecture supports future language files without API changes.

## Decision: Generated Files in Repository

### Rationale

1. **Visible in PRs**: Reviewers can see exactly what changed in generated code.

2. **No CI Build Step**: Installation and CI don't need to run the generator.

3. **Deterministic**: Same corpus always produces same output.

### Pre-Commit Hook

The generator runs via pre-commit hook when `corpus/*.txt` changes, ensuring generated files stay in sync.

## Performance Expectations

| Metric | v1.x (WASM) | v2.0 (Native) | Target |
|--------|-------------|---------------|--------|
| Generation time | ~615ns | <1ms | <10ms |
| Package size (TS) | ~150KB | <50KB | <100KB |
| Package size (Py) | ~2MB | <30KB | <50KB |
| Installation time | 30-60s | <10s | <30s |
| Runtime deps | 0 | 0 | 0 |

## Conclusion

The native dual-language approach trades marginal performance (~1000x slower but still microseconds) for:

- Universal compatibility (especially Cloudflare Workers)
- Simpler installation (no build tools)
- Easier maintenance (TypeScript/Python vs Rust)
- Smaller package sizes

This is the right trade-off for a library whose primary value is convenience, not raw performance.

