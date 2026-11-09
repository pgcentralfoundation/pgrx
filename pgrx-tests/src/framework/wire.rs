//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! The test framework's client for version 3.0 of the PostgreSQL frontend/backend protocol.
//!
//! The framework only talks to the server it started, which `initdb` sets up with trust
//! authentication, and only runs simple queries. So this client connects over TCP, accepts
//! only trust authentication, and runs queries with the simple query protocol, which returns
//! values as text. It checks every length and boundary in a server message before using it,
//! and reports malformed input as an [`Error`] instead of panicking.

use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::rc::Rc;

/// Protocol version 3.0, as sent in the startup message.
const PROTOCOL_VERSION: i32 = 3 << 16;

/// The Terminate message.
const TERMINATE: [u8; 5] = [b'X', 0, 0, 0, 4];

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("the connection to the server is closed")]
    Closed,
    #[error("unexpected message from the server: {0}")]
    Protocol(String),
    #[error(
        "the server requested {0} authentication, but the pgrx test client supports only trust authentication"
    )]
    Authentication(&'static str),
    #[error(transparent)]
    Db(Box<DbError>),
}

/// The fields of an `ErrorResponse` that the framework reports.
#[derive(Debug, thiserror::Error)]
#[error("{severity}: {message}")]
pub(super) struct DbError {
    pub(super) severity: String,
    pub(super) code: String,
    pub(super) message: String,
    pub(super) detail: Option<String>,
    pub(super) hint: Option<String>,
    pub(super) where_: Option<String>,
    pub(super) schema: Option<String>,
    pub(super) table: Option<String>,
    pub(super) file: Option<String>,
    pub(super) line: Option<String>,
}

/// A session that runs simple queries.
pub(super) struct Client<S: Read + Write = TcpStream> {
    /// `None` once an I/O or protocol error leaves the position in the server's messages
    /// unknown. Dropping the stream closes the connection, and later queries fail with
    /// [`Error::Closed`] instead of misreading the stream.
    stream: Option<BufReader<S>>,
}

impl Client {
    /// Connect to `host` and `port` over TCP and start a session as `user` in `database`.
    pub(super) fn connect(
        host: &str,
        port: u16,
        user: &str,
        database: &str,
    ) -> Result<Self, Error> {
        Self::start(TcpStream::connect((host, port))?, user, database)
    }
}

impl<S: Read + Write> Client<S> {
    /// Start a session on `stream` and wait until the server is ready for a query.
    fn start(stream: S, user: &str, database: &str) -> Result<Self, Error> {
        let mut startup = PROTOCOL_VERSION.to_be_bytes().to_vec();
        // Rust strings need UTF-8, so ask the server to send text in it.
        for s in ["user", user, "database", database, "client_encoding", "UTF8"] {
            push_cstr(&mut startup, s)?;
        }
        startup.push(0);
        let mut stream = BufReader::new(stream);
        stream.get_mut().write_all(&frame(None, &startup)?)?;

        loop {
            let (tag, bytes) = read_message(&mut stream)?;
            let mut body = Body(&bytes);
            match tag {
                b'R' => match body.i32()? {
                    0 => body.finish()?,
                    method => return Err(Error::Authentication(auth_method(method))),
                },
                // Parameter values, the cancellation key, and notices aren't needed.
                b'S' | b'K' | b'N' => {}
                b'E' => return Err(Error::Db(DbError::parse(body)?)),
                b'Z' => return Ok(Self { stream: Some(stream) }),
                tag => return Err(unexpected(tag)),
            }
        }
    }

