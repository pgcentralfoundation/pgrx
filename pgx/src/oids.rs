#![allow(non_camel_case_types)]
use crate::pg_sys;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum PgBuiltInOids {
    BOOLOID = pg_sys::BOOLOID as isize,
    BYTEAOID = pg_sys::BYTEAOID as isize,
    CHAROID = pg_sys::CHAROID as isize,
    NAMEOID = pg_sys::NAMEOID as isize,
    INT8OID = pg_sys::INT8OID as isize,
    INT2OID = pg_sys::INT2OID as isize,
    INT2VECTOROID = pg_sys::INT2VECTOROID as isize,
    INT4OID = pg_sys::INT4OID as isize,
    REGPROCOID = pg_sys::REGPROCOID as isize,
    TEXTOID = pg_sys::TEXTOID as isize,
    OIDOID = pg_sys::OIDOID as isize,
    TIDOID = pg_sys::TIDOID as isize,
    XIDOID = pg_sys::XIDOID as isize,
    CIDOID = pg_sys::CIDOID as isize,
    OIDVECTOROID = pg_sys::OIDVECTOROID as isize,
    JSONOID = pg_sys::JSONOID as isize,
    XMLOID = pg_sys::XMLOID as isize,
    PGNODETREEOID = pg_sys::PGNODETREEOID as isize,
    PGNDISTINCTOID = pg_sys::PGNDISTINCTOID as isize,
    PGDEPENDENCIESOID = pg_sys::PGDEPENDENCIESOID as isize,
    PGDDLCOMMANDOID = pg_sys::PGDDLCOMMANDOID as isize,
    POINTOID = pg_sys::POINTOID as isize,
    LSEGOID = pg_sys::LSEGOID as isize,
    PATHOID = pg_sys::PATHOID as isize,
    BOXOID = pg_sys::BOXOID as isize,
    POLYGONOID = pg_sys::POLYGONOID as isize,
    LINEOID = pg_sys::LINEOID as isize,
    FLOAT4OID = pg_sys::FLOAT4OID as isize,
    FLOAT8OID = pg_sys::FLOAT8OID as isize,
    UNKNOWNOID = pg_sys::UNKNOWNOID as isize,
    CIRCLEOID = pg_sys::CIRCLEOID as isize,
    CASHOID = pg_sys::CASHOID as isize,
    MACADDROID = pg_sys::MACADDROID as isize,
    INETOID = pg_sys::INETOID as isize,
    CIDROID = pg_sys::CIDROID as isize,
    MACADDR8OID = pg_sys::MACADDR8OID as isize,
    INT2ARRAYOID = pg_sys::INT2ARRAYOID as isize,
    INT4ARRAYOID = pg_sys::INT4ARRAYOID as isize,
    TEXTARRAYOID = pg_sys::TEXTARRAYOID as isize,
    OIDARRAYOID = pg_sys::OIDARRAYOID as isize,
    FLOAT4ARRAYOID = pg_sys::FLOAT4ARRAYOID as isize,
    ACLITEMOID = pg_sys::ACLITEMOID as isize,
    CSTRINGARRAYOID = pg_sys::CSTRINGARRAYOID as isize,
    BPCHAROID = pg_sys::BPCHAROID as isize,
    VARCHAROID = pg_sys::VARCHAROID as isize,
    DATEOID = pg_sys::DATEOID as isize,
    TIMEOID = pg_sys::TIMEOID as isize,
    TIMESTAMPOID = pg_sys::TIMESTAMPOID as isize,
    TIMESTAMPTZOID = pg_sys::TIMESTAMPTZOID as isize,
    INTERVALOID = pg_sys::INTERVALOID as isize,
    TIMETZOID = pg_sys::TIMETZOID as isize,
    BITOID = pg_sys::BITOID as isize,
    VARBITOID = pg_sys::VARBITOID as isize,
    NUMERICOID = pg_sys::NUMERICOID as isize,
    REFCURSOROID = pg_sys::REFCURSOROID as isize,
    REGPROCEDUREOID = pg_sys::REGPROCEDUREOID as isize,
    REGOPEROID = pg_sys::REGOPEROID as isize,
    REGOPERATOROID = pg_sys::REGOPERATOROID as isize,
    REGCLASSOID = pg_sys::REGCLASSOID as isize,
    REGTYPEOID = pg_sys::REGTYPEOID as isize,
    REGROLEOID = pg_sys::REGROLEOID as isize,
    REGNAMESPACEOID = pg_sys::REGNAMESPACEOID as isize,
    REGTYPEARRAYOID = pg_sys::REGTYPEARRAYOID as isize,
    UUIDOID = pg_sys::UUIDOID as isize,
    LSNOID = pg_sys::LSNOID as isize,
    TSVECTOROID = pg_sys::TSVECTOROID as isize,
    GTSVECTOROID = pg_sys::GTSVECTOROID as isize,
    TSQUERYOID = pg_sys::TSQUERYOID as isize,
    REGCONFIGOID = pg_sys::REGCONFIGOID as isize,
    REGDICTIONARYOID = pg_sys::REGDICTIONARYOID as isize,
    JSONBOID = pg_sys::JSONBOID as isize,
    INT4RANGEOID = pg_sys::INT4RANGEOID as isize,
    RECORDOID = pg_sys::RECORDOID as isize,
    RECORDARRAYOID = pg_sys::RECORDARRAYOID as isize,
    CSTRINGOID = pg_sys::CSTRINGOID as isize,
    ANYOID = pg_sys::ANYOID as isize,
    ANYARRAYOID = pg_sys::ANYARRAYOID as isize,
    VOIDOID = pg_sys::VOIDOID as isize,
    TRIGGEROID = pg_sys::TRIGGEROID as isize,
    EVTTRIGGEROID = pg_sys::EVTTRIGGEROID as isize,
    LANGUAGE_HANDLEROID = pg_sys::LANGUAGE_HANDLEROID as isize,
    INTERNALOID = pg_sys::INTERNALOID as isize,
    OPAQUEOID = pg_sys::OPAQUEOID as isize,
    ANYELEMENTOID = pg_sys::ANYELEMENTOID as isize,
    ANYNONARRAYOID = pg_sys::ANYNONARRAYOID as isize,
    ANYENUMOID = pg_sys::ANYENUMOID as isize,
    FDW_HANDLEROID = pg_sys::FDW_HANDLEROID as isize,
    INDEX_AM_HANDLEROID = pg_sys::INDEX_AM_HANDLEROID as isize,
    TSM_HANDLEROID = pg_sys::TSM_HANDLEROID as isize,
    ANYRANGEOID = pg_sys::ANYRANGEOID as isize,

    ABSTIMEOID = pg_sys::pg10_specific::ABSTIMEOID as isize,
    RELTIMEOID = pg_sys::pg10_specific::RELTIMEOID as isize,
    TINTERVALOID = pg_sys::pg10_specific::TINTERVALOID as isize,

    XMLARRAYOID = pg_sys::pg11_specific::XMLARRAYOID as isize,
    TSVECTORARRAYOID = pg_sys::pg11_specific::TSVECTORARRAYOID as isize,
    GTSVECTORARRAYOID = pg_sys::pg11_specific::GTSVECTORARRAYOID as isize,
    TSQUERYARRAYOID = pg_sys::pg11_specific::TSQUERYARRAYOID as isize,
    REGCONFIGARRAYOID = pg_sys::pg11_specific::REGCONFIGARRAYOID as isize,
    REGDICTIONARYARRAYOID = pg_sys::pg11_specific::REGDICTIONARYARRAYOID as isize,
    JSONBARRAYOID = pg_sys::pg11_specific::JSONBARRAYOID as isize,
    TXID_SNAPSHOTOID = pg_sys::pg11_specific::TXID_SNAPSHOTOID as isize,
    TXID_SNAPSHOTARRAYOID = pg_sys::pg11_specific::TXID_SNAPSHOTARRAYOID as isize,
    INT4RANGEARRAYOID = pg_sys::pg11_specific::INT4RANGEARRAYOID as isize,
    NUMRANGEOID = pg_sys::pg11_specific::NUMRANGEOID as isize,
    NUMRANGEARRAYOID = pg_sys::pg11_specific::NUMRANGEARRAYOID as isize,
    TSRANGEOID = pg_sys::pg11_specific::TSRANGEOID as isize,
    TSRANGEARRAYOID = pg_sys::pg11_specific::TSRANGEARRAYOID as isize,
    TSTZRANGEOID = pg_sys::pg11_specific::TSTZRANGEOID as isize,
    TSTZRANGEARRAYOID = pg_sys::pg11_specific::TSTZRANGEARRAYOID as isize,
    DATERANGEOID = pg_sys::pg11_specific::DATERANGEOID as isize,
    DATERANGEARRAYOID = pg_sys::pg11_specific::DATERANGEARRAYOID as isize,
    INT8RANGEOID = pg_sys::pg11_specific::INT8RANGEOID as isize,
    INT8RANGEARRAYOID = pg_sys::pg11_specific::INT8RANGEARRAYOID as isize,
    JSONARRAYOID = pg_sys::pg11_specific::JSONARRAYOID as isize,
    SMGROID = pg_sys::pg11_specific::SMGROID as isize,
    LINEARRAYOID = pg_sys::pg11_specific::LINEARRAYOID as isize,
    CIRCLEARRAYOID = pg_sys::pg11_specific::CIRCLEARRAYOID as isize,
    MONEYARRAYOID = pg_sys::pg11_specific::MONEYARRAYOID as isize,
    BOOLARRAYOID = pg_sys::pg11_specific::BOOLARRAYOID as isize,
    BYTEAARRAYOID = pg_sys::pg11_specific::BYTEAARRAYOID as isize,
    CHARARRAYOID = pg_sys::pg11_specific::CHARARRAYOID as isize,
    NAMEARRAYOID = pg_sys::pg11_specific::NAMEARRAYOID as isize,
    INT2VECTORARRAYOID = pg_sys::pg11_specific::INT2VECTORARRAYOID as isize,
    REGPROCARRAYOID = pg_sys::pg11_specific::REGPROCARRAYOID as isize,
    TIDARRAYOID = pg_sys::pg11_specific::TIDARRAYOID as isize,
    XIDARRAYOID = pg_sys::pg11_specific::XIDARRAYOID as isize,
    CIDARRAYOID = pg_sys::pg11_specific::CIDARRAYOID as isize,
    OIDVECTORARRAYOID = pg_sys::pg11_specific::OIDVECTORARRAYOID as isize,
    BPCHARARRAYOID = pg_sys::pg11_specific::BPCHARARRAYOID as isize,
    VARCHARARRAYOID = pg_sys::pg11_specific::VARCHARARRAYOID as isize,
    INT8ARRAYOID = pg_sys::pg11_specific::INT8ARRAYOID as isize,
    POINTARRAYOID = pg_sys::pg11_specific::POINTARRAYOID as isize,
    LSEGARRAYOID = pg_sys::pg11_specific::LSEGARRAYOID as isize,
    PATHARRAYOID = pg_sys::pg11_specific::PATHARRAYOID as isize,
    BOXARRAYOID = pg_sys::pg11_specific::BOXARRAYOID as isize,
    FLOAT8ARRAYOID = pg_sys::pg11_specific::FLOAT8ARRAYOID as isize,
    ABSTIMEARRAYOID = pg_sys::pg11_specific::ABSTIMEARRAYOID as isize,
    RELTIMEARRAYOID = pg_sys::pg11_specific::RELTIMEARRAYOID as isize,
    TINTERVALARRAYOID = pg_sys::pg11_specific::TINTERVALARRAYOID as isize,
    POLYGONARRAYOID = pg_sys::pg11_specific::POLYGONARRAYOID as isize,
    ACLITEMARRAYOID = pg_sys::pg11_specific::ACLITEMARRAYOID as isize,
    MACADDRARRAYOID = pg_sys::pg11_specific::MACADDRARRAYOID as isize,
    MACADDR8ARRAYOID = pg_sys::pg11_specific::MACADDR8ARRAYOID as isize,
    INETARRAYOID = pg_sys::pg11_specific::INETARRAYOID as isize,
    CIDRARRAYOID = pg_sys::pg11_specific::CIDRARRAYOID as isize,
    TIMESTAMPARRAYOID = pg_sys::pg11_specific::TIMESTAMPARRAYOID as isize,
    DATEARRAYOID = pg_sys::pg11_specific::DATEARRAYOID as isize,
    TIMEARRAYOID = pg_sys::pg11_specific::TIMEARRAYOID as isize,
    REFCURSORARRAYOID = pg_sys::pg11_specific::REFCURSORARRAYOID as isize,
    VARBITARRAYOID = pg_sys::pg11_specific::VARBITARRAYOID as isize,
    BITARRAYOID = pg_sys::pg11_specific::BITARRAYOID as isize,
    TIMETZARRAYOID = pg_sys::pg11_specific::TIMETZARRAYOID as isize,
    TIMESTAMPTZARRAYOID = pg_sys::pg11_specific::TIMESTAMPTZARRAYOID as isize,
    INTERVALARRAYOID = pg_sys::pg11_specific::INTERVALARRAYOID as isize,
    NUMERICARRAYOID = pg_sys::pg11_specific::NUMERICARRAYOID as isize,
    UUIDARRAYOID = pg_sys::pg11_specific::UUIDARRAYOID as isize,
    REGPROCEDUREARRAYOID = pg_sys::pg11_specific::REGPROCEDUREARRAYOID as isize,
    REGOPERARRAYOID = pg_sys::pg11_specific::REGOPERARRAYOID as isize,
    REGOPERATORARRAYOID = pg_sys::pg11_specific::REGOPERATORARRAYOID as isize,
    REGCLASSARRAYOID = pg_sys::pg11_specific::REGCLASSARRAYOID as isize,
    REGROLEARRAYOID = pg_sys::pg11_specific::REGROLEARRAYOID as isize,
    REGNAMESPACEARRAYOID = pg_sys::pg11_specific::REGNAMESPACEARRAYOID as isize,
    PG_LSNARRAYOID = pg_sys::pg11_specific::PG_LSNARRAYOID as isize,
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum PgOid {
    InvalidOid,
    Custom(pg_sys::Oid),
    BuiltIn(PgBuiltInOids),
}

impl PgOid {
    pub fn from(oid: pg_sys::Oid) -> PgOid {
        match oid {
            pg_sys::InvalidOid => PgOid::InvalidOid,
            pg_sys::BOOLOID => PgOid::BuiltIn(PgBuiltInOids::BOOLOID),
            pg_sys::BYTEAOID => PgOid::BuiltIn(PgBuiltInOids::BYTEAOID),
            pg_sys::CHAROID => PgOid::BuiltIn(PgBuiltInOids::CHAROID),
            pg_sys::NAMEOID => PgOid::BuiltIn(PgBuiltInOids::NAMEOID),
            pg_sys::INT8OID => PgOid::BuiltIn(PgBuiltInOids::INT8OID),
            pg_sys::INT2OID => PgOid::BuiltIn(PgBuiltInOids::INT2OID),
            pg_sys::INT2VECTOROID => PgOid::BuiltIn(PgBuiltInOids::INT2VECTOROID),
            pg_sys::INT4OID => PgOid::BuiltIn(PgBuiltInOids::INT4OID),
            pg_sys::REGPROCOID => PgOid::BuiltIn(PgBuiltInOids::REGPROCOID),
            pg_sys::TEXTOID => PgOid::BuiltIn(PgBuiltInOids::TEXTOID),
            pg_sys::OIDOID => PgOid::BuiltIn(PgBuiltInOids::OIDOID),
            pg_sys::TIDOID => PgOid::BuiltIn(PgBuiltInOids::TIDOID),
            pg_sys::XIDOID => PgOid::BuiltIn(PgBuiltInOids::XIDOID),
            pg_sys::CIDOID => PgOid::BuiltIn(PgBuiltInOids::CIDOID),
            pg_sys::OIDVECTOROID => PgOid::BuiltIn(PgBuiltInOids::OIDVECTOROID),
            pg_sys::JSONOID => PgOid::BuiltIn(PgBuiltInOids::JSONOID),
            pg_sys::XMLOID => PgOid::BuiltIn(PgBuiltInOids::XMLOID),
            pg_sys::PGNODETREEOID => PgOid::BuiltIn(PgBuiltInOids::PGNODETREEOID),
            pg_sys::PGNDISTINCTOID => PgOid::BuiltIn(PgBuiltInOids::PGNDISTINCTOID),
            pg_sys::PGDEPENDENCIESOID => PgOid::BuiltIn(PgBuiltInOids::PGDEPENDENCIESOID),
            pg_sys::PGDDLCOMMANDOID => PgOid::BuiltIn(PgBuiltInOids::PGDDLCOMMANDOID),
            pg_sys::POINTOID => PgOid::BuiltIn(PgBuiltInOids::POINTOID),
            pg_sys::LSEGOID => PgOid::BuiltIn(PgBuiltInOids::LSEGOID),
            pg_sys::PATHOID => PgOid::BuiltIn(PgBuiltInOids::PATHOID),
            pg_sys::BOXOID => PgOid::BuiltIn(PgBuiltInOids::BOXOID),
            pg_sys::POLYGONOID => PgOid::BuiltIn(PgBuiltInOids::POLYGONOID),
            pg_sys::LINEOID => PgOid::BuiltIn(PgBuiltInOids::LINEOID),
            pg_sys::FLOAT4OID => PgOid::BuiltIn(PgBuiltInOids::FLOAT4OID),
            pg_sys::FLOAT8OID => PgOid::BuiltIn(PgBuiltInOids::FLOAT8OID),
            pg_sys::UNKNOWNOID => PgOid::BuiltIn(PgBuiltInOids::UNKNOWNOID),
            pg_sys::CIRCLEOID => PgOid::BuiltIn(PgBuiltInOids::CIRCLEOID),
            pg_sys::CASHOID => PgOid::BuiltIn(PgBuiltInOids::CASHOID),
            pg_sys::MACADDROID => PgOid::BuiltIn(PgBuiltInOids::MACADDROID),
            pg_sys::INETOID => PgOid::BuiltIn(PgBuiltInOids::INETOID),
            pg_sys::CIDROID => PgOid::BuiltIn(PgBuiltInOids::CIDROID),
            pg_sys::MACADDR8OID => PgOid::BuiltIn(PgBuiltInOids::MACADDR8OID),
            pg_sys::INT2ARRAYOID => PgOid::BuiltIn(PgBuiltInOids::INT2ARRAYOID),
            pg_sys::INT4ARRAYOID => PgOid::BuiltIn(PgBuiltInOids::INT4ARRAYOID),
            pg_sys::TEXTARRAYOID => PgOid::BuiltIn(PgBuiltInOids::TEXTARRAYOID),
            pg_sys::OIDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::OIDARRAYOID),
            pg_sys::FLOAT4ARRAYOID => PgOid::BuiltIn(PgBuiltInOids::FLOAT4ARRAYOID),
            pg_sys::ACLITEMOID => PgOid::BuiltIn(PgBuiltInOids::ACLITEMOID),
            pg_sys::CSTRINGARRAYOID => PgOid::BuiltIn(PgBuiltInOids::CSTRINGARRAYOID),
            pg_sys::BPCHAROID => PgOid::BuiltIn(PgBuiltInOids::BPCHAROID),
            pg_sys::VARCHAROID => PgOid::BuiltIn(PgBuiltInOids::VARCHAROID),
            pg_sys::DATEOID => PgOid::BuiltIn(PgBuiltInOids::DATEOID),
            pg_sys::TIMEOID => PgOid::BuiltIn(PgBuiltInOids::TIMEOID),
            pg_sys::TIMESTAMPOID => PgOid::BuiltIn(PgBuiltInOids::TIMESTAMPOID),
            pg_sys::TIMESTAMPTZOID => PgOid::BuiltIn(PgBuiltInOids::TIMESTAMPTZOID),
            pg_sys::INTERVALOID => PgOid::BuiltIn(PgBuiltInOids::INTERVALOID),
            pg_sys::TIMETZOID => PgOid::BuiltIn(PgBuiltInOids::TIMETZOID),
            pg_sys::BITOID => PgOid::BuiltIn(PgBuiltInOids::BITOID),
            pg_sys::VARBITOID => PgOid::BuiltIn(PgBuiltInOids::VARBITOID),
            pg_sys::NUMERICOID => PgOid::BuiltIn(PgBuiltInOids::NUMERICOID),
            pg_sys::REFCURSOROID => PgOid::BuiltIn(PgBuiltInOids::REFCURSOROID),
            pg_sys::REGPROCEDUREOID => PgOid::BuiltIn(PgBuiltInOids::REGPROCEDUREOID),
            pg_sys::REGOPEROID => PgOid::BuiltIn(PgBuiltInOids::REGOPEROID),
            pg_sys::REGOPERATOROID => PgOid::BuiltIn(PgBuiltInOids::REGOPERATOROID),
            pg_sys::REGCLASSOID => PgOid::BuiltIn(PgBuiltInOids::REGCLASSOID),
            pg_sys::REGTYPEOID => PgOid::BuiltIn(PgBuiltInOids::REGTYPEOID),
            pg_sys::REGROLEOID => PgOid::BuiltIn(PgBuiltInOids::REGROLEOID),
            pg_sys::REGNAMESPACEOID => PgOid::BuiltIn(PgBuiltInOids::REGNAMESPACEOID),
            pg_sys::REGTYPEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::REGTYPEARRAYOID),
            pg_sys::UUIDOID => PgOid::BuiltIn(PgBuiltInOids::UUIDOID),
            pg_sys::LSNOID => PgOid::BuiltIn(PgBuiltInOids::LSNOID),
            pg_sys::TSVECTOROID => PgOid::BuiltIn(PgBuiltInOids::TSVECTOROID),
            pg_sys::GTSVECTOROID => PgOid::BuiltIn(PgBuiltInOids::GTSVECTOROID),
            pg_sys::TSQUERYOID => PgOid::BuiltIn(PgBuiltInOids::TSQUERYOID),
            pg_sys::REGCONFIGOID => PgOid::BuiltIn(PgBuiltInOids::REGCONFIGOID),
            pg_sys::REGDICTIONARYOID => PgOid::BuiltIn(PgBuiltInOids::REGDICTIONARYOID),
            pg_sys::JSONBOID => PgOid::BuiltIn(PgBuiltInOids::JSONBOID),
            pg_sys::INT4RANGEOID => PgOid::BuiltIn(PgBuiltInOids::INT4RANGEOID),
            pg_sys::RECORDOID => PgOid::BuiltIn(PgBuiltInOids::RECORDOID),
            pg_sys::RECORDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::RECORDARRAYOID),
            pg_sys::CSTRINGOID => PgOid::BuiltIn(PgBuiltInOids::CSTRINGOID),
            pg_sys::ANYOID => PgOid::BuiltIn(PgBuiltInOids::ANYOID),
            pg_sys::ANYARRAYOID => PgOid::BuiltIn(PgBuiltInOids::ANYARRAYOID),
            pg_sys::VOIDOID => PgOid::BuiltIn(PgBuiltInOids::VOIDOID),
            pg_sys::TRIGGEROID => PgOid::BuiltIn(PgBuiltInOids::TRIGGEROID),
            pg_sys::EVTTRIGGEROID => PgOid::BuiltIn(PgBuiltInOids::EVTTRIGGEROID),
            pg_sys::LANGUAGE_HANDLEROID => PgOid::BuiltIn(PgBuiltInOids::LANGUAGE_HANDLEROID),
            pg_sys::INTERNALOID => PgOid::BuiltIn(PgBuiltInOids::INTERNALOID),
            pg_sys::OPAQUEOID => PgOid::BuiltIn(PgBuiltInOids::OPAQUEOID),
            pg_sys::ANYELEMENTOID => PgOid::BuiltIn(PgBuiltInOids::ANYELEMENTOID),
            pg_sys::ANYNONARRAYOID => PgOid::BuiltIn(PgBuiltInOids::ANYNONARRAYOID),
            pg_sys::ANYENUMOID => PgOid::BuiltIn(PgBuiltInOids::ANYENUMOID),
            pg_sys::FDW_HANDLEROID => PgOid::BuiltIn(PgBuiltInOids::FDW_HANDLEROID),
            pg_sys::INDEX_AM_HANDLEROID => PgOid::BuiltIn(PgBuiltInOids::INDEX_AM_HANDLEROID),
            pg_sys::TSM_HANDLEROID => PgOid::BuiltIn(PgBuiltInOids::TSM_HANDLEROID),
            pg_sys::ANYRANGEOID => PgOid::BuiltIn(PgBuiltInOids::ANYRANGEOID),
            pg_sys::pg10_specific::ABSTIMEOID => PgOid::BuiltIn(PgBuiltInOids::ABSTIMEOID),
            pg_sys::pg10_specific::RELTIMEOID => PgOid::BuiltIn(PgBuiltInOids::RELTIMEOID),
            pg_sys::pg10_specific::TINTERVALOID => PgOid::BuiltIn(PgBuiltInOids::TINTERVALOID),
            pg_sys::pg11_specific::XMLARRAYOID => PgOid::BuiltIn(PgBuiltInOids::XMLARRAYOID),
            pg_sys::pg11_specific::TSVECTORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TSVECTORARRAYOID)
            }
            pg_sys::pg11_specific::GTSVECTORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::GTSVECTORARRAYOID)
            }
            pg_sys::pg11_specific::TSQUERYARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TSQUERYARRAYOID)
            }
            pg_sys::pg11_specific::REGCONFIGARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGCONFIGARRAYOID)
            }
            pg_sys::pg11_specific::REGDICTIONARYARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGDICTIONARYARRAYOID)
            }
            pg_sys::pg11_specific::JSONBARRAYOID => PgOid::BuiltIn(PgBuiltInOids::JSONBARRAYOID),
            pg_sys::pg11_specific::TXID_SNAPSHOTOID => {
                PgOid::BuiltIn(PgBuiltInOids::TXID_SNAPSHOTOID)
            }
            pg_sys::pg11_specific::TXID_SNAPSHOTARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TXID_SNAPSHOTARRAYOID)
            }
            pg_sys::pg11_specific::INT4RANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::INT4RANGEARRAYOID)
            }
            pg_sys::pg11_specific::NUMRANGEOID => PgOid::BuiltIn(PgBuiltInOids::NUMRANGEOID),
            pg_sys::pg11_specific::NUMRANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::NUMRANGEARRAYOID)
            }
            pg_sys::pg11_specific::TSRANGEOID => PgOid::BuiltIn(PgBuiltInOids::TSRANGEOID),
            pg_sys::pg11_specific::TSRANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TSRANGEARRAYOID)
            }
            pg_sys::pg11_specific::TSTZRANGEOID => PgOid::BuiltIn(PgBuiltInOids::TSTZRANGEOID),
            pg_sys::pg11_specific::TSTZRANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TSTZRANGEARRAYOID)
            }
            pg_sys::pg11_specific::DATERANGEOID => PgOid::BuiltIn(PgBuiltInOids::DATERANGEOID),
            pg_sys::pg11_specific::DATERANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::DATERANGEARRAYOID)
            }
            pg_sys::pg11_specific::INT8RANGEOID => PgOid::BuiltIn(PgBuiltInOids::INT8RANGEOID),
            pg_sys::pg11_specific::INT8RANGEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::INT8RANGEARRAYOID)
            }
            pg_sys::pg11_specific::JSONARRAYOID => PgOid::BuiltIn(PgBuiltInOids::JSONARRAYOID),
            pg_sys::pg11_specific::SMGROID => PgOid::BuiltIn(PgBuiltInOids::SMGROID),
            pg_sys::pg11_specific::LINEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::LINEARRAYOID),
            pg_sys::pg11_specific::CIRCLEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::CIRCLEARRAYOID),
            pg_sys::pg11_specific::MONEYARRAYOID => PgOid::BuiltIn(PgBuiltInOids::MONEYARRAYOID),
            pg_sys::pg11_specific::BOOLARRAYOID => PgOid::BuiltIn(PgBuiltInOids::BOOLARRAYOID),
            pg_sys::pg11_specific::BYTEAARRAYOID => PgOid::BuiltIn(PgBuiltInOids::BYTEAARRAYOID),
            pg_sys::pg11_specific::CHARARRAYOID => PgOid::BuiltIn(PgBuiltInOids::CHARARRAYOID),
            pg_sys::pg11_specific::NAMEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::NAMEARRAYOID),
            pg_sys::pg11_specific::INT2VECTORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::INT2VECTORARRAYOID)
            }
            pg_sys::pg11_specific::REGPROCARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGPROCARRAYOID)
            }
            pg_sys::pg11_specific::TIDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::TIDARRAYOID),
            pg_sys::pg11_specific::XIDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::XIDARRAYOID),
            pg_sys::pg11_specific::CIDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::CIDARRAYOID),
            pg_sys::pg11_specific::OIDVECTORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::OIDVECTORARRAYOID)
            }
            pg_sys::pg11_specific::BPCHARARRAYOID => PgOid::BuiltIn(PgBuiltInOids::BPCHARARRAYOID),
            pg_sys::pg11_specific::VARCHARARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::VARCHARARRAYOID)
            }
            pg_sys::pg11_specific::INT8ARRAYOID => PgOid::BuiltIn(PgBuiltInOids::INT8ARRAYOID),
            pg_sys::pg11_specific::POINTARRAYOID => PgOid::BuiltIn(PgBuiltInOids::POINTARRAYOID),
            pg_sys::pg11_specific::LSEGARRAYOID => PgOid::BuiltIn(PgBuiltInOids::LSEGARRAYOID),
            pg_sys::pg11_specific::PATHARRAYOID => PgOid::BuiltIn(PgBuiltInOids::PATHARRAYOID),
            pg_sys::pg11_specific::BOXARRAYOID => PgOid::BuiltIn(PgBuiltInOids::BOXARRAYOID),
            pg_sys::pg11_specific::FLOAT8ARRAYOID => PgOid::BuiltIn(PgBuiltInOids::FLOAT8ARRAYOID),
            pg_sys::pg11_specific::ABSTIMEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::ABSTIMEARRAYOID)
            }
            pg_sys::pg11_specific::RELTIMEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::RELTIMEARRAYOID)
            }
            pg_sys::pg11_specific::TINTERVALARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TINTERVALARRAYOID)
            }
            pg_sys::pg11_specific::POLYGONARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::POLYGONARRAYOID)
            }
            pg_sys::pg11_specific::ACLITEMARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::ACLITEMARRAYOID)
            }
            pg_sys::pg11_specific::MACADDRARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::MACADDRARRAYOID)
            }
            pg_sys::pg11_specific::MACADDR8ARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::MACADDR8ARRAYOID)
            }
            pg_sys::pg11_specific::INETARRAYOID => PgOid::BuiltIn(PgBuiltInOids::INETARRAYOID),
            pg_sys::pg11_specific::CIDRARRAYOID => PgOid::BuiltIn(PgBuiltInOids::CIDRARRAYOID),
            pg_sys::pg11_specific::TIMESTAMPARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TIMESTAMPARRAYOID)
            }
            pg_sys::pg11_specific::DATEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::DATEARRAYOID),
            pg_sys::pg11_specific::TIMEARRAYOID => PgOid::BuiltIn(PgBuiltInOids::TIMEARRAYOID),
            pg_sys::pg11_specific::REFCURSORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REFCURSORARRAYOID)
            }
            pg_sys::pg11_specific::VARBITARRAYOID => PgOid::BuiltIn(PgBuiltInOids::VARBITARRAYOID),
            pg_sys::pg11_specific::BITARRAYOID => PgOid::BuiltIn(PgBuiltInOids::BITARRAYOID),
            pg_sys::pg11_specific::TIMETZARRAYOID => PgOid::BuiltIn(PgBuiltInOids::TIMETZARRAYOID),
            pg_sys::pg11_specific::TIMESTAMPTZARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::TIMESTAMPTZARRAYOID)
            }
            pg_sys::pg11_specific::INTERVALARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::INTERVALARRAYOID)
            }
            pg_sys::pg11_specific::NUMERICARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::NUMERICARRAYOID)
            }
            pg_sys::pg11_specific::UUIDARRAYOID => PgOid::BuiltIn(PgBuiltInOids::UUIDARRAYOID),
            pg_sys::pg11_specific::REGPROCEDUREARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGPROCEDUREARRAYOID)
            }
            pg_sys::pg11_specific::REGOPERARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGOPERARRAYOID)
            }
            pg_sys::pg11_specific::REGOPERATORARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGOPERATORARRAYOID)
            }
            pg_sys::pg11_specific::REGCLASSARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGCLASSARRAYOID)
            }
            pg_sys::pg11_specific::REGROLEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGROLEARRAYOID)
            }
            pg_sys::pg11_specific::REGNAMESPACEARRAYOID => {
                PgOid::BuiltIn(PgBuiltInOids::REGNAMESPACEARRAYOID)
            }
            pg_sys::pg11_specific::PG_LSNARRAYOID => PgOid::BuiltIn(PgBuiltInOids::PG_LSNARRAYOID),

            custom_oid => PgOid::Custom(custom_oid),
        }
    }

    pub fn value(self) -> pg_sys::Oid {
        match self {
            PgOid::InvalidOid => pg_sys::InvalidOid,
            PgOid::Custom(custom) => custom as pg_sys::Oid,
            PgOid::BuiltIn(builtin) => builtin as pg_sys::Oid,
        }
    }
}
