//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};

unsafe impl SqlTranslatable for crate::FunctionCallInfoBaseData {
    const SCHEMA_KEY: &'static str = "FunctionCallInfoBaseData";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::Skip);
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = Ok(ReturnsRef::One(SqlMappingRef::Skip));
}

unsafe impl SqlTranslatable for crate::PlannerInfo {
    const SCHEMA_KEY: &'static str = "PlannerInfo";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("internal"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("internal")));
}

unsafe impl SqlTranslatable for crate::IndexAmRoutine {
    const SCHEMA_KEY: &'static str = "IndexAmRoutine";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("internal"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("internal")));
}

unsafe impl SqlTranslatable for crate::TableAmRoutine {
    const SCHEMA_KEY: &'static str = "TableAmRoutine";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("internal"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("internal")));
}

unsafe impl SqlTranslatable for crate::FdwRoutine {
    const SCHEMA_KEY: &'static str = "FdwRoutine";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("fdw_handler"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("fdw_handler")));
}

unsafe impl SqlTranslatable for crate::BOX {
    const SCHEMA_KEY: &'static str = "BOX";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("box"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("box")));
}

unsafe impl SqlTranslatable for crate::CIRCLE {
    const SCHEMA_KEY: &'static str = "CIRCLE";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("circle"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("circle")));
}

unsafe impl SqlTranslatable for crate::Point {
    const SCHEMA_KEY: &'static str = "Point";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("point"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("point")));
}

unsafe impl SqlTranslatable for crate::ItemPointerData {
    const SCHEMA_KEY: &'static str = "ItemPointerData";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("tid"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("tid")));
}

unsafe impl SqlTranslatable for crate::Datum {
    const SCHEMA_KEY: &'static str = "Datum";
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Err(ArgumentError::Datum);
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = Err(ReturnsError::Datum);
}
