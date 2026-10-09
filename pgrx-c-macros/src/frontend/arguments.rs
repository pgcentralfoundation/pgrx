//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Decode recorded compiler arguments using the installation's command-line convention. Missing
//! historical metadata selects the caller's explicit invocation profile; it never invents flags.
//! Windows paths retain their backslashes, unlike POSIX shell quoting used by Unix installations.

use super::FrontendError;

/// Split recorded CFLAGS without mistaking missing historical metadata for literal compiler inputs.
/// `None` means the installation did not record flags; callers must record that they selected a
/// current invocation profile instead. Empty and PostgreSQL's `not recorded` sentinel have that
/// meaning. Malformed quoting, NUL bytes, and real metadata-query failures remain errors.
pub fn split_recorded_cflags(
    flags: &str,
    windows: bool,
) -> Result<Option<Vec<String>>, FrontendError> {
    if flags.contains('\0') {
        return Err(FrontendError::Arguments("recorded CFLAGS contain a NUL byte".into()));
    }
    if flags.trim().is_empty() || flags.trim() == "not recorded" {
        return Ok(None);
    }
    let arguments = if windows {
        split_windows_arguments(flags)?
    } else {
        shlex::split(flags).ok_or_else(|| {
            FrontendError::Arguments("recorded CFLAGS contain invalid POSIX quoting".into())
        })?
    };
    Ok(Some(arguments))
}

/// Apply the Microsoft C runtime's argument quoting rules to a flag list, without the executable
/// name's special argv[0] parsing. Backslashes are literal except immediately before a double quote;
/// paired backslashes precede a quote delimiter, and an odd final backslash escapes that quote.
fn split_windows_arguments(flags: &str) -> Result<Vec<String>, FrontendError> {
    let mut arguments = Vec::new();
    let mut token = String::new();
    let mut chars = flags.chars().peekable();
    let mut quoted = false;
    let mut started = false;
    while let Some(character) = chars.next() {
        match character {
            ' ' | '\t' if !quoted => {
                if started {
                    arguments.push(std::mem::take(&mut token));
                    started = false;
                }
            }
            '\\' => {
                started = true;
                let mut count = 1;
                while chars.peek() == Some(&'\\') {
                    chars.next();
                    count += 1;
                }
                if chars.peek() == Some(&'"') {
                    chars.next();
                    token.extend(std::iter::repeat_n('\\', count / 2));
                    if count % 2 == 1 {
                        token.push('"');
                    } else if quoted && chars.peek() == Some(&'"') {
                        chars.next();
                        token.push('"');
                    } else {
                        quoted = !quoted;
                    }
                } else {
                    token.extend(std::iter::repeat_n('\\', count));
                }
            }
            '"' => {
                started = true;
                if quoted && chars.peek() == Some(&'"') {
                    chars.next();
                    token.push('"');
                } else {
                    quoted = !quoted;
                }
            }
            _ => {
                started = true;
                token.push(character);
            }
        }
    }
    if quoted {
        return Err(FrontendError::Arguments(
            "recorded CFLAGS contain invalid Windows quoting".into(),
        ));
    }
    if started {
        arguments.push(token);
    }
    Ok(arguments)
}

/// Prove quoted Windows paths and escaped quote/backslash boundaries retain their actual bytes.
#[cfg(test)]
mod tests {
    use super::*;

    /// Drive paths, UNC paths, spaces, empty arguments, and macro strings obey Microsoft quoting.
    #[test]
    fn windows_paths_and_macro_quotes_survive_recorded_flag_splitting() {
        assert_eq!(
            split_recorded_cflags(r#"/IC:\pg\include /I"C:\Program Files\Postgres\include" /I\\server\share /DNAME=\"value\" """#, true).unwrap().unwrap(),
            [r"/IC:\pg\include", r"/IC:\Program Files\Postgres\include", r"/I\\server\share", "/DNAME=\"value\"", ""]
        );
        assert_eq!(
            split_windows_arguments(r#""C:\directory\\" "a""b" plain\end"#).unwrap(),
            [r"C:\directory\", "a\"b", r"plain\end"]
        );
    }

    /// Missing metadata stays distinct from malformed flags and preserves POSIX quoting on Unix.
    #[test]
    fn absent_flags_do_not_hide_invalid_recorded_quoting() {
        for value in ["", "  ", "not recorded"] {
            assert_eq!(split_recorded_cflags(value, true).unwrap(), None);
        }
        assert_eq!(
            split_recorded_cflags("'-DNAME=\"value\"' -O2", false).unwrap().unwrap(),
            ["-DNAME=\"value\"", "-O2"]
        );
        assert!(split_recorded_cflags("/I\"unterminated", true).is_err());
        assert!(split_recorded_cflags("'unterminated", false).is_err());
        assert!(split_recorded_cflags("-DVALUE=\0", false).is_err());
    }
}
