# Muslim - Prayer Time Management CLI

A simple Rust CLI tool for managing prayer times and checking time remaining until the next prayer.

## Features

- 📿 Track 5 daily prayers (Fajr, Dhuhr, Asr, Maghrib, Isha)
- ⏰ Show time remaining until next prayer
- ⚙️ Simple TOML configuration
- 🔧 Easy setup and config editing

## Installation

```bash
cargo build --release
sudo cp target/release/muslim /usr/local/bin/
```

## Usage

### Initial Setup

Create the default configuration file:

```bash
muslim setup
```

This creates `~/.config/muslim/main.toml` with dummy prayer times.

### Check Next Prayer

Simply run:

```bash
muslim
```

Example output:
```
Next prayer: Dhuhr at 12:45
Time remaining: 2 hours 15 minutes
```

### Edit Configuration

Open the config file in your editor:

```bash
muslim config main
```

This opens `~/.config/muslim/main.toml` in `$EDITOR` (or falls back to nano/vim).

## Configuration

Edit `~/.config/muslim/main.toml` to set your prayer times:

```toml
[prayers]
fajr = 530      # 5:30 AM
dhuhr = 1245    # 12:45 PM
asr = 1615      # 4:15 PM
maghrib = 1830  # 6:30 PM
isha = 2000     # 8:00 PM
```

**Time Format**: Use HHMM format as integers (e.g., 530 = 5:30 AM, 1830 = 6:30 PM)

## Examples

**Current time: 10:30 AM**
```
$ muslim
Next prayer: Dhuhr at 12:45
Time remaining: 2 hours 15 minutes
```

**Current time: 9:00 PM (after Isha)**
```
$ muslim
Next prayer: Fajr at 05:30 (tomorrow)
Time remaining: 8 hours 30 minutes
```

## MVP Features

This is the MVP (Minimum Viable Product) version with:
- Single prayer schedule
- Manual time configuration
- Simple time-to-next-prayer display

## Future Enhancements

Planned for future versions:
- Multiple mosque schedules
- Travel time calculations
- Context-based scheduling
- Desktop notifications
- Automatic prayer time calculation

## Development

### Local Testing
```bash
# Quick reinstall during development
./scripts/install-local.sh

# Or manually
cargo install --path .
```

### Publishing
See [PUBLISHING.md](PUBLISHING.md) for detailed guide on:
- Testing with pre-release versions (alpha/beta)
- Publishing to crates.io
- Version management workflow

Quick version bump:
```bash
./scripts/bump-version.sh
```

## License

MIT
