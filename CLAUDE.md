# 🤖 Claude Development Notes

> **For AI Assistants:** This document contains important context about the project structure, decisions, and conventions. Read this first before making changes.

## 📋 Project Overview

**Project:** pq-jwt (Post-Quantum JWT Library)
**Language:** Rust
**Purpose:** Quantum-resistant JWT implementation using ML-DSA (FIPS 204) signatures
**Repository:** https://github.com/MKSinghDev/pq-jwt-rust
**Author:** MKSingh ([@MKSingh_Dev](https://x.com/MKSingh_Dev))

## 🌳 Repository Structure

```
pq-jwt-rust/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── algorithm.rs        # MlDsaAlgo enum (44, 65, 87 variants)
│   ├── header.rs           # JWT header structure
│   ├── keygen.rs           # Keypair generation
│   ├── sign.rs             # JWT signing logic
│   └── verify.rs           # JWT verification logic
├── .circleci/
│   └── config.yml          # CircleCI workflows (primary CI)
├── .github/
│   ├── workflows/
│   │   ├── ci.yml          # GitHub Actions - PR testing
│   │   ├── publish.yml     # GitHub Actions - Auto-publish
│   │   └── release-check.yml # Pre-release validation
│   └── ISSUE_TEMPLATE/
│       └── release.md      # Release checklist template
├── .coderabbit.yaml        # CodeRabbit AI review config
├── Cargo.toml              # Project metadata
├── README.md               # User documentation
├── PUBLISHING.md           # Publishing guide
├── CI_SETUP.md            # CI configuration guide
├── WORKFLOWS_SUMMARY.md   # Quick reference
├── SETUP_COMPLETE.md      # Initial setup checklist
├── RELEASE.sh             # Automated release script
└── CLAUDE.md              # This file (AI assistant notes)
```

## 🎯 Key Conventions

### Branch Strategy
- **Default branch:** `trunk` (NOT main or master)
- **Feature branches:** Create from trunk, merge back to trunk
- **Protection:** trunk is protected, requires PR + CI pass
- **Releases:** ONLY from trunk branch via git tags

### Version Management
- **Format:** Semantic versioning (x.y.z)
- **Location:** `Cargo.toml` version field
- **Tagging:** `v` prefix (e.g., `v0.1.0`)
- **Sync:** Tag version MUST match Cargo.toml version

### Code Style
- **Formatting:** `cargo fmt` (enforced in CI)
- **Linting:** `cargo clippy -- -D warnings` (enforced in CI)
- **No emojis:** Unless user explicitly requests
- **Documentation:** Required for all public APIs
- **Tests:** Required for new public functions

## 🔧 CI/CD Architecture

### Three-Tier CI System

#### 1. CircleCI (Primary - Public Repo Optimized)
- **Purpose:** Fast, comprehensive testing
- **Triggers:** All PRs and trunk commits
- **Features:**
  - Fast PR checks (~3-4 min)
  - Comprehensive trunk checks with coverage
  - Nightly security audits
  - Benchmark tracking
- **Config:** `.circleci/config.yml`

#### 2. GitHub Actions (Secondary - Release Automation)
- **Purpose:** Multi-platform testing + automated publishing
- **Triggers:**
  - `ci.yml`: PRs to trunk
  - `publish.yml`: Tag push (v*.*.*)
  - `release-check.yml`: PRs with "release" label/title
- **Features:**
  - Linux/macOS/Windows testing
  - Auto-publish to crates.io
  - GitHub Release creation
- **Secret Required:** `CARGO_REGISTRY_TOKEN` (GitHub Secrets)

#### 3. CodeRabbit (AI Review)
- **Purpose:** Automated code reviews
- **Triggers:** All PRs
- **Features:**
  - Rust-specific checks
  - Security scanning
  - Performance suggestions
  - Custom rules for this project
- **Config:** `.coderabbit.yaml`

### CI Workflow Matrix

| Event | CircleCI | GitHub Actions | CodeRabbit | Result |
|-------|----------|---------------|------------|--------|
| PR created | ✅ Fast checks | ✅ Multi-platform | ✅ Review | Must pass to merge |
| Merge to trunk | ✅ Full suite | ❌ Not triggered | ❌ N/A | Artifacts stored |
| Tag push (`v*.*.*`) | ❌ Not triggered | ✅ Publish | ❌ N/A | Auto-publish to crates.io |
| Nightly (00:00 UTC) | ✅ Security audit | ❌ Not triggered | ❌ N/A | Monitors dependencies |

## 📦 Module Design

### Public API (lib.rs)
```rust
pub use algorithm::MlDsaAlgo;
pub use keygen::generate_keypair;
pub use sign::sign;
pub use verify::verify;
```

**Principle:** Minimal surface area, maximum clarity

### Module Responsibilities

| Module | Responsibility | Key Types |
|--------|---------------|-----------|
| `algorithm.rs` | Algorithm variants | `MlDsaAlgo` enum |
| `header.rs` | JWT header structure | `JwtHeader` struct |
| `keygen.rs` | Key generation | `generate_keypair()` |
| `sign.rs` | JWT signing | `sign()` |
| `verify.rs` | JWT verification | `verify()` |

### Error Handling
- **Type:** `Result<T, String>` (simple error messages)
- **Convention:** Descriptive error strings, e.g., "Invalid hex private key: {error}"
- **No panic!** Library code should never panic (enforced by CodeRabbit)

## 🔒 Security Considerations

### Key Management
- Keys stored as **hex-encoded strings** for easy serialization
- Private keys: ~2-5 KB (hex)
- Public keys: ~1.3-2.6 KB (hex)
- **Never log or display private keys** in examples

### Signature Sizes
- ML-DSA-44: ~2.4 KB (NIST Category 2)
- ML-DSA-65: ~3.3 KB (NIST Category 3) ⭐ **Recommended**
- ML-DSA-87: ~4.6 KB (NIST Category 5)

### Common Pitfalls to Avoid
1. ❌ Using `.unwrap()` in library code
2. ❌ Using `panic!` in library code
3. ❌ Hardcoding keys
4. ❌ Logging sensitive data
5. ❌ Exposing internal error types

## 🧪 Testing Strategy

### Test Coverage Requirements
- All public functions must have tests
- Edge cases: invalid inputs, wrong keys, malformed JWTs
- Integration tests in `lib.rs`
- Doc tests for examples

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_workflow() { /* ... */ }

    #[test]
    fn test_error_cases() { /* ... */ }
}
```

## 📝 Documentation Standards

### Public API Documentation
```rust
/// Brief one-line description
///
/// # Arguments
/// * `param` - Description
///
/// # Returns
/// * `Ok(value)` - Success case
/// * `Err(String)` - Error case
///
/// # Example
/// ```
/// use pq_jwt::{generate_keypair, MlDsaAlgo};
///
/// let (priv_key, pub_key) = generate_keypair(MlDsaAlgo::Dsa65)?;
/// # Ok::<(), String>(())
/// ```
pub fn function_name() { /* ... */ }
```

### README.md Sections
1. Badges
2. Quick intro
3. Features
4. Installation
5. Quick start
6. Usage examples
7. Security levels table
8. Performance benchmarks
9. API reference
10. Security considerations
11. Why post-quantum?
12. Integration examples
13. Contributing
14. License

## 🚀 Release Process

### Automated (Recommended)
```bash
./RELEASE.sh 0.2.0
```

### Manual Steps
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Commit: `git commit -am "chore: bump version to 0.2.0"`
4. Push to trunk: `git push origin trunk`
5. Create tag: `git tag v0.2.0`
6. Push tag: `git push origin v0.2.0`
7. Wait for GitHub Actions to publish

### Version Bump Rules (SemVer)
- **Patch (0.1.0 → 0.1.1):** Bug fixes, no API changes
- **Minor (0.1.0 → 0.2.0):** New features, backward compatible
- **Major (0.1.0 → 1.0.0):** Breaking changes

## 🔄 Common Operations

### Adding a New Feature
1. Create feature branch: `git checkout -b feature/name`
2. Implement feature
3. Add tests
4. Add documentation
5. Run local checks:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test --all-features
   cargo doc --no-deps
   ```
6. Create PR to trunk
7. Address CodeRabbit feedback
8. Wait for CI to pass
9. Merge

### Fixing a Bug
1. Write failing test first
2. Fix the bug
3. Verify test passes
4. Update documentation if needed
5. Follow PR process above

### Updating Dependencies
```bash
cargo update
cargo test --all-features
# Check for breaking changes
# Update Cargo.toml if needed
```

## 🐛 Troubleshooting Guide

### Build Failures

**Problem:** `cargo build` fails
**Solutions:**
- Check Rust version: `rustc --version` (min 1.70.0)
- Update dependencies: `cargo update`
- Clear cache: `cargo clean && cargo build`

**Problem:** Tests fail
**Solutions:**
- Run specific test: `cargo test test_name -- --nocapture`
- Check for race conditions
- Verify test data is valid

### CI Failures

**CircleCI not running:**
- Verify project is connected in CircleCI dashboard
- Check `.circleci/config.yml` syntax
- Ensure branch filters are correct

**GitHub Actions publish fails:**
- Verify `CARGO_REGISTRY_TOKEN` secret exists
- Check if version already published
- Ensure tag matches Cargo.toml version

**CodeRabbit silent:**
- Check app is installed
- Verify repository permissions
- Check `.coderabbit.yaml` syntax

## 💡 Design Decisions

### Why Hex-Encoded Keys?
- **Pro:** Easy serialization, human-readable, works everywhere
- **Con:** 2x size vs binary
- **Decision:** Prioritize ease of use over size

### Why String Errors?
- **Pro:** Simple, flexible, easy to understand
- **Con:** Not structured, harder to match on
- **Decision:** Simplicity for v1, may add error types later

### Why ML-DSA-65 as Default?
- NIST Category 3 (equivalent to AES-192)
- Best security/performance balance
- Recommended by NIST for most applications

### Why CircleCI + GitHub Actions?
- **CircleCI:** Better for public repos, faster, free tier generous
- **GitHub Actions:** Native to GitHub, good for releases
- **Together:** Best of both worlds

### Why CodeRabbit?
- AI-powered reviews catch subtle issues
- Learns project patterns
- Rust-specific knowledge
- Reduces reviewer burden

## 📚 Important Links

- **Crates.io:** https://crates.io/crates/pq-jwt
- **Docs.rs:** https://docs.rs/pq-jwt
- **Repository:** https://github.com/MKSinghDev/pq-jwt-rust
- **CircleCI:** https://app.circleci.com/pipelines/github/MKSinghDev/pq-jwt-rust
- **GitHub Actions:** https://github.com/MKSinghDev/pq-jwt-rust/actions

## 🎓 ML-DSA Background

### Algorithm Variants
- **ML-DSA-44:** Based on Dilithium2, NIST Category 2
- **ML-DSA-65:** Based on Dilithium3, NIST Category 3
- **ML-DSA-87:** Based on Dilithium5, NIST Category 5

### Dependencies
- `ml-dsa` crate version: `0.1.0-rc.1`
- Based on NIST FIPS 204 standard
- Implements Module-Lattice Digital Signature Algorithm

### Performance Characteristics
- Key generation: Sub-millisecond
- Signing: ~0.5-1 ms
- Verification: ~0.2-0.3 ms
- Signature size: Large (3-5 KB) but quantum-resistant

## 🔮 Future Considerations

### Potential Enhancements
- [ ] Add support for JWT claims validation
- [ ] Implement token expiration checking
- [ ] Add JWK (JSON Web Key) format support
- [ ] Create async variants of functions
- [ ] Add benchmarks suite
- [ ] Implement no_std support (if feasible)
- [ ] Add PEM key format support
- [ ] Create CLI tool for key generation

### Breaking Changes to Avoid
- Changing public API signatures
- Removing public functions
- Changing error messages that users might match on
- Modifying serialization format

### Migration Notes
When making breaking changes:
1. Bump major version
2. Document all breaking changes in CHANGELOG.md
3. Provide migration guide
4. Consider deprecation period for removals

## 🤝 Contributing Guidelines

When reviewing/generating code for this project:

### Do:
✅ Follow Rust idioms and conventions
✅ Add tests for all new functionality
✅ Document public APIs thoroughly
✅ Use descriptive variable names
✅ Keep functions focused and small
✅ Handle errors explicitly with Result
✅ Run clippy and address warnings
✅ Format code with rustfmt

### Don't:
❌ Use .unwrap() in library code
❌ Add panic! to library code
❌ Create breaking changes without version bump
❌ Add dependencies without justification
❌ Skip documentation
❌ Ignore CodeRabbit suggestions without reason
❌ Commit directly to trunk
❌ Add emojis unless explicitly requested

## 🔑 Secret Management

### GitHub Secrets
- `CARGO_REGISTRY_TOKEN` - For publishing to crates.io
  - Scope: Write access to pq-jwt crate
  - Expiry: Set reminder to rotate every 90 days
  - Never log or expose in CI

### Local Development
- No secrets needed for development
- Tests generate ephemeral keys
- CI uses only the crates.io token

## 📊 Metrics & Monitoring

### CI Success Rates
- Target: >95% pass rate
- Monitor via CircleCI/GitHub Actions dashboards
- Investigate flaky tests immediately

### Code Coverage
- Target: >80% line coverage
- Measured by tarpaulin
- Reported to Codecov

### Performance Benchmarks
- Run on trunk after merge
- Track trends over time
- Alert on >10% regression

## 🎯 Project Goals

### Primary Goals
1. **Security First:** Quantum-resistant, cryptographically sound
2. **Easy to Use:** Simple API, clear documentation
3. **Well Tested:** High coverage, edge cases handled
4. **Production Ready:** Stable, reliable, maintainable

### Non-Goals
- Supporting classical algorithms (use other libraries)
- Implementing full JWT claims validation (may add later)
- No_std support (maybe future)
- Async variants (maybe future)

## 📝 Changelog Conventions

Format for CHANGELOG.md (if created):

```markdown
## [Version] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```

## 🆘 When Things Go Wrong

### Emergency Rollback
```bash
# If bad version published
cargo yank --vers 0.2.0

# Fix issue, bump to 0.2.1
./RELEASE.sh 0.2.1
```

### CI Completely Broken
1. Fix locally first
2. Push fix to feature branch
3. Test in PR
4. Merge to trunk
5. Monitor CI

### Security Vulnerability Found
1. Create security advisory on GitHub
2. Fix in private branch
3. Coordinate release
4. Publish patched version
5. Announce via GitHub Security Advisory

## 🎉 Success Criteria

A change is successful when:
- ✅ All CI checks pass (CircleCI + GitHub Actions)
- ✅ CodeRabbit approves or concerns addressed
- ✅ Code coverage maintained or improved
- ✅ Documentation updated
- ✅ Tests added/updated
- ✅ No breaking changes (or major version bumped)
- ✅ Clippy warnings addressed
- ✅ Code formatted

---

## 📌 Quick Reference for AI Assistants

**Default branch:** `trunk`
**CI System:** CircleCI (primary), GitHub Actions (release)
**Code review:** CodeRabbit
**Testing:** `cargo test --all-features`
**Linting:** `cargo clippy -- -D warnings`
**Formatting:** `cargo fmt`
**Release:** `./RELEASE.sh X.Y.Z`
**Docs:** Multiple files (README, PUBLISHING, CI_SETUP, etc.)

**Before making changes:**
1. Read this file
2. Check existing conventions
3. Run tests locally
4. Follow style guide
5. Update docs

**Key files to never modify without good reason:**
- `.circleci/config.yml`
- `.github/workflows/*.yml`
- `.coderabbit.yaml`
- `Cargo.toml` (except version bumps)

---

**Last Updated:** 2025-01-02
**Document Version:** 1.0
**Maintained By:** Claude Code (for AI assistant reference)
