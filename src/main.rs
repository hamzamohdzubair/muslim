mod config;
mod prayer;

use anyhow::{Context, Result};
use chrono::Local;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "muslim")]
#[command(about = "Prayer time management CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show next prayer time with beautiful display
    Next,
    /// Create default config file
    Setup,
    /// Open config file in editor
    Config { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Next) => {
            show_next_prayer()?;
        }
        Some(Commands::Setup) => {
            config::create_default_config()?;
        }
        Some(Commands::Config { name }) => {
            if name == "main" {
                open_config_in_editor()?;
            } else {
                anyhow::bail!("Unknown config name: {}", name);
            }
        }
        None => {
            // Show help when no command is provided (standard Rust CLI behavior)
            Cli::parse_from(["muslim", "--help"]);
        }
    }

    Ok(())
}

fn show_next_prayer() -> Result<()> {
    let config = config::load_config()
        .context("Failed to load config. Run 'muslim setup' to create a default config file.")?;

    let now = Local::now().time();
    let next = prayer::get_next_prayer(
        config.prayers.fajr,
        config.prayers.dhuhr,
        config.prayers.asr,
        config.prayers.maghrib,
        config.prayers.isha,
        now,
    );

    let time_str = next.time.format("%H:%M").to_string();
    let duration_str = prayer::format_duration(next.duration);

    // Create left side (mosque art)
    let left_lines = get_mosque_art_lines();

    // Create right side (prayer times)
    let right_lines = create_prayer_times_display(&config, &next, &time_str, &duration_str);

    // Print side by side
    println!();
    print_side_by_side(&left_lines, &right_lines);
    println!();

    Ok(())
}

fn create_prayer_times_display(
    config: &config::Config,
    next: &prayer::NextPrayer,
    time_str: &str,
    duration_str: &str,
) -> Vec<String> {
    let mut lines = Vec::new();

    // Colors
    let cyan = "\x1b[36m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";
    let dim = "\x1b[2m";

    lines.push(String::new());
    lines.push(format!("{}{}  Prayer Times{}", bold, green, reset));
    lines.push(format!("{}  ─────────────{}", dim, reset));
    lines.push(String::new());

    // Next prayer highlight
    if next.is_tomorrow {
        lines.push(format!(
            "  {}▶ Next:{} {} at {} {}(tomorrow){}",
            yellow, reset, next.prayer, time_str, dim, reset
        ));
    } else {
        lines.push(format!(
            "  {}▶ Next:{} {} at {}",
            yellow, reset, next.prayer, time_str
        ));
    }
    lines.push(format!(
        "  {}  Time remaining: {}{}",
        dim, duration_str, reset
    ));
    lines.push(String::new());
    lines.push(format!("{}  Today's Schedule{}", bold, reset));
    lines.push(format!("{}  ───────────────{}", dim, reset));

    // Prayer times list
    let prayers = [
        ("Fajr", config.prayers.fajr, prayer::Prayer::Fajr),
        ("Dhuhr", config.prayers.dhuhr, prayer::Prayer::Dhuhr),
        ("Asr", config.prayers.asr, prayer::Prayer::Asr),
        ("Maghrib", config.prayers.maghrib, prayer::Prayer::Maghrib),
        ("Isha", config.prayers.isha, prayer::Prayer::Isha),
    ];

    for (name, time, prayer_type) in prayers {
        let time_display = format_time_display(time);
        let is_next = next.prayer == prayer_type && !next.is_tomorrow;

        if is_next {
            lines.push(format!(
                "  {}{} ● {:<8} {}{}{}",
                bold, cyan, name, time_display, reset, cyan
            ));
        } else {
            lines.push(format!("  {}  {:<8} {}{}", dim, name, time_display, reset));
        }
    }

    lines.push(String::new());
    lines
}

fn print_side_by_side(left: &[String], right: &[String]) {
    let max_lines = left.len().max(right.len());
    let left_width: usize = 18;

    for i in 0..max_lines {
        let left_line = left.get(i).map(|s| s.as_str()).unwrap_or("");
        let right_line = right.get(i).map(|s| s.as_str()).unwrap_or("");

        // Strip ANSI codes for width calculation
        let left_display_width = strip_ansi(left_line).len();
        let padding = left_width.saturating_sub(left_display_width);

        println!("{}{}{}", left_line, " ".repeat(padding), right_line);
    }
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn get_mosque_art_lines() -> Vec<String> {
    // Colors
    let yellow = "\x1b[33m";
    let green = "\x1b[32m";
    let cyan = "\x1b[36m";
    let bright_yellow = "\x1b[93m";
    let reset = "\x1b[0m";

    vec![
        format!("      {}☪{}", bright_yellow, reset),
        format!("     {}███{}", yellow, reset),
        format!("    {}█████{}", yellow, reset),
        format!("   {}███████{}", green, reset),
        format!("  {} ███{}█{}███{}", green, cyan, green, reset),
        format!("  {} ███{}█{}███{}", green, cyan, green, reset),
        format!(" {}█████████{}", green, reset),
        format!("  {} ███████{}", green, reset),
        format!("   {}█████{}", cyan, reset),
        format!("   {}█████{}", cyan, reset),
        format!("   {}█████{}", cyan, reset),
        format!(" {}█████████{}", green, reset),
        String::new(),
    ]
}

fn format_time_display(hhmm: u16) -> String {
    let time = prayer::parse_time(hhmm);
    format!("{:>5}", time.format("%H:%M"))
}

fn open_config_in_editor() -> Result<()> {
    let config_path = config::get_config_path()?;

    // Try to get editor from $EDITOR, fallback to nano, then vim
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        if which("nano") {
            "nano".to_string()
        } else if which("vim") {
            "vim".to_string()
        } else {
            "vi".to_string()
        }
    });

    Command::new(&editor)
        .arg(&config_path)
        .status()
        .with_context(|| format!("Failed to open editor: {}", editor))?;

    Ok(())
}

fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
