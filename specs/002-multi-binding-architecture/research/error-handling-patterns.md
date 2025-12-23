# Error Handling & Debugging Patterns Research

**Created**: 2025-01-27  
**Status**: In Progress  
**Goal**: Identify best practices for error handling in multi-binding architectures

## Error Message Patterns

### Current Implementation Analysis

**Rust Core Error Types** (from `joy-generator/src/error.rs`):
- `CorpusLoadError`: "Failed to load corpus: {0}"
- `InvalidLanguageCodeError`: "Invalid language code: {0}"
- `InvalidTimezoneError`: "Invalid timezone: {0}"
- `TemplateVariableError`: "Template variable error: {0}"

**Python Bindings Error Handling**:
- PyO3 converts Rust `Result<T, E>` to Python exceptions
- Errors raised as `PyRuntimeError` with formatted messages
- Fallback messages returned for recoverable errors (graceful degradation)
- Example: `PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to generate message: {}", e))`

**TypeScript/WASM Bindings Error Handling**:
- wasm-bindgen converts Rust `Result` to JavaScript exceptions
- Errors caught in try/catch blocks
- Error messages checked via `error.message.includes("ErrorType")`
- Fallback messages returned for recoverable errors
- **Critical Issue**: No error handling around WASM initialization - results in empty error objects

### Error Message Requirements (from Constitution)

Error messages MUST be:
- Clear and actionable
- Include context (what failed, why it failed)
- Provide guidance on how to fix the issue

### Error Message Patterns by Environment

#### Cloudflare Workers

**Current Problem**: Empty error objects `{"error": {}, "message": "Failed to load..."}`

**Required Pattern**:
```javascript
try {
  await initializeWasm();
} catch (error) {
  // Provide detailed error information
  return {
    error: {
      type: "WASM_INITIALIZATION_ERROR",
      message: error.message,
      stack: error.stack,
      cause: error.cause
    },
    message: `Failed to initialize WASM: ${error.message}`,
    guidance: "Check WASM file path and ensure WebAssembly.instantiate is supported"
  };
}
```

**Key Components**:
1. **Error type/category**: `WASM_INITIALIZATION_ERROR`, `WASM_FUNCTION_ERROR`, etc.
2. **Specific failure point**: "WASM loading", "WASM instantiation", "function call", etc.
3. **Actionable guidance**: What to check, how to fix

#### Node.js

**Current Pattern**: Works but could be improved
- Errors thrown as JavaScript exceptions
- Error messages include error type from Rust
- Fallback messages for recoverable errors

**Recommended Pattern**:
```javascript
try {
  result = await generateGoodbye(options);
} catch (error) {
  if (error.message.includes("CorpusLoadError")) {
    // Return fallback with context
    return fallbackMessage;
  }
  // Re-throw with additional context
  throw new Error(`Message generation failed: ${error.message}. Check corpus loading.`);
}
```

#### Python

**Current Pattern**: Good error handling
- PyO3 automatically converts Rust errors to Python exceptions
- Error messages include context from Rust
- Fallback messages for recoverable errors

**Example**:
```python
try:
    result = generate_goodbye(options)
except RuntimeError as e:
    # Error message includes context from Rust
    # e.g., "Failed to generate message: Invalid language code: xx"
    raise
```

### Error Message Templates

**WASM Initialization Errors**:
```
Error Type: WASM_INITIALIZATION_ERROR
Message: Failed to initialize WASM module: {specific_error}
Failure Point: {WASM_loading | WASM_instantiation | binding_setup}
Guidance: {actionable_steps}
```

**Function Call Errors**:
```
Error Type: WASM_FUNCTION_ERROR
Message: Function call failed: {function_name}: {specific_error}
Failure Point: {parameter_validation | WASM_call | result_processing}
Guidance: {actionable_steps}
```

**Example Implementation**:
```javascript
function createDetailedError(error, type, failurePoint, guidance) {
  return {
    error: {
      type: type,
      message: error.message,
      failurePoint: failurePoint,
      stack: error.stack
    },
    message: `${type}: ${error.message}`,
    guidance: guidance
  };
}
```

## Debugging Strategies

### Cloudflare Workers WASM Debugging

