# Publishing Guide

This document explains how to publish `joyous-departures` packages to npm and PyPI.

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

- **npm**: The package `@siviter-xyz/joyous-departures` will be created on first publish
- **PyPI**: The package `joyous-departures` will be created on first publish

## Publishing Process

### Automatic Publishing (Recommended)

**Semantic Release Workflow:**

The project uses [semantic-release](https://github.com/semantic-release/semantic-release) to automatically:
- Analyze commit messages (conventional commits)
- Determine version bump (major/minor/patch)
- Update version in all package files
- Create git tag
- Generate changelog
- Trigger publish workflow

**How it works:**

1. **Make commits with conventional commit messages:**
   ```bash
   git commit -m "feat: add new feature"      # Triggers minor version bump
   git commit -m "fix: fix bug"                # Triggers patch version bump
   git commit -m "feat!: breaking change"     # Triggers major version bump
   ```

2. **Push to main branch:**
   ```bash
   git push origin main
   ```

3. **After CI passes, semantic-release will:**
   - Analyze commits since last release
   - Determine next version
   - Update `Cargo.toml`, `package.json`, and `pyproject.toml`
   - Create git tag (e.g., `v0.1.1`)
   - Generate/update `CHANGELOG.md`
   - Push tag and changes to repository
   - Trigger publish workflow automatically

**Manual Tag Publishing (Alternative):**

If you need to publish manually:
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

If you prefer to publish manually without semantic-release:

```bash
# 1. Create a version tag
git tag v0.1.0

# 2. Update version in all files (if needed)
./scripts/update-versions.sh 0.1.0

# 3. Build packages
./scripts/build.sh

# 4. Publish
cd bindings/typescript
pnpm publish --access public

cd ../python
maturin publish
```

**Note:** The `scripts/version.sh` script has been replaced by `scripts/update-versions.sh` which is used by semantic-release. For manual version updates, use `update-versions.sh` directly.

## Version Management

Use semantic versioning:
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

## Verification

After publishing, verify packages are available:

```bash
# npm
pnpm info @siviter-xyz/joyous-departures

# PyPI
pip index versions joyous-departures
```


