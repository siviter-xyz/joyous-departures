# Core Definition & Porting Patterns Research

**Created**: 2025-01-27  
**Status**: In Progress  
**Goal**: Evaluate patterns for defining core functionality that ports cleanly to multiple bindings

## Interface Definition Languages

### Overview

Interface Definition Languages (IDL) provide a language-neutral way to define APIs that can be used to generate bindings for multiple languages.

### IDL Tools Evaluation

**Protocol Buffers (protobuf)**:
- **Pros**: Widely used, strong tooling, good type system
- **Cons**: Not designed for Rust-first projects, requires .proto files, adds build complexity
- **Use Case**: Good for RPC services, less suitable for library bindings

**Apache Thrift**:
- **Pros**: Multi-language support, RPC framework included
- **Cons**: Complex setup, not Rust-native, overkill for simple libraries
- **Use Case**: Large distributed systems, not ideal for library bindings

**Rust-Specific IDL Approaches**:
- **Rust macros**: Use proc macros to generate bindings from Rust code
- **Annotations**: Use attributes to mark code for binding generation
- **Custom IDL**: Define API in separate IDL file, generate Rust + bindings

### Current Project Approach

**Manual API Contracts**: 
- Separate contract files: `contracts/rust-api.md`, `contracts/python-api.md`, `contracts/typescript-api.md`
- Manual consistency maintenance
- No automatic generation

**Pros**:
- ✅ Full control over API design
- ✅ Language-specific optimizations possible
- ✅ No build-time code generation overhead

**Cons**:
- ⚠️ Manual consistency maintenance
- ⚠️ Risk of API drift
- ⚠️ No automatic type checking across bindings

### Recommendation

For current project size and complexity, **manual contracts are sufficient**. IDL would add complexity without significant benefit. Consider IDL if:
- API grows significantly (>20 functions)
- Multiple developers working on bindings
- Need for automatic type checking across languages

## Code Generation Approaches

### Rust Macro-Based Generation

**Approach**: Use Rust proc macros to generate bindings from annotated Rust code

**Example**:
```rust
#[multi_bind(python, typescript)]
pub fn generate_goodbye(options: CoreGoodbyeOptions) -> Result<String, GoodbyeError> {
    // Implementation
}
```

**Pros**:
- ✅ Single source of truth (Rust code)
- ✅ Type safety from Rust compiler
- ✅ No separate IDL files

**Cons**:
- ⚠️ Requires custom proc macro development
- ⚠️ Less flexibility for language-specific optimizations
- ⚠️ Complex macro implementation

### Annotation-Based Generation

**Approach**: Use attributes to mark code for binding generation

**Example**:
```rust
#[export_python]
#[export_typescript]
pub struct GoodbyeOptions { ... }
```