**Current Problem**: Silent failures with empty error objects make debugging impossible

**Debugging Checklist**:
1. **Check WASM initialization**:
   - Verify WASM file is accessible
   - Check `import.meta.url` resolution
   - Verify `fetch()` works in Workers environment
   - Check `WebAssembly.instantiate()` is called correctly

2. **Check top-level await**:
   - Verify no top-level await in module imports
   - Check wrapper files for `await` at module level
   - Ensure initialization happens in fetch handler

3. **Check error handling**:
   - Wrap all WASM operations in try/catch
   - Log errors with full context
   - Return descriptive error messages

4. **Use Cloudflare Workers logs**:
   - Enable detailed logging
   - Check `console.log` output in Workers dashboard
   - Use `console.error` for error details

**Recommended Debugging Pattern**:
```javascript
let wasmModule = null;
let initError = null;

export default {
  async fetch(request, env, ctx) {
    if (!wasmModule && !initError) {
      try {
        console.log('[DEBUG] Initializing WASM...');
        wasmModule = await initializeWasm();
        console.log('[DEBUG] WASM initialized successfully');
      } catch (error) {
        initError = error;
        console.error('[ERROR] WASM initialization failed:', {
          message: error.message,
          stack: error.stack,
          cause: error.cause
        });
        return new Response(JSON.stringify({
          error: {
            type: 'WASM_INITIALIZATION_ERROR',
            message: error.message,
            stack: error.stack
          }
        }), {
          status: 500,
          headers: { 'Content-Type': 'application/json' }
        });
      }
    }
    
    if (initError) {
      return new Response(JSON.stringify({
        error: {
          type: 'WASM_INITIALIZATION_ERROR',
          message: initError.message
        }
      }), { status: 500 });
    }
    
    // Use WASM module...
  }
}
```

### Node.js WASM Debugging

**Strategies**:
1. Use Node.js debugger with source maps
2. Check WASM file path resolution
3. Verify `fs.readFileSync` works
4. Check `WebAssembly.instantiate` compatibility
5. Use `console.log` for initialization steps

### Python Extension Debugging

**Strategies**:
1. Use Python debugger (pdb) with extension modules
2. Check PyO3 error messages (include Rust error context)
3. Verify extension module loads correctly
4. Check Python version compatibility
5. Use `print()` or logging for debugging

### Common Failure Modes

1. **WASM Loading Failures**:
   - File not found
   - Incorrect path resolution
   - Network errors (fetch)

2. **WASM Instantiation Failures**:
   - Invalid WASM binary
   - Missing imports
   - Memory allocation failures

3. **Binding Initialization Failures**:
   - `__wbg_set_wasm` not called
   - Incorrect exports
   - Type mismatches

4. **Function Call Failures**:
   - Invalid parameters
   - WASM module not initialized
   - Memory errors

## Error Propagation Patterns

### Rust Core → Python Bindings

**Pattern**: PyO3 automatic conversion
- Rust `Result<T, E>` → Python exception
- `Ok(value)` → Returns value
- `Err(error)` → Raises Python exception with error message

**Example**:
```rust
#[pyfunction]
fn generate_goodbye(options: CoreGoodbyeOptions) -> PyResult<String> {
    match rust_generate_goodbye(&options) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyErr::new::<PyRuntimeError, _>(format!("Failed: {}", e)))
    }
}
```

**Error Message Flow**:
1. Rust: `GoodbyeError::InvalidLanguageCodeError("xx")`
2. PyO3: Converts to `PyRuntimeError`
3. Python: Raises `RuntimeError("Failed: Invalid language code: xx")`

### Rust Core → TypeScript/WASM Bindings

**Pattern**: wasm-bindgen automatic conversion
- Rust `Result<T, E>` → JavaScript exception
- `Ok(value)` → Returns value
- `Err(error)` → Throws JavaScript error with error message

**Example**:
```rust
#[wasm_bindgen]
pub fn generate_goodbye(...) -> String {
    match rust_generate_goodbye(&options) {
        Ok(result) => result,
        Err(e) => panic!("{}", e) // wasm-bindgen converts panic to JS error
    }
}
```

