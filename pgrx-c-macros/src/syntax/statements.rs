//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Structured statement parsing extends the expression parser without a second expression
//! arena. Scopes track source order, the first return, and whether every path returns. The
//! accepted grammar includes blocks, conditionals, local declarations, expressions, and returns;
//! unsupported scope-changing constructs fail rather than becoming textual Rust substitutions.
//! Root boundary metadata preserves the C dangling-else contract during emission.

use super::*;

/// Parsed source-order statements with first-return and all-paths-return metadata for one lexical
/// scope.
struct ParsedScope {
    /// Source-order statements retained with explicit lexical scopes.
    statements: Vec<Statement>,
    /// Original replacement-list token coordinates used to explain syntax and reconstruct source.
    tokens: TokenRange,
    /// The first return encountered in source order, independently of whether all paths return.
    first_return: Option<(NodeId, TokenRange)>,
    /// Whether every path through this supported statement or scope returns from its caller.
    always_returns: bool,
}

/// One structured statement and its return metadata before it is integrated into an enclosing scope.
struct ParsedStatement {
    /// The parsed structured statement whose return metadata is propagated to its parent scope.
    statement: Statement,
    /// The first return encountered in source order, independently of whether all paths return.
    first_return: Option<(NodeId, TokenRange)>,
    /// Whether every path through this supported statement or scope returns from its caller.
    always_returns: bool,
}

