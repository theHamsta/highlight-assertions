//! From https://github.com/tree-sitter/tree-sitter/blob/67de9435b109f9bd8bf5957d11ff13161966d262/cli/src/query_testing.rs#L27-L124
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use tree_sitter::{Language, Parser};

/// A position in a multi-line text document, in terms of rows and columns.
///
/// Rows and columns are zero-based.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CaptureInfo {
    pub name: String,
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Assertion {
    pub position: Point,
    pub expected_capture_name: String,
}

/// Parse the given source code, finding all of the comments that contain
/// highlighting assertions. Return a vector of (position, expected highlight name)
/// pairs.
pub fn parse_position_comments(
    parser: &mut Parser,
    language: Language,
    source: &[u8],
    comment_node: &str,
) -> Result<Vec<Assertion>> {
    let mut result = Vec::new();
    let mut assertion_ranges = Vec::new();

    // Parse the code.
    parser.set_included_ranges(&[]).unwrap();
    parser.set_language(language).unwrap();
    let tree = parser.parse(source, None).unwrap();

    // Walk the tree, finding comment nodes that contain assertions.
    let mut ascending = false;
    let mut cursor = tree.root_node().walk();
    loop {
        if ascending {
            let node = cursor.node();

            // Find every comment node.
            if node.kind().contains(comment_node) {
                if let Ok(text) = node.utf8_text(source) {
                    let mut position = node.start_position();
                    if position.row > 0 {
                        // Find the arrow character ("^" or '<-") in the comment. A left arrow
                        // refers to the column where the comment node starts. An up arrow refers
                        // to its own column.
                        let mut has_left_caret = false;
                        let mut has_arrow = false;
                        let mut arrow_end = 0;
                        for (i, c) in text.char_indices() {
                            arrow_end = i + 1;
                            if c == '-' && has_left_caret {
                                has_arrow = true;
                                break;
                            }
                            if c == '^' {
                                has_arrow = true;
                                position.column += i;
                                break;
                            }
                            has_left_caret = c == '<';
                        }
                        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[!\\w_\\-.]+").unwrap());

                        // If the comment node contains an arrow and a highlight name, record the
                        // highlight name and the position.
                        if let (true, Some(mat)) = (
                            has_arrow,
                            REGEX
                                .find(&text[arrow_end..]),
                        ) {
                            assertion_ranges.push((node.start_position(), node.end_position()));
                            let tree_sitter::Point { row, column } = position;
                            result.push(Assertion {
                                position: Point { row, column },
                                expected_capture_name: mat.as_str().to_string(),
                            });
                        }
                    }
                }
            }

            // Continue walking the tree.
            if cursor.goto_next_sibling() {
                ascending = false;
            } else if !cursor.goto_parent() {
                break;
            }
        } else if !cursor.goto_first_child() {
            ascending = true;
        }
    }

    // Adjust the row number in each assertion's position to refer to the line of
    // code *above* the assertion. There can be multiple lines of assertion comments,
    // so the positions may have to be decremented by more than one row.
    let mut i = 0;
    for assertion in result.iter_mut() {
        loop {
            let on_assertion_line = assertion_ranges[i..]
                .iter()
                .any(|(start, _)| start.row == assertion.position.row);
            if on_assertion_line {
                assertion.position.row -= 1;
            } else {
                while i < assertion_ranges.len()
                    && assertion_ranges[i].0.row < assertion.position.row
                {
                    i += 1;
                }
                break;
            }
        }
    }

    // The assertions can end up out of order due to the line adjustments.
    result.sort_unstable_by_key(|a| a.position);

    Ok(result)
}
