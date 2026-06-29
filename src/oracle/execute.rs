//! Side-specific build/run execution for the oracle.
//!
//! This module converts trial layouts and command templates into
//! `InvocationOutcome` values. It does not decide interestingness; it only
//! executes commands, validates build completion, and captures outcomes.

use std::time::{Duration, Instant};

use crate::{
    command_template::{TemplateContext, expand_template},
    config::TrialSide,
    logging,
    runner::{CommandOutcome, ExitStatusSummary, InvocationOutcome},
    workspace::{SideLayout, TrialLayout},
};

use super::{Oracle, checks::validate_build_completed};

impl Oracle {
    /// Executes optional build and required run command for one side.
    pub(super) fn execute_side(
        &self,
        layout: &TrialLayout,
        side: TrialSide,
    ) -> anyhow::Result<InvocationOutcome> {
        let side_layout = layout.side(side)?;
        let build = if let Some(template) = self.config.build.command_for_side(side) {
            let command = self.expand_for_side(template, side_layout)?;
            logging::info(format_args!("starting side {} build", side.as_label()));
            let started = Instant::now();
            let outcome = self.runner.run(&command, &side_layout.dir)?;
            logging::info(format_args!(
                "finished side {} build in {}",
                side.as_label(),
                format_duration(started.elapsed())
            ));
            validate_build_completed(&outcome)?;
            Some(outcome)
        } else {
            None
        };

        let run_template = match side {
            TrialSide::A => &self.config.run_a,
            TrialSide::B => &self.config.run_b,
        };
        let run_command = self.expand_for_side(run_template, side_layout)?;
        logging::info(format_args!("starting side {} run", side.as_label()));
        let started = Instant::now();
        let run = self.runner.run(&run_command, &side_layout.dir)?;
        logging::info(format_args!(
            "finished side {} run in {}",
            side.as_label(),
            format_duration(started.elapsed())
        ));

        Ok(InvocationOutcome { build, run })
    }

    /// Expands command placeholders for the side-specific trial layout.
    fn expand_for_side(&self, template: &str, side_layout: &SideLayout) -> anyhow::Result<String> {
        expand_template(
            template,
            &TemplateContext {
                input: &side_layout.input_path,
                dir: &side_layout.dir,
                stem: &self.stem,
                output: &side_layout.output_dir,
            },
        )
    }
}

/// Builds a placeholder run outcome for rare command-start failures.
pub(super) fn empty_failed_run() -> CommandOutcome {
    CommandOutcome {
        status: ExitStatusSummary::Unknown,
        stdout: Vec::new(),
        stderr: Vec::new(),
        timed_out: false,
        duration: Duration::ZERO,
        stdout_truncated: false,
        stderr_truncated: false,
    }
}

/// Formats a duration compactly for reducer progress logs.
fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis < 1_000 {
        format!("{millis}ms")
    } else {
        format!("{:.3}s", duration.as_secs_f64())
    }
}
