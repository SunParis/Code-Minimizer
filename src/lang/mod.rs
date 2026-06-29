//! Language adapter registry and language-neutral parse structures.
//!
//! Reducer logic depends only on this module's trait and summary types. A new
//! language can join the reducer by implementing [`LanguageAdapter`] and adding
//! a registry entry.

use serde::{Deserialize, Serialize};

use crate::{
    edit::TextRange,
    error::CodeMinimizerError,
    ir::{
        Confidence, DependencyEdge, DependencyGraph, DependencyKey, DependencyKind, NodeFlags,
        NodeId, NodeKind, NodeRole, ProgramIndex, ScopeId, ScopeKind, ScopeSummary, SiblingList,
        SiblingListId, SiblingListKind,
    },
    reducer::{candidate::StageKind, group::CandidateGroup},
};

pub mod java;
pub mod javascript;
pub mod tree_sitter_common;

/// Language-specific parser and candidate generator.
pub trait LanguageAdapter: Send + Sync {
    /// Returns the stable language id used in reports.
    fn language_id(&self) -> &'static str;

    /// Parses source text and returns a language-neutral program summary.
    fn parse(&self, source: &str, file_name: &str) -> anyhow::Result<ParsedProgram>;

    /// Builds the language-neutral index used by reducer stages.
    fn build_index(&self, parsed: &ParsedProgram) -> anyhow::Result<ProgramIndex> {
        Ok(build_basic_index(parsed, self.language_id()))
    }

    /// Generates candidate groups for the given algorithmic stage.
    ///
    /// The reducer retargets returned snapshot-less groups to the current
    /// snapshot before use. Language adapters may return no groups when the
    /// language-neutral generator fully covers a stage.
    fn generate_groups(
        &self,
        _stage: StageKind,
        _parsed: &ParsedProgram,
        _index: &ProgramIndex,
        _score: &crate::ir::ComplexityScore,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        Ok(Vec::new())
    }

    /// Optionally normalizes source after an accepted edit.
    fn normalize_after_accept(&self, source: &str, _file_name: &str) -> anyhow::Result<String> {
        Ok(source.to_owned())
    }
}

/// Parsed source with both the tree-sitter tree and a lifetime-free summary.
#[derive(Debug)]
pub struct ParsedProgram {
    /// Current source text.
    pub source: String,
    /// File name used inside trial directories.
    pub file_name: String,
    /// Tree-sitter parse tree. Reducer logic should not inspect this directly.
    pub tree: Option<tree_sitter::Tree>,
    /// Language-neutral root node summary.
    pub root: NodeSummary,
    /// Parse diagnostics collected from error and missing nodes.
    pub diagnostics: Vec<ParseDiagnostic>,
}

impl ParsedProgram {
    /// Builds a synthetic parse used by reducer unit tests.
    pub fn synthetic(source: &str, file_name: &str, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self {
            source: source.to_owned(),
            file_name: file_name.to_owned(),
            tree: None,
            root: NodeSummary {
                kind: "source".into(),
                range: TextRange {
                    start: 0,
                    end: source.len(),
                },
                children: Vec::new(),
                named: true,
                has_error: !diagnostics.is_empty(),
            },
            diagnostics,
        }
    }
}

/// Lifetime-free summary of a tree-sitter node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeSummary {
    /// Tree-sitter node kind.
    pub kind: String,
    /// Byte range covered by the node.
    pub range: TextRange,
    /// Child node summaries.
    pub children: Vec<NodeSummary>,
    /// Whether tree-sitter marks this as a named node.
    pub named: bool,
    /// Whether this node or any descendant contains parse errors.
    pub has_error: bool,
}

impl NodeSummary {
    /// Returns the exact source text covered by this node.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.range.start..self.range.end]
    }

    /// Visits this node and all descendants in pre-order.
    pub fn walk_preorder<'a>(&'a self, out: &mut Vec<&'a NodeSummary>) {
        out.push(self);
        for child in &self.children {
            child.walk_preorder(out);
        }
    }

    /// Returns true when this node has an ancestor or descendant kind match in direct children.
    pub fn has_child_kind(&self, kind: &str) -> bool {
        self.children.iter().any(|child| child.kind == kind)
    }
}

/// Parse diagnostic represented without tree-sitter lifetimes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParseDiagnostic {
    /// English diagnostic message.
    pub message: String,
    /// Byte range associated with the diagnostic.
    pub range: TextRange,
}

/// Returns a registered language adapter for a user-provided language id.
pub fn adapter_for(language: &str) -> anyhow::Result<Box<dyn LanguageAdapter>> {
    match language {
        "js" | "javascript" => Ok(Box::new(javascript::JavaScriptAdapter::new())),
        "java" => Ok(Box::new(java::JavaAdapter::new())),
        other => Err(CodeMinimizerError::UnsupportedLanguage(other.to_owned()).into()),
    }
}