**Error Message Flow**:
1. Rust: `GoodbyeError::InvalidLanguageCodeError("xx")`
2. wasm-bindgen: Converts panic to JavaScript Error
3. JavaScript: Throws `Error("Invalid language code: xx")`

**Issue**: Panic in WASM can cause issues - better to return Result and handle in JS

### Error Translation Between Layers

**Current Approach**: Direct error propagation
- Rust errors → Binding errors → User errors
- Error messages preserved through layers
- Type information may be lost

**Recommended Approach**: Structured error translation
```rust
// Rust core
pub enum GoodbyeError {
    CorpusLoadError(String),
    InvalidLanguageCodeError(String),
}

// TypeScript binding
interface GoodbyeError {
    type: 'CORPUS_LOAD_ERROR' | 'INVALID_LANGUAGE_CODE_ERROR';
    message: string;
    details?: string;
}

// Translate Rust error to structured JS error
function translateError(rustError: Error): GoodbyeError {
    if (rustError.message.includes('CorpusLoadError')) {
        return {
            type: 'CORPUS_LOAD_ERROR',
            message: 'Failed to load message corpus',
            details: rustError.message
        };
    }
    // ...
}
```

## Failure Point Detection

### Failure Point Categories

1. **WASM Loading**: File loading, path resolution, network errors
2. **WASM Initialization**: Module instantiation, binding setup
3. **Function Call**: Parameter validation, WASM execution, result processing

### Detecting Failure Points

**Pattern**: Wrap each stage with try/catch and identify failure point

```javascript
async function initializeWasm() {
  let failurePoint = null;
  
  try {
    // Stage 1: Load WASM file
    failurePoint = 'WASM_LOADING';
    const wasmUrl = new URL('./module.wasm', import.meta.url);
    const response = await fetch(wasmUrl);
    if (!response.ok) {
      throw new Error(`Failed to fetch WASM: ${response.status} ${response.statusText}`);
    }
    
    // Stage 2: Get ArrayBuffer
    failurePoint = 'WASM_BUFFER';
    const wasmBuffer = await response.arrayBuffer();
    
    // Stage 3: Instantiate WASM
    failurePoint = 'WASM_INSTANTIATION';
    const wasmModule = await WebAssembly.instantiate(wasmBuffer, imports);
    
    // Stage 4: Initialize bindings
    failurePoint = 'BINDING_INITIALIZATION';
    imports.__wbg_set_wasm(wasmModule.instance.exports);
    
    return wasmModule;
  } catch (error) {
    return {
      error: {
        type: 'WASM_INITIALIZATION_ERROR',
        failurePoint: failurePoint,
        message: error.message,
        stack: error.stack
      }
    };
  }
}
```

### Error Categorization Strategy

**Error Categories**:
- `WASM_LOADING_ERROR`: File not found, network error, path resolution
- `WASM_INSTANTIATION_ERROR`: Invalid WASM, missing imports, memory error
- `BINDING_INITIALIZATION_ERROR`: `__wbg_set_wasm` failure, export mismatch
- `FUNCTION_CALL_ERROR`: Parameter validation, execution error, result processing

**Implementation**:
```javascript
function categorizeError(error, context) {
  if (error.message.includes('fetch')) {
    return { category: 'WASM_LOADING_ERROR', failurePoint: 'WASM_LOADING' };
  }
  if (error.message.includes('WebAssembly.instantiate')) {
    return { category: 'WASM_INSTANTIATION_ERROR', failurePoint: 'WASM_INSTANTIATION' };
  }
  if (error.message.includes('__wbg_set_wasm')) {
    return { category: 'BINDING_INITIALIZATION_ERROR', failurePoint: 'BINDING_INITIALIZATION' };
  }
  if (context === 'function_call') {
    return { category: 'FUNCTION_CALL_ERROR', failurePoint: 'FUNCTION_EXECUTION' };
  }
  return { category: 'UNKNOWN_ERROR', failurePoint: 'UNKNOWN' };
}
```

### Actionable Guidance Generation

**Pattern**: Generate guidance based on error category and failure point

