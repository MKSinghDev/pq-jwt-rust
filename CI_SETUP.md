# 🔧 CI/CD Setup Guide

This document explains how to set up all the CI/CD integrations for `pq-jwt-rust`.

## 📊 Overview

This repository uses **three** complementary CI/CD systems:

1. **CircleCI** - Primary testing and build system (public repos)
2. **GitHub Actions** - Release automation and secondary checks
3. **CodeRabbit** - AI-powered code reviews

---

## 🔵 CircleCI Setup

CircleCI is used as the primary CI system because it's optimized for public repositories.

### Features
- ✅ Fast PR checks (format, clippy, tests)
- ✅ Comprehensive trunk branch testing
- ✅ Code coverage reporting
- ✅ Security audits
- ✅ Nightly automated checks
- ✅ Benchmark tracking

### Setup Steps

1. **Connect Repository**
   - Go to https://circleci.com/
   - Sign in with GitHub
   - Click "Set Up Project"
   - Select `pq-jwt-rust`
   - CircleCI will automatically detect `.circleci/config.yml`

2. **Configure Environment** (Optional)
   - Project Settings → Environment Variables
   - Add any required secrets (if needed)

3. **Status Badge**
   Add to README.md:
   ```markdown
   [![CircleCI](https://circleci.com/gh/MKSinghDev/pq-jwt-rust.svg?style=shield)](https://circleci.com/gh/MKSinghDev/pq-jwt-rust)
   ```

### Workflows

#### PR Checks (All branches except trunk)
- Format checking
- Clippy lints
- Unit tests
- Build verification
- Security audit
- Documentation build

#### Trunk Checks (trunk branch only)
- All PR checks PLUS:
- Code coverage with Codecov
- Benchmark execution
- Performance tracking

#### Nightly Builds
- Runs daily at midnight UTC
- Security audits
- Dependency checks
- Performance benchmarks

### Configuration File
Location: `.circleci/config.yml`

Key features:
- Caching for faster builds
- Parallel job execution
- Artifact storage
- Multiple workflows for different scenarios

---

## 🟢 GitHub Actions Setup

GitHub Actions handles releases and provides secondary CI checks.

### Features
- ✅ Multi-platform testing (Linux, macOS, Windows)
- ✅ Automated publishing to crates.io
- ✅ Pre-release validation
- ✅ GitHub Release creation
- ✅ Security scanning

### Setup Steps

1. **Add crates.io Token** (Required for publishing)
   - Go to https://crates.io/settings/tokens
   - Create new token: "GitHub Actions - pq-jwt"
   - Copy the token
   - Go to GitHub repo → Settings → Secrets and variables → Actions
   - New secret: `CARGO_REGISTRY_TOKEN`
   - Paste the token

2. **Workflows** (Already configured)
   - `.github/workflows/ci.yml` - PR testing
   - `.github/workflows/publish.yml` - Automated publishing
   - `.github/workflows/release-check.yml` - Pre-release validation

### Workflows

#### CI Workflow
**Trigger:** Pull requests to trunk

**Jobs:**
- Test on Linux, macOS, Windows
- Test on stable and nightly Rust
- Format checking
- Clippy lints
- Security audit
- Documentation build
- Code coverage
- MSRV verification

#### Publish Workflow
**Trigger:** Tag push (`v*.*.*`)

**Jobs:**
- Quality checks (tests, clippy, format)
- Version verification
- Build release
- **Publish to crates.io**
- Create GitHub Release

#### Release Check Workflow
**Trigger:** PRs with "release" label/title

**Jobs:**
- Verify version bump
- Check CHANGELOG updated
- Dry-run publish
- Post checklist comment

---

## 🤖 CodeRabbit Setup

CodeRabbit provides AI-powered code reviews on every PR.

### Features
- ✅ Automated code reviews
- ✅ Security vulnerability detection
- ✅ Performance suggestions
- ✅ Best practice recommendations
- ✅ Rust-specific checks
- ✅ Integration with CircleCI

### Setup Steps

1. **Install CodeRabbit App**
   - Go to https://github.com/apps/coderabbitai
   - Click "Install"
   - Select `pq-jwt-rust` repository
   - Grant permissions

2. **Configuration** (Already set up)
   - File: `.coderabbit.yaml`
   - Customized for Rust projects
   - Integrated with CircleCI

### What CodeRabbit Checks

#### Code Quality
- Unsafe code usage
- `.unwrap()` and `.expect()` calls
- Panic usage in library code
- Error handling patterns
- Memory safety
- Thread safety

#### Performance
- Inefficient algorithms
- Unnecessary allocations
- Clone usage
- Iterator optimization

#### Security
- Potential vulnerabilities
- Unsafe patterns
- Input validation
- Cryptographic best practices

#### Style & Best Practices
- Idiomatic Rust
- API design
- Documentation quality
- Test coverage

### Custom Rules

Configured in `.coderabbit.yaml`:

1. **Require tests** for new public functions
2. **Require documentation** for public APIs
3. **Warn on `.unwrap()`** usage
4. **Warn on `panic!`** in library code
5. **Check CHANGELOG** for significant changes

### Review Levels

CodeRabbit provides three levels based on PR size:

- **Small PRs** (<100 lines): Quick review
- **Medium PRs** (100-500 lines): Detailed review
- **Large PRs** (>500 lines): Comprehensive review with warnings

---

## 🔄 CI Workflow Overview

### On Pull Request Creation

