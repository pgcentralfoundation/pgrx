//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Rustfmt leaves definitions containing transcriber repetitions untouched.
//! Lay out their original source slices instead of rewriting Rust syntax.

//! Generated macro bodies need a formatter that understands token trees where rustfmt may
//! leave a macro transcriber on one long line. This module preserves source token spellings
//! while grouping delimiters, generic arguments, and compound punctuation for layout. Width
//! estimates select compact or indented forms; formatting does not change the emitted macro
//! contract or reinterpret C semantics.

/// Inspect generated Rust token groups without reinterpreting their original source spellings.
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
/// Keep source byte ranges explicit while laying out generated token trees.
use std::ops::Range;

/// The preferred generated macro-body width used by source-preserving layout decisions.
const WIDTH: usize = 100;

/// Indent macro definitions while preserving their tokens, literals and comments.
///
/// Other Rust source is retained for the binding writer's final rustfmt pass.
pub fn format_rust_macros(source: &str) -> Result<String, String> {
    let tokens: TokenStream =
        source.parse().map_err(|error| format!("invalid generated Rust: {error}"))?;
    let mut bodies = Vec::new();
    definitions(tokens, &mut bodies);
    let mut result = String::with_capacity(source.len());
    let mut cursor = 0;
    for body in bodies {
        let node = Node::group(body, source);
        result.push_str(&source[cursor..node.range.start]);
        let line = result.rsplit('\n').next().unwrap_or("");
        let indent = line.bytes().take_while(|byte| matches!(byte, b' ' | b'\t')).count();
        let column = line.chars().count();
        let mut layout = Layout { source, output: result, column };
        layout.node(&node, indent, true);
        result = layout.output;
        cursor = node.range.end;
    }
    result.push_str(&source[cursor..]);
    Ok(result)
}

/// Locate macro transcriber groups so custom layout touches generated bodies rather than unrelated
/// Rust syntax.
fn definitions(tokens: TokenStream, bodies: &mut Vec<Group>) {
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        if matches!(&token, TokenTree::Ident(ident) if ident == "macro_rules")
            && matches!(tokens.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '!')
        {
            tokens.next();
            if matches!(tokens.peek(), Some(TokenTree::Ident(_))) {
                tokens.next();
                if let Some(TokenTree::Group(group)) = tokens.peek() {
                    bodies.push(group.clone());
                    tokens.next();
                    continue;
                }
            }
        }
        if let TokenTree::Group(group) = token {
            definitions(group.stream(), bodies);
        }
    }
}

/// A source-preserving token-layout node with a cached compact width for generated macro formatting.
struct Node {
    /// The original Rust source byte range copied by formatting rather than regenerated from token
    /// spelling.
    range: Range<usize>,
    /// Atomic source run or explicit delimiter tree used only for layout decisions.
    kind: Kind,
    /// Precomputed compact-render width used to avoid repeated subtree measurement.
    width: usize,
}

/// Formatting structure only; semantic tokens remain in their original source ranges.
enum Kind {
    /// A source token run whose internal punctuation must stay together during layout.
    Atom,
    /// A delimiter group with children and source punctuation used only for layout.
    Group {
        /// The opening delimiter text retained from the generated Rust source.
        open: Range<usize>,
        /// The closing delimiter text retained from the generated Rust source.
        close: Range<usize>,
        /// The token-tree delimiter controlling layout grouping rather than semantic evaluation.
        delimiter: char,
        /// Ordered child layout nodes, retaining original token-tree order.
        children: Vec<Node>,
    },
}

