//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Constructors that parse documents and queries with a text search configuration.
//!
//! Postgres raises an error for a text longer than 1 MB. That error propagates.
//!
//! The text is passed to Postgres without encoding conversion, like other `&str` values
//! converted to `text`. If the database encoding is not UTF-8, Postgres misreads non-ASCII text.
//!
//! Postgres emits a `NOTICE` when a query contains no lexemes, such as an empty or stop-word-only
//! text, and returns an empty query.
use super::{ParseError, PgFunction, TsQueryBuf, TsVectorBuf, catch_parse_errors, encodable};
use crate::datum::IntoDatum;
use crate::direct_function_call;
use crate::pg_sys::{self, PgTryBuilder, errcodes::PgSqlErrorCode};
use std::ffi::CString;

/// A text search configuration, such as `english`, identified by its OID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TsConfig(pg_sys::Oid);

impl TsConfig {
    /// Returns the configuration named by the `default_text_search_config` setting.
    ///
    /// Postgres raises an error if the setting names no configuration.
    pub fn current() -> Self {
        // SAFETY: the function has no preconditions; with `true` it raises an error on failure.
        Self(unsafe { pg_sys::getTSCurrentConfig(true) })
    }

    /// Looks up a configuration by its possibly schema-qualified name, as the `regconfig` cast
    /// does. Returns `None` if the name is malformed or names no configuration.
    ///
    /// Unlike the cast, this does not accept `-` or a string of digits as an OID. A non-ASCII
    /// name is not found when the database encoding is not UTF-8 but extends ASCII, as `EUC_JP`
    /// does.
    pub fn by_name(name: &str) -> Option<Self> {
        if name == "-" || (!name.is_empty() && name.bytes().all(|b| b.is_ascii_digit())) {
            return None;
        }
        if !encodable(name) {
            return None;
        }
        let cstring = CString::new(name).ok()?;
        PgTryBuilder::new(|| {
            let arg = pg_sys::Datum::from(cstring.as_ptr());
            // SAFETY: `regconfigin` reads the NUL-terminated string, which outlives the call.
            let oid =
                unsafe { direct_function_call::<pg_sys::Oid>(pg_sys::regconfigin, &[Some(arg)]) };
            oid.map(Self)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_UNDEFINED_OBJECT, |_| None)
        // Before Postgres 16, `regconfigin` raises `ERRCODE_UNDEFINED_SCHEMA` for a name whose
        // schema does not exist. That code shares this variant's SQLSTATE. Later versions
        // report the configuration as undefined instead.
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_SCHEMA_NAME, |_| None)
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_NAME, |_| None)
        .catch_when(PgSqlErrorCode::ERRCODE_SYNTAX_ERROR, |_| None)
        .catch_when(PgSqlErrorCode::ERRCODE_FEATURE_NOT_SUPPORTED, |_| None)
        .execute()
    }

    fn datum(self) -> pg_sys::Datum {
        self.0.into_datum().expect("an OID is not null")
    }
}

/// Calls `f` with `text` as a `text` datum and frees the datum afterwards.
fn with_text<R>(text: &str, f: impl FnOnce(pg_sys::Datum) -> R) -> R {
    let datum = text.into_datum().expect("text is not null");
    let result = f(datum);
    // SAFETY: `into_datum` allocated the text with `palloc`, and `f` returned.
    unsafe { pg_sys::pfree(datum.cast_mut_ptr()) };
    result
}

impl TsQueryBuf {
    fn from_text_using(f: PgFunction, config: TsConfig, text: &str) -> Self {
        with_text(text, |text| {
            // SAFETY: `f` takes a `regconfig` and a `text`, and returns a fresh `tsquery`.
            unsafe { Self::call_owned(f, &[Some(config.datum()), Some(text)]) }
        })
    }

    /// Parses `text` as a query with the current configuration, like `to_tsquery`.
    pub fn from_text(text: &str) -> Result<Self, ParseError> {
        Self::from_text_with(TsConfig::current(), text)
    }

    /// Parses `text` as a query with `config`, like `to_tsquery`.
    pub fn from_text_with(config: TsConfig, text: &str) -> Result<Self, ParseError> {
        catch_parse_errors(|| Self::from_text_using(pg_sys::to_tsquery_byid, config, text))
    }

    /// Matches all the words in `text`, like `plainto_tsquery`, with the current configuration.
    pub fn from_plain_text(text: &str) -> Self {
        Self::from_plain_text_with(TsConfig::current(), text)
    }

    /// Matches all the words in `text`, like `plainto_tsquery`.
    pub fn from_plain_text_with(config: TsConfig, text: &str) -> Self {
        Self::from_text_using(pg_sys::plainto_tsquery_byid, config, text)
    }

    /// Matches the words in `text` in order, like `phraseto_tsquery`, with the current
    /// configuration.
    pub fn from_phrase_text(text: &str) -> Self {
        Self::from_phrase_text_with(TsConfig::current(), text)
    }

    /// Matches the words in `text` in order, like `phraseto_tsquery`.
    pub fn from_phrase_text_with(config: TsConfig, text: &str) -> Self {
        Self::from_text_using(pg_sys::phraseto_tsquery_byid, config, text)
    }

    /// Parses `text` in web search syntax, like `websearch_to_tsquery`, with the current
    /// configuration.
    pub fn from_websearch_text(text: &str) -> Self {
        Self::from_websearch_text_with(TsConfig::current(), text)
    }

    /// Parses `text` in web search syntax, like `websearch_to_tsquery`.
    pub fn from_websearch_text_with(config: TsConfig, text: &str) -> Self {
        Self::from_text_using(pg_sys::websearch_to_tsquery_byid, config, text)
    }
}

impl TsVectorBuf {
    /// Parses `text` as a document with the current configuration, like `to_tsvector`.
    pub fn from_text(text: &str) -> Self {
        Self::from_text_with(TsConfig::current(), text)
    }

    /// Parses `text` as a document with `config`, like `to_tsvector`.
    pub fn from_text_with(config: TsConfig, text: &str) -> Self {
        with_text(text, |text| {
            // SAFETY: `to_tsvector_byid` takes a `regconfig` and a `text`, and returns a fresh
            // `tsvector`.
            unsafe {
                Self::call_owned(pg_sys::to_tsvector_byid, &[Some(config.datum()), Some(text)])
            }
        })
    }
}
