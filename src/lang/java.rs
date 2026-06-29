//! Java language adapter.
//!
//! Java has stricter structural rules than JavaScript, so this adapter avoids
//! obvious invalid edits such as deleting the only public top-level class. Other
//! candidates still rely on the oracle to catch type errors, definite-assignment
//! failures, and classpath/package issues.

use crate::{
    edit::{Edit, TextRange},
    ir::{ComplexityScore, ProgramIndex, SnapshotId},
    lang::{
        LanguageAdapter, NodeSummary, ParsedProgram,
        tree_sitter_common::{
            child_of_kind, delete_node_with_padding, parse_with_language, preorder_nodes,
            subtree_contains_kind,
        },
    },
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
        ordering::normalize_candidates,
    },
};

/// Java adapter backed by tree-sitter.
#[derive(Clone, Debug, Default)]
pub struct JavaAdapter;

impl JavaAdapter {
    /// Creates a Java adapter.
    pub fn new() -> Self {
        Self
    }
}

impl LanguageAdapter for JavaAdapter {
    fn language_id(&self) -> &'static str {
        "java"
    }

    fn parse(&self, source: &str, file_name: &str) -> anyhow::Result<ParsedProgram> {
        parse_with_language(tree_sitter_java::LANGUAGE.into(), source, file_name)
    }

    fn generate_groups(
        &self,
        stage: StageKind,
        parsed: &ParsedProgram,
        _index: &ProgramIndex,
        _score: &ComplexityScore,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        Ok(java_groups_for_stage(parsed, stage))
    }
}

/// Generates one Java syntax-aware group for an algorithmic stage.
fn java_groups_for_stage(parsed: &ParsedProgram, stage: StageKind) -> Vec<CandidateGroup> {
    let candidates = normalize_candidates(java_candidates_for_stage(parsed, stage));
    if candidates.is_empty() {
        return Vec::new();
    }

    let Some((kind, strategy, priority, description)) = java_group_config(stage) else {
        return Vec::new();
    };

    vec![CandidateGroup::new(
        format!("java:{}:syntax", stage.as_str()),
        SnapshotId::ROOT,
        stage,
        kind,
        description,
        candidates,
        strategy,
        priority,
    )]
}

/// Returns group metadata for Java syntax-aware candidates.
fn java_group_config(
    stage: StageKind,
) -> Option<(CandidateGroupKind, GroupStrategy, i32, &'static str)> {
    Some(match stage {
        StageKind::AggressiveFunctionElimination => (
            CandidateGroupKind::DeclarationFamily,
            GroupStrategy::WholeThenChunksThenSingles,
            280,
            "Java function and type declaration simplifications",
        ),
        StageKind::AggressiveBlockElimination => (
            CandidateGroupKind::ControlPathSet,
            GroupStrategy::Alternatives,
            260,
            "Java block and control-flow simplifications",
        ),
        StageKind::StatementAndSiblingReduction => (
            CandidateGroupKind::SiblingList,
            GroupStrategy::WholeThenChunksThenSingles,
            220,
            "Java statement reductions",
        ),
        StageKind::DeadDeclarationAndOutputCleanup => (
            CandidateGroupKind::OutputNoise,
            GroupStrategy::WholeThenChunksThenSingles,
            240,
            "Java dead declaration and output cleanup",
        ),
        StageKind::ExpressionLiteralTypeCleanup => (
            CandidateGroupKind::LiteralShrinkSet,
            GroupStrategy::Alternatives,
            160,
            "Java expression, literal, import, and trivia cleanup",
        ),
        StageKind::RuntimeCostReduction
        | StageKind::BaselineAndIndex
        | StageKind::FinalOneMinimalSweep
        | StageKind::BlankLineCleanup => return None,
    })
}

