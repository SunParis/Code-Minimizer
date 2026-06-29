//! Candidate edits, reducer stages, and semantic transform identifiers.
//!
//! The reducer is organized around coarse algorithmic stages. Each candidate
//! also records the semantic transform it proposes so ordering, grouping, and
//! reports can explain why the edit was attempted without depending on
//! language-specific syntax categories.

use serde::{Deserialize, Serialize};

use crate::{
    edit::Edit,
    ir::{NodeId, SnapshotId, SymbolId, score::ScoreDelta},
    reducer::edit_plan::{EditIntent, EditPlan, EditPlanId, EditSafety},
};

/// Stable candidate identifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CandidateId(pub String);

/// Algorithmic reducer stage.
///
/// Stages are intentionally language-neutral. They describe why a candidate is
/// being tried, not the exact syntax node that will be edited.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum StageKind {
    /// Build the first parsed snapshot and baseline oracle observation.
    BaselineAndIndex,
    /// Reduce static runtime cost before expensive structural work.
    RuntimeCostReduction,
    /// Remove or neutralize function call sites, then delete functions.
    AggressiveFunctionElimination,
    /// Delete, empty, or unwrap large blocks and owner constructs.
    AggressiveBlockElimination,
    /// Reduce statement sibling lists with chunks, windows, and singles.
    StatementAndSiblingReduction,
    /// Remove dead declarations and output-only data flow.
    DeadDeclarationAndOutputCleanup,
    /// Clean expressions, literals, types, signatures, imports, and trivia.
    ExpressionLiteralTypeCleanup,
    /// Final single-candidate sweep used to reach practical 1-minimality.
    FinalOneMinimalSweep,
    /// Final validated removal of blank and whitespace-only lines.
    BlankLineCleanup,
}

impl StageKind {
    /// Returns the stable fixed-point stage order.
    pub fn ordered() -> &'static [StageKind] {
        &[
            StageKind::RuntimeCostReduction,
            StageKind::AggressiveFunctionElimination,
            StageKind::AggressiveBlockElimination,
            StageKind::StatementAndSiblingReduction,
            StageKind::DeadDeclarationAndOutputCleanup,
            StageKind::ExpressionLiteralTypeCleanup,
        ]
    }

    /// Returns all stages whose single candidates participate in the final sweep.
    pub fn final_sweep_sources() -> &'static [StageKind] {
        Self::ordered()
    }

    /// Returns a stable English stage name for logs and reports.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BaselineAndIndex => "BaselineAndIndex",
            Self::RuntimeCostReduction => "RuntimeCostReduction",
            Self::AggressiveFunctionElimination => "AggressiveFunctionElimination",
            Self::AggressiveBlockElimination => "AggressiveBlockElimination",
            Self::StatementAndSiblingReduction => "StatementAndSiblingReduction",
            Self::DeadDeclarationAndOutputCleanup => "DeadDeclarationAndOutputCleanup",
            Self::ExpressionLiteralTypeCleanup => "ExpressionLiteralTypeCleanup",
            Self::FinalOneMinimalSweep => "FinalOneMinimalSweep",
            Self::BlankLineCleanup => "BlankLineCleanup",
        }
    }

    /// Returns a concise human-facing stage title for progress logs.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::BaselineAndIndex => "Baseline and index",
            Self::RuntimeCostReduction => "Runtime cost reduction",
            Self::AggressiveFunctionElimination => "Aggressive function elimination",
            Self::AggressiveBlockElimination => "Aggressive block elimination",
            Self::StatementAndSiblingReduction => "Statement and sibling reduction",
            Self::DeadDeclarationAndOutputCleanup => "Dead declaration and output cleanup",
            Self::ExpressionLiteralTypeCleanup => "Expression, literal, and type cleanup",
            Self::FinalOneMinimalSweep => "Final one-minimal sweep",
            Self::BlankLineCleanup => "Blank-line cleanup",
        }
    }

    /// Returns a short explanation of what the stage is trying to simplify.
    pub fn purpose(self) -> &'static str {
        match self {
            Self::BaselineAndIndex => {
                "validate the original A/B difference and build the first parsed index"
            }
            Self::RuntimeCostReduction => {
                "lower static runtime-cost signals such as loop bounds before broader deletion"
            }
            Self::AggressiveFunctionElimination => {
                "remove call sites and function-like declarations when the oracle still passes"
            }
            Self::AggressiveBlockElimination => {
                "delete, empty, or unwrap larger control-flow and block regions"
            }
            Self::StatementAndSiblingReduction => {
                "reduce related statement lists with chunks, complements, and singles"
            }
            Self::DeadDeclarationAndOutputCleanup => {
                "remove dead declarations and output-only data flow"
            }
            Self::ExpressionLiteralTypeCleanup => {
                "shrink literals, expressions, signatures, imports, trivia, and leftovers"
            }
            Self::FinalOneMinimalSweep => {
                "try remaining candidates one at a time after normal stages stop accepting changes"
            }
            Self::BlankLineCleanup => {
                "remove blank and whitespace-only lines only when parser and oracle validation still pass"
            }
        }
    }
}

