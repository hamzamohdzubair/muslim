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
    /// Create default config file
    Setup,
    /// Open config file in editor
    Config { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
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
            show_next_prayer()?;
        }
    }

    Ok(())
}

fn show_next_prayer() -> Result<()> {
    let config = config::load_config().context(
        "Failed to load config. Run 'muslim setup' to create a default config file.",
    )?;

    let now = Local::now().time();
    let next = prayer::get_next_prayer(
        config.prayers.fajr,
        config.prayers.dhuhr,
        config.prayers.asr,
        config.prayers.maghrib,
        config.prayers.isha,
        now,
    );

    let time_str = next.time.format("%H:%M");
    let duration_str = prayer::format_duration(next.duration);

    if next.is_tomorrow {
        println!("Next prayer: {} at {} (tomorrow)", next.prayer, time_str);
    } else {
        println!("Next prayer: {} at {}", next.prayer, time_str);
    }
    println!("Time remaining: {}", duration_str);

    Ok(())
}

fn open_config_in_editor() -> Result<()> {
    let config_path = config::get_config_path()?;

    // Try to get editor from $EDITOR, fallback to nano, then vim
    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| {
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
