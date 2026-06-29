//! Semantic wrappers around low-level text edits.
//!
//! `Edit::Multi` knows only how to edit byte ranges. `EditPlan` records the
//! reducer intent, affected snapshot ids, and safety metadata so reports and
//! retry strategies can explain why an edit was attempted.

use serde::{Deserialize, Serialize};

use crate::{
    edit::Edit,
    ir::{NodeId, SnapshotId, SymbolId},
};

use super::candidate::CandidateEffect;

/// Stable edit-plan identifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EditPlanId(pub String);

/// Language-neutral intent for a candidate edit.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EditIntent {
    /// Delete comment or trivia.
    DeleteTrivia,
    /// Delete user-visible output.
    DeleteOutput,
    /// Delete a declaration.
    DeleteDeclaration,
    /// Delete an executable statement.
    DeleteStatement,
    /// Replace a block with a smaller legal body.
    ReplaceBlock,
    /// Simplify a control-flow construct.
    SimplifyControlFlow,
    /// Simplify an expression or call.
    SimplifyExpression,
    /// Shrink literal or data content.
    ShrinkLiteral,
    /// Simplify signature, type, or member shape.
    SimplifySignature,
    /// Cleanup redundant leftovers.
    Cleanup,
    /// Coordinated or adapter-specific edit.
    Coordinated,
}

/// Static safety estimate for an edit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EditSafety {
    /// Low risk according to syntax and local structure.
    LowRisk,
    /// Medium risk and likely to need oracle validation.
    MediumRisk,
    /// High risk and likely to fail without dependencies.
    HighRisk,
    /// Speculative edit based on incomplete information.
    Speculative,
}

/// A semantic candidate plan tested atomically.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EditPlan {
    /// Plan id.
    pub id: EditPlanId,
    /// Snapshot version all ranges and ids belong to.
    pub snapshot: SnapshotId,
    /// Primary target node when known.
    pub primary_target: Option<NodeId>,
    /// Low-level edits applied atomically.
    pub edits: Vec<Edit>,
    /// Semantic edit intent.
    pub intent: EditIntent,
    /// Static safety estimate.
    pub safety: EditSafety,
    /// Nodes affected by this plan.
    pub affected_nodes: Vec<NodeId>,
    /// Symbols affected by this plan.
    pub affected_symbols: Vec<SymbolId>,
    /// Expected effect summary.
    pub expected_effect: CandidateEffect,
}

impl EditPlan {
    /// Converts the plan to one low-level edit.
    pub fn as_edit(&self) -> Edit {
        if self.edits.len() == 1 {
            self.edits[0].clone()
        } else {
            Edit::Multi(self.edits.clone())
        }
    }
}