**Pros**:
- ✅ Simple annotation syntax
- ✅ Works with existing Rust code
- ✅ Can be selective (only export what's needed)

**Cons**:
- ⚠️ Still requires code generator
- ⚠️ May not handle all Rust types perfectly

### Separate Definition Files

**Approach**: Define API in separate definition file (YAML, JSON, etc.), generate all bindings

**Example**:
```yaml
functions:
  - name: generate_goodbye
    parameters:
      - name: options
        type: GoodbyeOptions
    returns: string
    errors: [CorpusLoadError, InvalidLanguageCodeError]
```

**Pros**:
- ✅ Language-neutral definition
- ✅ Can generate documentation too
- ✅ Clear separation of API from implementation

**Cons**:
- ⚠️ Duplicate definition (in Rust and definition file)
- ⚠️ Risk of definition/implementation drift
- ⚠️ Requires code generator

### Current Project Approach

**Manual Binding Implementation**:
- Rust core defines API
- Python bindings manually wrap Rust API
- TypeScript bindings manually wrap Rust API
- Consistency maintained via contract files

**Recommendation**: Continue with manual approach for current project. Code generation would add complexity without clear benefit for small API.

## Type Safety Across Bindings

### Current Type Mapping

**Rust → Python**:
- `String` → `str`
- `Option<String>` → `Optional[str]`
- `HashMap<String, String>` → `Dict[str, str]`
- `Result<T, E>` → Exception (via PyO3)
- `bool` → `bool`

**Rust → TypeScript**:
- `String` → `string`
- `Option<String>` → `string | undefined`
- `HashMap<String, String>` → `Record<string, string>` or `{ [key: string]: string }`
- `Result<T, E>` → Exception (via wasm-bindgen)
- `bool` → `boolean`

### Type Safety Strategies

**1. Manual Type Definitions** (Current Approach):
- Define types in each language separately
- Maintain consistency manually
- Use contract files as reference

**Pros**: Full control, language-specific optimizations  
**Cons**: Manual maintenance, risk of drift

**2. Generated Type Definitions**:
- Generate TypeScript `.d.ts` from Rust (wasm-bindgen does this)
- Generate Python type stubs from Rust (tools available)
- Single source of truth

**Pros**: Automatic consistency  
**Cons**: May not be perfectly idiomatic

**3. Shared Type Definitions**:
- Define types in IDL or shared format
- Generate types for all languages

**Pros**: Guaranteed consistency  
**Cons**: Requires code generation infrastructure

### Current Project Status

**TypeScript**: ✅ Automatic `.d.ts` generation via wasm-bindgen  
**Python**: ⚠️ Manual type annotations (could use type stubs)

**Recommendation**: 
- Continue with wasm-bindgen automatic TypeScript types
- Consider generating Python type stubs if API grows
- Maintain contract files as documentation

## Version Synchronization

### Current Approach

**Cargo Workspace Version**:
- Single version in `[workspace.package]` in root `Cargo.toml`
- All workspace members inherit version
- Python and TypeScript packages use same version

**Version Management**:
- Semantic versioning (MAJOR.MINOR.PATCH)
- Version updated via git tags
- CI/CD publishes all packages with same version

**Pros**:
- ✅ Single source of truth for version
- ✅ All packages stay in sync
- ✅ Simple version management

**Cons**:
- ⚠️ All packages must version together
- ⚠️ Breaking change in one binding affects all

### Alternative Approaches

**Independent Versioning**:
- Each binding has own version
- Core has separate version
- More flexible but complex

**Semantic Versioning with Compatibility**:
- Core version drives compatibility
- Bindings can have patch versions independently
- Major/minor versions stay in sync

### Breaking Change Management

**Current Strategy**:
- Breaking changes require major version bump
- All packages version together
- Clear migration path in changelog

**Recommendation**: Continue with current approach (workspace version). Independent versioning adds complexity without clear benefit for this project size.

## Porting Process Recommendations

### Current Porting Process

**Adding New Feature**:
1. Implement in Rust core (`joy-generator`)
2. Add tests in Rust
3. Expose via Python binding (`bindings/python/src/lib.rs`)
4. Expose via TypeScript binding (`bindings/typescript/src/lib.rs`)
5. Update contract files
6. Add tests in Python and TypeScript
7. Update documentation

**Manual Process**: Each step done manually, consistency maintained via contract files

### Recommended Porting Process

**1. Define in Core**:
- Implement feature in Rust core
- Add comprehensive tests
- Document in Rust code

**2. Update Contracts**:
- Update `contracts/rust-api.md`
- Update `contracts/python-api.md`
- Update `contracts/typescript-api.md`
- Ensure consistency

**3. Implement Python Binding**:
- Add PyO3 wrapper in `bindings/python/src/lib.rs`
- Handle type conversions
- Add error handling
- Add Python tests

**4. Implement TypeScript Binding**:
- Add wasm-bindgen wrapper in `bindings/typescript/src/lib.rs`
- Handle type conversions
- Add error handling
- Add TypeScript tests

**5. Update Documentation**:
- Update README examples
- Update API documentation
- Add migration guide if breaking change

### Automation Opportunities

**Potential Automations**:
1. **Contract Validation**: Script to check contract files match implementation
2. **Type Checking**: Generate and validate type definitions
3. **Test Generation**: Generate basic tests from contracts
4. **Documentation Generation**: Generate API docs from contracts

**Recommendation**: Start with contract validation script. Other automations can be added as project grows.

