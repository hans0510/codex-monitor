use std::fmt;
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use codex_token_core::{
    aggregate_usage_now, discover_codex_home, TokenUsage, UsageError, UsageReport, APP_NAME,
};

#[derive(Debug, Parser)]
#[command(name = APP_NAME)]
#[command(about = "Local Codex token usage summary", long_about = None)]
struct Cli {
    #[arg(
        long = "codex-home",
        value_name = "PATH",
        global = true,
        help = "Override Codex home for local logs (--codex-home)"
    )]
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

    if report.session_files.is_empty() {
        return Err(CliError::NoSessionLogs {
            searched: vec![
                codex_home.join("sessions"),
                codex_home.join("archived_sessions"),
            ],
        });
    }

    print_summary(&codex_home, &report);
    Ok(())
}

fn print_summary(codex_home: &std::path::Path, report: &UsageReport) {
    println!("Codex Token Summary");
    println!("Codex home: {}", codex_home.display());
    println!("Session files: {}", report.session_files.len());
    println!("Sessions: {}", report.sessions.len());
    println!();
    println!(
        "{:<11} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Range", "Input", "Cached", "Output", "Reasoning", "Total"
    );
    print_usage_row("Today", report.summary.today);
    print_usage_row("This week", report.summary.this_week);
    print_usage_row("This month", report.summary.this_month);
    print_usage_row("All time", report.summary.all_time);

    if let Some(latest) = report
        .sessions
        .iter()
        .max_by_key(|session| session.last_event_at)
    {
        println!();
        println!(
            "Latest session: {} ({} total)",
            latest.session_id, latest.total.total_tokens
        );
    }

    if !report.diagnostics.is_empty() {
        println!();
        println!("Warnings:");
        for diagnostic in report.diagnostics.iter().take(5) {
            println!(
                "- {}:{} {}",
                diagnostic.path.display(),
                diagnostic.line_number,
                diagnostic.message
            );
        }
        if report.diagnostics.len() > 5 {
            println!("- ... {} more", report.diagnostics.len() - 5);
        }
    }
}

fn print_usage_row(label: &str, usage: TokenUsage) {
    println!(
        "{:<11} {:>10} {:>10} {:>10} {:>10} {:>10}",
        label,
        usage.input_tokens,
        usage.cached_input_tokens,
        usage.output_tokens,
        usage.reasoning_output_tokens,
        usage.total_tokens
    );
}

#[derive(Debug)]
enum CliError {
    MissingCodexHome,
    NoSessionLogs { searched: Vec<PathBuf> },
    Usage(UsageError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCodexHome => write!(f, "Could not resolve Codex home"),
            Self::NoSessionLogs { searched } => {
                writeln!(f, "No Codex session logs found")?;
                writeln!(f, "Searched:")?;
                for path in searched {
                    writeln!(f, "- {}", path.display())?;
                }
                Ok(())
            }
            Self::Usage(error) => write!(f, "{error}"),
        }
    }
}

impl From<UsageError> for CliError {
    fn from(error: UsageError) -> Self {
        Self::Usage(error)
    }
}
