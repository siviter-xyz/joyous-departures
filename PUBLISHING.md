# Publishing Guide

This document explains how to publish `joy-goodbye` packages to npm and PyPI.

## Prerequisites

### 1. GitHub Secrets

You need to add two secrets to your GitHub repository:

1. **NPM_TOKEN** - npm access token
   - Go to https://www.npmjs.com/settings/YOUR_USERNAME/tokens
   - Create a new "Automation" token
   - Copy the token
   - In GitHub: Settings → Secrets and variables → Actions → New repository secret
   - Name: `NPM_TOKEN`
   - Value: Your npm token

2. **PYPI_API_TOKEN** - PyPI API token
   - Go to https://pypi.org/manage/account/token/
   - Create a new API token
   - Copy the token
   - In GitHub: Settings → Secrets and variables → Actions → New repository secret
   - Name: `PYPI_API_TOKEN`
   - Value: Your PyPI token

### 2. Package Registration

Before first publish, you may need to:

- **npm**: The package `@siviter-xyz/joy-goodbye` will be created on first publish
- **PyPI**: The package `joy-goodbye` will be created on first publish

## Publishing Process

### Automatic Publishing (Recommended)

1. **Create a version tag:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **GitHub Actions will automatically:**
   - Extract version from tag
   - Update package.json and pyproject.toml
   - Build packages
   - Publish to npm and PyPI

### Manual Publishing (Alternative)

If you prefer to publish manually:

```bash
# 1. Update version in all files
./scripts/version.sh

# 2. Build packages
./scripts/build.sh

# 3. Publish
cd bindings/typescript
pnpm publish --access public

cd ../python
maturin publish
```

## Version Management

Use semantic versioning:
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

## Verification

After publishing, verify packages are available:

```bash
# npm
pnpm info @siviter-xyz/joy-goodbye

# PyPI
pip index versions joy-goodbye
```

