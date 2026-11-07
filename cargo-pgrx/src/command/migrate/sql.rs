//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Just enough SQL lexing to find which objects a migration fragment creates, alters or drops.
//!
//! This is not a SQL parser. Comments, string literals and dollar-quoted bodies are understood so
//! that their contents can't be mistaken for statements, and the object named at the start of a
//! `CREATE`, `ALTER` or `DROP` is read. Nothing else is: references from inside function bodies,
//! views and dynamic SQL are invisible here.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verb {
    Create,
    /// `CREATE OR REPLACE`, which can either create an object or change one that already exists
    Replace,
    Alter,
    Drop,
}

/// An object named by a statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObjectRef {
    pub(crate) verb: Verb,
    pub(crate) kind: &'static str,
    /// The unqualified name. Which schema an unqualified name resolves to depends on the
    /// `search_path` when the script runs, so schemas are not compared.
    pub(crate) name: String,
    /// A routine's argument types, or `None` if the statement has no argument list. `DROP FUNCTION
    /// f` is legal and applies to whichever overload exists.
    pub(crate) args: Option<Vec<String>>,
}

impl ObjectRef {
    pub(crate) fn same_object(&self, other: &ObjectRef) -> bool {
        self.kind == other.kind
            && self.name == other.name
            && match (&self.args, &other.args) {
                (Some(a), Some(b)) => a == b,
                _ => true,
            }
    }
}

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.name)?;
        match &self.args {
            Some(args) => write!(f, "({})", args.join(", ")),
            None => Ok(()),
        }
    }
}

/// Finds the objects each top-level statement in `sql` creates, alters or drops.
pub(crate) fn touched_objects(sql: &str) -> Vec<ObjectRef> {
    lex_statements(sql).iter().flat_map(|tokens| parse_statement(tokens)).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// An unquoted identifier or keyword, folded to lower case as Postgres folds it
    Word(String),
    /// A double-quoted identifier, whose case is significant
    Quoted(String),
    Punct(char),
    /// A string, dollar-quoted string or number: nothing this module needs to look inside
    Literal,
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || !b.is_ascii()
}

fn is_ident_continue(b: u8) -> bool {
    is_ident_start(b) || b.is_ascii_digit() || b == b'$'
}