/// Generates Java candidates for one algorithmic stage.
fn java_candidates_for_stage(parsed: &ParsedProgram, stage: StageKind) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let nodes = preorder_nodes(&parsed.root);
    let public_top_level_class_count = parsed
        .root
        .children
        .iter()
        .filter(|node| node.kind == "class_declaration" && is_public_class(node))
        .count();

    for node in nodes {
        match stage {
            StageKind::AggressiveFunctionElimination
                if is_java_function_like_declaration(&node.kind) =>
            {
                if node.kind == "method_declaration"
                    && method_name(node, &parsed.source) == Some("main")
                {
                    continue;
                }
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
            StageKind::AggressiveFunctionElimination if is_java_top_level_type(&node.kind) => {
                if !is_direct_child_of_program(&parsed.root, node) {
                    continue;
                }
                if !(node.kind == "class_declaration"
                    && is_public_class(node)
                    && public_top_level_class_count <= 1)
                {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::DeleteDeadDeclaration,
                        node,
                        "Delete top-level type declaration",
                        120,
                    );
                }
            }
            StageKind::AggressiveBlockElimination if is_empty_java_loop_or_control(node) => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::DeleteWholeConstruct,
                    node,
                    "Delete empty control-flow statement",
                    260,
                );
            }
            StageKind::AggressiveBlockElimination if node.kind == "block" => {
                if let Some(parent_method) = find_method_parent(&parsed.root, &node.range) {
                    for replacement in method_body_replacements(parent_method, &parsed.source) {
                        candidates.push(replace_candidate(
                            stage,
                            TransformKind::EmptyBlockBody,
                            node,
                            &replacement,
                            "Replace method body with a minimal body",
                            220,
                        ));
                    }
                } else {
                    candidates.push(replace_candidate(
                        stage,
                        TransformKind::EmptyBlockBody,
                        node,
                        "{}",
                        "Replace block with an empty block",
                        200,
                    ));
                }
            }
            StageKind::AggressiveBlockElimination if is_java_control_flow(&node.kind) => {
                candidates.push(replace_candidate(
                    stage,
                    TransformKind::DeleteWholeConstruct,
                    node,
                    "{}",
                    "Replace control-flow statement with an empty block",
                    250,
                ));
            }
            StageKind::StatementAndSiblingReduction if is_java_statement(&node.kind) => {
                if is_java_output_statement(node, &parsed.source) {
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
                if is_java_output_statement(node, &parsed.source) =>
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
            StageKind::DeadDeclarationAndOutputCleanup
                if is_java_member_or_local_declaration(&node.kind) =>
            {
                if is_java_top_level_type(&node.kind)
                    && is_direct_child_of_program(&parsed.root, node)
                {
                    continue;
                }
                if node.kind == "local_variable_declaration"
                    || should_skip_referenced_declaration(parsed, node)
                {
                    continue;
                }
                if node.kind == "method_declaration"
                    && method_name(node, &parsed.source) == Some("main")
                {
                    continue;
                }
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
            StageKind::DeadDeclarationAndOutputCleanup
                if node.kind == "method_declaration" && is_empty_java_method(node) =>
            {
                if method_name(node, &parsed.source) != Some("main")
                    && !should_skip_referenced_declaration(parsed, node)
                {
                    push_delete(
                        &mut candidates,
                        parsed,
                        stage,
                        TransformKind::DeleteFunctionDecl,
                        node,
                        "Delete empty method declaration",
                        250,
                    );
                }
            }
            StageKind::DeadDeclarationAndOutputCleanup if is_empty_java_loop_or_control(node) => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::DeleteWholeConstruct,
                    node,
                    "Delete empty control-flow statement",
                    115,
                );
            }
            StageKind::DeadDeclarationAndOutputCleanup
                if is_deletable_empty_java_block(&parsed.root, node) =>
            {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::Cleanup,
                    node,
                    "Delete empty block statement",
                    130,
                );
            }
            StageKind::ExpressionLiteralTypeCleanup
                if matches!(node.kind.as_str(), "line_comment" | "block_comment") =>
            {
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
            StageKind::ExpressionLiteralTypeCleanup if node.kind == "import_declaration" => {
                push_delete(
                    &mut candidates,
                    parsed,
                    stage,
                    TransformKind::Cleanup,
                    node,
                    "Delete import declaration",
                    160,
                );
            }
            StageKind::ExpressionLiteralTypeCleanup if is_java_expression(&node.kind) => {
                for replacement in ["0", "false", "null", "\"\""] {
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
            StageKind::ExpressionLiteralTypeCleanup if is_java_literal(&node.kind) => {
                for replacement in java_literal_replacements(&node.kind) {
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

/// Returns true when a declaration is likely still referenced.
///
/// This conservative filter avoids spending many oracle trials on declarations
/// that are guaranteed to fail compilation when deleted alone. Statement and
/// block stages can still delete local variables together with their uses.
fn should_skip_referenced_declaration(parsed: &ParsedProgram, node: &NodeSummary) -> bool {
    if matches!(node.kind.as_str(), "constructor_declaration") {
        return true;
    }

    let Some(name) = declaration_identifier(node, &parsed.source) else {
        return false;
    };
    count_identifier_occurrences(&parsed.source, name) > 1
}

/// Extracts the main declared identifier for simple declarations.
fn declaration_identifier<'a>(node: &'a NodeSummary, source: &'a str) -> Option<&'a str> {
    if node.kind == "method_declaration" {
        return method_name(node, source);
    }

    find_identifier(node).map(|identifier| identifier.text(source))
}

/// Finds the first identifier descendant in source order.
fn find_identifier(node: &NodeSummary) -> Option<&NodeSummary> {
    if node.kind == "identifier" {
        return Some(node);
    }
    node.children.iter().find_map(find_identifier)
}

/// Counts whole Java identifier occurrences in source text.
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
            !before.is_some_and(is_java_identifier_part)
                && !after.is_some_and(is_java_identifier_part)
        })
        .count()
}

/// Returns true when a character can be part of a Java identifier.
fn is_java_identifier_part(ch: char) -> bool {
    ch == '_' || ch == '$' || ch.is_ascii_alphanumeric()
}

/// Adds a deletion candidate for a Java node.
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

/// Creates a replacement candidate for Java.
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

/// Returns a stable candidate id.
fn candidate_id(transform: TransformKind, node: &NodeSummary, operation: &str) -> String {
    format!(
        "{}:{}:{}:{}",
        transform.as_str(),
        node.range.start,
        node.range.end,
        operation
    )
}

/// Returns true for Java function-like declarations.
fn is_java_function_like_declaration(kind: &str) -> bool {
    matches!(kind, "method_declaration" | "constructor_declaration")
}

/// Returns true for Java top-level type declarations.
fn is_java_top_level_type(kind: &str) -> bool {
    matches!(
        kind,
        "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "annotation_type_declaration"
    )
}

/// Returns true for declaration nodes that can often be deleted.
fn is_java_member_or_local_declaration(kind: &str) -> bool {
    matches!(
        kind,
        "field_declaration"
            | "method_declaration"
            | "constructor_declaration"
            | "local_variable_declaration"
            | "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
    )
}

/// Returns true for executable Java statements.
fn is_java_statement(kind: &str) -> bool {
    matches!(
        kind,
        "expression_statement"
            | "return_statement"
            | "throw_statement"
            | "if_statement"
            | "for_statement"
            | "enhanced_for_statement"
            | "while_statement"
            | "do_statement"
            | "try_statement"
            | "switch_expression"
            | "switch_statement"
            | "assert_statement"
            | "break_statement"
            | "continue_statement"
            | "synchronized_statement"
            | "local_variable_declaration"
    )
}

/// Returns true for Java stdout/stderr print statements that fuzzers often emit as noise.
fn is_java_output_statement(node: &NodeSummary, source: &str) -> bool {
    if node.kind != "expression_statement" {
        return false;
    }

    let text = node.text(source);
    text.contains("System.out.print")
        || text.contains("System.err.print")
        || text.contains("java.lang.System.out.print")
        || text.contains("java.lang.System.err.print")
        || text.contains("FuzzerUtils.out.print")
        || text.contains("FuzzerUtils.err.print")
        || text.contains(".printStackTrace(System.err)")
        || text.contains(".printStackTrace(System.out)")
        || text.contains(".printStackTrace(FuzzerUtils.err)")
        || text.contains(".printStackTrace(FuzzerUtils.out)")
}

/// Returns true for Java control-flow constructs.
fn is_java_control_flow(kind: &str) -> bool {
    matches!(
        kind,
        "if_statement"
            | "for_statement"
            | "enhanced_for_statement"
            | "while_statement"
            | "do_statement"
            | "try_statement"
            | "switch_expression"
            | "switch_statement"
            | "synchronized_statement"
    )
}

/// Returns true for method declarations whose block contains no real statements.
fn is_empty_java_method(node: &NodeSummary) -> bool {
    if node.kind != "method_declaration" {
        return false;
    }
    child_of_kind(node, "block").is_some_and(block_has_no_statements)
}

/// Returns true for empty loops, if statements, try statements, and synchronized blocks.
fn is_empty_java_loop_or_control(node: &NodeSummary) -> bool {
    is_java_control_flow(&node.kind) && node.children.iter().any(block_has_no_statements)
}

/// Returns true when a Java block has no statement-like direct children.
fn block_has_no_statements(node: &NodeSummary) -> bool {
    if node.kind != "block" {
        return false;
    }
    !node.children.iter().any(|child| {
        is_java_statement(&child.kind)
            || is_java_control_flow(&child.kind)
            || child.kind == "local_variable_declaration"
    })
}

/// Returns true for empty block statements that can be tried as standalone deletions.
///
/// Java requires blocks in method bodies, constructors, class declarations,
/// catch clauses, and many control-flow constructs. The only low-risk empty
/// block deletion is a nested block whose direct parent is another executable
/// block, because Java treats that nested block as a statement.
fn is_deletable_empty_java_block(root: &NodeSummary, node: &NodeSummary) -> bool {
    node.kind == "block"
        && block_has_no_statements(node)
        && parent_of_range(root, &node.range).is_some_and(|parent| parent.kind == "block")
}

/// Finds the direct parent whose child has the requested range.
fn parent_of_range<'a>(root: &'a NodeSummary, range: &TextRange) -> Option<&'a NodeSummary> {
    if root.children.iter().any(|child| child.range == *range) {
        return Some(root);
    }

    root.children
        .iter()
        .find_map(|child| parent_of_range(child, range))
}

