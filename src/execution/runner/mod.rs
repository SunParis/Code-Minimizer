//! Process execution with timeout and bounded output capture.
//!
//! Each build or run command is executed through the platform shell. The reducer
//! intentionally does not sandbox user commands; instead, it isolates trial
//! files in temporary directories and documents that commands execute on the
//! local machine.

mod outcome;
mod output;
mod process;
mod signals;

pub use outcome::{CommandOutcome, ExitStatusClass, ExitStatusSummary, InvocationOutcome};
pub use process::CommandRunner;
pub use signals::{
    install_signal_handlers, received_signal, signal_exit_code,
    terminate_active_processes_for_shutdown,
};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn runner_captures_stdout_and_stderr() {
        let dir = tempdir().unwrap();
        let runner = CommandRunner::new(Duration::from_secs(2), 1024);
        let outcome = runner
            .run("printf hello; printf error >&2", dir.path())
            .unwrap();

        assert_eq!(outcome.status, ExitStatusSummary::Code(0));
        assert_eq!(outcome.stdout, b"hello");
        assert_eq!(outcome.stderr, b"error");
    }

    #[test]
    fn runner_reports_timeout() {
        let dir = tempdir().unwrap();
        let runner = CommandRunner::new(Duration::from_millis(50), 1024);
        let outcome = runner.run("sleep 1", dir.path()).unwrap();

        assert!(outcome.timed_out);
        assert_eq!(outcome.status, ExitStatusSummary::TimedOut);
    }

    #[test]
    fn runner_truncates_large_output_but_drains_pipe() {
        let dir = tempdir().unwrap();
        let runner = CommandRunner::new(Duration::from_secs(2), 4);
        let outcome = runner.run("printf 123456789", dir.path()).unwrap();

        assert_eq!(outcome.stdout, b"1234");
        assert!(outcome.stdout_truncated);
    }

    #[cfg(unix)]
    #[test]
    fn runner_timeout_kills_shell_descendants() {
        let dir = tempdir().unwrap();
        let marker = dir.path().join("descendant-finished");
        let marker_text = marker.display().to_string();
        let runner = CommandRunner::new(Duration::from_millis(100), 1024);

        let outcome = runner
            .run(
                &format!("sh -c 'sleep 2; touch \"{marker_text}\"'"),
                dir.path(),
            )
            .unwrap();
        thread::sleep(Duration::from_millis(2500));

        assert!(outcome.timed_out);
        assert!(
            !marker.exists(),
            "timed-out command descendant should have been killed"
        );
    }
}