/// Splits `sql` into statements at top-level semicolons.
fn lex_statements(sql: &str) -> Vec<Vec<Token>> {
    let bytes = sql.as_bytes();
    let mut statements = Vec::new();
    let mut current = Vec::new();
    let mut i = 0;

    while let Some(&b) = bytes.get(i) {
        let next = bytes.get(i + 1).copied();
        match b {
            _ if b.is_ascii_whitespace() => i += 1,
            b'-' if next == Some(b'-') => i = end_of_line(bytes, i),
            b'/' if next == Some(b'*') => i = end_of_block_comment(bytes, i),
            // psql meta-commands, such as the `\echo` guard at the top of a script
            b'\\' if current.is_empty() => i = end_of_line(bytes, i),
            b'\'' => {
                i = end_of_string(bytes, i, false);
                current.push(Token::Literal);
            }
            b'"' => {
                let (name, end) = quoted_identifier(sql, i);
                i = end;
                current.push(Token::Quoted(name));
            }
            b'$' => match dollar_tag(sql, i) {
                Some(tag) => {
                    i = end_of_dollar_quote(sql, i, tag);
                    current.push(Token::Literal);
                }
                None => {
                    i += 1;
                    current.push(Token::Punct('$'));
                }
            },
            b';' => {
                i += 1;
                if !current.is_empty() {
                    statements.push(std::mem::take(&mut current));
                }
            }
            b'@' if placeholder_end(bytes, i).is_some() => {
                let end = placeholder_end(bytes, i).unwrap();
                current.push(Token::Word(sql[i..end].to_ascii_lowercase()));
                i = end;
            }
            _ if is_ident_start(b) => {
                let len = bytes[i..].iter().take_while(|&&b| is_ident_continue(b)).count();
                let word = sql[i..i + len].to_ascii_lowercase();
                i += len;
                if word == "e" && bytes.get(i) == Some(&b'\'') {
                    i = end_of_string(bytes, i, true);
                    current.push(Token::Literal);
                } else {
                    current.push(Token::Word(word));
                }
            }
            _ if b.is_ascii_digit() => {
                i += bytes[i..]
                    .iter()
                    .take_while(|&&b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_')
                    .count();
                current.push(Token::Literal);
            }
            _ => {
                // Multi-byte characters start an identifier, so `b` is a whole ASCII character.
                i += 1;
                current.push(Token::Punct(char::from(b)));
            }
        }
    }

    if !current.is_empty() {
        statements.push(current);
    }
    statements
}

fn end_of_line(bytes: &[u8], start: usize) -> usize {
    bytes[start..].iter().position(|&b| b == b'\n').map_or(bytes.len(), |n| start + n)
}

/// Block comments nest in Postgres.
fn end_of_block_comment(bytes: &[u8], start: usize) -> usize {
    let mut depth = 0usize;
    let mut i = start;
    while i < bytes.len() {
        match (bytes[i], bytes.get(i + 1)) {
            (b'/', Some(b'*')) => {
                depth += 1;
                i += 2;
            }
            (b'*', Some(b'/')) => {
                depth -= 1;
                i += 2;
                if depth == 0 {
                    return i;
                }
            }
            _ => i += 1,
        }
    }
    bytes.len()
}

fn end_of_string(bytes: &[u8], start: usize, backslash_escapes: bool) -> usize {
    let mut i = start + 1;
    while let Some(&b) = bytes.get(i) {
        match b {
            b'\\' if backslash_escapes => i += 2,
            b'\'' if bytes.get(i + 1) == Some(&b'\'') => i += 2,
            b'\'' => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

fn quoted_identifier(sql: &str, start: usize) -> (String, usize) {
    let bytes = sql.as_bytes();
    let mut name = String::new();
    let mut i = start + 1;
    let mut run_start = i;
    while let Some(&b) = bytes.get(i) {
        if b != b'"' {
            i += 1;
        } else if bytes.get(i + 1) == Some(&b'"') {
            name.push_str(&sql[run_start..=i]);
            i += 2;
            run_start = i;
        } else {
            name.push_str(&sql[run_start..i]);
            return (name, i + 1);
        }
    }
    name.push_str(&sql[run_start..]);
    (name, bytes.len())
}

/// The `$tag$` opening a dollar-quoted string at `start`, if there is one. A `$` can also be a
/// positional parameter, which is why this is not a plain delimiter match.
fn dollar_tag(sql: &str, start: usize) -> Option<&str> {
    let bytes = sql.as_bytes();
    let tag_start = start + 1;
    if bytes.get(tag_start).is_some_and(u8::is_ascii_digit) {
        return None;
    }
    let len = bytes[tag_start..].iter().take_while(|&&b| is_ident_start(b) || b.is_ascii_digit());
    let end = tag_start + len.count();
    (bytes.get(end) == Some(&b'$')).then(|| &sql[start..=end])
}

fn end_of_dollar_quote(sql: &str, start: usize, tag: &str) -> usize {
    let body = start + tag.len();
    sql[body..].find(tag).map_or(sql.len(), |n| body + n + tag.len())
}

/// The end of an `@extschema@`-style placeholder at `start`, which Postgres substitutes before
/// parsing and which otherwise lexes as operators around a word.
fn placeholder_end(bytes: &[u8], start: usize) -> Option<usize> {
    let name = &bytes[start + 1..];
    let len = name.iter().take_while(|&&b| is_ident_continue(b) || b == b':').count();
    (len > 0 && name.get(len) == Some(&b'@')).then_some(start + len + 2)
}

/// Object kinds worth tracking, with multi-word kinds before their prefixes.
const KINDS: &[(&[&str], &str)] = &[
    (&["materialized", "view"], "MATERIALIZED VIEW"),
    (&["foreign", "table"], "FOREIGN TABLE"),
    (&["event", "trigger"], "EVENT TRIGGER"),
    (&["operator", "class"], "OPERATOR CLASS"),
    (&["operator", "family"], "OPERATOR FAMILY"),
    (&["aggregate"], "AGGREGATE"),
    (&["domain"], "DOMAIN"),
    (&["function"], "FUNCTION"),
    (&["index"], "INDEX"),
    (&["operator"], "OPERATOR"),
    (&["procedure"], "PROCEDURE"),
    (&["schema"], "SCHEMA"),
    (&["sequence"], "SEQUENCE"),
    (&["table"], "TABLE"),
    (&["trigger"], "TRIGGER"),
    (&["type"], "TYPE"),
    (&["view"], "VIEW"),
];

/// Words that can sit between `CREATE` and the kind of object.
const MODIFIERS: &[&str] =
    &["constraint", "global", "local", "recursive", "temp", "temporary", "unique", "unlogged"];

struct Cursor<'a> {
    tokens: &'a [Token],
}

impl Cursor<'_> {
    fn eat_word(&mut self, word: &str) -> bool {
        self.eat_words(&[word])
    }

    /// Consumes `words` only if all of them come next.
    fn eat_words(&mut self, words: &[&str]) -> bool {
        let matches = self.tokens.len() >= words.len()
            && words
                .iter()
                .zip(self.tokens)
                .all(|(word, token)| matches!(token, Token::Word(w) if w == word));
        if matches {
            self.tokens = &self.tokens[words.len()..];
        }
        matches
    }

    fn eat_punct(&mut self, punct: char) -> bool {
        let matches = self.tokens.first() == Some(&Token::Punct(punct));
        if matches {
            self.tokens = &self.tokens[1..];
        }
        matches
    }

    fn at_word(&self, word: &str) -> bool {
        matches!(self.tokens.first(), Some(Token::Word(w)) if w == word)
    }

    /// Moves just past the next `word`, if there is one.
    fn skip_past_word(&mut self, word: &str) -> bool {
        match self.tokens.iter().position(|token| matches!(token, Token::Word(w) if w == word)) {
            Some(n) => {
                self.tokens = &self.tokens[n + 1..];
                true
            }
            None => false,
        }
    }

    fn identifier(&mut self) -> Option<String> {
        let (Token::Word(name) | Token::Quoted(name)) = self.tokens.first()? else {
            return None;
        };
        self.tokens = &self.tokens[1..];
        Some(name.clone())
    }

    /// Reads a possibly schema-qualified name and returns its last component.
    fn name(&mut self) -> Option<String> {
        let mut name = self.identifier()?;
        while self.eat_punct('.') {
            name = self.identifier()?;
        }
        Some(name)
    }

    fn operator_name(&mut self) -> Option<String> {
        if self.identifier().is_some() {
            self.eat_punct('.');
        }
        let mut name = String::new();
        while let Some(&Token::Punct(c)) = self.tokens.first()
            && c != '('
        {
            name.push(c);
            self.tokens = &self.tokens[1..];
        }
        (!name.is_empty()).then_some(name)
    }

    /// Reads a parenthesized argument list as normalized type names.
    fn routine_args(&mut self) -> Option<Vec<String>> {
        if !self.eat_punct('(') {
            return None;
        }

        let mut args = Vec::new();
        let mut current = Vec::new();
        let mut depth = 1usize;
        while let Some((token, rest)) = self.tokens.split_first() {
            self.tokens = rest;
            match token {
                Token::Punct('(') => depth += 1,
                Token::Punct(')') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Token::Punct(',') if depth == 1 => {
                    args.extend(argument_type(&std::mem::take(&mut current)));
                }
                // Anything nested is a type modifier, as in `numeric(10,2)`, which isn't part of
                // a routine's identity.
                _ if depth == 1 => current.push(token),
                _ => {}
            }
        }
        args.extend(argument_type(&current));
        Some(args)
    }
}

fn parse_statement(tokens: &[Token]) -> Vec<ObjectRef> {
    let mut cursor = Cursor { tokens };
    let verb = if cursor.eat_word("create") {
        if cursor.eat_words(&["or", "replace"]) { Verb::Replace } else { Verb::Create }
    } else if cursor.eat_word("alter") {
        Verb::Alter
    } else if cursor.eat_word("drop") {
        Verb::Drop
    } else {
        return Vec::new();
    };

    while MODIFIERS.iter().any(|modifier| cursor.eat_word(modifier)) {}

    let Some(&(_, kind)) = KINDS.iter().find(|(words, _)| cursor.eat_words(words)) else {
        return Vec::new();
    };

    cursor.eat_word("concurrently");
    let _ = cursor.eat_words(&["if", "not", "exists"]) || cursor.eat_words(&["if", "exists"]);
    cursor.eat_word("only");

    let mut objects = Vec::new();
    // `CREATE INDEX ON t (x)` leaves the name to Postgres.
    if !(kind == "INDEX" && cursor.at_word("on")) {
        loop {
            let name = if kind == "OPERATOR" { cursor.operator_name() } else { cursor.name() };
            let Some(name) = name else {
                break;
            };
            let args = matches!(kind, "AGGREGATE" | "FUNCTION" | "PROCEDURE")
                .then(|| cursor.routine_args())
                .flatten();
            objects.push(ObjectRef { verb, kind, name, args });

            // Only `DROP` takes a list of objects.
            if verb != Verb::Drop || !cursor.eat_punct(',') {
                break;
            }
        }
    }

    // An index or trigger depends on its table as surely as an `ALTER TABLE` does.
    if matches!(kind, "INDEX" | "TRIGGER") && cursor.skip_past_word("on") {
        cursor.eat_word("only");
        if let Some(name) = cursor.name() {
            objects.push(ObjectRef { verb: Verb::Alter, kind: "TABLE", name, args: None });
        }
    }

    objects
}

/// SQL's multi-word type names, which can't be told from an argument name by position alone.
const MULTI_WORD_TYPES: &[&[&str]] = &[
    &["timestamp", "without", "time", "zone"],
    &["timestamp", "with", "time", "zone"],
    &["time", "without", "time", "zone"],
    &["time", "with", "time", "zone"],
    &["double", "precision"],
    &["character", "varying"],
    &["bit", "varying"],
];

const TYPE_ALIASES: &[(&str, &str)] = &[
    ("bigint", "int8"),
    ("bit varying", "varbit"),
    ("boolean", "bool"),
    ("char", "bpchar"),
    ("character", "bpchar"),
    ("character varying", "varchar"),
    ("decimal", "numeric"),
    ("double precision", "float8"),
    ("float", "float8"),
    ("int", "int4"),
    ("integer", "int4"),
    ("real", "float4"),
    ("smallint", "int2"),
    ("time with time zone", "timetz"),
    ("time without time zone", "time"),
    ("timestamp with time zone", "timestamptz"),
    ("timestamp without time zone", "timestamp"),
];

/// Normalizes one argument declaration to its type, or `None` if it isn't part of the routine's
/// identity: Postgres identifies a routine by its input arguments only.
fn argument_type(tokens: &[&Token]) -> Option<String> {
    let mut words: Vec<&str> = Vec::new();
    let mut array_dimensions = 0;
    for token in tokens {
        match token {
            Token::Word(word) if word == "default" => break,
            Token::Punct('=') => break,
            Token::Word(word) | Token::Quoted(word) => words.push(word),
            Token::Punct('.') => {
                words.pop(); // the schema of a qualified type
            }
            Token::Punct('[') => array_dimensions += 1,
            _ => {}
        }
    }

    match tokens.first() {
        Some(Token::Word(mode)) if mode == "out" => return None,
        Some(Token::Word(mode)) if matches!(mode.as_str(), "in" | "inout" | "variadic") => {
            words.remove(0);
        }
        _ => {}
    }

    let multi_word = MULTI_WORD_TYPES.iter().find(|name| words.ends_with(name));
    let name = match multi_word {
        Some(name) => name.join(" "),
        None => (*words.last()?).to_string(),
    };
    let canonical = TYPE_ALIASES
        .iter()
        .find(|(alias, _)| *alias == name)
        .map_or(name.as_str(), |(_, canonical)| canonical);
    Some(format!("{canonical}{}", "[]".repeat(array_dimensions)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn objects(sql: &str) -> Vec<String> {
        touched_objects(sql).iter().map(ToString::to_string).collect()
    }

    fn only(sql: &str) -> ObjectRef {
        let mut found = touched_objects(sql);
        assert_eq!(found.len(), 1, "{sql}: {found:?}");
        found.remove(0)
    }

    #[test]
    fn reads_the_verb_kind_and_name() {
        let object = only("CREATE OR REPLACE FUNCTION my_func(a int) RETURNS void LANGUAGE sql;");
        assert_eq!(object.verb, Verb::Replace);
        assert_eq!(object.to_string(), "FUNCTION my_func(int4)");

        assert_eq!(only("CREATE TABLE t (id int)").verb, Verb::Create);

        assert_eq!(only("ALTER TABLE t ADD COLUMN x int").verb, Verb::Alter);
        assert_eq!(only("DROP TYPE IF EXISTS status_t").verb, Verb::Drop);
    }

    #[test]
    fn splits_on_top_level_semicolons() {
        assert_eq!(
            objects("CREATE TABLE a (id int); CREATE VIEW b AS SELECT 1; DROP TABLE c"),
            ["TABLE a", "VIEW b", "TABLE c"]
        );
    }

    #[test]
    fn multi_word_kinds_and_modifiers_do_not_become_names() {
        assert_eq!(objects("CREATE MATERIALIZED VIEW a AS SELECT 1"), ["MATERIALIZED VIEW a"]);
        assert_eq!(objects("CREATE TEMP TABLE a (id int)"), ["TABLE a"]);
        assert_eq!(objects("CREATE UNLOGGED TABLE a (id int)"), ["TABLE a"]);
        assert_eq!(objects("CREATE FOREIGN TABLE a (id int) SERVER s"), ["FOREIGN TABLE a"]);
        assert_eq!(
            objects("CREATE OPERATOR CLASS c FOR TYPE t USING btree AS OPERATOR 1 <"),
            ["OPERATOR CLASS c"]
        );
    }

    #[test]
    fn existence_clauses_and_only_do_not_become_names() {
        assert_eq!(objects("CREATE TABLE IF NOT EXISTS a (id int)"), ["TABLE a"]);
        assert_eq!(objects("DROP FUNCTION IF EXISTS f(int)"), ["FUNCTION f(int4)"]);
        assert_eq!(objects("ALTER TABLE ONLY a ADD COLUMN x int"), ["TABLE a"]);
        assert_eq!(objects("ALTER TABLE IF EXISTS ONLY a ADD COLUMN x int"), ["TABLE a"]);
    }

    #[test]
    fn indexes_and_triggers_touch_their_table() {
        assert_eq!(objects("CREATE UNIQUE INDEX a ON t (x)"), ["INDEX a", "TABLE t"]);
        assert_eq!(
            objects("CREATE INDEX CONCURRENTLY IF NOT EXISTS a ON ONLY s.t (x)"),
            ["INDEX a", "TABLE t"]
        );
        assert_eq!(objects("CREATE INDEX ON t (x)"), ["TABLE t"]);
        assert_eq!(
            objects("CREATE TRIGGER tg BEFORE UPDATE OF c ON t FOR EACH ROW EXECUTE FUNCTION f()"),
            ["TRIGGER tg", "TABLE t"]
        );
        assert_eq!(objects("DROP INDEX a"), ["INDEX a"]);
    }

    #[test]
    fn drop_can_name_several_objects() {
        assert_eq!(objects("DROP TABLE a, s.b CASCADE"), ["TABLE a", "TABLE b"]);
        assert_eq!(
            objects("DROP FUNCTION f(int), g(text)"),
            ["FUNCTION f(int4)", "FUNCTION g(text)"]
        );
    }

    #[test]
    fn names_drop_their_schema_and_fold_unless_quoted() {
        assert_eq!(objects("ALTER TABLE S.Customers ADD x int"), ["TABLE customers"]);
        assert_eq!(objects(r#"ALTER TABLE "Customers" ADD x int"#), ["TABLE Customers"]);
        assert_eq!(objects(r#"ALTER TABLE "a""b" ADD x int"#), [r#"TABLE a"b"#]);
        assert_eq!(objects("CREATE FUNCTION @extschema@.f() RETURNS int"), ["FUNCTION f()"]);
    }

    #[test]
    fn routine_identity_ignores_how_arguments_are_written() {
        let expected = ["FUNCTION f(numeric, int4, varchar, float8, timestamptz)"];
        assert_eq!(
            objects(
                "CREATE FUNCTION f(a numeric(10,2), b integer DEFAULT 1, c character varying(5) = 'x',
                 d double precision, e timestamp with time zone) RETURNS void"
            ),
            expected
        );
        assert_eq!(
            objects("DROP FUNCTION f(decimal, int, varchar, float8, timestamptz)"),
            expected
        );
    }

    #[test]
    fn routine_identity_uses_input_arguments_only() {
        assert_eq!(
            objects(
                "CREATE FUNCTION f(IN a int, OUT b text, INOUT c bool, VARIADIC d text[]) AS ''"
            ),
            ["FUNCTION f(int4, bool, text[])"]
        );
    }

    #[test]
    fn routine_identity_drops_type_schemas() {
        assert_eq!(
            objects(r#"DROP FUNCTION f(a pg_catalog.int4, "s"."T")"#),
            ["FUNCTION f(int4, T)"]
        );
    }

    #[test]
    fn generated_pgrx_sql_parses() {
        assert_eq!(
            objects(
                r#"CREATE  FUNCTION "hello"("name" TEXT) RETURNS TEXT STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'hello_wrapper';"#
            ),
            ["FUNCTION hello(text)"]
        );
    }

    #[test]
    fn a_drop_without_arguments_matches_every_overload() {
        let create = only("CREATE FUNCTION f(int) RETURNS int");
        assert!(only("DROP FUNCTION f").same_object(&create));
        assert!(only("DROP FUNCTION f(integer)").same_object(&create));
        assert!(!only("DROP FUNCTION f(text)").same_object(&create));
        assert!(!only("DROP PROCEDURE f(int)").same_object(&create));
    }

    #[test]
    fn dollar_quoted_bodies_are_opaque() {
        let sql = "CREATE FUNCTION g() RETURNS void LANGUAGE plpgsql AS $$
            BEGIN PERFORM 1; CREATE TABLE scratch (id int); DROP TABLE other; END $$;
            CREATE TABLE after (id int);";
        assert_eq!(objects(sql), ["FUNCTION g()", "TABLE after"]);

        let tagged = "DO $body$ BEGIN EXECUTE $$ DROP TABLE a; $$; END $body$; DROP TABLE b;";
        assert_eq!(objects(tagged), ["TABLE b"]);
    }

    #[test]
    fn comment_markers_inside_strings_are_not_comments() {
        let sql = "COMMENT ON TABLE t IS 'see -- notes /* here';\nCREATE TABLE a (id int);";
        assert_eq!(objects(sql), ["TABLE a"]);
    }

    #[test]
    fn semicolons_inside_strings_do_not_end_statements() {
        assert_eq!(objects("SELECT 'a; DROP TABLE b;'; DROP TABLE c"), ["TABLE c"]);
        assert_eq!(objects("SELECT E'it\\'s; DROP TABLE b;'; DROP TABLE c"), ["TABLE c"]);
        assert_eq!(objects("SELECT 'it''s; DROP TABLE b;'; DROP TABLE c"), ["TABLE c"]);
    }

    #[test]
    fn comments_and_meta_commands_are_skipped() {
        let sql = "\\echo Use ALTER EXTENSION to load this file. \\quit
            -- DROP TABLE a;
            /* DROP TABLE b; /* nested */ DROP TABLE c; */
            DROP TABLE d;";
        assert_eq!(objects(sql), ["TABLE d"]);
    }

    #[test]
    fn a_positional_parameter_is_not_a_dollar_quote() {
        assert_eq!(objects("SELECT $1; DROP TABLE a"), ["TABLE a"]);
    }

    #[test]
    fn operators_are_matched_by_name() {
        assert!(only("DROP OPERATOR === (int, int)").same_object(&only(
            "CREATE OPERATOR === (LEFTARG = int, RIGHTARG = int, FUNCTION = f)"
        )));
    }

    #[test]
    fn untracked_statements_are_ignored() {
        assert!(objects("SELECT 1; GRANT ALL ON t TO u; ALTER EXTENSION e ADD TABLE t").is_empty());
    }

    #[test]
    fn unterminated_input_does_not_panic() {
        for sql in ["CREATE TABLE 'a", "/* a", "$$ a", "\"a", "CREATE FUNCTION f(", "@", "$"] {
            touched_objects(sql);
        }
    }

    #[test]
    fn non_ascii_text_lexes() {
        assert_eq!(
            objects("DROP TABLE café; -- ünïcode\nDROP TABLE \"naïve\""),
            ["TABLE café", "TABLE naïve"]
        );
    }
}
