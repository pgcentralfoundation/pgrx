use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

#[cfg(any(
    feature = "pg16",
    feature = "pg15",
    feature = "pg14",
    feature = "pg13",
    feature = "pg12"
))]
unsafe impl SqlTranslatable for crate::FunctionCallInfoBaseData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Skip)
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::Skip))
    }
}

#[cfg(any(feature = "pg11"))]
unsafe impl SqlTranslatable for crate::FunctionCallInfoData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Skip)
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::Skip))
    }
}

unsafe impl SqlTranslatable for crate::PlannerInfo {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("internal"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("internal")))
    }
}

unsafe impl SqlTranslatable for crate::IndexAmRoutine {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("internal"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("internal")))
    }
}
unsafe impl SqlTranslatable for crate::FdwRoutine {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("fdw_handler"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("fdw_handler")))
    }
}

unsafe impl SqlTranslatable for crate::BOX {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("box"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("box")))
    }
}

unsafe impl SqlTranslatable for crate::Point {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("point"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("point")))
    }
}

unsafe impl SqlTranslatable for crate::ItemPointerData {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("tid"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("tid")))
    }
}

unsafe impl SqlTranslatable for crate::Datum {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Err(ArgumentError::Datum)
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Err(ReturnsError::Datum)
    }
}
