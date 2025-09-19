use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};
use dirs_next::config_dir;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const GOAL_DAYS: i32 = 11_111;

#[derive(Debug, Serialize, Deserialize)]
struct StreakState {
    start_date: NaiveDate,
    last_date: NaiveDate,
    current_streak_days: i32,
    longest_streak_days: i32,
}

impl StreakState {
    fn new(today: NaiveDate) -> Self {
        Self {
            start_date: today,
            last_date: today,
            current_streak_days: 1,
            longest_streak_days: 1,
        }
    }
}

#[derive(Parser)]
#[command(name="rust-streak", about="Seinfeld-method streak tracker for daily Rust coding")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Tick,
    Show,
    Reset,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let state_path = ensure_state_file_path()?;
    match cli.cmd.unwrap_or(Cmd::Show) {
        Cmd::Tick => cmd_tick(&state_path)?,
        Cmd::Show => cmd_show(&state_path)?,
        Cmd::Reset => cmd_reset(&state_path)?,
    }
    Ok(())
}

fn ensure_state_file_path() -> Result<PathBuf> {
    let mut p = config_dir().context("no config dir")?;
    p.push("rust-streak");
    fs::create_dir_all(&p).ok();
    p.push("state.json");
    Ok(p)
}

fn load_state(p: &PathBuf) -> Option<StreakState> {
    let bytes = fs::read(p).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn save_state(p: &PathBuf, s: &StreakState) -> Result<()> {
    let pretty = serde_json::to_string_pretty(s)?;
    fs::write(p, pretty)?;
    Ok(())
}

fn today() -> NaiveDate {
    Local::now().date_naive()
}

fn cmd_tick(p: &PathBuf) -> Result<()> {
    let t = today();
    let mut s = load_state(p).unwrap_or_else(|| StreakState::new(t));

    if t == s.last_date {
        println!("{}", "✅ Already ticked for today.".green());
        banner(&s);
        return Ok(());
    }

    let days_since = (t - s.last_date).num_days();
    if days_since == 1 {
        s.current_streak_days += 1;
    } else {
        println!("{}", "⚠️ Chain broke — restarting today.".yellow());
        s.current_streak_days = 1;
        s.start_date = t;
    }
    s.last_date = t;
    if s.current_streak_days > s.longest_streak_days {
        s.longest_streak_days = s.current_streak_days;
    }
    save_state(p, &s)?;
    println!("{}", "🧱 Ticked today!".green());
    banner(&s);
    Ok(())
}

fn cmd_show(p: &PathBuf) -> Result<()> {
    let s = load_state(p).unwrap_or_else(|| StreakState::new(today()));
    banner(&s);
    Ok(())
}

fn cmd_reset(p: &PathBuf) -> Result<()> {
    let s = StreakState::new(today());
    save_state(p, &s)?;
    println!("{}", "🔄 Reset streak.".yellow());
    banner(&s);
    Ok(())
}

fn banner(s: &StreakState) {
    let t = today();
    let chain = "█".repeat((s.current_streak_days.min(30)) as usize);
    println!("{}", format!("RUST STREAK — {} days", s.current_streak_days).bold());
    println!("Chain: {}", chain.green());
    println!("Longest: {} | Goal: {}", s.longest_streak_days, GOAL_DAYS);

    // Seed 1: daily plan file
    if let Some(mut plan) = config_dir() {
        plan.push("rust-streak");
        plan.push("today.txt");
        if let Ok(txt) = fs::read_to_string(plan) {
            if !txt.trim().is_empty() {
                println!("{}", format!("📋 Today’s plan: {}", txt.trim()).blue());
            }
        }
    }

    if t == s.last_date {
        println!("{}", "🌱 You’ve ticked today. Keep going!".green());
    } else {
        println!("{}", "⏳ Not ticked today yet.".yellow());
    }
}
