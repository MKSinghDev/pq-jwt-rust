# 📦 Publishing Guide

This document explains how to publish `pq-jwt` to crates.io using GitHub Actions.

## 🔑 One-Time Setup

### 1. Get a crates.io API Token

1. Go to [crates.io](https://crates.io/) and log in
2. Navigate to **Account Settings** → **API Tokens**
3. Click **New Token**
4. Give it a name (e.g., "GitHub Actions - pq-jwt")
5. Copy the token (you won't see it again!)

### 2. Add Token to GitHub Secrets

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `CARGO_REGISTRY_TOKEN`
5. Value: Paste your crates.io token
6. Click **Add secret**

### 3. Verify Ownership (First Publish Only)

If this is your first time publishing this crate:

```bash
# Login to crates.io
cargo login

# Publish manually for the first time
cargo publish

# OR do a dry run first
cargo publish --dry-run
```

After the first manual publish, GitHub Actions can handle subsequent releases.

## 🚀 Release Process

### Step 1: Update Version

Edit `Cargo.toml` and bump the version:

```toml
[package]
name = "pq-jwt"
version = "0.2.0"  # ← Update this
```

### Step 2: Update CHANGELOG (Optional but Recommended)

Create or update `CHANGELOG.md`:

```markdown
## [0.2.0] - 2025-01-15

### Added
- New feature X
- Support for Y

### Fixed
- Bug fix Z

### Changed
- Improved performance for A
```

### Step 3: Commit and Push

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin trunk
```

### Step 4: Create and Push Tag

```bash
# Create a tag matching the version
git tag v0.2.0

# Push the tag to GitHub
git push origin v0.2.0
```

### Step 5: Watch the Magic Happen! ✨

The GitHub Action will automatically:
1. ✅ Run all tests
2. ✅ Check code formatting
3. ✅ Run clippy lints
4. ✅ Verify version matches tag
5. ✅ Build release binary
6. 🚀 Publish to crates.io
7. 📝 Create GitHub Release with notes

## 📋 Workflows Explained

### 1. **CI Workflows**

#### **GitHub Actions** (`ci.yml`)
**Triggers:** Pull requests to trunk

**What it does:**
- Tests on Linux, macOS, and Windows
- Tests on stable and nightly Rust
- Runs code formatting checks
- Runs clippy lints
- Generates code coverage
- Runs security audit
- Checks documentation builds
- Verifies MSRV (Minimum Supported Rust Version)

**Status:** ![CI](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)

#### **CircleCI**
**Triggers:** All PRs and commits

**What it does:**
- Fast PR checks (format, clippy, tests)
- Comprehensive trunk checks (includes coverage, benchmarks)
- Nightly security audits
- Builds artifacts and documentation

**Status:** [![CircleCI](https://circleci.com/gh/MKSinghDev/pq-jwt-rust.svg?style=shield)](https://circleci.com/gh/MKSinghDev/pq-jwt-rust)

### 2. **Publish Workflow** (`publish.yml`)

**Triggers:** When you push a tag like `v*.*.*` (e.g., `v0.2.0`)

**What it does:**
- Runs all quality checks
- Verifies tag matches Cargo.toml version
- Publishes to crates.io
- Creates GitHub Release

**Usage:**
```bash
git tag v0.2.0
git push origin v0.2.0
```

### 3. **Release Check Workflow** (`release-check.yml`)

**Triggers:** Pull requests to trunk with "release" in title or label

**What it does:**
- Verifies version was bumped
- Checks all tests pass
- Does dry-run publish
- Verifies docs build
- Warns about TODO/FIXME comments
- Posts summary comment on PR

**Usage:**
Create a PR with title like: "Release v0.2.0" or add the `release` label.

## 🎯 Quick Reference

### Patch Release (0.1.0 → 0.1.1)

```bash
# 1. Update version in Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml

# 2. Commit
git commit -am "chore: bump version to 0.1.1"

# 3. Tag and push
git tag v0.1.1
git push origin trunk v0.1.1
```

### Minor Release (0.1.0 → 0.2.0)

```bash
# 1. Update version
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# 2. Update CHANGELOG.md
echo "## [0.2.0] - $(date +%Y-%m-%d)" >> CHANGELOG.md

# 3. Commit and tag
git commit -am "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin trunk v0.2.0
```

### Major Release (0.1.0 → 1.0.0)

```bash
# 1. Update version
sed -i 's/version = "0.1.0"/version = "1.0.0"/' Cargo.toml

# 2. Update CHANGELOG.md with breaking changes
echo "## [1.0.0] - $(date +%Y-%m-%d)" >> CHANGELOG.md
echo "### Breaking Changes" >> CHANGELOG.md

# 3. Commit and tag
git commit -am "chore: bump version to 1.0.0"
git tag v1.0.0
git push origin trunk v1.0.0
```

## 🔍 Monitoring Releases

### Check Workflow Status

Go to: https://github.com/MKSinghDev/pq-jwt-rust/actions

### View Published Versions

Go to: https://crates.io/crates/pq-jwt

### Check Download Stats

```bash
# Install cargo-info
cargo install cargo-info

# Check stats
cargo info pq-jwt
```

## 🐛 Troubleshooting

### ❌ "Version already exists"

**Problem:** You're trying to publish a version that already exists on crates.io.

**Solution:**
```bash
# Bump the version
sed -i 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml

# Delete old tag
git tag -d v0.1.0
git push --delete origin v0.1.0

# Create new tag
git tag v0.1.1
git push origin v0.1.1
```

### ❌ "Authentication failed"

**Problem:** `CARGO_REGISTRY_TOKEN` is invalid or expired.

**Solution:**
1. Generate a new token on crates.io
2. Update the GitHub secret
3. Re-run the workflow

### ❌ "Version mismatch"

**Problem:** Tag version doesn't match Cargo.toml version.

**Solution:**
```bash
# If tag is correct, update Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml
git commit -am "fix: update version to match tag"
git push

# If Cargo.toml is correct, update tag
git tag -d v0.2.0
git push --delete origin v0.2.0
git tag v0.2.0
git push origin v0.2.0
```

### ❌ "Tests failed"

**Problem:** Tests are failing in the workflow.

**Solution:**
```bash
# Run tests locally first
cargo test --all-features

# Fix the issues, then push
git commit -am "fix: resolve test failures"
git push
```

### ❌ "Clippy warnings"

**Problem:** Code has clippy warnings.

**Solution:**
```bash
# Check locally
cargo clippy -- -D warnings

# Fix warnings
cargo fix --allow-dirty

# Commit fixes
git commit -am "fix: resolve clippy warnings"
git push
```

## 📊 Best Practices

### Version Numbers (SemVer)

- **Patch** (0.1.0 → 0.1.1): Bug fixes, no breaking changes
- **Minor** (0.1.0 → 0.2.0): New features, no breaking changes
- **Major** (0.1.0 → 1.0.0): Breaking changes

### Pre-Release Checklist

- [ ] All tests pass locally: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated (if exists)
- [ ] README.md updated (if needed)
- [ ] Examples still work
- [ ] Security audit passes: `cargo audit`

### Release Frequency

- **Hot fixes:** As needed
- **Patch releases:** Weekly or bi-weekly
- **Minor releases:** Monthly
- **Major releases:** When necessary (breaking changes)

## 🎓 Advanced: Manual Publishing

If you need to publish manually:

```bash
# 1. Login to crates.io
cargo login

# 2. Dry run (verify everything)
cargo publish --dry-run

# 3. Publish for real
cargo publish

# 4. Create GitHub release manually
gh release create v0.2.0 --generate-notes
```

## 🔒 Security Notes

- ✅ **DO** use GitHub Secrets for tokens
- ✅ **DO** use scoped tokens (limit to specific crates)
- ✅ **DO** rotate tokens periodically
- ❌ **DON'T** commit tokens to the repository
- ❌ **DON'T** share tokens in public channels
- ❌ **DON'T** use personal tokens for CI

## 📚 Resources

- [Cargo Documentation - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Policies](https://crates.io/policies)
- [Semantic Versioning](https://semver.org/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)

## 🆘 Need Help?

If you encounter issues not covered here:

1. Check [GitHub Actions logs](https://github.com/MKSinghDev/pq-jwt-rust/actions)
2. Review [crates.io documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
3. Open an [issue](https://github.com/MKSinghDev/pq-jwt-rust/issues)

---

**Happy Publishing! 🎉**
