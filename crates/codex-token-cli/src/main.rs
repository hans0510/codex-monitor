use std::fmt;
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use codex_token_core::{
    aggregate_usage_now, discover_codex_home, UsageError, UsageReport, APP_NAME,
};

#[derive(Debug, Parser)]
#[command(name = APP_NAME)]
#[command(about = "Local Codex token usage summary", long_about = None)]
struct Cli {
    #[arg(long = "codex-home", value_name = "PATH", global = true)]
    codex_home: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Summary,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Summary) {
        Commands::Summary => run_summary(cli.codex_home),
    }
}

fn run_summary(codex_home_override: Option<PathBuf>) -> Result<(), CliError> {
    let codex_home =
        discover_codex_home(codex_home_override.as_deref()).ok_or(CliError::MissingCodexHome)?;
    let report = aggregate_usage_now(&codex_home)?;

    print_basic_summary(&codex_home, &report);
    Ok(())
}

fn print_basic_summary(codex_home: &std::path::Path, report: &UsageReport) {
    println!("Codex Token Summary");
    println!("Codex home: {}", codex_home.display());
    println!("Session files: {}", report.session_files.len());
    println!("Sessions: {}", report.sessions.len());
    println!("All time total: {}", report.summary.all_time.total_tokens);
}

#[derive(Debug)]
enum CliError {
    MissingCodexHome,
    Usage(UsageError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCodexHome => write!(f, "Could not resolve Codex home"),
            Self::Usage(error) => write!(f, "{error}"),
        }
    }
}

impl From<UsageError> for CliError {
    fn from(error: UsageError) -> Self {
        Self::Usage(error)
    }
}
