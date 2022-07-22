use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, ReturnVariant, ReturnVariantError, SqlTranslatable, SqlVariant,
};

impl SqlTranslatable for crate::FunctionCallInfoBaseData {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("uuid")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "uuid",
        ))))
    }
}

impl SqlTranslatable for crate::PlannerInfo {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("internal")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "internal",
        ))))
    }
}
