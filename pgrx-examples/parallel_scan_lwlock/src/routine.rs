use pgrx::callconv::*;
use pgrx::pgbox::*;
use pgrx::pg_sys::*;
use pgrx::pgrx_sql_entity_graph::metadata::*;

pub(crate) struct CounterFdwRoutine(pub FdwRoutine);

unsafe impl SqlTranslatable for CounterFdwRoutine {
    fn argument_sql() -> std::result::Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("fdw_handler"))
    }

    fn return_sql() -> std::result::Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("fdw_handler")))
    }
}

unsafe impl BoxRet for CounterFdwRoutine {
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> pgrx::datum::Datum<'fcx> {
        let mut pgbox = unsafe { PgBox::<FdwRoutine>::alloc_node(NodeTag::T_FdwRoutine) };
        *pgbox = self.0;
        let datum = Datum::from(pgbox.into_pg());
        fcinfo.return_raw_datum(datum)
    }
}
