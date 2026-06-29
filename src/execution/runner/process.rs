//! Blocking shell-command runner.
//!
//! This module turns one command template expansion into one captured
//! `CommandOutcome`. It delegates output draining to `output` and process-group
//! cleanup to `signals` so command execution remains readable.

use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use wait_timeout::ChildExt;

use super::{
    outcome::{CommandOutcome, ExitStatusSummary},
    output::{CapturedOutput, join_reader, read_limited},
    signals::{
        ActiveProcessGuard, configure_child_process_group, received_signal, terminate_child,
    },
};

/// Blocking shell-command runner.
#[derive(Clone, Debug)]
pub struct CommandRunner {
    timeout: Duration,
    max_output_bytes: usize,
}

impl CommandRunner {
    /// Creates a runner with a per-command timeout and output cap.
    pub fn new(timeout: Duration, max_output_bytes: usize) -> Self {
        Self {
            timeout,
            max_output_bytes,
        }
    }

    /// Executes a command through `sh -c` in the provided working directory.
    pub fn run(&self, command: &str, cwd: &Path) -> anyhow::Result<CommandOutcome> {
        if let Some(signal) = received_signal() {
            anyhow::bail!("Shutdown signal {signal} received before command start");
        }

        let started = Instant::now();
        let mut command_builder = Command::new("sh");
        command_builder
            .arg("-c")
            .arg(command)
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        configure_child_process_group(&mut command_builder);

        let mut child = command_builder
            .spawn()
            .map_err(|error| anyhow::anyhow!("Failed to start command '{command}': {error}"))?;
        let child_pid = child.id();
        let _process_guard = ActiveProcessGuard::register(child_pid);
        if let Some(signal) = received_signal() {
            terminate_child(&mut child, child_pid)?;
            anyhow::bail!("Shutdown signal {signal} received after command start");
        }

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture command stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture command stderr"))?;

        let max_stdout = self.max_output_bytes;
        let stdout_thread = thread::spawn(move || read_limited(stdout, max_stdout));
        let max_stderr = self.max_output_bytes;
        let stderr_thread = thread::spawn(move || read_limited(stderr, max_stderr));

        let wait_result = child
            .wait_timeout(self.timeout)
            .map_err(|error| anyhow::anyhow!("Failed while waiting for command: {error}"))?;

        let (timed_out, status) = match wait_result {
            Some(status) => (false, ExitStatusSummary::from_exit_status(status)),
            None => {
                terminate_child(&mut child, child_pid)?;
                (true, ExitStatusSummary::TimedOut)
            }
        };

        let CapturedOutput {
            bytes: stdout,
            truncated: stdout_truncated,
        } = join_reader(stdout_thread, "stdout")?;
        let CapturedOutput {
            bytes: stderr,
            truncated: stderr_truncated,
        } = join_reader(stderr_thread, "stderr")?;

        Ok(CommandOutcome {
            status,
            stdout,
            stderr,
            timed_out,
            duration: started.elapsed(),
            stdout_truncated,
            stderr_truncated,
        })
    }
}
