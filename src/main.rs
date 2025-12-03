pub mod day01;
pub mod day02;

use color_eyre::Result;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

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

    info!("Advent of Code 2025");
    {
        let _span = tracing::info_span!("aoc").entered();
        day01::solve()?;
        day02::solve()?;
    }
    Ok(())
}
