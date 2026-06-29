//! JavaScript language adapter.
//!
//! The adapter uses tree-sitter-javascript to generate syntax-aware candidates.
//! Candidates are intentionally optimistic: static analysis proposes plausible
//! deletions or replacements, and the oracle decides whether runtime behavior
//! still preserves the A/B diff.

use crate::{
    edit::{Edit, TextRange},
    ir::{ComplexityScore, ProgramIndex, SnapshotId},
    lang::{
        LanguageAdapter, NodeSummary, ParsedProgram,
        tree_sitter_common::{delete_node_with_padding, parse_with_language, preorder_nodes},
    },
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
        ordering::normalize_candidates,
    },
};

/// JavaScript adapter backed by tree-sitter.
#[derive(Clone, Debug, Default)]
pub struct JavaScriptAdapter;

impl JavaScriptAdapter {
    /// Creates a JavaScript adapter.
    pub fn new() -> Self {
        Self
    }
}

impl LanguageAdapter for JavaScriptAdapter {
    fn language_id(&self) -> &'static str {
        "javascript"
    }

    fn parse(&self, source: &str, file_name: &str) -> anyhow::Result<ParsedProgram> {
        parse_with_language(tree_sitter_javascript::LANGUAGE.into(), source, file_name)
    }

    fn generate_groups(
        &self,
        stage: StageKind,
        parsed: &ParsedProgram,
        _index: &ProgramIndex,
        _score: &ComplexityScore,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        Ok(js_groups_for_stage(parsed, stage))
    }
}

/// Generates one JavaScript syntax-aware group for an algorithmic stage.
fn js_groups_for_stage(parsed: &ParsedProgram, stage: StageKind) -> Vec<CandidateGroup> {
    let candidates = normalize_candidates(js_candidates_for_stage(parsed, stage));
    if candidates.is_empty() {
        return Vec::new();
    }

    let Some((kind, strategy, priority, description)) = js_group_config(stage) else {
        return Vec::new();
    };

    vec![CandidateGroup::new(
        format!("javascript:{}:syntax", stage.as_str()),
        SnapshotId::ROOT,
        stage,
        kind,
        description,
        candidates,
        strategy,
        priority,
    )]
}

/// Returns group metadata for JavaScript syntax-aware candidates.
fn js_group_config(
    stage: StageKind,
) -> Option<(CandidateGroupKind, GroupStrategy, i32, &'static str)> {
    Some(match stage {
        StageKind::AggressiveFunctionElimination => (
            CandidateGroupKind::DeclarationFamily,
            GroupStrategy::SinglesOnly,
            150,
            "JavaScript unreferenced function and top-level declaration simplifications",
        ),
        StageKind::AggressiveBlockElimination => (
            CandidateGroupKind::ControlPathSet,
            GroupStrategy::Alternatives,
            260,
            "JavaScript block and control-flow simplifications",
        ),
        StageKind::StatementAndSiblingReduction => (
            CandidateGroupKind::SiblingList,
            GroupStrategy::WholeThenChunksThenSingles,
            220,
            "JavaScript statement reductions",
        ),
        StageKind::DeadDeclarationAndOutputCleanup => (
            CandidateGroupKind::OutputNoise,
            GroupStrategy::WholeThenChunksThenSingles,
            240,
            "JavaScript dead declaration and output cleanup",
        ),
        StageKind::ExpressionLiteralTypeCleanup => (
            CandidateGroupKind::LiteralShrinkSet,
            GroupStrategy::Alternatives,
            160,
            "JavaScript expression, literal, and trivia cleanup",
        ),
        StageKind::RuntimeCostReduction
        | StageKind::BaselineAndIndex
        | StageKind::FinalOneMinimalSweep
        | StageKind::BlankLineCleanup => return None,
    })
}

