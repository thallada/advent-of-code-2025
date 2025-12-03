mod runner;

pub mod day01;
pub mod day02;

use clap::Parser;
use color_eyre::Result;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "Advent of Code 2025")]
#[command(about = "Solutions for Advent of Code 2025", long_about = None)]
struct Args {
    /// Day to run (1-25). If not specified, runs all days.
    #[arg(short, long)]
    day: Option<u8>,

    /// Part to run (1 or 2). If not specified, runs all parts.
    #[arg(short, long)]
    part: Option<u8>,
}

runner::days! {
    1 => day01,
    2 => day02,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_span_events(FmtSpan::CLOSE),
        )
        .init();

    let args = Args::parse();

    info!("Advent of Code 2025");
    let _span = tracing::info_span!("aoc").entered();

    run_days(args.day, args.part)?;

    Ok(())
}
