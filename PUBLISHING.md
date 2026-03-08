# Publishing Guide for Muslim CLI

This guide explains how to test and publish the `muslim` crate using pre-release versions.

## Quick Overview

**Key Concept:** crates.io doesn't have "nightly" channels. Instead, use semantic versioning with pre-release tags:
- `0.1.0-alpha.1`, `0.1.0-alpha.2`, ... → Early testing
- `0.1.0-beta.1`, `0.1.0-beta.2`, ... → Feature-complete testing
- `0.1.0` → Stable release

## Phase 1: Local Testing (Before Publishing)

Test locally without publishing to crates.io:

```bash
# Install from local directory
cargo install --path .

# Test the binary
muslim setup
muslim

# Make changes, then reinstall
cargo uninstall muslim
cargo install --path .

# Repeat until satisfied
```

**Advantages:**
- No publishing needed
- Instant updates
- Can iterate quickly
- No version number pollution

## Phase 2: Publish Pre-release (Alpha Testing)

Once ready for wider testing, publish alpha versions:

### Step 1: Prepare Cargo.toml

Update version to alpha:

```toml
[package]
version = "0.1.0-alpha.1"  # First alpha
```

**Update these fields:**
```toml
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/muslim"
homepage = "https://github.com/yourusername/muslim"
```

### Step 2: One-time Setup

```bash
# Get API token from https://crates.io/settings/tokens
cargo login
# Paste your token when prompted
```

### Step 3: Publish Alpha Version

```bash
# Verify package contents
cargo package --list

# Do a dry run
cargo publish --dry-run

# Actually publish
cargo publish
```

### Step 4: Install Your Alpha Version

```bash
# Uninstall local version first
cargo uninstall muslim

# Install alpha from crates.io
cargo install muslim --version 0.1.0-alpha.1
```

### Step 5: Iterate with More Alphas

When you fix bugs or add features:

```toml
# Cargo.toml
version = "0.1.0-alpha.2"  # Increment alpha number
```

```bash
cargo publish
cargo install muslim --version 0.1.0-alpha.2
```

**Version progression:**
```
0.1.0-alpha.1 → First test
0.1.0-alpha.2 → Bug fixes
0.1.0-alpha.3 → More fixes
...
```

## Phase 3: Beta Testing

When features are complete and mostly stable:

```toml
# Cargo.toml
version = "0.1.0-beta.1"
```

```bash
cargo publish
cargo install muslim --version 0.1.0-beta.1
```

Beta indicates feature-freeze, only bug fixes remain.

## Phase 4: Stable Release

When fully tested and ready:

```toml
# Cargo.toml
version = "0.1.0"  # Remove pre-release tag
```

```bash
cargo publish

# Users can now install without version specifier
cargo install muslim  # Gets latest stable (0.1.0)
```

## Version Selection Behavior

```bash
# Latest stable only (excludes pre-releases)
cargo install muslim

# Specific version
cargo install muslim --version 0.1.0-alpha.1
cargo install muslim --version 0.1.0-beta.1
cargo install muslim --version 0.1.0

# Latest including pre-releases
cargo install muslim --version "*"

# Install from git (bypass crates.io)
cargo install --git https://github.com/yourusername/muslim

# Install from local path
cargo install --path .
```

## Complete Workflow Example

```bash
# 1. Local development and testing
cargo install --path .
muslim  # test
# make changes...
cargo uninstall muslim && cargo install --path .

# 2. Ready for alpha testing
# Edit Cargo.toml: version = "0.1.0-alpha.1"
cargo publish
cargo install muslim --version 0.1.0-alpha.1

# 3. Found bugs, fixed them
# Edit Cargo.toml: version = "0.1.0-alpha.2"
cargo publish
cargo install muslim --version 0.1.0-alpha.2

# 4. Features complete, enter beta
# Edit Cargo.toml: version = "0.1.0-beta.1"
cargo publish
cargo install muslim --version 0.1.0-beta.1

# 5. All testing done, go stable
# Edit Cargo.toml: version = "0.1.0"
cargo publish
cargo install muslim  # Installs stable 0.1.0
```

