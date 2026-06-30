//! End-to-end reducer tests for JavaScript and Java fixtures.
//!
//! The tests use simple shell commands to simulate two runtimes that differ on
//! stdout. They still exercise parsing, candidate generation, oracle execution,
//! rollback, and final output writing.

use std::{fs, time::Duration};

use code_minimizer::{
    ReduceConfig, ReducerEngine,
    config::{
        BuildConfig, DiffMode, PreserveExit, ReducerLimits, ReductionAlgorithm, SizeStopConfig,
    },
    edit::Edit,
    lang::{LanguageAdapter, java::JavaAdapter},
    reducer::candidate::StageKind,
};
use tempfile::tempdir;

/// Builds a compact reducer config for fixture tests.
fn fixture_config(
    language: &str,
    input_name: &str,
    source: &str,
) -> (tempfile::TempDir, ReduceConfig) {
    let dir = tempdir().unwrap();
    let input = dir.path().join(input_name);
    let output = dir.path().join(format!("{input_name}.min"));
    fs::write(&input, source).unwrap();

    let config = ReduceConfig::new(
        language.to_owned(),
        input,
        Some(output),
        "printf A".to_owned(),
        "printf B".to_owned(),
        BuildConfig::None,
        Duration::from_secs(2),
        1024,
        1,
        PreserveExit::SameClass,
        DiffMode::AnyChannel,
        false,
        None,
        1,
        ReducerLimits {
            max_rounds: 3,
            max_trials: 100,
            stop_size: SizeStopConfig::default(),
        },
    )
    .unwrap();

    (dir, config)
}

#[test]
fn reduces_javascript_fixture_while_preserving_diff() {
    let source = r#"
function unused() {
  let value = 123;
  return value;
}
function main() {
  let keep = "still valid";
  console.log(keep);
}
main();
"#;
    let (_dir, config) = fixture_config("js", "case.js", source);
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Reducer should shrink the JavaScript fixture"
    );
    assert!(
        minimized.len() < source.len(),
        "Output file should contain a smaller JavaScript program"
    );
}

#[test]
fn weighted_random_algorithm_reduces_javascript_fixture() {
    // This fixture is intentionally small and uses shell printf commands for
    // both sides. It exercises the new scheduler without depending on a real
    // JavaScript runtime or external compiler.
    let source = r#"
function main() {
  let keep = 1;
  let removeA = 2;
  let removeB = 3;
  console.log(keep);
}
main();
"#;
    let (_dir, config) = fixture_config("js", "weighted.js", source);
    let output = config.output_path.clone();

    // Select the experimental algorithm while keeping all oracle semantics the
    // same as the structured integration tests.
    let mut config = config.with_algorithm(ReductionAlgorithm::WeightedRandom);
    config.limits.max_trials = 40;

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Weighted random reducer should shrink the JavaScript fixture"
    );
    assert!(
        minimized.len() < source.len(),
        "Output file should contain a smaller JavaScript program"
    );
}

#[test]
fn final_blank_line_cleanup_runs_after_structured_algorithm() {
    let source = "function main() {\n  let keep = 1;\n\n   \n  console.log(keep);\n}\nmain();\n";
    let (_dir, config) = fixture_config("js", "blank-lines.js", source);
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Reducer should shrink the JavaScript fixture"
    );
    assert!(
        !minimized.lines().any(|line| line.trim().is_empty()),
        "Final shared cleanup should remove blank and whitespace-only lines"
    );
}

#[test]
fn final_blank_line_cleanup_still_runs_after_size_stop() {
    let source = "function main() {\n  let keep = 1;\n\n   \n  console.log(keep);\n}\nmain();\n";
    let (_dir, mut config) = fixture_config("js", "blank-lines-size-stop.js", source);
    config.limits.stop_size = SizeStopConfig {
        bytes: Some(source.len()),
        percent: None,
    };
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Final cleanup should still shrink source after the size stop is already satisfied"
    );
    assert!(
        !minimized.lines().any(|line| line.trim().is_empty()),
        "Size stop should not skip the final shared blank-line cleanup"
    );
}