/// Generates JavaScript candidates for one algorithmic stage.
fn js_candidates_for_stage(parsed: &ParsedProgram, stage: StageKind) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let nodes = preorder_nodes(&parsed.root);

    for node in nodes {
        match stage {
            StageKind::AggressiveFunctionElimination
                if is_js_function_like_declaration(&node.kind) =>
            {
                if !should_skip_referenced_declaration(parsed, node) {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::DeleteFunctionDecl,
                        node,
                        "Delete function declaration",
                        180,
                    );
                }
            }
            StageKind::AggressiveFunctionElimination if is_js_top_level_declaration(&node.kind) => {
                if is_direct_child_of_program(&parsed.root, node)
                    && !should_skip_referenced_declaration(parsed, node)
                {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::DeleteDeadDeclaration,
                        node,
                        "Delete top-level declaration",
                        120,
                    );
                }
            }
            StageKind::AggressiveBlockElimination if node.kind == "statement_block" => {
                candidates.push(replace_candidate(
                    stage,
                    TransformKind::EmptyBlockBody,
                    node,
                    "{}",
                    "Replace statement block with an empty block",
                    220,
                ));
            }
            StageKind::AggressiveBlockElimination if is_js_control_flow(&node.kind) => {
                candidates.push(replace_candidate(
                    stage,
                    TransformKind::DeleteWholeConstruct,
                    node,
                    ";",
                    "Replace control-flow statement with an empty statement",
                    250,
                ));
            }
            StageKind::StatementAndSiblingReduction if is_js_statement(&node.kind) => {
                if is_js_output_statement(node, &parsed.source) {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::RemoveOutputOnlyVariable,
                        node,
                        "Delete output statement",
                        300,
                    );
                } else {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::DeleteStatementChunk,
                        node,
                        "Delete statement",
                        70,
                    );
                }
            }
            StageKind::DeadDeclarationAndOutputCleanup
                if is_js_output_statement(node, &parsed.source) =>
            {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::RemoveOutputOnlyVariable,
                    node,
                    "Delete output statement",
                    300,
                );
            }
            StageKind::DeadDeclarationAndOutputCleanup if is_js_declaration(&node.kind) => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::DeleteDeadDeclaration,
                    node,
                    "Delete declaration",
                    230,
                );
            }
            StageKind::ExpressionLiteralTypeCleanup if node.kind == "comment" => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::Cleanup,
                    node,
                    "Delete comment",
                    120,
                );
            }
            StageKind::ExpressionLiteralTypeCleanup if is_js_expression(&node.kind) => {
                for replacement in ["undefined", "null", "false", "0", "\"\"", "[]", "{}"] {
                    candidates.push(replace_candidate(
                        stage,
                        TransformKind::ReplaceCallWithValue,
                        node,
                        replacement,
                        "Replace expression with a simple expression",
                        45,
                    ));
                }
            }
            StageKind::ExpressionLiteralTypeCleanup if is_js_literal(&node.kind) => {
                for replacement in js_literal_replacements(&node.kind) {
                    candidates.push(replace_candidate(
                        stage,
                        TransformKind::ShrinkLiteral,
                        node,
                        replacement,
                        "Shorten literal",
                        40,
                    ));
                }
            }
            StageKind::ExpressionLiteralTypeCleanup if node.kind == "empty_statement" => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::Cleanup,
                    node,
                    "Delete empty statement",
                    35,
                );
            }
            _ => {}
        }
    }

    candidates
}

/// Adds a deletion candidate for a node.
fn push_delete(
    candidates: &mut Vec<Candidate>,
    parsed: &ParsedProgram,
    stage: StageKind,
    transform: TransformKind,
    node: &NodeSummary,
    description: &str,
    priority: i32,
) {
    let edit = delete_node_with_padding(&parsed.source, node);
    let removed = edit.removed_bytes() as isize;
    candidates.push(Candidate::new(
        stage,
        transform,
        candidate_id(transform, node, "delete"),
        description,
        edit,
        priority,
        -removed,
    ));
}

/// Creates a replacement candidate.
fn replace_candidate(
    stage: StageKind,
    transform: TransformKind,
    node: &NodeSummary,
    replacement: &str,
    description: &str,
    priority: i32,
) -> Candidate {
    let range = TextRange {
        start: node.range.start,
        end: node.range.end,
    };
    let delta = replacement.len() as isize - range.len() as isize;
    Candidate::new(
        stage,
        transform,
        candidate_id(transform, node, replacement),
        description,
        Edit::Replace {
            range,
            replacement: replacement.to_owned(),
        },
        priority,
        delta,
    )
}

