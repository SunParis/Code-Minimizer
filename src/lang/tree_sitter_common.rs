//! Shared tree-sitter parsing helpers for language adapters.
//!
//! The helpers convert tree-sitter's lifetime-bound nodes into owned summaries
//! and diagnostics so the reducer never has to carry parser lifetimes.

use tree_sitter::{Node, Parser};

use crate::{
    edit::{Edit, TextRange},
    error::CodeMinimizerError,
    lang::{NodeSummary, ParseDiagnostic, ParsedProgram},
};

/// Parses source with a tree-sitter language and builds a parsed program.
pub fn parse_with_language(
    language: tree_sitter::Language,
    source: &str,
    file_name: &str,
) -> anyhow::Result<ParsedProgram> {
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .map_err(|error| anyhow::anyhow!("Failed to configure tree-sitter parser: {error:?}"))?;

    let tree = parser.parse(source, None).ok_or_else(|| {
        CodeMinimizerError::ParseFailed("tree-sitter returned no parse tree".into())
    })?;
    let root_node = tree.root_node();
    let root = summarize_node(root_node);
    let mut diagnostics = Vec::new();
    collect_diagnostics(root_node, &mut diagnostics);

    Ok(ParsedProgram {
        source: source.to_owned(),
        file_name: file_name.to_owned(),
        tree: Some(tree),
        root,
        diagnostics,
    })
}

/// Converts a tree-sitter node to an owned summary recursively.
fn summarize_node(node: Node<'_>) -> NodeSummary {
    let mut cursor = node.walk();
    let children = node
        .children(&mut cursor)
        .map(summarize_node)
        .collect::<Vec<_>>();

    NodeSummary {
        kind: node.kind().to_owned(),
        range: TextRange {
            start: node.start_byte(),
            end: node.end_byte(),
        },
        children,
        named: node.is_named(),
        has_error: node.has_error() || node.is_error() || node.is_missing(),
    }
}

/// Collects parse errors and missing nodes from the tree.
fn collect_diagnostics(node: Node<'_>, diagnostics: &mut Vec<ParseDiagnostic>) {
    if node.is_error() || node.is_missing() {
        diagnostics.push(ParseDiagnostic {
            message: format!("Parse diagnostic at node kind '{}'", node.kind()),
            range: TextRange {
                start: node.start_byte(),
                end: node.end_byte(),
            },
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_diagnostics(child, diagnostics);
    }
}

/// Returns all nodes in pre-order.
pub fn preorder_nodes(root: &NodeSummary) -> Vec<&NodeSummary> {
    let mut nodes = Vec::new();
    root.walk_preorder(&mut nodes);
    nodes
}

/// Builds a deletion edit that also consumes nearby whitespace when safe.
pub fn delete_node_with_padding(source: &str, node: &NodeSummary) -> Edit {
    Edit::Delete(range_with_line_padding(source, &node.range))
}

/// Expands a range to include one following newline or same-line surrounding spaces.
pub fn range_with_line_padding(source: &str, range: &TextRange) -> TextRange {
    let mut start = range.start;
    let mut end = range.end;
    let bytes = source.as_bytes();

    while start > 0 && matches!(bytes[start - 1], b' ' | b'\t') {
        start -= 1;
    }

    while end < bytes.len() && matches!(bytes[end], b' ' | b'\t') {
        end += 1;
    }

    if end < bytes.len() && bytes[end] == b'\r' {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b'\n' {
        end += 1;
    } else if start > 0 && bytes[start - 1] == b'\n' {
        start -= 1;
    }

    TextRange { start, end }
}

/// Returns true when the node kind appears inside the current subtree.
pub fn subtree_contains_kind(node: &NodeSummary, kind: &str) -> bool {
    if node.kind == kind {
        return true;
    }
    node.children
        .iter()
        .any(|child| subtree_contains_kind(child, kind))
}

/// Finds the first direct child of a specific kind.
pub fn child_of_kind<'a>(node: &'a NodeSummary, kind: &str) -> Option<&'a NodeSummary> {
    node.children.iter().find(|child| child.kind == kind)
}

/// Returns true if a node is a direct child of a parent with the given kind.
pub fn direct_parent_kind(root: &NodeSummary, target: &TextRange, kind: &str) -> bool {
    fn visit(node: &NodeSummary, target: &TextRange, kind: &str) -> bool {
        for child in &node.children {
            if &child.range == target {
                return node.kind == kind;
            }
            if visit(child, target, kind) {
                return true;
            }
        }
        false
    }
    visit(root, target, kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_with_line_padding_consumes_following_newline() {
        let source = "a\n  b;\nc";
        let range = TextRange { start: 4, end: 6 };
        assert_eq!(
            range_with_line_padding(source, &range),
            TextRange { start: 2, end: 7 }
        );
    }
}
