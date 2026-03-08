# Quick Start Guide

## Your Workflow: Nightly → Stable

### Phase 1: Local Testing (Start Here)

```bash
# Install locally (no publishing)
cargo install --path .

# Or use the helper script
./scripts/install-local.sh

# Test it
muslim

# Make changes, then reinstall
./scripts/install-local.sh
```

**Do this until you're happy with the changes.**

---

### Phase 2: Alpha Testing (Test on Real System)

When ready to test like a real user:

```bash
# 1. Update version in Cargo.toml
version = "0.1.0-alpha.1"

# 2. Publish to crates.io
cargo publish

# 3. Install from crates.io (like a real user would)
cargo install muslim --version 0.1.0-alpha.1

# 4. Test it
muslim
```

**Found bugs? Fix them and publish alpha.2, alpha.3, etc.**

---

### Phase 3: Stable Release

When everything works:

```bash
# 1. Update version in Cargo.toml
version = "0.1.0"

# 2. Publish
cargo publish

# 3. Now anyone can install it
cargo install muslim  # Gets stable version automatically
```

---

## Perfect Alpha? Promote to Stable!

If your alpha works perfectly, promote it to stable **without changing any code**:

```bash
# Currently at 0.1.0-alpha.1 and it's perfect?
./scripts/promote-to-stable.sh

# This will:
# 1. Change version from 0.1.0-alpha.1 → 0.1.0
# 2. Run tests
# 3. Publish to crates.io (same code!)
# 4. Optionally yank the alpha
```

**Result:** Users get the exact code you tested, just marked as stable.

---

## Commands Cheat Sheet

### Development
```bash
# Local install (fastest, use during development)
cargo install --path .

# Reinstall after changes
./scripts/install-local.sh

# Uninstall
cargo uninstall muslim
```

### Testing Pre-releases
```bash
# Install specific alpha
cargo install muslim --version 0.1.0-alpha.1

# Install specific beta
cargo install muslim --version 0.1.0-beta.1

# Force reinstall
cargo install muslim --version 0.1.0-alpha.2 --force
```

### Publishing
```bash
# First time only: get API token from https://crates.io/settings/tokens
cargo login

# Publish current version
cargo publish

# Publish with helper script
./scripts/bump-version.sh
```

### Version Management
```bash
# Check current version
grep '^version' Cargo.toml

# Update version manually
# Edit Cargo.toml: version = "0.1.0-alpha.2"

# Or use helper script
./scripts/bump-version.sh
```

---

## Version Naming

```
0.1.0-alpha.1   → Early testing, expect bugs
0.1.0-alpha.2   → Bug fixes
0.1.0-beta.1    → Feature-complete, final testing
0.1.0           → Stable release
0.1.1           → Bug fix update
0.2.0           → New features
```

---

## Example Timeline

```bash
# Day 1-5: Local development
./scripts/install-local.sh  # Test locally

# Week 1: Alpha testing
# Cargo.toml: version = "0.1.0-alpha.1"
cargo publish
cargo install muslim --version 0.1.0-alpha.1

# Week 2: Beta testing
# Cargo.toml: version = "0.1.0-beta.1"
cargo publish
cargo install muslim --version 0.1.0-beta.1

# Week 3: Stable release
# Cargo.toml: version = "0.1.0"
cargo publish
cargo install muslim  # ← Stable!
```

---

## Important Notes

**No "nightly" flag exists!**
- ❌ `cargo install muslim --nightly` (doesn't exist)
- ✅ `cargo install muslim --version 0.1.0-alpha.1` (use this)

**Cannot delete published versions**
- You can only "yank" them: `cargo yank --version 0.1.0-alpha.1`
- Yanked versions won't be installed by new users
- Test locally first to avoid publishing broken versions

**Pre-release vs Stable**
- `cargo install muslim` → installs stable only (0.1.0)
- `cargo install muslim --version 0.1.0-alpha.1` → installs specific pre-release
- Pre-releases are perfect for your "nightly" workflow

---

## Before Publishing Checklist

- [ ] `cargo test` passes
- [ ] `cargo build --release` works
- [ ] Tested locally with `cargo install --path .`
- [ ] Updated version in `Cargo.toml`
- [ ] Updated `README.md` if needed
- [ ] Run `cargo publish --dry-run` first

---

## Need Help?

- Full guide: See [PUBLISHING.md](PUBLISHING.md)
- Questions: Check [README.md](README.md)
