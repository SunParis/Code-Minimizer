//! Reducer engine behavior tests.
//!
//! These tests cover the high-level reducer workflow with a custom adapter.
//! Lower-level objective, grouping, and reporting helpers are tested in their
//! own modules so this file stays focused on orchestration.

use std::{fs, time::Duration};

use tempfile::tempdir;

use crate::{
    config::{BuildConfig, DiffMode, PreserveExit, ReduceConfig, ReducerLimits, SizeStopConfig},
    edit::{Edit, TextRange},
    ir::{ComplexityScore, ProgramIndex, SnapshotId},
    lang::{LanguageAdapter, ParseDiagnostic, ParsedProgram},
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        engine::ReducerEngine,
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
    },
};

/// Minimal adapter used to test engine orchestration without depending on a real language.
struct SimpleAdapter;

impl LanguageAdapter for SimpleAdapter {
    fn language_id(&self) -> &'static str {
        "simple"
    }

    fn parse(&self, source: &str, file_name: &str) -> anyhow::Result<ParsedProgram> {
        let diagnostics = if source.contains("BAD") {
            vec![ParseDiagnostic {
                message: "bad token".into(),
                range: TextRange { start: 0, end: 3 },
            }]
        } else {
            Vec::new()
        };
        Ok(ParsedProgram::synthetic(source, file_name, diagnostics))
    }

    fn generate_groups(
        &self,
        stage: StageKind,
        parsed: &ParsedProgram,
        _index: &ProgramIndex,
        _score: &ComplexityScore,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        if stage != StageKind::StatementAndSiblingReduction {
            return Ok(Vec::new());
        }
        if let Some(index) = parsed.source.find("remove") {
            let candidate = Candidate::new(
                stage,
                TransformKind::DeleteStatementChunk,
                "remove-word",
                "Delete removable word",
                Edit::Delete(TextRange {
                    start: index,
                    end: index + "remove".len(),
                }),
                10,
                -6,
            );
            return Ok(vec![CandidateGroup::new(
                "simple:statements",
                SnapshotId::ROOT,
                stage,
                CandidateGroupKind::SiblingList,
                "Simple statement candidates",
                vec![candidate],
                GroupStrategy::SinglesOnly,
                10,
            )]);
        }
        Ok(Vec::new())
    }
}

#[test]
fn engine_accepts_reducing_candidate_with_custom_adapter() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("case.txt");
    let output = dir.path().join("case.min.txt");
    fs::write(&input, "keep remove").unwrap();

    let config = ReduceConfig::new(
        "js".into(),
        input,
        Some(output.clone()),
        "printf A".into(),
        "printf B".into(),
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
            max_rounds: 2,
            max_trials: 10,
            stop_size: SizeStopConfig::default(),
        },
    )
    .unwrap();

    let mut engine = ReducerEngine::new_with_adapter_for_tests(config, Box::new(SimpleAdapter));
    let summary = engine.reduce().unwrap();

    assert_eq!(fs::read_to_string(output).unwrap(), "keep ");
    assert_eq!(summary.accepted_trials, 1);
}
