//! Oracle behavior tests.
//!
//! Tests live outside `mod.rs` so the production oracle flow remains readable
//! while still testing baseline validation, confirmation handling, and workspace
//! cleanup behavior in the same module privacy boundary.

use std::{fs, time::Duration};

use tempfile::tempdir;

use super::*;
use crate::config::{BuildConfig, PreserveExit, ReducerLimits};

/// Builds a minimal oracle config for shell-command tests.
fn config(run_a: &str, run_b: &str) -> ReduceConfig {
    let dir = tempdir().unwrap();
    let input = dir.path().join("case.js");
    fs::write(&input, "console.log(1);").unwrap();
    ReduceConfig::new(
        "js".into(),
        input,
        None,
        run_a.into(),
        run_b.into(),
        BuildConfig::None,
        Duration::from_secs(2),
        1024,
        1,
        PreserveExit::SameClass,
        DiffMode::AnyChannel,
        false,
        None,
        1,
        ReducerLimits::default(),
    )
    .unwrap()
}

/// Builds a minimal oracle config with custom confirmation count.
fn config_with_confirm_runs(run_a: &str, run_b: &str, confirm_runs: usize) -> ReduceConfig {
    let dir = tempdir().unwrap();
    let input = dir.path().join("case.js");
    fs::write(&input, "console.log(1);").unwrap();
    ReduceConfig::new(
        "js".into(),
        input,
        None,
        run_a.into(),
        run_b.into(),
        BuildConfig::None,
        Duration::from_secs(2),
        1024,
        confirm_runs,
        PreserveExit::SameClass,
        DiffMode::AnyChannel,
        false,
        None,
        1,
        ReducerLimits::default(),
    )
    .unwrap()
}

#[test]
fn baseline_requires_output_difference() {
    let oracle = Oracle::new(config("printf same", "printf same")).unwrap();
    let workspace = SessionWorkspace::new(false).unwrap();
    let error = oracle.establish_baseline(&workspace, "x").unwrap_err();
    assert!(
        error.to_string().contains("Baseline is not interesting"),
        "Unexpected error: {error}"
    );
}

#[test]
fn candidate_is_rejected_when_diff_disappears() {
    let oracle = Oracle::new(config("printf A", "printf B")).unwrap();
    let workspace = SessionWorkspace::new(false).unwrap();
    let baseline = oracle.establish_baseline(&workspace, "x").unwrap();

    let same_oracle = Oracle::new(config("printf same", "printf same")).unwrap();
    let decision = same_oracle
        .evaluate_candidate(&workspace, "x", 1, &baseline)
        .unwrap();

    assert!(!decision.accepted);
}

#[test]
fn candidate_requires_all_confirmations_to_preserve_diff() {
    let stable_oracle = Oracle::new(config("printf A", "printf B")).unwrap();
    let workspace = SessionWorkspace::new(false).unwrap();
    let baseline = stable_oracle.establish_baseline(&workspace, "x").unwrap();

    let dir = tempdir().unwrap();
    let marker = dir.path().join("seen");
    let marker_text = marker.display().to_string();
    let run_a = format!(
        "if [ -f \"{marker_text}\" ]; then printf same; else touch \"{marker_text}\"; printf A; fi"
    );
    let flaky_oracle = Oracle::new(config_with_confirm_runs(&run_a, "printf same", 2)).unwrap();
    let decision = flaky_oracle
        .evaluate_candidate(&workspace, "x", 1, &baseline)
        .unwrap();

    assert!(!decision.accepted);
    assert!(
        decision
            .reason
            .as_deref()
            .is_some_and(|reason| reason.contains("confirmation 2")),
        "Unexpected rejection reason: {:?}",
        decision.reason
    );
}

#[test]
fn candidate_trials_reuse_current_workspace_directory() {
    let oracle = Oracle::new(config("printf A", "printf B")).unwrap();
    let workspace = SessionWorkspace::new(false).unwrap();
    let baseline = oracle.establish_baseline(&workspace, "x").unwrap();

    oracle
        .evaluate_candidate(&workspace, "candidate one", 1, &baseline)
        .unwrap();
    oracle
        .evaluate_candidate(&workspace, "candidate two", 2, &baseline)
        .unwrap();

    let trials = fs::read_dir(workspace.root().join("trials"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let history = fs::read_dir(workspace.root().join("history"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    assert_eq!(trials, vec!["current".to_owned()]);
    assert!(history.iter().any(|name| name == "trial-1.diff"));
    assert!(history.iter().any(|name| name == "trial-2.diff"));
    assert!(
        workspace
            .root()
            .join("trials/current/current.diff")
            .exists()
    );
}

#[test]
fn diff_mode_rejects_exit_only_mode() {
    let error = DiffMode::parse("exit").unwrap_err();
    assert!(
        error.to_string().contains("Invalid diff mode"),
        "Unexpected error: {error}"
    );
}
