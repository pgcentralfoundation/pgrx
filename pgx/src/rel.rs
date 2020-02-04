use crate::{name_data_to_str, pg_sys, PgBox};

/// RelationGetRelationName
///		Returns the rel's name.
///
/// Note that the name is only unique within the containing namespace.
pub fn relation_get_relation_name(rel: &PgBox<pg_sys::RelationData>) -> &str {
    let rd_rel: PgBox<pg_sys::FormData_pg_class> = PgBox::from_pg(rel.rd_rel);
    name_data_to_str(&rd_rel.relname)
}

/// RelationGetRelid
///	Returns the OID of the relation
pub fn relation_get_id(rel: &PgBox<pg_sys::RelationData>) -> pg_sys::Oid {
    rel.rd_id
}

/// RelationGetNamespace
///		Returns the rel's namespace OID.
pub fn relation_get_namespace_oid(rel: &PgBox<pg_sys::RelationData>) -> pg_sys::Oid {
    let rd_rel: PgBox<pg_sys::FormData_pg_class> = PgBox::from_pg(rel.rd_rel);
    rd_rel.relnamespace
}
