# Can You Promote Alpha to Stable Without Code Changes?

## Short Answer: YES! ✅

You can publish the **exact same code** as both alpha and stable. Just change the version number.

## Practical Example

```bash
# 1. Test with alpha
version = "0.1.0-alpha.1"
cargo publish
cargo install muslim --version 0.1.0-alpha.1

# 2. Test for a week... everything perfect!

# 3. Promote to stable (NO CODE CHANGES)
./scripts/promote-to-stable.sh

# Behind the scenes it does:
# - Changes: version = "0.1.0"
# - Runs: cargo publish
# - Same code, different version!

# 4. Users get the stable
cargo install muslim  # Gets 0.1.0 (your tested alpha code)
```

## What Exists on crates.io After

```
muslim 0.1.0-alpha.1  ← The version you tested
muslim 0.1.0          ← Same code, marked stable
```

Both versions coexist. Users automatically get the stable one.

## This Is Standard Practice

Real examples:
- **Rust**: `1.75.0-beta.5` → `1.75.0` (same code)
- **Tokio**: `1.35.0-rc.1` → `1.35.0` (same code)
- **Serde**: `1.0.0-rc.3` → `1.0.0` (same code)

## Why This Works

1. **Pre-releases are for testing** - That's their purpose
2. **Version is metadata** - It signals "I tested this, it's stable now"
3. **No code changes needed** - The testing validated the code
4. **Users trust stable** - They know it's been tested

## Your Complete Workflow

```bash
# Development Phase
cargo install --path .  # Fast iteration

# Alpha Phase (testing like a real user)
version = "0.1.0-alpha.1"
cargo publish
cargo install muslim --version 0.1.0-alpha.1
# Use it for real for a week...

# Stable Phase (alpha was perfect!)
./scripts/promote-to-stable.sh  # Changes version to 0.1.0
# Publishes same code as stable

# Done!
cargo install muslim  # Users get your tested code
```

## The Magic Script

Created for you: `./scripts/promote-to-stable.sh`

**What it does:**
1. Detects current pre-release version (0.1.0-alpha.1)
2. Strips suffix → 0.1.0
3. Updates Cargo.toml
4. Runs tests
5. Publishes to crates.io
6. Asks if you want to yank the alpha

**One command to go from alpha to stable!**

## Key Points

✅ Same code can be published as both alpha and stable
✅ Just the version number changes
✅ Both versions coexist on crates.io (this is fine)
✅ Users automatically get stable, not pre-releases
✅ This is how the entire Rust ecosystem works
✅ Use `./scripts/promote-to-stable.sh` to automate it

## When NOT To Do This

❌ If alpha had bugs you fixed in stable
- Then they're different code, use alpha.2 instead

✅ Alpha was perfect, zero changes needed
- Perfect use case for promotion!

## Summary

**Your question:** "Can we publish same code as alpha then stable?"

**Answer:** YES! This is exactly what pre-releases are for. Test with alpha, if perfect, promote to stable by just changing version number. No code changes needed. This is standard practice.

**Tool created:** `./scripts/promote-to-stable.sh` does this automatically!