/// Semantic transform category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TransformKind {
    /// Shrink a loop bound or trip-count-driving initializer.
    ShrinkLoopBound,
    /// Remove a call site statement.
    RemoveCallSite,
    /// Replace a call expression with a deterministic default value.
    ReplaceCallWithValue,
    /// Delete a function-like declaration.
    DeleteFunctionDecl,
    /// Delete a whole owner construct such as `if`, `for`, `try`, or `switch`.
    DeleteWholeConstruct,
    /// Replace a block or body with an empty/minimal body.
    EmptyBlockBody,
    /// Remove construct prefix/braces while preserving body statements.
    UnwrapBlock,
    /// Delete a statement chunk, window, or single statement.
    DeleteStatementChunk,
    /// Delete a dead declaration.
    DeleteDeadDeclaration,
    /// Remove data only used by output/noise operations.
    RemoveOutputOnlyVariable,
    /// Shrink a literal or collection.
    ShrinkLiteral,
    /// Clean imports, trivia, signatures, or low-risk leftovers.
    Cleanup,
}

impl TransformKind {
    /// Returns a short stable name for logs, ids, and report readability.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShrinkLoopBound => "ShrinkLoopBound",
            Self::RemoveCallSite => "RemoveCallSite",
            Self::ReplaceCallWithValue => "ReplaceCallWithValue",
            Self::DeleteFunctionDecl => "DeleteFunctionDecl",
            Self::DeleteWholeConstruct => "DeleteWholeConstruct",
            Self::EmptyBlockBody => "EmptyBlockBody",
            Self::UnwrapBlock => "UnwrapBlock",
            Self::DeleteStatementChunk => "DeleteStatementChunk",
            Self::DeleteDeadDeclaration => "DeleteDeadDeclaration",
            Self::RemoveOutputOnlyVariable => "RemoveOutputOnlyVariable",
            Self::ShrinkLiteral => "ShrinkLiteral",
            Self::Cleanup => "Cleanup",
        }
    }
}

/// Expected effect of a candidate before oracle validation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CandidateEffect {
    /// Nodes expected to disappear if the candidate is accepted.
    pub removes_nodes: Vec<NodeId>,
    /// Nodes expected to remain but with changed text.
    pub replaces_nodes: Vec<NodeId>,
    /// Symbols that may be broken by this candidate and therefore need oracle validation.
    pub may_break_symbols: Vec<SymbolId>,
    /// Estimated score change used only for ordering and reporting.
    pub expected_score_delta: ScoreDelta,
    /// Static risk estimate. Risk never directly decides acceptance.
    pub risk: CandidateRisk,
}

impl CandidateEffect {
    /// Creates an effect summary for a simple size-changing edit.
    pub fn from_size_delta(estimated_size_delta: isize, risk: CandidateRisk) -> Self {
        Self {
            removes_nodes: Vec::new(),
            replaces_nodes: Vec::new(),
            may_break_symbols: Vec::new(),
            expected_score_delta: ScoreDelta::from_size_delta(estimated_size_delta),
            risk,
        }
    }
}

/// Candidate risk classification used for ordering and diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CandidateRisk {
    /// Low-risk edits such as comment or empty-statement removal.
    Low,
    /// Medium-risk edits such as output statement removal or literal shrinkage.
    Medium,
    /// High-risk edits such as declaration deletion or coordinated signature edits.
    High,
    /// Speculative edits based on incomplete static information.
    Speculative,
}

/// One transformation candidate generated from the current parsed source.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    /// Stable identifier for logging and debugging.
    pub id: CandidateId,
    /// Snapshot version this candidate was generated from.
    pub snapshot: SnapshotId,
    /// Algorithmic stage that owns this candidate.
    pub stage: StageKind,
    /// Primary target node when the adapter can provide one.
    pub target: Option<NodeId>,
    /// English explanation of the candidate.
    pub description: String,
    /// Semantic edit plan tested by the reducer.
    pub plan: EditPlan,
    /// Source edit to test atomically.
    ///
    /// This field is preserved for adapter and test compatibility. The reducer
    /// applies `plan`, and constructors keep `edit` synchronized with the first
    /// plan edit.
    pub edit: Edit,
    /// Higher-priority candidates are attempted earlier.
    pub priority: i32,
    /// Estimated byte-size delta. Negative values shrink the source.
    pub estimated_size_delta: isize,
    /// Static effect estimate used for reports and candidate ordering.
    pub effect: CandidateEffect,
    /// Candidate ids that must be accepted first.
    pub requires: Vec<CandidateId>,
    /// Candidate ids that must not be applied in the same chunk.
    pub conflicts_with: Vec<CandidateId>,
    /// Nodes whose ids become invalid if this candidate is accepted.
    pub invalidates: Vec<NodeId>,
    /// Reserved dependency list for future multi-candidate constraints.
    pub dependencies: Vec<CandidateId>,
    /// Semantic transform category.
    pub transform: TransformKind,
}