#[test]
fn final_blank_line_cleanup_still_runs_after_size_stop_for_weighted_random() {
    let source = "function main() {\n  let keep = 1;\n\n   \n  console.log(keep);\n}\nmain();\n";
    let (_dir, mut config) = fixture_config("js", "blank-lines-size-stop-weighted.js", source);

    // The target is already satisfied before the weighted-random scheduler
    // starts. This catches regressions where the optional size stop prevents
    // the engine-owned final cleanup from removing blank and whitespace-only
    // lines after an algorithm exits early.
    config.limits.stop_size = SizeStopConfig {
        bytes: Some(source.len()),
        percent: None,
    };
    let config = config.with_algorithm(ReductionAlgorithm::WeightedRandom);
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Weighted random final cleanup should still shrink source after the size stop is already satisfied"
    );
    assert!(
        !minimized.lines().any(|line| line.trim().is_empty()),
        "Size stop should not skip weighted random's final shared blank-line cleanup"
    );
}

#[test]
fn final_blank_line_cleanup_rolls_back_when_stage_confirmation_fails() {
    let source = "function main() {\n  console.log(1);\n\n}\nmain();\n";
    let (dir, mut config) = fixture_config("js", "blank-lines-rollback.js", source);
    let output = config.output_path.clone();
    let report_path = dir.path().join("blank-lines-rollback-report.json");
    let cleaned_counter = dir.path().join("blank-line-cleaned-count");
    let cleaned_counter_arg = cleaned_counter.to_string_lossy();
    let stateful_run = format!(
        r#"if ! grep -q 'function main' {{input}} || ! grep -q 'console.log(1)' {{input}} || ! grep -q 'main();' {{input}}; then printf reject; exit 0; fi; blank=$(grep -c '^[[:space:]]*$' {{input}}); if [ "$blank" -eq 0 ]; then cleaned=$(cat "{cleaned_counter_arg}" 2>/dev/null || echo 0); cleaned=$((cleaned + 1)); printf '%s' "$cleaned" > "{cleaned_counter_arg}"; if [ "$cleaned" -eq 1 ]; then printf interesting; else printf reject; fi; else printf interesting; fi"#
    );

    // The oracle accepts the individual blank-line deletion while the final
    // cleanup confirmation deliberately rejects the fully cleaned source. This
    // models a rare non-monotonic or stateful oracle and verifies that the
    // engine restores the source accepted before the cleanup stage.
    config.run_a = stateful_run;
    config.run_b = "printf reject".to_owned();
    config.limits.max_rounds = 0;
    config.json_report_path = Some(report_path.clone());

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();
    let report: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(report_path).unwrap()).unwrap();

    assert_eq!(
        minimized, source,
        "Rejected final blank-line cleanup should roll back to the pre-cleanup accepted source"
    );
    assert_eq!(
        summary.final_size,
        source.len(),
        "Summary should report the restored source size after rollback"
    );
    assert!(
        summary.rejected_trials > 0,
        "The stage-level confirmation rejection should be reported"
    );
    let blank_line_report = report["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|stage| stage["stage"] == "BlankLineCleanup")
        .unwrap();
    assert_eq!(
        blank_line_report["accepted"], 0,
        "Rolled-back cleanup should not remain reported as accepted"
    );
    assert!(
        blank_line_report["rejected"].as_u64().unwrap() > 0,
        "Rolled-back cleanup should be reflected as rejected in the stage report"
    );
}

#[test]
fn size_stop_target_stops_after_reaching_requested_bytes() {
    let source = r#"
function unused() {
  let value = 123;
  return value;
}
function main() {
  let keep = "still valid";
  console.log(keep);
}
main();
"#;
    let (_dir, mut config) = fixture_config("js", "size-stop.js", source);
    config.limits.stop_size = SizeStopConfig {
        bytes: Some(source.len().saturating_sub(1)),
        percent: None,
    };
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Reducer should shrink at least once before the size stop target is reached"
    );
    assert_eq!(
        summary.final_size,
        minimized.len(),
        "Summary size should match the written output"
    );
}