/// Parse bounded replacement grammar while preserving original indices and independently proved type
/// holes.
impl<F: Fn(&str) -> bool> Parser<'_, F> {
    /// Select expression or structured-statement grammar while preserving complete replacement
    /// boundaries.
    pub(super) fn replacement(
        &mut self,
        allow_statements: bool,
    ) -> Result<(NodeId, Option<StatementBody>), SyntaxError> {
        if !allow_statements
            || !matches!(self.spelling(), Some("return" | "do" | "{" | ";" | "if"))
            || self.spelling().is_some_and(|name| self.parameters.contains_key(name))
        {
            return self.expression(0, 0).map(|root| (root, None));
        }
        let requires_boundary = self.spelling() == Some("if");
        let ParsedScope { statements, tokens, first_return, always_returns } =
            self.statement_scope(0, true, true)?;
        let (root, return_tokens) = match first_return {
            Some((root, tokens)) => (root, Some(tokens)),
            None => (self.push(ExpressionKind::Empty, tokens), None),
        };
        let body =
            StatementBody { statements, tokens, return_tokens, always_returns, requires_boundary };
        let mut declarations = HashMap::new();
        let mut scopes = body
            .statements
            .iter()
            .map(|statement| (statement, body.tokens.end))
            .collect::<Vec<_>>();
        while let Some((statement, end)) = scopes.pop() {
            match statement {
                Statement::Declaration { name, tokens, .. } => {
                    if self.parameters.contains_key(name.as_str())
                        || declarations.insert(name.as_str(), (tokens.start, end)).is_some()
                    {
                        return Err(SyntaxError {
                            kind: SyntaxErrorKind::Statement,
                            tokens: *tokens,
                            message:
                                "local declarations cannot shadow macro formals or other locals"
                                    .into(),
                        });
                    }
                }
                Statement::Block { statements, tokens } => {
                    scopes.extend(statements.iter().map(|statement| (statement, tokens.end)));
                }
                Statement::If { then_branch, else_branch, .. } => {
                    scopes.push((then_branch.as_ref(), end));
                    if let Some(otherwise) = else_branch {
                        scopes.push((otherwise.as_ref(), end));
                    }
                }
                _ => {}
            }
        }
        for node in &self.nodes {
            if let ExpressionKind::Identifier { name } = &node.kind
                && let Some(&(start, end)) = declarations.get(name.as_str())
            {
                let message = if node.tokens.start < start {
                    Some("local declarations cannot change an earlier identifier's binding")
                } else if node.tokens.start >= end {
                    Some("a local identifier occurs outside its C block scope")
                } else {
                    None
                };
                if let Some(message) = message {
                    return Err(SyntaxError {
                        kind: SyntaxErrorKind::Statement,
                        tokens: node.tokens,
                        message: message.into(),
                    });
                }
            }
        }
        Ok((root, Some(body)))
    }

    /// Parse an ordered block and derive first-return and all-paths-return metadata without
    /// flattening lexical scope.
    fn statement_scope(
        &mut self,
        depth: usize,
        root: bool,
        optional_final_semicolon: bool,
    ) -> Result<ParsedScope, SyntaxError> {
        if depth >= MAX_DEPTH {
            return Err(self.error(
                SyntaxErrorKind::BudgetExceeded,
                format!("statement wrappers exceed the {MAX_DEPTH}-level analysis budget"),
            ));
        }
        let start = self.tokens[self.position].index;
        if self.spelling() == Some(";") {
            self.position += 1;
            return Ok(ParsedScope {
                statements: Vec::new(),
                tokens: TokenRange { start, end: start + 1 },
                first_return: None,
                always_returns: false,
            });
        }
        let do_block = self.spelling() == Some("do");
        if do_block {
            self.position += 1;
        }
        let block = do_block || self.spelling() == Some("{");
        let mut statements = Vec::new();
        let mut first_return = None;
        let mut always_returns = false;
        if block {
            self.expect("{")?;
            while self.spelling() != Some("}") {
                if self.spelling().is_none() {
                    return Err(
                        self.error(SyntaxErrorKind::Statement, "a statement block is not closed")
                    );
                }
                if always_returns {
                    return Err(self.error(
                        SyntaxErrorKind::Statement,
                        "no statement may follow an unconditional return",
                    ));
                }
                if self.spelling() == Some(";") {
                    self.position += 1;
                    continue;
                }
                let parsed = self.statement(depth + 1, false, true)?;
                first_return = first_return.or(parsed.first_return);
                always_returns = parsed.always_returns;
                statements.push(parsed.statement);
            }
        } else {
            let parsed = self.statement(depth, optional_final_semicolon, false)?;
            statements.push(parsed.statement);
            first_return = parsed.first_return;
            always_returns = parsed.always_returns;
        }
        if block {
            self.expect("}")?;
        }
        if do_block {
            if self.spelling() == Some("while") && self.parameters.contains_key("while") {
                return Err(self.error(
                    SyntaxErrorKind::Statement,
                    "a do/while wrapper requires a fixed loop keyword, not a macro formal",
                ));
            }
            self.expect("while")?;
            self.expect("(")?;
            let Some(token) = self.tokens.get(self.position) else {
                return Err(self.unexpected());
            };
            if token.token.kind != TokenKind::Literal
                || !parse_integer_literal(&token.token.spelling)
                    .is_ok_and(|literal| literal.value == 0)
            {
                return Err(self.error(
                    SyntaxErrorKind::Statement,
                    "only a literal-zero do/while statement wrapper is supported",
                ));
            }
            self.position += 1;
            self.expect(")")?;
            if !optional_final_semicolon || self.spelling().is_some() {
                self.expect(";")?;
            }
        }
        if block && !do_block && root && self.spelling() == Some(";") {
            self.position += 1;
        }
        let end = self.tokens[self.position - 1].index + 1;
        Ok(ParsedScope {
            statements,
            tokens: TokenRange { start, end },
            first_return,
            always_returns,
        })
    }

    /// Parse one supported structured statement and keep its control-flow effects separate from
    /// expression nodes.
    fn statement(
        &mut self,
        depth: usize,
        optional_final_semicolon: bool,
        allow_declaration: bool,
    ) -> Result<ParsedStatement, SyntaxError> {
        if depth >= MAX_DEPTH {
            return Err(self.error(
                SyntaxErrorKind::BudgetExceeded,
                format!("statement nesting exceeds the {MAX_DEPTH}-level analysis budget"),
            ));
        }
        let start = self.tokens.get(self.position).ok_or_else(|| self.unexpected())?.index;
        let keyword = self.spelling().filter(|name| !self.parameters.contains_key(*name));
        match keyword {
            Some("if") => {
                self.position += 1;
                let condition_start =
                    self.tokens.get(self.position).ok_or_else(|| self.unexpected())?.index;
                self.expect("(")?;
                let operand = self.expression(0, 0)?;
                self.expect(")")?;
                let condition = self.push(
                    ExpressionKind::Group { operand },
                    TokenRange {
                        start: condition_start,
                        end: self.tokens[self.position - 1].index + 1,
                    },
                );
                let then_branch = self.statement(depth + 1, optional_final_semicolon, false)?;
                let else_branch =
                    if self.spelling() == Some("else") && !self.parameters.contains_key("else") {
                        self.position += 1;
                        Some(self.statement(depth + 1, optional_final_semicolon, false)?)
                    } else {
                        None
                    };
                let first_return = then_branch
                    .first_return
                    .or_else(|| else_branch.as_ref().and_then(|branch| branch.first_return));
                let always_returns = then_branch.always_returns
                    && else_branch.as_ref().is_some_and(|branch| branch.always_returns);
                let end = self.tokens[self.position - 1].index + 1;
                Ok(ParsedStatement {
                    statement: Statement::If {
                        condition,
                        then_branch: Box::new(then_branch.statement),
                        else_branch: else_branch.map(|branch| Box::new(branch.statement)),
                        tokens: TokenRange { start, end },
                    },
                    first_return,
                    always_returns,
                })
            }
            Some("return") => {
                let (statement, returned) = self.return_operand(!optional_final_semicolon)?;
                Ok(ParsedStatement {
                    statement,
                    first_return: Some(returned),
                    always_returns: true,
                })
            }
            Some("{") | Some("do") => {
                let ParsedScope { statements, tokens, first_return, always_returns } =
                    self.statement_scope(depth, false, optional_final_semicolon)?;
                Ok(ParsedStatement {
                    statement: Statement::Block { statements, tokens },
                    first_return,
                    always_returns,
                })
            }
            Some(";") => {
                self.position += 1;
                Ok(ParsedStatement {
                    statement: Statement::Block {
                        statements: Vec::new(),
                        tokens: TokenRange { start, end: start + 1 },
                    },
                    first_return: None,
                    always_returns: false,
                })
            }
            _ => Ok(ParsedStatement {
                statement: self.ordinary_statement(allow_declaration, optional_final_semicolon)?,
                first_return: None,
                always_returns: false,
            }),
        }
    }

    /// Parse a return value through the shared expression arena and reject missing or trailing
    /// syntax.
    fn return_operand(
        &mut self,
        needs_semicolon: bool,
    ) -> Result<(Statement, (NodeId, TokenRange)), SyntaxError> {
        let start = self.tokens[self.position].index;
        self.expect("return")?;
        let expression = self.expression(0, 0)?;
        if needs_semicolon || self.spelling().is_some() {
            self.expect(";")?;
        }
        let tokens = TokenRange { start, end: self.tokens[self.position - 1].index + 1 };
        Ok((Statement::Return { expression, tokens }, (expression, tokens)))
    }

    /// Parse a declaration or expression statement with the required C semicolon boundary.
    fn ordinary_statement(
        &mut self,
        allow_declaration: bool,
        optional_final_semicolon: bool,
    ) -> Result<Statement, SyntaxError> {
        let start = self.tokens[self.position].index;
        if let Some((name_position, type_name)) = self.local_declaration() {
            if !allow_declaration {
                return Err(self.error(
                    SyntaxErrorKind::Statement,
                    "a declaration requires a C compound block",
                ));
            }
            let name = self.tokens[name_position].token.spelling.clone();
            self.position = name_position + 1;
            let initializer = if self.spelling() == Some("=") {
                self.position += 1;
                Some(self.expression(1, 0)?)
            } else {
                None
            };
            self.expect(";")?;
            let end = self.tokens[self.position - 1].index + 1;
            return Ok(Statement::Declaration {
                name,
                type_name,
                initializer,
                tokens: TokenRange { start, end },
            });
        }
        let expression = self.expression(0, 0)?;
        if !optional_final_semicolon || self.spelling().is_some() {
            self.expect(";")?;
        }
        let end = self.tokens[self.position - 1].index + 1;
        Ok(Statement::Expression { expression, tokens: TokenRange { start, end } })
    }

    // One flat declarator, with a compiler-recognized type. Arrays, function
    // declarators, storage classes and declaration lists remain deferred.
    /// Recognize supported local type/name syntax without confusing declarations with caller
    /// captures.
    fn local_declaration(&self) -> Option<(usize, String)> {
        let mut words = Vec::new();
        for (position, lexeme) in self.tokens.iter().enumerate().skip(self.position) {
            let spelling = lexeme.token.spelling.as_str();
            if lexeme.token.kind == TokenKind::Identifier
                && !words.is_empty()
                && self
                    .tokens
                    .get(position + 1)
                    .is_some_and(|next| matches!(next.token.spelling.as_str(), "=" | ";"))
            {
                let type_name = words.join(" ");
                if (self.is_type)(&type_name) {
                    return Some((position, type_name));
                }
            }
            if (!matches!(lexeme.token.kind, TokenKind::Identifier | TokenKind::Keyword)
                && spelling != "*")
                || self.parameters.contains_key(spelling)
            {
                break;
            }
            words.push(spelling);
        }
        None
    }
}