impl Candidate {
    /// Constructs a candidate with no dependencies.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        stage: StageKind,
        transform: TransformKind,
        id: impl Into<String>,
        description: impl Into<String>,
        edit: Edit,
        priority: i32,
        estimated_size_delta: isize,
    ) -> Self {
        let id = CandidateId(id.into());
        let description = description.into();
        let risk = risk_for_transform(transform, &description);
        let effect = CandidateEffect::from_size_delta(estimated_size_delta, risk);
        let plan = EditPlan {
            id: EditPlanId(id.0.clone()),
            snapshot: SnapshotId::ROOT,
            primary_target: None,
            edits: vec![edit.clone()],
            intent: intent_for_transform(transform, &description),
            safety: safety_for_risk(risk),
            affected_nodes: Vec::new(),
            affected_symbols: Vec::new(),
            expected_effect: effect.clone(),
        };
        Self {
            id,
            snapshot: SnapshotId::ROOT,
            stage,
            target: None,
            description,
            plan,
            edit,
            priority,
            estimated_size_delta,
            effect,
            requires: Vec::new(),
            conflicts_with: Vec::new(),
            invalidates: Vec::new(),
            dependencies: Vec::new(),
            transform,
        }
    }

    /// Returns this candidate retargeted to a freshly parsed snapshot.
    pub fn for_snapshot(mut self, snapshot: SnapshotId, target: Option<NodeId>) -> Self {
        self.snapshot = snapshot;
        self.target = target;
        self.plan.snapshot = snapshot;
        self.plan.primary_target = target;
        self
    }

    /// Returns true when this candidate is aimed at removable user-visible output.
    pub fn is_output_noise(&self) -> bool {
        self.description.to_ascii_lowercase().contains("output")
            || self.priority >= 200
            || matches!(self.plan.intent, EditIntent::DeleteOutput)
            || matches!(self.transform, TransformKind::RemoveOutputOnlyVariable)
    }
}

/// Classifies semantic transforms into edit intents.
fn intent_for_transform(transform: TransformKind, description: &str) -> EditIntent {
    let lower = description.to_ascii_lowercase();
    if lower.contains("output") {
        return EditIntent::DeleteOutput;
    }

    match transform {
        TransformKind::ShrinkLoopBound | TransformKind::DeleteWholeConstruct => {
            EditIntent::SimplifyControlFlow
        }
        TransformKind::RemoveCallSite | TransformKind::DeleteStatementChunk => {
            EditIntent::DeleteStatement
        }
        TransformKind::ReplaceCallWithValue => EditIntent::SimplifyExpression,
        TransformKind::DeleteFunctionDecl | TransformKind::DeleteDeadDeclaration => {
            EditIntent::DeleteDeclaration
        }
        TransformKind::EmptyBlockBody | TransformKind::UnwrapBlock => EditIntent::ReplaceBlock,
        TransformKind::RemoveOutputOnlyVariable => EditIntent::DeleteOutput,
        TransformKind::ShrinkLiteral => EditIntent::ShrinkLiteral,
        TransformKind::Cleanup if lower.contains("trivia") || lower.contains("comment") => {
            EditIntent::DeleteTrivia
        }
        TransformKind::Cleanup if lower.contains("signature") || lower.contains("parameter") => {
            EditIntent::SimplifySignature
        }
        TransformKind::Cleanup => EditIntent::Cleanup,
    }
}

/// Estimates static risk for ordering and diagnostics.
fn risk_for_transform(transform: TransformKind, description: &str) -> CandidateRisk {
    let lower = description.to_ascii_lowercase();
    if lower.contains("output") {
        return CandidateRisk::Medium;
    }

    match transform {
        TransformKind::Cleanup => CandidateRisk::Low,
        TransformKind::ShrinkLoopBound
        | TransformKind::RemoveCallSite
        | TransformKind::DeleteStatementChunk
        | TransformKind::RemoveOutputOnlyVariable
        | TransformKind::ShrinkLiteral => CandidateRisk::Medium,
        TransformKind::DeleteFunctionDecl | TransformKind::DeleteDeadDeclaration => {
            CandidateRisk::High
        }
        TransformKind::ReplaceCallWithValue
        | TransformKind::DeleteWholeConstruct
        | TransformKind::EmptyBlockBody
        | TransformKind::UnwrapBlock => CandidateRisk::Speculative,
    }
}

/// Converts risk to the conservative edit safety label used by reports.
fn safety_for_risk(risk: CandidateRisk) -> EditSafety {
    match risk {
        CandidateRisk::Low => EditSafety::LowRisk,
        CandidateRisk::Medium => EditSafety::MediumRisk,
        CandidateRisk::High => EditSafety::HighRisk,
        CandidateRisk::Speculative => EditSafety::Speculative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_id_can_label_synthetic_chunks() {
        let id = CandidateId("chunk".to_owned());
        assert_eq!(id.0, "chunk");
    }
}