/// Returns a stable candidate id based on transform, range, and operation.
fn candidate_id(transform: TransformKind, node: &NodeSummary, operation: &str) -> String {
    format!(
        "{}:{}:{}:{}",
        transform.as_str(),
        node.range.start,
        node.range.end,
        operation
    )
}

/// Returns true when a JavaScript declaration is likely still referenced.
///
/// Adapter-level declaration deletion is a late conservative cleanup path. When
/// a declaration name appears elsewhere as a whole identifier, deleting the
/// declaration alone usually creates a predictable runtime or reference error,
/// so the reducer leaves that code for statement/expression stages instead.
fn should_skip_referenced_declaration(parsed: &ParsedProgram, node: &NodeSummary) -> bool {
    let Some(name) = declaration_identifier(node, &parsed.source) else {
        return false;
    };
    count_identifier_occurrences(&parsed.source, name) > 1
}

/// Extracts the primary declared name for simple JavaScript declarations.
fn declaration_identifier<'a>(node: &'a NodeSummary, source: &'a str) -> Option<&'a str> {
    find_identifier(node).map(|identifier| identifier.text(source))
}

/// Finds the first identifier descendant in source order.
fn find_identifier(node: &NodeSummary) -> Option<&NodeSummary> {
    if node.kind == "identifier" {
        return Some(node);
    }
    node.children.iter().find_map(find_identifier)
}

/// Counts whole JavaScript identifier occurrences in source text.
fn count_identifier_occurrences(source: &str, name: &str) -> usize {
    if name.is_empty() {
        return 0;
    }

    source
        .match_indices(name)
        .filter(|(start, _)| {
            let end = start + name.len();
            let before = source[..*start].chars().next_back();
            let after = source[end..].chars().next();
            !before.is_some_and(is_js_identifier_part) && !after.is_some_and(is_js_identifier_part)
        })
        .count()
}

/// Returns true when a character can be part of a JavaScript identifier.
fn is_js_identifier_part(ch: char) -> bool {
    ch == '_' || ch == '$' || ch.is_ascii_alphanumeric()
}

/// Returns true for JavaScript function-like declarations.
fn is_js_function_like_declaration(kind: &str) -> bool {
    matches!(kind, "function_declaration" | "method_definition")
}

/// Returns true for declarations that can appear at program top level.
fn is_js_top_level_declaration(kind: &str) -> bool {
    matches!(
        kind,
        "function_declaration"
            | "class_declaration"
            | "lexical_declaration"
            | "variable_declaration"
            | "import_statement"
            | "export_statement"
    )
}

/// Returns true for declaration-like nodes in JavaScript.
fn is_js_declaration(kind: &str) -> bool {
    matches!(
        kind,
        "function_declaration"
            | "class_declaration"
            | "lexical_declaration"
            | "variable_declaration"
            | "import_statement"
            | "export_statement"
    )
}

/// Returns true for executable statement nodes.
fn is_js_statement(kind: &str) -> bool {
    matches!(
        kind,
        "expression_statement"
            | "return_statement"
            | "throw_statement"
            | "if_statement"
            | "for_statement"
            | "for_in_statement"
            | "while_statement"
            | "do_statement"
            | "try_statement"
            | "switch_statement"
            | "break_statement"
            | "continue_statement"
            | "debugger_statement"
    )
}

/// Returns true for JavaScript console output statements that are often removable noise.
fn is_js_output_statement(node: &NodeSummary, source: &str) -> bool {
    if node.kind != "expression_statement" {
        return false;
    }

    let text = node.text(source);
    text.contains("console.log")
        || text.contains("console.error")
        || text.contains("console.warn")
        || text.contains("process.stdout.write")
        || text.contains("process.stderr.write")
}

