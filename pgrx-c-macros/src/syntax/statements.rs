//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use super::*;

struct ParsedScope {
    statements: Vec<Statement>,
    tokens: TokenRange,
    terminal: Option<(NodeId, TokenRange)>,
}

impl<F: Fn(&str) -> bool> Parser<'_, F> {
    pub(super) fn replacement(
        &mut self,
        allow_statements: bool,
    ) -> Result<(NodeId, Option<StatementBody>), SyntaxError> {
        if !allow_statements
            || !matches!(self.spelling(), Some("return" | "do" | "{" | ";"))
            || self.spelling().is_some_and(|name| self.parameters.contains_key(name))
        {
            return self.expression(0, 0).map(|root| (root, None));
        }
        let ParsedScope { statements, tokens, terminal } = self.statement_scope(0)?;
        let (root, return_tokens) = match terminal {
            Some((root, tokens)) => (root, Some(tokens)),
            None => (self.push(ExpressionKind::Empty, tokens), None),
        };
        let body = StatementBody { statements, tokens, return_tokens };
        let mut declarations = HashMap::new();
        let mut scopes = vec![(body.statements.as_slice(), body.tokens.end)];
        while let Some((statements, end)) = scopes.pop() {
            for statement in statements {
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
                        scopes.push((statements.as_slice(), tokens.end));
                    }
                    _ => {}
                }
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

    fn statement_scope(&mut self, depth: usize) -> Result<ParsedScope, SyntaxError> {
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
                terminal: None,
            });
        }
        let do_block = self.spelling() == Some("do");
        if do_block {
            self.position += 1;
        }
        let block = do_block || self.spelling() == Some("{");
        let mut statements = Vec::new();
        let mut terminal = None;
        if block {
            self.expect("{")?;
            while self.spelling() != Some("}") {
                if self.spelling().is_none() {
                    return Err(
                        self.error(SyntaxErrorKind::Statement, "a statement block is not closed")
                    );
                }
                if terminal.is_some() {
                    return Err(self.error(
                        SyntaxErrorKind::Statement,
                        "only a terminal return is supported; no statement may follow it",
                    ));
                }
                match self.spelling() {
                    Some(";") => self.position += 1,
                    Some("return") if !self.parameters.contains_key("return") => {
                        let (statement, returned) = self.return_operand(true)?;
                        statements.push(statement);
                        terminal = Some(returned);
                    }
                    Some("{") | Some("do")
                        if self.spelling() == Some("{") || !self.parameters.contains_key("do") =>
                    {
                        let ParsedScope { statements: nested, tokens, terminal: returned } =
                            self.statement_scope(depth + 1)?;
                        statements.push(Statement::Block { statements: nested, tokens });
                        terminal = returned;
                    }
                    _ => statements.push(self.ordinary_statement()?),
                }
            }
        } else {
            let (statement, returned) = self.return_operand(false)?;
            statements.push(statement);
            terminal = Some(returned);
        }
        if block {
            self.expect("}")?;
        }
        if do_block {
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
                    "only a literal-zero do/while wrapper is supported for straight-line statements",
                ));
            }
            self.position += 1;
            self.expect(")")?;
        }
        if block && self.spelling() == Some(";") {
            self.position += 1;
        }
        let end = self.tokens[self.position - 1].index + 1;
        Ok(ParsedScope { statements, tokens: TokenRange { start, end }, terminal })
    }

    fn return_operand(
        &mut self,
        needs_semicolon: bool,
    ) -> Result<(Statement, (NodeId, TokenRange)), SyntaxError> {
        let start = self.tokens[self.position].index;
        self.expect("return")?;
        let expression = self.expression(0, 0)?;
        if needs_semicolon || self.spelling() == Some(";") {
            self.expect(";")?;
        }
        let tokens = TokenRange { start, end: self.tokens[self.position - 1].index + 1 };
        Ok((Statement::Return { expression, tokens }, (expression, tokens)))
    }

    fn ordinary_statement(&mut self) -> Result<Statement, SyntaxError> {
        let start = self.tokens[self.position].index;
        if let Some((name_position, type_name)) = self.local_declaration() {
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
        self.expect(";")?;
        let end = self.tokens[self.position - 1].index + 1;
        Ok(Statement::Expression { expression, tokens: TokenRange { start, end } })
    }

    // One flat declarator, with a compiler-recognized type. Arrays, function
    // declarators, storage classes and declaration lists remain deferred.
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