    /// Run `sql`, which may hold several statements, and return the rows they produced.
    ///
    /// A server error is returned once the server is ready for another query, so the
    /// session stays usable.
    pub(super) fn simple_query(&mut self, sql: &str) -> Result<Vec<Row>, Error> {
        let mut body = Vec::with_capacity(sql.len() + 1);
        push_cstr(&mut body, sql)?;
        let query = frame(Some(b'Q'), &body)?;
        let stream = self.stream.as_mut().ok_or(Error::Closed)?;

        let mut response = Response::default();
        let result = stream
            .get_mut()
            .write_all(&query)
            .map_err(Error::from)
            .and_then(|()| response.read(stream));
        match (result, response.error) {
            (Ok(()), None) => Ok(response.rows),
            (Ok(()), Some(error)) => Err(Error::Db(error)),
            (Err(e), error) => {
                self.stream = None;
                // A FATAL error ends the session without ReadyForQuery, so report it
                // rather than the closed connection.
                Err(error.map_or(e, Error::Db))
            }
        }
    }
}

impl<S: Read + Write> Drop for Client<S> {
    fn drop(&mut self) {
        if let Some(stream) = &mut self.stream {
            // The server also ends the session when the connection closes, so a failure
            // here doesn't matter.
            let _ = stream.get_mut().write_all(&TERMINATE);
        }
    }
}

/// A row of a query result.
#[derive(Debug)]
pub(super) struct Row {
    columns: Rc<[String]>,
    values: Vec<Option<String>>,
}

impl Row {
    /// Parse a DataRow that follows a RowDescription of `columns`.
    fn parse(columns: Rc<[String]>, mut body: Body<'_>) -> Result<Self, Error> {
        let count = body.i16()?;
        if usize::try_from(count).ok() != Some(columns.len()) {
            return Err(Error::Protocol(format!(
                "DataRow has {count} values for {} columns",
                columns.len()
            )));
        }
        let values = (0..columns.len())
            .map(|_| match body.i32()? {
                -1 => Ok(None),
                len => {
                    let len = usize::try_from(len)
                        .map_err(|_| Error::Protocol(format!("value has length {len}")))?;
                    text(body.take(len)?).map(Some)
                }
            })
            .collect::<Result<_, _>>()?;
        body.finish()?;
        Ok(Self { columns, values })
    }

    /// The text of the value in `column`, or `None` if it's NULL or there's no such column.
    pub(super) fn get(&self, column: &str) -> Option<&str> {
        let index = self.columns.iter().position(|name| name == column)?;
        self.values.get(index)?.as_deref()
    }
}

/// The messages that answer one query.
#[derive(Default)]
struct Response {
    rows: Vec<Row>,
    error: Option<Box<DbError>>,
}

impl Response {
    /// Read messages through the ReadyForQuery that ends the response.
    fn read(&mut self, stream: &mut impl Read) -> Result<(), Error> {
        let mut columns = None;
        loop {
            let (tag, bytes) = read_message(stream)?;
            let body = Body(&bytes);
            match tag {
                b'T' => columns = Some(parse_row_description(body)?),
                b'D' => {
                    let columns = columns.clone().ok_or_else(|| {
                        Error::Protocol("DataRow without a RowDescription".to_string())
                    })?;
                    self.rows.push(Row::parse(columns, body)?);
                }
                // CommandComplete ends a statement's rows.
                b'C' => columns = None,
                b'E' => {
                    let error = DbError::parse(body)?;
                    // A FATAL error can follow the error that stopped the query.
                    self.error.get_or_insert(error);
                }
                // EmptyQueryResponse, and messages that can arrive at any time.
                b'I' | b'S' | b'N' | b'A' => {}
                b'Z' => return Ok(()),
                tag => return Err(unexpected(tag)),
            }
        }
    }
}

/// Parse a RowDescription into its column names.
fn parse_row_description(mut body: Body<'_>) -> Result<Rc<[String]>, Error> {
    let count = body.i16()?;
    let count = usize::try_from(count)
        .map_err(|_| Error::Protocol(format!("RowDescription has {count} columns")))?;
    let columns = (0..count)
        .map(|_| {
            let name = text(body.cstr()?)?;
            // Skip the table OID, column number, type OID, type size, and type modifier.
            body.take(16)?;
            match body.i16()? {
                0 => Ok(name),
                format => {
                    Err(Error::Protocol(format!("column {name:?} has format {format}, not text")))
                }
            }
        })
        .collect::<Result<_, _>>()?;
    body.finish()?;
    Ok(columns)
}