/// Builds a conservative language-neutral index from a parsed tree.
pub fn build_basic_index(parsed: &ParsedProgram, language_id: &str) -> ProgramIndex {
    let mut index = ProgramIndex::default();
    let root_scope = ScopeSummary {
        id: ScopeId(0),
        kind: ScopeKind::Global,
        owner: NodeId(0),
        parent: None,
        declared_symbols: Vec::new(),
    };
    index.scopes.push(root_scope);

    flatten_node(
        &parsed.root,
        None,
        0,
        0,
        ScopeId(0),
        &parsed.source,
        language_id,
        &mut index,
    );
    index.root = NodeId(0);
    rebuild_sibling_lists(&mut index);
    rebuild_containment_edges(&mut index);
    index
}

/// Recursively flattens the tree-sitter summary into snapshot-local IR nodes.
#[allow(clippy::too_many_arguments)]
fn flatten_node(
    node: &NodeSummary,
    parent: Option<NodeId>,
    depth: u32,
    sibling_index: u32,
    scope: ScopeId,
    source: &str,
    language_id: &str,
    index: &mut ProgramIndex,
) -> NodeId {
    let id = NodeId(index.nodes.len());
    let kind = classify_node_kind(language_id, &node.kind);
    let role = classify_node_role(language_id, node, source);
    let flags = NodeFlags {
        has_error: node.has_error,
        named: node.named,
        requires_replacement: matches!(
            &kind,
            NodeKind::FunctionDecl | NodeKind::ControlFlow | NodeKind::Loop
        ),
        protected: is_protected_node(language_id, node, source),
    };

    index.nodes.push(crate::ir::NodeSummary {
        id,
        kind,
        range: node.range.clone(),
        parent,
        children: Vec::new(),
        depth,
        sibling_index,
        named: node.named,
        scope: Some(scope),
        symbol: None,
        role,
        flags,
        language_kind: node.kind.clone(),
    });

    let child_scope = if opens_scope(&node.kind) {
        let new_scope = ScopeId(index.scopes.len());
        index.scopes.push(ScopeSummary {
            id: new_scope,
            kind: scope_kind_for_node(language_id, &node.kind),
            owner: id,
            parent: Some(scope),
            declared_symbols: Vec::new(),
        });
        new_scope
    } else {
        scope
    };

    for (child_index, child) in node.children.iter().enumerate() {
        let child_id = flatten_node(
            child,
            Some(id),
            depth + 1,
            child_index as u32,
            child_scope,
            source,
            language_id,
            index,
        );
        index.nodes[id.0].children.push(child_id);
    }

    if matches!(index.nodes[id.0].role, NodeRole::EntryPoint) {
        index.entry_points.push(id);
    }

    id
}

/// Maps parser node kinds into reducer node categories.
fn classify_node_kind(language_id: &str, kind: &str) -> NodeKind {
    if kind == "program" {
        return NodeKind::Program;
    }
    if matches!(kind, "line_comment" | "block_comment" | "comment") {
        return NodeKind::Trivia;
    }
    if kind.contains("import") || kind == "package_declaration" {
        return NodeKind::Import;
    }
    if matches!(
        kind,
        "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "annotation_type_declaration"
    ) {
        return NodeKind::TypeDecl;
    }
    if matches!(
        kind,
        "method_declaration"
            | "constructor_declaration"
            | "function_declaration"
            | "function"
            | "arrow_function"
            | "generator_function_declaration"
            | "method_definition"
    ) {
        return NodeKind::FunctionDecl;
    }
    if is_loop_kind(kind) {
        return NodeKind::Loop;
    }
    if is_control_flow_kind(kind) {
        return NodeKind::ControlFlow;
    }
    if matches!(
        kind,
        "block" | "statement_block" | "class_body" | "switch_block"
    ) {
        return NodeKind::Block;
    }
    if is_statement_kind(language_id, kind) {
        return NodeKind::Statement;
    }
    if kind.contains("call") || kind == "method_invocation" {
        return NodeKind::CallExpr;
    }
    if is_literal_kind(kind) {
        return NodeKind::Literal;
    }
    if kind.contains("declaration") || kind == "formal_parameter" || kind == "field_declaration" {
        return NodeKind::Declaration;
    }
    if kind.contains("expression") || kind == "identifier" || kind == "field_access" {
        return NodeKind::Expression;
    }
    NodeKind::Other
}

/// Returns a reducer role for special nodes.
fn classify_node_role(language_id: &str, node: &NodeSummary, source: &str) -> NodeRole {
    if is_output_node(language_id, node, source) {
        return NodeRole::OutputOperation;
    }
    if is_entry_node(language_id, node, source) {
        return NodeRole::EntryPoint;
    }
    if node.kind.contains("declaration") {
        return NodeRole::DeclarationOwner;
    }
    NodeRole::Ordinary
}

/// Returns true when this node opens a new lexical or structural scope.
fn opens_scope(kind: &str) -> bool {
    matches!(
        kind,
        "program"
            | "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "method_declaration"
            | "constructor_declaration"
            | "function_declaration"
            | "function"
            | "arrow_function"
            | "method_definition"
            | "block"
            | "statement_block"
    )
}

