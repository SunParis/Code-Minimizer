//! Command-line entry point for Code Minimizer.
//!
//! All user-facing text emitted by the binary is English by project policy.

use anyhow::Context;
use clap::Parser;
use code_minimizer::cli::{Cli, Commands};
use code_minimizer::logging;
use code_minimizer::reducer::engine::ReducerEngine;
use code_minimizer::runner::{install_signal_handlers, signal_exit_code};

fn main() -> anyhow::Result<()> {
    install_signal_handlers();
    let cli = Cli::parse();

    match cli.command {
        Commands::Reduce(args) => {
            let config = args
                .into_config()
                .context("Failed to build reduce configuration")?;
            let mut engine = ReducerEngine::new(config)?;
            let summary = engine.reduce()?;
            logging::info(format_args!(
                "written: {} ({} -> {} bytes, trials={}, accepted={})",
                summary.output_path.display(),
                summary.original_size,
                summary.final_size,
                summary.total_trials,
                summary.accepted_trials
            ));
            if let Some(signal) = summary.interrupted_by_signal {
                std::process::exit(signal_exit_code(signal));
            }
        }
    }

    Ok(())
}