impl DbError {
    /// Parse the body of an ErrorResponse.
    fn parse(mut body: Body<'_>) -> Result<Box<Self>, Error> {
        let (mut severity, mut code, mut message) = (None, None, None);
        let (mut detail, mut hint, mut where_) = (None, None, None);
        let (mut schema, mut table, mut file, mut line) = (None, None, None, None);
        loop {
            let field = body.u8()?;
            if field == 0 {
                break;
            }
            // The fields are only displayed, so text in another encoding is fine.
            let value = Some(String::from_utf8_lossy(body.cstr()?).into_owned());
            match field {
                b'S' => severity = value,
                b'C' => code = value,
                b'M' => message = value,
                b'D' => detail = value,
                b'H' => hint = value,
                b'W' => where_ = value,
                b's' => schema = value,
                b't' => table = value,
                b'F' => file = value,
                b'L' => line = value,
                _ => {}
            }
        }
        body.finish()?;

        let missing = |field| Error::Protocol(format!("ErrorResponse has no {field}"));
        Ok(Box::new(Self {
            severity: severity.ok_or_else(|| missing("severity"))?,
            code: code.ok_or_else(|| missing("SQLSTATE code"))?,
            message: message.ok_or_else(|| missing("message"))?,
            detail,
            hint,
            where_,
            schema,
            table,
            file,
            line,
        }))
    }
}

/// A message body that checks every read against its end.
struct Body<'a>(&'a [u8]);

impl<'a> Body<'a> {
    fn take(&mut self, len: usize) -> Result<&'a [u8], Error> {
        let (bytes, rest) = self.0.split_at_checked(len).ok_or_else(too_short)?;
        self.0 = rest;
        Ok(bytes)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let (bytes, rest) = self.0.split_first_chunk().ok_or_else(too_short)?;
        self.0 = rest;
        Ok(*bytes)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        self.array().map(u8::from_be_bytes)
    }

    fn i16(&mut self) -> Result<i16, Error> {
        self.array().map(i16::from_be_bytes)
    }

    fn i32(&mut self) -> Result<i32, Error> {
        self.array().map(i32::from_be_bytes)
    }

    /// Take a NUL-terminated string, without the NUL.
    fn cstr(&mut self) -> Result<&'a [u8], Error> {
        let len = self
            .0
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| Error::Protocol("string has no NUL terminator".to_string()))?;
        let s = self.take(len)?;
        self.take(1)?;
        Ok(s)
    }

    /// Check that nothing is left over.
    fn finish(self) -> Result<(), Error> {
        match self.0 {
            [] => Ok(()),
            _ => Err(Error::Protocol("message has bytes after its contents".to_string())),
        }
    }
}

fn too_short() -> Error {
    Error::Protocol("message ends before its contents".to_string())
}

fn unexpected(tag: u8) -> Error {
    Error::Protocol(format!("message type {:?}", char::from(tag)))
}

/// Read one backend message and return its type and body.
fn read_message(stream: &mut impl Read) -> Result<(u8, Vec<u8>), Error> {
    let mut header = [0; 5];
    stream.read_exact(&mut header).map_err(|e| match e.kind() {
        io::ErrorKind::UnexpectedEof => Error::Closed,
        _ => Error::Io(e),
    })?;
    let [tag, len @ ..] = header;
    let len = i32::from_be_bytes(len);
    // The length counts itself but not the type byte.
    let body_len = u32::try_from(len).ok().and_then(|len| len.checked_sub(4)).ok_or_else(|| {
        Error::Protocol(format!("message type {:?} has length {len}", char::from(tag)))
    })?;
    // Reading through `take` grows the buffer only as bytes arrive, so a corrupt length
    // can't force a large allocation.
    let mut body = Vec::new();
    stream.by_ref().take(u64::from(body_len)).read_to_end(&mut body)?;
    if body.len() as u64 != u64::from(body_len) {
        return Err(Error::Closed);
    }
    Ok((tag, body))
}