#[test]
fn size_stop_percent_stops_after_reaching_original_size_ratio() {
    let source = r#"
function unused() {
  let value = 123;
  return value;
}
function main() {
  let keep = "still valid";
  console.log(keep);
}
main();
"#;
    let (_dir, mut config) = fixture_config("js", "size-stop-percent.js", source);
    config.limits.stop_size = SizeStopConfig {
        bytes: None,
        percent: Some(99),
    };

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();

    assert!(
        summary.final_size * 100 <= summary.original_size * 99,
        "Reducer should stop only after reaching the configured size percentage"
    );
}

#[test]
fn reduces_java_fixture_while_preserving_diff() {
    let source = r#"
import java.util.Arrays;
public class Test {
  static int unusedField = 123;
  static void unused() {
    int value = 10;
    value++;
  }
  public static void main(String[] args) {
    int keep = 1;
    System.out.println(keep);
  }
}
"#;
    let (_dir, config) = fixture_config("java", "Test.java", source);
    let output = config.output_path.clone();

    let mut engine = ReducerEngine::new(config).unwrap();
    let summary = engine.reduce().unwrap();
    let minimized = fs::read_to_string(output).unwrap();

    assert!(
        summary.final_size < summary.original_size,
        "Reducer should shrink the Java fixture"
    );
    assert!(
        minimized.contains("public class Test"),
        "Java adapter should preserve the public top-level class"
    );
    assert!(
        !minimized.contains("System.out.println(keep);"),
        "Reducer should delete removable Java output statements before lower-value edits"
    );
}

#[test]
fn java_fixture_generates_high_priority_deletions_for_output_noise() {
    let source = include_str!("../test_cases/java/Test.java");
    let adapter = JavaAdapter::new();
    let parsed = adapter.parse(source, "Test.java").unwrap();
    let index = adapter.build_index(&parsed).unwrap();
    let score = code_minimizer::ir::ComplexityScore::compute(&index, source);
    let candidates = adapter
        .generate_groups(
            StageKind::StatementAndSiblingReduction,
            &parsed,
            &index,
            &score,
        )
        .unwrap()
        .into_iter()
        .flat_map(|group| group.candidates)
        .collect::<Vec<_>>();

    let output_deletions = candidates
        .iter()
        .filter(|candidate| candidate.description == "Delete output statement")
        .collect::<Vec<_>>();

    assert!(
        output_deletions.len() >= 20,
        "The real Java fixture should produce many high-priority output deletion candidates"
    );
    assert!(
        output_deletions
            .iter()
            .all(|candidate| candidate.priority > 100),
        "Every output deletion candidate should have high priority"
    );
    assert!(
        output_deletions
            .iter()
            .any(|candidate| match &candidate.edit {
                Edit::Delete(range) =>
                    source[range.start..range.end].contains("System.out.println"),
                _ => false,
            }),
        "The real Java fixture should produce a high-priority deletion candidate for System.out.println noise"
    );
    assert!(
        output_deletions
            .iter()
            .any(|candidate| match &candidate.edit {
                Edit::Delete(range) =>
                    source[range.start..range.end].contains("FuzzerUtils.out.println"),
                _ => false,
            }),
        "The real Java fixture should produce a high-priority deletion candidate for FuzzerUtils.out.println noise"
    );
    assert!(
        output_deletions
            .iter()
            .any(|candidate| match &candidate.edit {
                Edit::Delete(range) => source[range.start..range.end].contains("printStackTrace"),
                _ => false,
            }),
        "The real Java fixture should produce a high-priority deletion candidate for stack trace noise"
    );

    assert!(
        output_deletions
            .iter()
            .all(|candidate| candidate.estimated_size_delta < 0),
        "Every output deletion should shrink the source"
    );
}