## Semantic Versioning Guide

For future releases:

```
0.1.0        → First stable release
0.1.1        → Bug fixes (patch)
0.2.0        → New features, backward compatible (minor)
1.0.0        → First major release (API stable)
1.0.1        → Bug fixes
1.1.0        → New features
2.0.0        → Breaking changes (major)
```

Pre-release examples:
```
0.2.0-alpha.1   → Testing next minor version
0.2.0-beta.1    → Feature-complete next minor version
1.0.0-rc.1      → Release candidate for major version
```

## Important Notes

### Yanking Versions

If you publish a broken version:

```bash
# Yank the version (prevents new installs, existing still work)
cargo yank --version 0.1.0-alpha.1

# Undo yank if needed
cargo yank --version 0.1.0-alpha.1 --undo
```

**Note:** You cannot delete published versions, only yank them.

### Testing on Multiple Machines

```bash
# Machine 1: Your dev machine
cargo install --path .

# Machine 2: Test server (use published version)
cargo install muslim --version 0.1.0-alpha.1

# Machine 3: Fresh environment
cargo install --git https://github.com/yourusername/muslim
```

### Pre-release Best Practices

1. **Alpha** - Early testing, expect bugs, API may change
2. **Beta** - Feature-complete, only bug fixes, API shouldn't change
3. **RC (Release Candidate)** - Final testing before stable
4. **Stable** - Production-ready

### Avoiding Version Pollution

Don't publish too many pre-releases. Instead:
- Test locally with `cargo install --path .`
- Only publish alphas when you need external testing
- Use git commits for rapid iteration
- Reserve publishing for milestone versions

## Publishing Checklist

Before `cargo publish`:

- [ ] Update version in `Cargo.toml`
- [ ] Update `README.md` if needed
- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo clippy` - no warnings
- [ ] Run `cargo fmt` - code formatted
- [ ] Run `cargo build --release` - builds successfully
- [ ] Test locally with `cargo install --path .`
- [ ] Update CHANGELOG.md (if you have one)
- [ ] Commit and tag version in git
- [ ] Run `cargo publish --dry-run`
- [ ] Run `cargo publish`

## Git Tagging for Releases

```bash
# Tag stable releases
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# Optionally tag pre-releases
git tag -a v0.1.0-alpha.1 -m "Alpha release 0.1.0-alpha.1"
git push origin v0.1.0-alpha.1
```

## Troubleshooting

**Problem:** "crate name is already taken"
- **Solution:** Choose a different name or request transfer from owner

**Problem:** "cargo install fails with old version"
- **Solution:** Force reinstall: `cargo install muslim --force`

**Problem:** "Want to test without publishing"
- **Solution:** Use `cargo install --path .` or `--git`

**Problem:** "Published wrong version"
- **Solution:** Yank it and publish corrected version with incremented number

## Example Timeline

```
Day 1:  Local dev → 0.1.0-alpha.1 published
Day 2:  Bug fixes → 0.1.0-alpha.2 published
Day 5:  More features → 0.1.0-alpha.3 published
Week 2: Feature complete → 0.1.0-beta.1 published
Week 3: Final testing → 0.1.0-beta.2 published
Week 4: Stable release → 0.1.0 published 🎉
```

## Summary

**For your specific workflow:**

```bash
# Testing phase (do this first, no publishing)
cargo install --path .
# test, make changes, repeat...

# Alpha testing (publish when ready for wider testing)
# Set version = "0.1.0-alpha.1"
cargo publish
cargo install muslim --version 0.1.0-alpha.1

# Stable release (when fully tested)
# Set version = "0.1.0"
cargo publish
cargo install muslim  # Gets stable automatically
```

**Key takeaway:** Use local installs for rapid iteration, only publish to crates.io when you want others to test or when releasing stable versions.
