# Publishing Workflow Examples

## Scenario 1: Alpha Perfect on First Try ✨

```
Day 1: Develop locally
├─ cargo install --path .
├─ Test, fix bugs, repeat
└─ Ready for testing!

Day 2: Publish alpha
├─ version = "0.1.0-alpha.1"
├─ cargo publish
├─ cargo install muslim --version 0.1.0-alpha.1
└─ Test for a week...

Day 7: Everything perfect! 🎉
├─ ./scripts/promote-to-stable.sh
├─ version automatically changes to "0.1.0"
├─ cargo publish (SAME CODE!)
└─ Done! Stable released.

Result:
✓ 0.1.0-alpha.1 exists (tested version)
✓ 0.1.0 exists (same code, marked stable)
✓ Users run: cargo install muslim → gets 0.1.0
```

**Key Point:** No code changes between alpha and stable!

---

## Scenario 2: Alpha Needs Fixes 🔧

```
Day 1: Publish alpha
├─ version = "0.1.0-alpha.1"
└─ cargo publish

Day 2: Found bugs!
├─ Fix code
├─ version = "0.1.0-alpha.2"
├─ cargo publish
└─ Test again...

Day 3: More bugs!
├─ Fix code
├─ version = "0.1.0-alpha.3"
└─ cargo publish

Day 5: Perfect!
├─ ./scripts/promote-to-stable.sh
├─ version = "0.1.0"
└─ cargo publish

Result:
✓ 0.1.0-alpha.1 exists (had bugs)
✓ 0.1.0-alpha.2 exists (had bugs)
✓ 0.1.0-alpha.3 exists (worked!)
✓ 0.1.0 exists (same as alpha.3, marked stable)
```

---

## Scenario 3: Using Release Candidates 🎯

```
Development
├─ cargo install --path .
└─ Local testing

Alpha Testing
├─ version = "0.1.0-alpha.1"
├─ cargo publish
└─ Fix bugs → alpha.2, alpha.3...

Beta Testing (Feature Complete)
├─ version = "0.1.0-beta.1"
├─ cargo publish
└─ Final bug fixes → beta.2

Release Candidate (This might be final!)
├─ version = "0.1.0-rc.1"
├─ cargo publish
└─ Week of testing...

Stable (RC was perfect!)
├─ version = "0.1.0"
└─ cargo publish (SAME CODE as rc.1)

Result:
✓ Multiple alphas and betas during development
✓ 0.1.0-rc.1 (release candidate = might be final)
✓ 0.1.0 (same code as rc.1, now stable)
```

---

## What Happens on crates.io

### After Publishing Alpha

```
crates.io shows:
├─ muslim 0.1.0-alpha.1 (pre-release)

Users:
├─ cargo install muslim → "No stable version found"
└─ cargo install muslim --version 0.1.0-alpha.1 → works!
```

### After Promoting to Stable

```
crates.io shows:
├─ muslim 0.1.0 (stable) ← Default
└─ muslim 0.1.0-alpha.1 (pre-release)

Users:
├─ cargo install muslim → gets 0.1.0
├─ cargo install muslim --version 0.1.0-alpha.1 → still works
└─ Both versions available, same code!
```

### After Yanking Alpha (Optional)

```
crates.io shows:
├─ muslim 0.1.0 (stable)
└─ muslim 0.1.0-alpha.1 (yanked)

Users:
├─ cargo install muslim → gets 0.1.0
└─ cargo install muslim --version 0.1.0-alpha.1 → error (yanked)

Note: Existing installs of alpha.1 still work
```

---

## File Changes: Alpha → Stable

**Only one file changes:**

```diff
# Cargo.toml
[package]
name = "muslim"
-version = "0.1.0-alpha.1"
+version = "0.1.0"
edition = "2021"
```

**Everything else stays the same:**
- src/ code (no changes)
- README.md (no changes)
- Tests (no changes)
- Dependencies (no changes)

---

## Commands Summary

```bash
# Test alpha works perfectly
cargo install muslim --version 0.1.0-alpha.1
muslim  # Test it

# Promote to stable (same code!)
./scripts/promote-to-stable.sh

# Now stable is available
cargo install muslim  # Gets 0.1.0 automatically
```

---

## Common Questions

### Q: Can I have both alpha and stable with same code?
**A:** Yes! This is normal and encouraged.

### Q: Will users accidentally install the alpha?
**A:** No. `cargo install muslim` only installs stable versions by default.

### Q: Should I delete the alpha after stable release?
**A:** No need. You can optionally "yank" it, but it's fine to leave it.

### Q: Is this how real projects work?
**A:** Yes! Examples:
- Rust itself: 1.75.0-beta.5 → 1.75.0 (same code)
- Tokio: 1.35.0-rc.1 → 1.35.0 (same code)
- Many others do this

### Q: What if I publish stable then find a bug?
**A:** Publish 0.1.1 with the fix. Never change published versions.

---

## Timeline Visualization

```
Local Development (days/weeks)
│
├─ cargo install --path .
├─ Fix bugs, iterate fast
│
▼
Alpha Testing (0.1.0-alpha.1)
│
├─ cargo publish
├─ Test for real
├─ Found bugs? → alpha.2, alpha.3...
│
▼
Perfect? Promote to Stable (0.1.0)
│
├─ ./scripts/promote-to-stable.sh
├─ Same code, different version
│
▼
Released! 🎉
│
├─ Users: cargo install muslim
└─ They get 0.1.0 (your tested alpha code)
```

---

## Best Practice

**Recommended progression:**

```
Development → Alpha → RC → Stable
   (local)    (test)  (final?) (done!)

0.1.0-alpha.1 → 0.1.0-rc.1 → 0.1.0
```

**Why RC?**
- RC (Release Candidate) means "this might be the final version"
- If RC is perfect, promote to stable with **zero code changes**
- If RC has issues, fix and release RC.2

**Your workflow:**
1. Test locally: `cargo install --path .`
2. Publish RC: `version = "0.1.0-rc.1"` + `cargo publish`
3. Test thoroughly for a week
4. Perfect? Run: `./scripts/promote-to-stable.sh`
5. Done! RC.1 and 0.1.0 have identical code

---

## Summary

✅ **You CAN publish identical code as both alpha and stable**
✅ **Just change the version number**
✅ **This is standard practice**
✅ **Use `./scripts/promote-to-stable.sh` to automate it**
✅ **Both versions will exist on crates.io (this is fine!)**
