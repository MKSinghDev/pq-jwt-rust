---
name: Release
about: Checklist for creating a new release
title: 'Release v0.X.X'
labels: release
assignees: MKSinghDev

---

## 🚀 Release Checklist

### Pre-Release

- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Documentation builds: `cargo doc --no-deps --all-features`
- [ ] Security audit passes: `cargo audit`
- [ ] Examples work correctly
- [ ] Benchmarks run successfully (if applicable)

### Version & Documentation

- [ ] Version bumped in `Cargo.toml`
- [ ] `CHANGELOG.md` updated with changes
- [ ] `README.md` updated (if needed)
- [ ] Breaking changes documented (if any)
- [ ] Migration guide provided (if breaking changes)

### Release Notes

Briefly describe the changes:

#### Added
-

#### Changed
-

#### Fixed
-

#### Breaking Changes (if any)
-

### Post-Release

- [ ] Tag created: `git tag vX.X.X`
- [ ] Tag pushed: `git push origin vX.X.X`
- [ ] GitHub Actions publish workflow completed successfully
- [ ] Package visible on [crates.io/crates/pq-jwt](https://crates.io/crates/pq-jwt)
- [ ] GitHub Release created
- [ ] Announcement posted (optional)

---

**Automated publish will trigger when tag is pushed!**