/// Frame a frontend message: the type byte, if any, then the length, which counts itself,
/// then the body.
fn frame(tag: Option<u8>, body: &[u8]) -> io::Result<Vec<u8>> {
    let len = i32::try_from(body.len() + 4)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "message is too long"))?;
    let mut message = Vec::with_capacity(body.len() + 5);
    message.extend(tag);
    message.extend_from_slice(&len.to_be_bytes());
    message.extend_from_slice(body);
    Ok(message)
}

/// Append `s` as a NUL-terminated string.
fn push_cstr(buf: &mut Vec<u8>, s: &str) -> io::Result<()> {
    if s.contains('\0') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{s:?} contains a NUL byte"),
        ));
    }
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);
    Ok(())
}

fn text(bytes: &[u8]) -> Result<String, Error> {
    String::from_utf8(bytes.to_vec())
        .map_err(|_| Error::Protocol("text is not valid UTF-8".to_string()))
}

/// Name the authentication method that an AuthenticationRequest `code` asks for.
fn auth_method(code: i32) -> &'static str {
    match code {
        2 => "Kerberos V5",
        3 => "cleartext password",
        5 => "MD5 password",
        7 => "GSSAPI",
        9 => "SSPI",
        10 => "SASL",
        _ => "an unsupported kind of",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stream that replays a server's bytes and records the client's.
    struct Script {
        input: io::Cursor<Vec<u8>>,
        output: Vec<u8>,
    }

    impl Script {
        fn new(input: Vec<u8>) -> Self {
            Self { input: io::Cursor::new(input), output: Vec::new() }
        }
    }

    impl Read for Script {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.input.read(buf)
        }
    }

    impl Write for Script {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn message(tag: u8, body: &[u8]) -> Vec<u8> {
        let len = i32::try_from(body.len() + 4).unwrap();
        [&[tag][..], &len.to_be_bytes(), body].concat()
    }

    fn ready() -> Vec<u8> {
        message(b'Z', b"I")
    }

    /// What a server sends to accept a trust-authenticated session.
    fn accept() -> Vec<u8> {
        [
            message(b'R', &0_i32.to_be_bytes()),
            message(b'S', b"server_version\x0018.4\0"),
            message(b'K', &[0, 0, 0, 42, 1, 2, 3, 4]),
            ready(),
        ]
        .concat()
    }

    fn row_description(names: &[&str]) -> Vec<u8> {
        let mut body = i16::try_from(names.len()).unwrap().to_be_bytes().to_vec();
        for name in names {
            body.extend_from_slice(name.as_bytes());
            // NUL, then table OID, column number, type OID, type size, type modifier, and
            // the text format code.
            body.extend_from_slice(&[0; 19]);
        }
        message(b'T', &body)
    }

    fn data_row(values: &[Option<&str>]) -> Vec<u8> {
        let mut body = i16::try_from(values.len()).unwrap().to_be_bytes().to_vec();
        for value in values {
            match value {
                Some(value) => {
                    body.extend_from_slice(&i32::try_from(value.len()).unwrap().to_be_bytes());
                    body.extend_from_slice(value.as_bytes());
                }
                None => body.extend_from_slice(&(-1_i32).to_be_bytes()),
            }
        }
        message(b'D', &body)
    }

    fn error_response(fields: &[(u8, &str)]) -> Vec<u8> {
        let mut body = Vec::new();
        for (field, value) in fields {
            body.push(*field);
            body.extend_from_slice(value.as_bytes());
            body.push(0);
        }
        body.push(0);
        message(b'E', &body)
    }

    fn start(script: &mut Script) -> Client<&mut Script> {
        Client::start(script, "postgres", "pgrx_tests").unwrap()
    }

    #[test]
    fn sends_startup_query_and_terminate_messages() {
        let mut script = Script::new([accept(), message(b'C', b"BEGIN\0"), ready()].concat());
        let mut client = start(&mut script);
        assert!(client.simple_query("BEGIN").unwrap().is_empty());
        drop(client);

        let startup: &[u8] = b"\0\0\0\x40\0\x03\0\0user\0postgres\0database\0pgrx_tests\0\
            client_encoding\0UTF8\0\0";
        let query: &[u8] = b"Q\0\0\0\x0aBEGIN\0";
        assert_eq!(script.output, [startup, query, &TERMINATE].concat());
    }

    #[test]
    fn returns_rows_as_text() {
        // A NoticeResponse has the same fields as an ErrorResponse.
        let mut notice = error_response(&[(b'S', "NOTICE"), (b'C', "00000"), (b'M', "hi")]);
        notice[0] = b'N';
        let response = [
            notice,
            row_description(&["sid", "nothing"]),
            data_row(&[Some("6730f2a1.1f40"), None]),
            message(b'C', b"SELECT 1\0"),
            message(b'S', b"TimeZone\0UTC\0"),
            row_description(&["n"]),
            data_row(&[Some("1")]),
            data_row(&[Some("2")]),
            message(b'C', b"SELECT 2\0"),
            ready(),
        ];
        let mut script = Script::new([accept(), response.concat()].concat());
        let mut client = start(&mut script);
        let rows = client.simple_query("SELECT ...; SELECT ...").unwrap();

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].get("sid"), Some("6730f2a1.1f40"));
        assert_eq!(rows[0].get("nothing"), None);
        assert_eq!(rows[0].get("n"), None);
        assert_eq!(rows[1].get("n"), Some("1"));
        assert_eq!(rows[2].get("n"), Some("2"));
    }

    #[test]
    fn server_error_leaves_the_session_usable() {
        let response = [
            error_response(&[
                (b'S', "ERROR"),
                (b'V', "ERROR"),
                (b'C', "42P01"),
                (b'M', "relation \"nope\" does not exist"),
                (b'D', "detail"),
                (b'H', "hint"),
                (b'W', "context"),
                (b's', "schema"),
                (b't', "table"),
                (b'F', "parse_relation.c"),
                (b'L', "1452"),
                (b'R', "parserOpenTable"),
            ]),
            message(b'Z', b"E"),
            message(b'C', b"ROLLBACK\0"),
            ready(),
        ];
        let mut script = Script::new([accept(), response.concat()].concat());
        let mut client = start(&mut script);

        let Err(Error::Db(error)) = client.simple_query("SELECT * FROM nope") else {
            panic!("expected a server error");
        };
        assert_eq!(error.severity, "ERROR");
        assert_eq!(error.code, "42P01");
        assert_eq!(error.message, "relation \"nope\" does not exist");
        assert_eq!(error.detail.as_deref(), Some("detail"));
        assert_eq!(error.hint.as_deref(), Some("hint"));
        assert_eq!(error.where_.as_deref(), Some("context"));
        assert_eq!(error.schema.as_deref(), Some("schema"));
        assert_eq!(error.table.as_deref(), Some("table"));
        assert_eq!(error.file.as_deref(), Some("parse_relation.c"));
        assert_eq!(error.line.as_deref(), Some("1452"));
        assert_eq!(error.to_string(), "ERROR: relation \"nope\" does not exist");

        assert!(client.simple_query("ROLLBACK").unwrap().is_empty());
    }

    #[test]
    fn reports_a_fatal_error_that_closes_the_connection() {
        let fatal = error_response(&[
            (b'S', "FATAL"),
            (b'C', "57P01"),
            (b'M', "terminating connection due to administrator command"),
        ]);
        let mut script = Script::new([accept(), fatal].concat());
        let mut client = start(&mut script);

        let Err(Error::Db(error)) = client.simple_query("SELECT 1") else {
            panic!("expected a server error");
        };
        assert_eq!(error.severity, "FATAL");
        assert!(matches!(client.simple_query("SELECT 1"), Err(Error::Closed)));
    }

    #[test]
    fn accepts_only_trust_authentication() {
        let requests: [(&[u8], &str); 3] = [
            (&[0, 0, 0, 3], "cleartext password"),
            (&[0, 0, 0, 5, 1, 2, 3, 4], "MD5 password"),
            (b"\0\0\0\x0aSCRAM-SHA-256\0\0", "SASL"),
        ];
        for (request, method) in requests {
            let mut script = Script::new(message(b'R', request));
            let error = Client::start(&mut script, "postgres", "pgrx_tests").err().unwrap();
            assert!(error.to_string().contains(method), "{error}");
            assert!(matches!(error, Error::Authentication(m) if m == method));
        }
    }

    #[test]
    fn reports_a_startup_error() {
        let mut script = Script::new(error_response(&[
            (b'S', "FATAL"),
            (b'C', "3D000"),
            (b'M', "database \"pgrx_tests\" does not exist"),
        ]));
        let error = Client::start(&mut script, "postgres", "pgrx_tests").err().unwrap();
        assert!(matches!(error, Error::Db(e) if e.code == "3D000"));
    }

    #[test]
    fn rejects_nul_bytes_before_sending() {
        let mut rejected = Script::new(accept());
        assert!(Client::start(&mut rejected, "post\0gres", "pgrx_tests").is_err());
        assert!(rejected.output.is_empty());

        let mut script = Script::new([accept(), message(b'I', b""), ready()].concat());
        let mut client = start(&mut script);
        let error = client.simple_query("SELECT '\0'").unwrap_err();
        assert!(matches!(&error, Error::Io(e) if e.kind() == io::ErrorKind::InvalidInput));
        // Nothing reached the server, so the session is still usable.
        assert!(client.simple_query("").unwrap().is_empty());
    }

    #[test]
    fn malformed_responses_close_the_session() {
        let header = |tag: u8, len: i32| [&[tag][..], &len.to_be_bytes()].concat();
        let column = || row_description(&["a"]);
        let binary_column = [&[0, 1][..], b"a\0", &[0; 16], &[0, 1]].concat();
        let cases = [
            (header(b'Z', 3), "'Z' has length 3"),
            (header(b'Z', -1), "'Z' has length -1"),
            (header(b'D', i32::MAX), "closed"),
            (vec![b'Z', 0, 0], "closed"),
            (data_row(&[Some("1")]), "DataRow without a RowDescription"),
            ([column(), data_row(&[Some("1"), Some("2")])].concat(), "2 values for 1 columns"),
            ([column(), message(b'D', &[0, 1, 255, 255, 255, 254])].concat(), "length -2"),
            ([column(), message(b'D', &[0, 1, 0, 0, 0, 9, 1])].concat(), "ends before"),
            ([column(), message(b'D', &[0, 1, 0, 0, 0, 1, 255])].concat(), "not valid UTF-8"),
            (
                [column(), message(b'D', &[0, 1, 255, 255, 255, 255, 0])].concat(),
                "after its contents",
            ),
            (message(b'T', &[255, 255]), "-1 columns"),
            (message(b'T', b"\0\x01a"), "no NUL terminator"),
            (message(b'T', &binary_column), "format 1, not text"),
            (error_response(&[(b'S', "ERROR"), (b'C', "XX000")]), "has no message"),
            (message(b'E', b"SERROR\0"), "ends before"),
            (message(b'G', &[0, 0, 0]), "message type 'G'"),
        ];
        for (response, expected) in cases {
            let mut script = Script::new([accept(), response].concat());
            let mut client = start(&mut script);
            let error = client.simple_query("SELECT 1").unwrap_err().to_string();
            assert!(error.contains(expected), "expected {expected:?}, got {error:?}");
            assert!(matches!(client.simple_query("SELECT 1"), Err(Error::Closed)), "{expected}");
        }
    }
}