/// Maps a scope-owning node to a scope kind.
fn scope_kind_for_node(language_id: &str, kind: &str) -> ScopeKind {
    if kind == "program" {
        ScopeKind::Global
    } else if matches!(
        kind,
        "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration"
    ) || (language_id == "javascript" && kind == "class_declaration")
    {
        ScopeKind::Type
    } else if matches!(
        kind,
        "method_declaration"
            | "constructor_declaration"
            | "function_declaration"
            | "function"
            | "arrow_function"
            | "method_definition"
    ) {
        ScopeKind::Function
    } else {
        ScopeKind::Block
    }
}

/// Returns true for loop parser node kinds.
fn is_loop_kind(kind: &str) -> bool {
    matches!(
        kind,
        "for_statement"
            | "enhanced_for_statement"
            | "for_in_statement"
            | "while_statement"
            | "do_statement"
    )
}

/// Returns true for non-loop control-flow parser node kinds.
fn is_control_flow_kind(kind: &str) -> bool {
    matches!(
        kind,
        "if_statement"
            | "switch_expression"
            | "switch_statement"
            | "try_statement"
            | "synchronized_statement"
    )
}

/// Returns true for executable statement parser node kinds.
fn is_statement_kind(language_id: &str, kind: &str) -> bool {
    let common = matches!(
        kind,
        "expression_statement"
            | "return_statement"
            | "throw_statement"
            | "break_statement"
            | "continue_statement"
            | "empty_statement"
            | "debugger_statement"
    );
    common
        || (language_id == "java" && kind == "local_variable_declaration")
        || (language_id == "javascript"
            && matches!(kind, "lexical_declaration" | "variable_declaration"))
}

/// Returns true for literal parser node kinds.
fn is_literal_kind(kind: &str) -> bool {
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
            | "string"
            | "template_string"
            | "number"
            | "null"
            | "undefined"
            | "array"
            | "object"
    )
}

/// Returns true when a node is a known output operation.
fn is_output_node(language_id: &str, node: &NodeSummary, source: &str) -> bool {
    if !matches!(
        node.kind.as_str(),
        "expression_statement" | "method_invocation" | "call_expression"
    ) {
        return false;
    }

    let text = node.text(source);
    if language_id == "java" {
        text.contains("System.out.print")
            || text.contains("System.err.print")
            || text.contains("java.lang.System.out.print")
            || text.contains("java.lang.System.err.print")
            || text.contains(".printStackTrace(")
    } else {
        text.contains("console.log")
            || text.contains("console.error")
            || text.contains("console.warn")
            || text.contains("process.stdout.write")
            || text.contains("process.stderr.write")
    }
}

/// Returns true when a node appears to be an entry point.
fn is_entry_node(language_id: &str, node: &NodeSummary, source: &str) -> bool {
    if language_id == "java" && node.kind == "method_declaration" {
        let text = node.text(source);
        return text.contains("main(") || text.contains(" main ");
    }
    false
}

/// Returns true when a generic deletion should avoid this node.
fn is_protected_node(language_id: &str, node: &NodeSummary, source: &str) -> bool {
    language_id == "java"
        && node.kind == "class_declaration"
        && node.text(source).contains("public class")
}

/// Builds sibling lists for block, class, import, and top-level structures.
fn rebuild_sibling_lists(index: &mut ProgramIndex) {
    let mut lists = Vec::new();
    for node in &index.nodes {
        let items = node
            .children
            .iter()
            .copied()
            .filter(|child_id| {
                let child = &index.nodes[child_id.0];
                matches!(
                    child.kind,
                    NodeKind::Import
                        | NodeKind::TypeDecl
                        | NodeKind::FunctionDecl
                        | NodeKind::Declaration
                        | NodeKind::Statement
                        | NodeKind::ControlFlow
                        | NodeKind::Loop
                )
            })
            .collect::<Vec<_>>();
        if items.len() < 2 {
            continue;
        }

        let kind = if items
            .iter()
            .all(|id| matches!(index.nodes[id.0].kind, NodeKind::Import))
        {
            SiblingListKind::Imports
        } else if matches!(node.kind, NodeKind::Program) {
            SiblingListKind::TopLevel
        } else if node.language_kind == "class_body" {
            SiblingListKind::Members
        } else if matches!(node.kind, NodeKind::Block) {
            SiblingListKind::Statements
        } else {
            SiblingListKind::Other
        };

        lists.push(SiblingList {
            id: SiblingListId(lists.len()),
            parent: node.id,
            kind,
            items,
        });
    }
    index.sibling_lists = lists;
}

/// Adds containment edges to the dependency graph.
fn rebuild_containment_edges(index: &mut ProgramIndex) {
    let mut edges = Vec::new();
    for node in &index.nodes {
        for child in &node.children {
            edges.push(DependencyEdge {
                from: DependencyKey::Node(node.id),
                to: DependencyKey::Node(*child),
                kind: DependencyKind::Containment,
                confidence: Confidence::Exact,
            });
        }
    }
    let reverse_edges = edges
        .iter()
        .map(|edge| DependencyEdge {
            from: edge.to,
            to: edge.from,
            kind: edge.kind,
            confidence: edge.confidence,
        })
        .collect::<Vec<_>>();
    index.dependency_graph = DependencyGraph {
        edges,
        reverse_edges,
    };
}
