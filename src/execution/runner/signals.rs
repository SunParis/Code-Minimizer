//! Process-group registration and shutdown cleanup.
//!
//! Commands are launched in their own process group on Unix so timeout and
//! Ctrl+C/SIGTERM cleanup can terminate shell descendants, not just the direct
//! `sh -c` process.

use std::{
    collections::HashSet,
    process::{Child, Command},
    sync::{
        LazyLock, Mutex, Once,
        atomic::{AtomicI32, Ordering},
    },
    thread,
    time::Duration,
};

use wait_timeout::ChildExt;

use crate::logging;

/// Grace period between SIGTERM and SIGKILL for timed-out or interrupted runs.
const SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_secs(1);

/// Process groups for commands currently owned by this process.
static ACTIVE_PROCESS_GROUPS: LazyLock<Mutex<HashSet<u32>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Ensures signal handlers and the monitor thread are installed once.
static SIGNAL_INSTALL: Once = Once::new();

/// Last shutdown signal observed by the async signal handler.
static RECEIVED_SIGNAL: AtomicI32 = AtomicI32::new(0);

/// Registration guard for one running shell command.
pub(super) struct ActiveProcessGuard {
    process_group: u32,
}

impl ActiveProcessGuard {
    /// Registers a process group until the guard is dropped.
    pub(super) fn register(process_group: u32) -> Self {
        if let Ok(mut active) = ACTIVE_PROCESS_GROUPS.lock() {
            active.insert(process_group);
        }
        Self { process_group }
    }
}

impl Drop for ActiveProcessGuard {
    /// Removes the process group from the global active set.
    fn drop(&mut self) {
        if let Ok(mut active) = ACTIVE_PROCESS_GROUPS.lock() {
            active.remove(&self.process_group);
        }
    }
}

/// Installs Ctrl+C/SIGTERM cleanup for child process groups.
pub fn install_signal_handlers() {
    SIGNAL_INSTALL.call_once(|| {
        install_platform_signal_handlers();
        thread::spawn(|| {
            loop {
                let signal = RECEIVED_SIGNAL.load(Ordering::SeqCst);
                if signal != 0 {
                    logging::error(format_args!(
                        "received signal {signal}; terminating active child processes before exit"
                    ));
                    terminate_active_processes_for_shutdown();
                    std::process::exit(128 + signal);
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    });
}

/// Sends SIGTERM to all active command process groups, then SIGKILL after a grace period.
pub fn terminate_active_processes_for_shutdown() {
    let groups = active_process_groups();
    if groups.is_empty() {
        return;
    }

    terminate_process_groups(&groups);
    thread::sleep(SHUTDOWN_GRACE_PERIOD);
    kill_process_groups(&groups);
}

/// Terminates one child process group for timeout handling.
pub(super) fn terminate_child(child: &mut Child, process_group: u32) -> anyhow::Result<()> {
    terminate_process_groups(&[process_group]);
    let _ = child.wait_timeout(SHUTDOWN_GRACE_PERIOD)?;
    kill_process_groups(&[process_group]);
    let _ = child.wait();
    Ok(())
}

/// Returns a snapshot of active process groups.
fn active_process_groups() -> Vec<u32> {
    ACTIVE_PROCESS_GROUPS
        .lock()
        .map(|active| active.iter().copied().collect())
        .unwrap_or_default()
}

/// Configures a spawned shell to become a new process-group leader on Unix.
#[cfg(unix)]
pub(super) fn configure_child_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;

    unsafe {
        command.pre_exec(|| {
            if setpgid(0, 0) == 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        });
    }
}

/// Non-Unix platforms currently run without process-group configuration.
#[cfg(not(unix))]
pub(super) fn configure_child_process_group(_command: &mut Command) {}

/// Installs platform signal handlers on Unix.
#[cfg(unix)]
fn install_platform_signal_handlers() {
    unsafe {
        signal(SIGINT, handle_signal);
        signal(SIGTERM, handle_signal);
    }
}

/// Non-Unix platforms currently rely on default signal behavior.
#[cfg(not(unix))]
fn install_platform_signal_handlers() {}

/// Async signal handler; it only stores the signal number.
extern "C" fn handle_signal(signal: i32) {
    RECEIVED_SIGNAL.store(signal, Ordering::SeqCst);
}

/// Sends SIGTERM to process groups on Unix.
#[cfg(unix)]
fn terminate_process_groups(process_groups: &[u32]) {
    for process_group in process_groups {
        signal_process_group(*process_group, SIGTERM);
    }
}

/// Non-Unix placeholder for SIGTERM cleanup.
#[cfg(not(unix))]
fn terminate_process_groups(_process_groups: &[u32]) {}

/// Sends SIGKILL to process groups on Unix.
#[cfg(unix)]
fn kill_process_groups(process_groups: &[u32]) {
    for process_group in process_groups {
        signal_process_group(*process_group, SIGKILL);
    }
}

/// Non-Unix placeholder for SIGKILL cleanup.
#[cfg(not(unix))]
fn kill_process_groups(_process_groups: &[u32]) {}

/// Sends one signal to a Unix process group.
#[cfg(unix)]
fn signal_process_group(process_group: u32, signal_number: i32) {
    if process_group <= i32::MAX as u32 {
        unsafe {
            kill(-(process_group as i32), signal_number);
        }
    }
}

#[cfg(unix)]
const SIGINT: i32 = 2;
#[cfg(unix)]
const SIGKILL: i32 = 9;
#[cfg(unix)]
const SIGTERM: i32 = 15;

#[cfg(unix)]
unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
    fn setpgid(pid: i32, pgid: i32) -> i32;
    fn signal(sig: i32, handler: extern "C" fn(i32)) -> extern "C" fn(i32);
}
