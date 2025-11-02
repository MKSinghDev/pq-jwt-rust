# 🚀 CI/CD Quick Reference

## 🔄 What Runs When

### On Pull Request (any branch → trunk)

| System | What Runs | Duration |
|--------|-----------|----------|
| **CircleCI** | Format, Clippy, Tests, Build, Security, Docs | ~3-4 min |
| **GitHub Actions** | Multi-platform tests, Coverage | ~5-8 min |
| **CodeRabbit** | AI code review | ~2-5 min |

**Required to pass:** All checks must be green ✅

---

### On Merge to Trunk

| System | What Runs | Duration |
|--------|-----------|----------|
| **CircleCI** | Full suite + Coverage + Benchmarks | ~5-7 min |

**Note:** Only releases happen from trunk, not regular merges.

---

### On Tag Push (`git push origin v0.1.0`)

| System | What Runs | Duration |
|--------|-----------|----------|
| **GitHub Actions** | Tests → Build → **Publish to crates.io** → Release | ~5-7 min |

**Result:** Package published automatically! 🎉

---

### Nightly (Automated)

| System | What Runs | Schedule |
|--------|-----------|----------|
| **CircleCI** | Tests, Security Audit, Benchmarks | 00:00 UTC |

---

## 📝 Quick Commands

### Release New Version

```bash
# Option 1: Automated (recommended)
./RELEASE.sh 0.2.0

# Option 2: Manual
# 1. Update Cargo.toml version
# 2. Commit changes
git commit -am "chore: bump version to 0.2.0"
git push origin trunk

# 3. Create and push tag
git tag v0.2.0
git push origin v0.2.0

# Wait for automation ✨
```

### Check CI Status

```bash
# CircleCI
open https://app.circleci.com/pipelines/github/MKSinghDev/pq-jwt-rust

# GitHub Actions
open https://github.com/MKSinghDev/pq-jwt-rust/actions

# CodeRabbit
# Check PR comments
```

### Local Pre-Push Checks

```bash
# Run the same checks CI will run
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all-features
cargo build --release
cargo audit
```

---

## 🎯 Key Rules

1. **trunk is protected** - All changes via PR
2. **All CI must pass** - No exceptions
3. **Tags trigger releases** - Only from trunk
4. **CodeRabbit reviews all PRs** - Address suggestions
5. **Nightly audits run automatically** - Monitor for security issues

---

## 🔧 Configuration Files

```
.
├── .circleci/
│   └── config.yml          # CircleCI workflows
├── .github/
│   └── workflows/
│       ├── ci.yml          # GitHub Actions - Tests
│       ├── publish.yml     # GitHub Actions - Release
│       └── release-check.yml  # Pre-release validation
├── .coderabbit.yaml        # CodeRabbit settings
├── PUBLISHING.md           # Detailed release guide
├── CI_SETUP.md            # Complete CI setup guide
└── RELEASE.sh             # Automated release script
```

---

## 🏷️ Status Badges

Add to README.md:

```markdown
[![CircleCI](https://circleci.com/gh/MKSinghDev/pq-jwt-rust.svg?style=shield)](https://circleci.com/gh/MKSinghDev/pq-jwt-rust)
![CI](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)
[![crates.io](https://img.shields.io/crates/v/pq-jwt.svg)](https://crates.io/crates/pq-jwt)
[![docs.rs](https://docs.rs/pq-jwt/badge.svg)](https://docs.rs/pq-jwt)
```

---

## 🚨 Troubleshooting

| Problem | Solution |
|---------|----------|
| CI not running | Check if CircleCI project is set up |
| Publish failing | Verify `CARGO_REGISTRY_TOKEN` secret |
| CodeRabbit silent | Check app installation & permissions |
| Tests timing out | Optimize tests or increase timeout |
| Version conflict | Ensure tag matches Cargo.toml |

---

## 📚 Full Documentation

- **[PUBLISHING.md](PUBLISHING.md)** - Complete publishing guide
- **[CI_SETUP.md](CI_SETUP.md)** - Detailed CI setup instructions
- **[README.md](README.md)** - Project documentation

---

**Need help?** Open an issue or check the full documentation above! 🙋‍♂️