/// Returns true for expression nodes where simple replacements are worth testing.
fn is_java_expression(kind: &str) -> bool {
    matches!(
        kind,
        "field_access"
            | "method_invocation"
            | "binary_expression"
            | "unary_expression"
            | "assignment_expression"
            | "ternary_expression"
            | "parenthesized_expression"
            | "array_access"
            | "object_creation_expression"
            | "cast_expression"
            | "decimal_integer_literal"
            | "decimal_floating_point_literal"
            | "string_literal"
            | "character_literal"
            | "true"
            | "false"
            | "null_literal"
    )
}

/// Returns true for Java literal nodes.
fn is_java_literal(kind: &str) -> bool {
    matches!(
        kind,
        "decimal_integer_literal"
            | "decimal_floating_point_literal"
            | "hex_integer_literal"
            | "octal_integer_literal"
            | "binary_integer_literal"
            | "string_literal"
            | "character_literal"
            | "true"
            | "false"
            | "null_literal"
    )
}

/// Returns replacement literals suitable for the Java literal kind.
fn java_literal_replacements(kind: &str) -> &'static [&'static str] {
    match kind {
        "decimal_integer_literal"
        | "hex_integer_literal"
        | "octal_integer_literal"
        | "binary_integer_literal" => &["0", "1"],
        "decimal_floating_point_literal" => &["0.0"],
        "string_literal" => &["\"\""],
        "character_literal" => &["'\\0'"],
        "true" | "false" => &["false", "true"],
        "null_literal" => &["null"],
        _ => &[],
    }
}

