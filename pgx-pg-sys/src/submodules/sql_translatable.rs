use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, ReturnVariant, ReturnVariantError, SqlMapping, SqlTranslatable,
};

#[cfg(any(feature = "pg14", feature = "pg13", feature = "pg12"))]
impl SqlTranslatable for crate::FunctionCallInfoBaseData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Skip)
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::Skip))
    }
}

#[cfg(any(feature = "pg10", feature = "pg11"))]
impl SqlTranslatable for crate::FunctionCallInfoData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Skip)
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::Skip))
    }
}

impl SqlTranslatable for crate::PlannerInfo {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("internal"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("internal")))
    }
}

impl SqlTranslatable for crate::IndexAmRoutine {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("internal"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("internal")))
    }
}
impl SqlTranslatable for crate::FdwRoutine {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("fdw_handler"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("fdw_handler")))
    }
}

impl SqlTranslatable for crate::BOX {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("box"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("box")))
    }
}

impl SqlTranslatable for crate::Point {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("box"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("box")))
    }
}

impl SqlTranslatable for crate::ItemPointerData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("tid"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("tid")))
    }
}

impl SqlTranslatable for crate::Datum {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Err(ArgumentError::Datum)
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Err(ReturnVariantError::Datum)
    }
}
