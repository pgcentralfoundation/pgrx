use pgrx::callconv::*;
use pgrx::pg_sys::*;
use pgrx::pgbox::*;
use pgrx::pgrx_sql_entity_graph::metadata::*;

pub(crate) struct CounterFdwRoutine(pub FdwRoutine);

unsafe impl SqlTranslatable for CounterFdwRoutine {
    const TYPE_IDENT: &'static str = pgrx::pgrx_resolved_type!(CounterFdwRoutine);
    const TYPE_ORIGIN: TypeOrigin = TypeOrigin::External;
    const ARGUMENT_SQL: std::result::Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("fdw_handler"));
    const RETURN_SQL: std::result::Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("fdw_handler")));
}

unsafe impl BoxRet for CounterFdwRoutine {
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> pgrx::datum::Datum<'fcx> {
        let mut pgbox = unsafe { PgBox::<FdwRoutine>::alloc_node(NodeTag::T_FdwRoutine) };
        *pgbox = self.0;
        let datum = Datum::from(pgbox.into_pg());
        unsafe { fcinfo.return_raw_datum(datum) }
    }
}