```javascript
function generateGuidance(errorCategory, failurePoint) {
  const guidance = {
    WASM_LOADING_ERROR: {
      WASM_LOADING: [
        'Check WASM file exists in package',
        'Verify import.meta.url resolves correctly',
        'Check network/fetch permissions in Workers'
      ]
    },
    WASM_INSTANTIATION_ERROR: {
      WASM_INSTANTIATION: [
        'Verify WASM binary is valid',
        'Check all required imports are provided',
        'Ensure WebAssembly.instantiate is supported'
      ]
    },
    BINDING_INITIALIZATION_ERROR: {
      BINDING_INITIALIZATION: [
        'Verify __wbg_set_wasm function exists',
        'Check WASM exports match expected bindings',
        'Ensure wasm-bindgen --target bundler was used'
      ]
    },
    FUNCTION_CALL_ERROR: {
      FUNCTION_EXECUTION: [
        'Check function parameters are valid',
        'Verify WASM module is initialized',
        'Check function signature matches binding'
      ]
    }
  };
  
  return guidance[errorCategory]?.[failurePoint] || ['Check error details and logs'];
}
```

## Error Message Templates

### Template Structure

```javascript
{
  error: {
    type: "ERROR_CATEGORY",
    failurePoint: "SPECIFIC_FAILURE_POINT",
    message: "Detailed error message",
    stack?: "Error stack trace",
    cause?: "Underlying error cause"
  },
  message: "User-friendly error message",
  guidance: ["Actionable step 1", "Actionable step 2", ...]
}
```

### Template Examples

#### WASM Initialization Error Template

```javascript
{
  error: {
    type: "WASM_INITIALIZATION_ERROR",
    failurePoint: "WASM_LOADING" | "WASM_INSTANTIATION" | "BINDING_INITIALIZATION",
    message: "Failed to initialize WASM module: {specific_error}",
    stack: "{error.stack}",
    cause: "{error.cause}"
  },
  message: "Failed to load WASM module. This may indicate a packaging or compatibility issue.",
  guidance: [
    "Verify WASM file is included in package",
    "Check import.meta.url resolves correctly",
    "Ensure WebAssembly.instantiate is supported in this environment"
  ]
}
```

#### Function Call Error Template

```javascript
{
  error: {
    type: "FUNCTION_CALL_ERROR",
    failurePoint: "PARAMETER_VALIDATION" | "WASM_EXECUTION" | "RESULT_PROCESSING",
    message: "Function call failed: {function_name}: {specific_error}",
    stack: "{error.stack}"
  },
  message: "Failed to execute function. Check parameters and ensure WASM module is initialized.",
  guidance: [
    "Verify all required parameters are provided",
    "Check parameter types match expected types",
    "Ensure WASM module was initialized successfully"
  ]
}
```

#### Corpus Load Error Template

```javascript
{
  error: {
    type: "CORPUS_LOAD_ERROR",
    failurePoint: "CORPUS_DECOMPRESSION" | "CORPUS_PARSING",
    message: "Failed to load message corpus: {specific_error}"
  },
  message: "Failed to load message corpus. Using fallback message.",
  guidance: [
    "This is typically a recoverable error",
    "Fallback message will be returned",
    "Check corpus file integrity if error persists"
  ]
}
```

### Implementation Example

```javascript
function createErrorResponse(error, category, failurePoint) {
  const templates = {
    WASM_INITIALIZATION_ERROR: {
      message: "Failed to initialize WASM module",
      guidance: getWasmInitGuidance(failurePoint)
    },
    FUNCTION_CALL_ERROR: {
      message: "Function execution failed",
      guidance: getFunctionCallGuidance(failurePoint)
    },
    // ... more templates
  };
  
  const template = templates[category] || {
    message: "An error occurred",
    guidance: ["Check error details", "Review logs"]
  };
  
  return {
    error: {
      type: category,
      failurePoint: failurePoint,
      message: error.message,
      stack: error.stack,
      cause: error.cause
    },
    message: template.message,
    guidance: template.guidance
  };
}
```

### Best Practices

1. **Always include error type**: Makes error handling programmatic
2. **Specify failure point**: Helps identify exact failure location
3. **Provide actionable guidance**: Tell users what to check/fix
4. **Include original error**: Preserve error chain for debugging
5. **Use consistent structure**: Makes error handling predictable