/// Returns true for control-flow statements worth replacing as a whole.
fn is_js_control_flow(kind: &str) -> bool {
    matches!(
        kind,
        "if_statement"
            | "for_statement"
            | "for_in_statement"
            | "while_statement"
            | "do_statement"
            | "try_statement"
            | "switch_statement"
    )
}

/// Returns true for expression nodes where simple replacement is usually syntactically legal.
fn is_js_expression(kind: &str) -> bool {
    matches!(
        kind,
        "identifier"
            | "member_expression"
            | "call_expression"
            | "binary_expression"
            | "unary_expression"
            | "assignment_expression"
            | "augmented_assignment_expression"
            | "ternary_expression"
            | "parenthesized_expression"
            | "subscript_expression"
            | "new_expression"
            | "array"
            | "object"
            | "string"
            | "number"
            | "true"
            | "false"
            | "null"
            | "undefined"
    )
}

/// Returns true for JavaScript literal nodes.
fn is_js_literal(kind: &str) -> bool {
    matches!(
        kind,
        "string" | "template_string" | "number" | "true" | "false" | "null" | "array" | "object"
    )
}

/// Returns replacement literals suitable for a node kind.
fn js_literal_replacements(kind: &str) -> &'static [&'static str] {
    match kind {
        "string" | "template_string" => &["\"\""],
        "number" => &["0", "1"],
        "true" | "false" => &["false", "true"],
        "null" => &["null"],
        "array" => &["[]"],
        "object" => &["{}"],
        _ => &[],
    }
}

/// Returns true when a node is a direct child of the program root.
fn is_direct_child_of_program(root: &NodeSummary, node: &NodeSummary) -> bool {
    root.kind == "program" && root.children.iter().any(|child| child.range == node.range)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidates_for_stage(
        adapter: &JavaScriptAdapter,
        parsed: &ParsedProgram,
        stage: StageKind,
    ) -> Vec<Candidate> {
        let index = adapter.build_index(parsed).unwrap();
        let score = crate::ir::ComplexityScore::compute(&index, &parsed.source);
        adapter
            .generate_groups(stage, parsed, &index, &score)
            .unwrap()
            .into_iter()
            .flat_map(|group| group.candidates)
            .collect()
    }

    #[test]
    fn javascript_adapter_parses_valid_source() {
        let adapter = JavaScriptAdapter::new();
        let parsed = adapter
            .parse("function f(){ return 1; }\n", "case.js")
            .unwrap();
        assert!(parsed.diagnostics.is_empty());
        assert_eq!(parsed.root.kind, "program");
    }

    #[test]
    fn javascript_adapter_generates_statement_candidates() {
        let adapter = JavaScriptAdapter::new();
        let parsed = adapter
            .parse("function f(){ let x = 1; return x; }\n", "case.js")
            .unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::StatementAndSiblingReduction);
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.description == "Delete statement")
        );
    }

    #[test]
    fn javascript_adapter_prioritizes_output_statement_deletion() {
        let adapter = JavaScriptAdapter::new();
        let parsed = adapter
            .parse(
                "function f(){ console.log('noise'); let keep = 1; }\n",
                "case.js",
            )
            .unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::StatementAndSiblingReduction);
        let output_candidate = candidates
            .iter()
            .find(|candidate| candidate.description == "Delete output statement")
            .expect("JavaScript adapter should generate an output statement deletion candidate");

        assert!(
            output_candidate.priority > 100,
            "Output statement deletion should be prioritized before ordinary statements"
        );
    }

    #[test]
    fn javascript_function_stage_skips_referenced_functions() {
        let adapter = JavaScriptAdapter::new();
        let source = "function helper(){ return 1; }\nfunction main(){ return helper(); }\n";
        let parsed = adapter.parse(source, "case.js").unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::AggressiveFunctionElimination);

        assert!(
            candidates.iter().all(|candidate| {
                candidate.description != "Delete function declaration"
                    || !matches!(
                        &candidate.edit,
                        Edit::Delete(range) if source[range.start..range.end].contains("function helper")
                    )
            }),
            "Referenced JavaScript functions should not be aggressive declaration-deletion candidates"
        );
    }
}
