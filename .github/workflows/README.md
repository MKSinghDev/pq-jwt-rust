# GitHub Actions Workflows

This directory contains automated CI/CD workflows for the `pq-jwt` project.

## 📋 Workflows Overview

### 1. CI Workflow (`ci.yml`)

**Purpose:** Continuous Integration - Runs on every push and PR

**Runs:**
- 🧪 **Tests**: On Linux, macOS, Windows with stable and nightly Rust
- 🎨 **Format Check**: Ensures code follows Rust formatting standards
- 📎 **Clippy**: Lints code for common mistakes
- 📊 **Coverage**: Generates code coverage reports
- 🔒 **Security Audit**: Checks for known vulnerabilities
- 📚 **Docs**: Verifies documentation builds correctly
- 🔧 **MSRV**: Tests Minimum Supported Rust Version

**Badge:** `![CI](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)`

### 2. Publish Workflow (`publish.yml`)

**Purpose:** Automated publishing to crates.io

**Triggers:** When you push a version tag (e.g., `v0.1.0`)

**Steps:**
1. Runs all quality checks (format, clippy, tests)
2. Verifies tag version matches Cargo.toml
3. Builds release binary
4. Publishes to crates.io using `CARGO_REGISTRY_TOKEN`
5. Creates GitHub Release with auto-generated notes

**Required Secret:** `CARGO_REGISTRY_TOKEN` (from crates.io)

### 3. Release Check Workflow (`release-check.yml`)

**Purpose:** Pre-release validation for PRs

**Triggers:** PRs to main with "release" in title or label

**Checks:**
- ✅ Version was bumped
- ✅ All tests pass
- ✅ Dry-run publish succeeds
- ✅ Documentation builds
- ⚠️ Warns about TODO/FIXME comments

**Output:** Posts checklist comment on the PR

## 🚀 Quick Start

### First Time Setup

1. Get crates.io token: https://crates.io/settings/tokens
2. Add to GitHub secrets: Settings → Secrets → Actions → `CARGO_REGISTRY_TOKEN`

### Publishing a Release

```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git commit -am "chore: bump version to 0.2.0"
git push

# 3. Create and push tag
git tag v0.2.0
git push origin v0.2.0

# 4. Watch the magic happen at:
# https://github.com/MKSinghDev/pq-jwt-rust/actions
```

## 📊 Workflow Matrix

| Workflow | Trigger | Runs On | Purpose |
|----------|---------|---------|---------|
| CI | Push, PR | Linux, macOS, Windows | Quality checks |
| Publish | Tag push (`v*.*.*`) | Linux | Publish to crates.io |
| Release Check | PR with "release" | Linux | Pre-release validation |

## 🔧 Customization

### Modify Test Matrix

Edit `ci.yml`:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]  # Remove Windows
    rust: [stable]  # Remove nightly
```

### Change MSRV

Edit `ci.yml`:

```yaml
- name: Setup Rust (MSRV)
  uses: dtolnay/rust-toolchain@master
  with:
    toolchain: 1.70.0  # ← Change this
```

### Skip Coverage

Remove or comment out the `coverage` job in `ci.yml`.

### Change Tag Pattern

Edit `publish.yml`:

```yaml
on:
  push:
    tags:
      - 'v*.*.*'  # ← Modify pattern here
```

## 🐛 Troubleshooting

### Workflow Not Running?

Check:
1. `.github/workflows/` files are committed
2. Workflow files have correct YAML syntax
3. You have the correct permissions

### Publish Failing?

Common issues:
- `CARGO_REGISTRY_TOKEN` not set or expired
- Version already exists on crates.io
- Tests failing in CI
- Tag version doesn't match Cargo.toml

See [PUBLISHING.md](../../PUBLISHING.md) for detailed troubleshooting.

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Publishing Guide](../../PUBLISHING.md)
- [Cargo Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)

## 🎯 Status Badges

Add these to your README.md:

```markdown
![CI](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)
![Publish](https://github.com/MKSinghDev/pq-jwt-rust/workflows/Publish/badge.svg)
[![crates.io](https://img.shields.io/crates/v/pq-jwt.svg)](https://crates.io/crates/pq-jwt)
[![Documentation](https://docs.rs/pq-jwt/badge.svg)](https://docs.rs/pq-jwt)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
```