1. **CircleCI** starts immediately:
   - Format check (~10s)
   - Clippy lints (~30s)
   - Tests (~45s)
   - Build (~60s)
   - Security audit (~20s)
   - Docs build (~30s)

2. **GitHub Actions** runs in parallel:
   - Multi-platform tests (~2-5 min)
   - Coverage report (~3 min)
   - Security scan (~1 min)

3. **CodeRabbit** reviews code:
   - Initial review (~2-5 min)
   - Posts comments inline
   - Provides summary

**Total time:** ~5-8 minutes for full CI

### On Merge to Trunk

1. **CircleCI** runs comprehensive checks:
   - All PR checks
   - Code coverage
   - Benchmarks
   - Stores artifacts

2. **GitHub Actions** may run if configured

### On Tag Push (Release)

1. **GitHub Actions** handles publishing:
   - Runs all quality checks
   - Verifies version
   - Publishes to crates.io
   - Creates GitHub Release

**Total time:** ~5-7 minutes

---

## 🎯 Branch Strategy

### Main Branch: `trunk`

- All releases happen from `trunk`
- Must pass all CI checks
- Protected branch (requires PR reviews)
- Tags are created from `trunk`

### Feature Branches

- Create from `trunk`
- PR to `trunk` triggers CI
- CircleCI and GitHub Actions run
- CodeRabbit reviews code
- Must pass all checks before merge

### Release Process

```bash
# On trunk branch
git tag v0.2.0
git push origin v0.2.0

# GitHub Actions automatically:
# 1. Runs tests
# 2. Publishes to crates.io
# 3. Creates GitHub Release
```

---

## 📊 CI Status Badges

Add these to your README.md:

```markdown
[![CircleCI](https://circleci.com/gh/MKSinghDev/pq-jwt-rust.svg?style=shield)](https://circleci.com/gh/MKSinghDev/pq-jwt-rust)
[![GitHub Actions](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)](https://github.com/MKSinghDev/pq-jwt-rust/actions)
[![codecov](https://codecov.io/gh/MKSinghDev/pq-jwt-rust/branch/trunk/graph/badge.svg)](https://codecov.io/gh/MKSinghDev/pq-jwt-rust)
[![crates.io](https://img.shields.io/crates/v/pq-jwt.svg)](https://crates.io/crates/pq-jwt)
[![docs.rs](https://docs.rs/pq-jwt/badge.svg)](https://docs.rs/pq-jwt)
```

---

## 🔒 Security

### Secrets Management

**Required Secrets:**
- `CARGO_REGISTRY_TOKEN` - For publishing to crates.io (GitHub Secrets)

**Never commit:**
- API tokens
- Credentials
- Private keys

### Security Scanning

- **CircleCI**: Runs `cargo audit` nightly
- **GitHub Actions**: Uses RustSec advisory database
- **CodeRabbit**: Checks for security vulnerabilities

---

## 🐛 Troubleshooting

### CircleCI Not Running

**Problem:** CircleCI doesn't start on PR

**Solution:**
1. Check if project is set up in CircleCI
2. Verify `.circleci/config.yml` syntax
3. Check branch filters in config

### GitHub Actions Failing

**Problem:** Publish workflow fails

**Solution:**
1. Verify `CARGO_REGISTRY_TOKEN` is set
2. Check if version already exists on crates.io
3. Ensure tag matches Cargo.toml version

### CodeRabbit Not Commenting

**Problem:** No reviews from CodeRabbit

**Solution:**
1. Check if CodeRabbit app is installed
2. Verify `.coderabbit.yaml` exists
3. Check repository permissions

### CI Takes Too Long

**Problem:** CI runs for >10 minutes

**Solution:**
1. Check if caching is working (CircleCI)
2. Reduce test matrix (GitHub Actions)
3. Optimize test suite

---

## 📈 Monitoring

### CircleCI
- Dashboard: https://app.circleci.com/pipelines/github/MKSinghDev/pq-jwt-rust
- View build history
- Check test results
- Download artifacts

### GitHub Actions
- Actions tab: https://github.com/MKSinghDev/pq-jwt-rust/actions
- View workflow runs
- Check logs
- Re-run failed jobs

### CodeRabbit
- View on PR comments
- Check review summaries
- Track suggestions

---

## 🎓 Best Practices

1. **Keep CI Fast**
   - Use caching aggressively
   - Run heavy jobs only on trunk
   - Parallelize when possible

2. **Monitor Flaky Tests**
   - Fix intermittent failures
   - Don't ignore CI failures
   - Keep test suite reliable

3. **Review CodeRabbit Suggestions**
   - Don't auto-dismiss
   - Learn from patterns
   - Apply feedback

4. **Keep Dependencies Updated**
   - Monitor nightly security audits
   - Update Rust toolchain regularly
   - Review Dependabot PRs

5. **Document CI Changes**
   - Comment workflow modifications
   - Update this guide
   - Notify team of changes

---

## 📚 Resources

- [CircleCI Documentation](https://circleci.com/docs/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [CodeRabbit Documentation](https://docs.coderabbit.ai/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Publishing Guide](PUBLISHING.md)

---

## 🆘 Getting Help

- CircleCI: https://discuss.circleci.com/
- GitHub Actions: https://github.community/
- CodeRabbit: support@coderabbit.ai
- Project Issues: https://github.com/MKSinghDev/pq-jwt-rust/issues

---

**Setup complete! 🎉**

Your repository now has enterprise-grade CI/CD with:
- ✅ Automated testing on multiple platforms
- ✅ AI-powered code reviews
- ✅ Security scanning
- ✅ Automated publishing
- ✅ Performance tracking
