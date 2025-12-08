# Examples

This directory contains example usage of the `joyous-departures` package for both Python and TypeScript.

## Python Example

Located in `py-usage/`.

### Setup

```bash
cd py-usage
uv venv --python 3.13
source .venv/bin/activate
uv pip install -e .
```

### Using Local Package (Before Publishing)

1. Build the Python wheel:
   ```bash
   cd ../../bindings/python
   bash scripts/build-wheel.sh
   ```

2. Copy `.env.local.example` to `.env.local`:
   ```bash
   cd ../../examples
   cp .env.local.example .env.local
   ```

3. Edit `.env.local` and set `PYTHON_PACKAGE_PATH` to your wheel:
   ```bash
   PYTHON_PACKAGE_PATH=../../target/wheels/joyous_departures-1.5.2-cp313-cp313-manylinux_2_34_x86_64.whl
   ```

4. Install local package:
   ```bash
   cd py-usage
   python setup_local.py
   ```

### Running the Example

```bash
python main.py
```

### Running Tests

```bash
# Install test dependencies
uv pip install -e ".[dev]"

# Run tests
pytest
```

## TypeScript Example

Located in `ts-usage/`.

### Setup

```bash
cd ts-usage
npm install
```

### Using Local Package (Before Publishing)

1. Build the TypeScript package:
   ```bash
   cd ../../bindings/typescript
   bash scripts/build-package.sh
   ```

2. Copy `.env.local.example` to `.env.local`:
   ```bash
   cd ../../examples
   cp .env.local.example .env.local
   ```

3. Edit `.env.local` and set `TYPESCRIPT_PACKAGE_PATH`:
   ```bash
   TYPESCRIPT_PACKAGE_PATH=../../bindings/typescript
   ```

4. Install local package:
   ```bash
   cd ts-usage
   bash setup_local.sh
   ```

### Running the Example

```bash
npm run start
```

### Running Tests

```bash
# Run tests
npm test

# Run tests in watch mode
npm run test:watch
```

### Cloudflare Workers Compatibility

The TypeScript example includes tests that verify Cloudflare Workers compatibility. The package is designed to work in restricted WASM environments that only allow `WebAssembly.instantiate()` with pre-compiled modules.

Run the Cloudflare Workers compatibility test:
```bash
npm test -- tests/cloudflare-workers.test.ts
```

## Test Coverage

Both examples include comprehensive tests that verify:

- ✅ Basic message generation
- ✅ Custom name and location
- ✅ Emoji handling (with/without)
- ✅ Timezone support
- ✅ Randomness (varied output)
- ✅ All example scenarios

The TypeScript example also includes Cloudflare Workers compatibility tests.