/// Build source-preserving formatting groups with cached compact widths.
impl Node {
    /// Build a layout tree for a token group while preserving original source ranges.
    fn group(group: Group, source: &str) -> Self {
        // Visit spans in source order. Proc-macro2 caches Unicode byte offsets;
        // visiting each enclosing closing span first would repeatedly scan it.
        let open = group.span_open().byte_range();
        let mut children = Vec::new();
        let mut end = open.end;
        for token in group.stream() {
            // Doc comments become synthetic attribute tokens sharing one span.
            // Check the opening span before descending into a synthetic group.
            let start = match &token {
                TokenTree::Group(group) => group.span_open().byte_range().start,
                token => token.span().byte_range().start,
            };
            if start < end {
                continue;
            }
            let node = match token {
                TokenTree::Group(group) => Self::group(group, source),
                token => {
                    let range = token.span().byte_range();
                    Self { width: range.len(), range, kind: Kind::Atom }
                }
            };
            end = node.range.end;
            children.push(node);
        }
        let close = group.span_close().byte_range();
        let delimiter = match group.delimiter() {
            Delimiter::Parenthesis => '(',
            Delimiter::Brace => '{',
            Delimiter::Bracket => '[',
            Delimiter::None => '\0',
        };
        Self::delimited(open, close, delimiter, angles(children, source), source)
    }

    /// Construct delimiter layout and precompute its compact width before choosing line breaks.
    fn delimited(
        open: Range<usize>,
        close: Range<usize>,
        delimiter: char,
        children: Vec<Node>,
        source: &str,
    ) -> Self {
        let mut width = open.len() + close.len();
        let mut end = open.end;
        for (index, child) in children.iter().enumerate() {
            let gap = &source[end..child.range.start];
            if index != 0 || !gap.trim().is_empty() {
                width += gap_width(gap, source, end, child.range.start);
            }
            width += child.width;
            end = child.range.end;
        }
        width += source[end..close.start].trim().len();
        Self {
            range: open.start..close.end,
            width,
            kind: Kind::Group { open, close, delimiter, children },
        }
    }
}

// Generic argument lists are not token-tree groups. Pair their delimiters once,
// then fold them into the same layout tree without changing either delimiter.
/// Recognize generic argument groups so Rust type paths can be laid out like other delimiters.
fn angles(nodes: Vec<Node>, source: &str) -> Vec<Node> {
    let mut pairs = vec![None; nodes.len()];
    let mut stack = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        let spelling = &source[node.range.clone()];
        if spelling == "<"
            && !compound(source, node.range.start)
            && !compound(source, node.range.end)
        {
            stack.push(index);
        } else if spelling == ">" && !source[node.range.end..].starts_with('=') {
            if let Some(open) = stack.pop() {
                pairs[open] = Some(index);
            }
        } else if spelling == ";" {
            stack.clear();
        }
    }
    /// Fold adjacent tokens into layout nodes without rewriting their source spelling.
    fn fold(
        nodes: &mut std::iter::Peekable<std::iter::Enumerate<std::vec::IntoIter<Node>>>,
        pairs: &[Option<usize>],
        end: usize,
        source: &str,
    ) -> Vec<Node> {
        let mut result = Vec::new();
        while nodes.peek().is_some_and(|(index, _)| *index < end) {
            let (index, node) = nodes.next().expect("peeked node");
            if let Some(close) = pairs[index] {
                let children = fold(nodes, pairs, close, source);
                let (_, closing) = nodes.next().expect("paired closing angle");
                result.push(Node::delimited(node.range, closing.range, '<', children, source));
            } else {
                result.push(node);
            }
        }
        result
    }
    fold(&mut nodes.into_iter().enumerate().peekable(), &pairs, usize::MAX, source)
}

/// Keep Rust compound punctuation and macro fragment syntax together across line breaks.
fn compound(source: &str, boundary: usize) -> bool {
    if boundary == 0 || boundary == source.len() {
        return false;
    }
    matches!(
        &source.as_bytes()[boundary - 1..=boundary],
        b"::"
            | b"=>"
            | b"->"
            | b".."
            | b".="
            | b"<<"
            | b">>"
            | b"&&"
            | b"||"
            | b"+="
            | b"-="
            | b"*="
            | b"/="
            | b"%="
            | b"^="
            | b"&="
            | b"|="
            | b"<="
            | b">="
            | b"=="
            | b"!="
    )
}