/// Returns true when a class declaration contains a public modifier.
fn is_public_class(node: &NodeSummary) -> bool {
    node.children
        .iter()
        .find(|child| child.kind == "modifiers")
        .is_some_and(|modifiers| subtree_contains_kind(modifiers, "public"))
}

/// Returns true when a Java declaration is a direct child of the program root.
fn is_direct_child_of_program(root: &NodeSummary, node: &NodeSummary) -> bool {
    root.kind == "program" && root.children.iter().any(|child| child.range == node.range)
}

/// Returns a simple method name when available.
fn method_name<'a>(node: &'a NodeSummary, source: &'a str) -> Option<&'a str> {
    child_of_kind(node, "identifier")
        .or_else(|| child_of_kind(node, "type_identifier"))
        .map(|child| child.text(source))
}

/// Finds the method declaration whose body is the target block.
fn find_method_parent<'a>(
    root: &'a NodeSummary,
    target_range: &TextRange,
) -> Option<&'a NodeSummary> {
    if root.kind == "method_declaration"
        && root
            .children
            .iter()
            .any(|child| child.kind == "block" && child.range == *target_range)
    {
        return Some(root);
    }

    for child in &root.children {
        if let Some(found) = find_method_parent(child, target_range) {
            return Some(found);
        }
    }
    None
}

