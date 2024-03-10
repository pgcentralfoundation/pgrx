#![cfg(feature = "chrono")]
use pgrx::prelude::*;

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use std::result::Result;

    use pgrx::datum::Date as PgrxDate;
    use pgrx::pg_test;
    use pgrx::DateTimeConversionError;

    use chrono::Datelike as _;
    use chrono::NaiveDate as ChronoNaiveDate;

    // Utility class for errors
    type DateTimeConversionResult<T> = Result<T, DateTimeConversionError>;

    /// Ensure simple conversion ([`pgrx::Date`] -> [`chrono::NaiveDate`]) works
    #[pg_test]
    fn chrono_simple_date_conversion() -> DateTimeConversionResult<()> {
        let original = PgrxDate::new(1970, 1, 1)?;
        let d = ChronoNaiveDate::try_from(original)?;
        assert_eq!(d.year(), original.year());
        assert_eq!(d.month(), 1);
        assert_eq!(d.day(), 1);
        Ok(())
    }
}