/// Measure required spacing between source ranges for the compact layout estimate.
fn gap_width(gap: &str, source: &str, previous: usize, next: usize) -> usize {
    if !gap.trim().is_empty() {
        gap.trim().len() + 2
    } else if (previous > 0 && matches!(source.as_bytes()[previous - 1], b',' | b';'))
        || (!gap.is_empty() && !matches!(source.as_bytes().get(next), Some(b',' | b';')))
    {
        1
    } else {
        0
    }
}

/// Accumulate formatted output while tracking the current column for compact-versus-indented
/// decisions.
struct Layout<'a> {
    /// Original generated Rust text whose token spellings and comments formatting preserves.
    source: &'a str,
    /// Accumulated formatted Rust source returned to the macro file writer.
    output: String,
    /// Current formatted-output column used for line-width decisions.
    column: usize,
}

/// Render the layout tree into readable macro bodies without rewriting semantic token spellings.
impl Layout<'_> {
    /// Append source text and update the output column used by width decisions.
    fn write(&mut self, text: &str) {
        self.output.push_str(text);
        if let Some((_, last)) = text.rsplit_once('\n') {
            self.column = last.chars().count();
        } else {
            self.column += text.chars().count();
        }
    }

    /// Start an indented line without accumulating trailing whitespace.
    fn newline(&mut self, indent: usize) {
        // Remove only indentation on an empty line. Spaces inside retained
        // line comments (including doc comments) are part of their source.
        let trimmed = self.output.trim_end_matches([' ', '\t']);
        if trimmed.ends_with('\n') {
            self.output.truncate(trimmed.len());
        }
        if !self.output.ends_with('\n') {
            self.output.push('\n');
        }
        self.output.extend(std::iter::repeat_n(' ', indent));
        self.column = indent;
    }

    /// Render a token-layout node compactly when it fits and recursively indent larger groups.
    fn node(&mut self, node: &Node, indent: usize, force: bool) {
        let Kind::Group { open, close, delimiter, children } = &node.kind else {
            self.write(&self.source[node.range.clone()]);
            return;
        };
        let broken = force || *delimiter == '{' || self.column + node.width > WIDTH;
        self.write(&self.source[open.clone()]);
        let mut end = open.end;
        for (index, child) in children.iter().enumerate() {
            let gap = &self.source[end..child.range.start];
            let previous = children.get(index.wrapping_sub(1));
            let separator = previous.map(|node| &self.source[node.range.clone()]);
            let boundary = !compound(self.source, child.range.start);
            let comment_line = separator.is_some_and(|text| text.starts_with("//"));
            let break_line = comment_line
                || broken
                    && boundary
                    && (index == 0
                        || separator.is_some_and(|text| matches!(text, "," | ";"))
                        || (*delimiter == '{'
                            && separator.is_some_and(|text| text.ends_with('}'))
                            && !matches!(&self.source[child.range.clone()], "else" | ";" | ",")));
            if break_line {
                self.newline(indent + 4);
            }
            if !gap.trim().is_empty() {
                if self.column != indent + 4 && !self.output.ends_with([' ', '\n']) {
                    self.write(" ");
                }
                self.write(gap.trim_start());
                if broken || gap.contains('\n') || gap.trim_start().starts_with("//") {
                    self.newline(indent + 4);
                } else if !self.output.ends_with([' ', '\t']) {
                    self.write(" ");
                }
            } else if !break_line
                && index != 0
                && gap_width(gap, self.source, end, child.range.start) != 0
                && !self.output.ends_with([' ', '\n'])
            {
                self.write(" ");
            }
            self.node(child, indent + if broken { 4 } else { 0 }, false);
            end = child.range.end;
        }
        let trailing = self.source[end..close.start].trim_start();
        if !trailing.trim().is_empty() {
            self.newline(indent + 4);
            self.write(trailing);
        }
        // A comment gap can end in a line comment after a block comment. Keep
        // the closing delimiter on its own line without re-lexing comments.
        if !trailing.is_empty()
            || children
                .last()
                .is_some_and(|child| self.source[child.range.clone()].starts_with("//"))
            || broken && !children.is_empty() && !compound(self.source, close.start)
        {
            self.newline(indent);
        }
        self.write(&self.source[close.clone()]);
    }
}