/// Creates minimal legal method bodies based on the return type text.
fn method_body_replacements(method: &NodeSummary, source: &str) -> Vec<String> {
    let return_type = method
        .children
        .iter()
        .find(|child| {
            matches!(
                child.kind.as_str(),
                "void_type"
                    | "integral_type"
                    | "floating_point_type"
                    | "boolean_type"
                    | "type_identifier"
                    | "scoped_type_identifier"
                    | "generic_type"
                    | "array_type"
            )
        })
        .map(|child| child.text(source).trim().to_owned())
        .unwrap_or_else(|| "void".to_owned());

    if return_type == "void" {
        return vec!["{}".to_owned()];
    }
    if return_type == "boolean" {
        return vec!["{ return false; }".to_owned(), "{}".to_owned()];
    }
    if return_type == "char" {
        return vec!["{ return '\\0'; }".to_owned(), "{}".to_owned()];
    }
    if matches!(
        return_type.as_str(),
        "byte" | "short" | "int" | "long" | "float" | "double"
    ) {
        return vec!["{ return 0; }".to_owned(), "{}".to_owned()];
    }

    vec!["{ return null; }".to_owned(), "{}".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidates_for_stage(
        adapter: &JavaAdapter,
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
    fn java_adapter_parses_valid_source() {
        let adapter = JavaAdapter::new();
        let parsed = adapter
            .parse(
                "public class Test { static int f(){ return 1; } }\n",
                "Test.java",
            )
            .unwrap();
        assert!(parsed.diagnostics.is_empty());
        assert_eq!(parsed.root.kind, "program");
    }

    #[test]
    fn java_adapter_generates_import_candidates() {
        let adapter = JavaAdapter::new();
        let parsed = adapter
            .parse("import java.util.*; public class Test {}\n", "Test.java")
            .unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::ExpressionLiteralTypeCleanup);
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.description == "Delete import declaration")
        );
    }

    #[test]
    fn java_adapter_prioritizes_output_statement_deletion() {
        let adapter = JavaAdapter::new();
        let parsed = adapter
            .parse(
                "class Test { static void f(){ System.out.println(\"noise\"); int keep = 1; } }\n",
                "Test.java",
            )
            .unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::StatementAndSiblingReduction);
        let output_candidate = candidates
            .iter()
            .find(|candidate| candidate.description == "Delete output statement")
            .expect("Java adapter should generate an output statement deletion candidate");

        assert!(
            output_candidate.priority > 100,
            "Output statement deletion should be prioritized before ordinary statements"
        );
    }

    #[test]
    fn java_output_statement_deletion_stays_parsable_and_simpler() {
        let adapter = JavaAdapter::new();
        let source = "public class Test { public static void main(String[] args) { int keep = 1; System.out.println(keep); } }\n";
        let parsed = adapter.parse(source, "Test.java").unwrap();
        let before_index = adapter.build_index(&parsed).unwrap();
        let before_score = crate::ir::ComplexityScore::compute(&before_index, source);
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::StatementAndSiblingReduction);
        let output_candidate = candidates
            .iter()
            .find(|candidate| candidate.description == "Delete output statement")
            .expect("Java adapter should generate a deletable output statement");

        let reduced = output_candidate.edit.apply(source).unwrap();
        let reduced_parsed = adapter.parse(&reduced, "Test.java").unwrap();
        assert!(
            reduced_parsed.diagnostics.is_empty(),
            "Deleting a Java output statement must preserve parse validity"
        );
        let after_index = adapter.build_index(&reduced_parsed).unwrap();
        let after_score = crate::ir::ComplexityScore::compute(&after_index, &reduced);
        assert!(
            after_score.is_strictly_less_than(&before_score),
            "Deleting a Java output statement must reduce structural complexity"
        );
    }

    #[test]
    fn java_method_body_replacement_uses_return_type() {
        let adapter = JavaAdapter::new();
        let parsed = adapter
            .parse(
                "class Test { static boolean f(){ return true; } }\n",
                "Test.java",
            )
            .unwrap();
        let candidates =
            candidates_for_stage(&adapter, &parsed, StageKind::AggressiveBlockElimination);
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.description == "Replace method body with a minimal body")
        );
    }

    #[test]
    fn java_cleanup_deletes_empty_nested_block_statements() {
        let adapter = JavaAdapter::new();
        let source = "class Test { static void f(){ {} int keep = 1; } }\n";
        let parsed = adapter.parse(source, "Test.java").unwrap();
        let candidates = candidates_for_stage(
            &adapter,
            &parsed,
            StageKind::DeadDeclarationAndOutputCleanup,
        );
        let empty_block = candidates
            .iter()
            .find(|candidate| candidate.description == "Delete empty block statement")
            .expect("Java cleanup should generate a candidate for a nested empty block statement");
        let reduced = empty_block.edit.apply(source).unwrap();

        assert!(
            !reduced.contains("{}"),
            "Deleting the nested empty block should remove the empty block text"
        );
        assert!(
            adapter
                .parse(&reduced, "Test.java")
                .unwrap()
                .diagnostics
                .is_empty(),
            "Deleting a nested empty block statement should preserve parse validity"
        );
    }
}
