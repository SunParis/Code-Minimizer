//! Bounded pipe readers for command stdout and stderr.
//!
//! The runner must keep draining pipes even after the retained byte limit is
//! reached; otherwise a noisy child could block forever while writing to a full
//! pipe. This module owns that drain-and-truncate behavior.

use std::{io::Read, thread};

/// Bounded output accumulated from one pipe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CapturedOutput {
    /// Retained prefix of the stream.
    pub bytes: Vec<u8>,
    /// Whether bytes were discarded after the retention cap was reached.
    pub truncated: bool,
}

/// Reads a pipe to EOF while retaining only the configured number of bytes.
pub(super) fn read_limited<R: Read>(
    mut reader: R,
    max_bytes: usize,
) -> anyhow::Result<CapturedOutput> {
    let mut retained = Vec::new();
    let mut truncated = false;
    let mut buffer = [0_u8; 8192];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        let remaining = max_bytes.saturating_sub(retained.len());
        if remaining > 0 {
            let to_keep = remaining.min(read);
            retained.extend_from_slice(&buffer[..to_keep]);
        }

        if read > remaining {
            truncated = true;
        }
    }

    Ok(CapturedOutput {
        bytes: retained,
        truncated,
    })
}

/// Joins a reader thread and preserves a clear stream-specific error message.
pub(super) fn join_reader(
    handle: thread::JoinHandle<anyhow::Result<CapturedOutput>>,
    stream_name: &str,
) -> anyhow::Result<CapturedOutput> {
    handle
        .join()
        .map_err(|_| anyhow::anyhow!("Output reader thread panicked while reading {stream_name}"))?
}
