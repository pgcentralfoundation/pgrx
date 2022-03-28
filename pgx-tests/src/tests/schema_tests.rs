/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;

#[pgx::pg_schema]
mod test_schema {
    use pgx::utils::sql_entity_graph::{PgxSql, SqlGraphEntity};
    use pgx::*;
    use serde::{Deserialize, Serialize};

    #[pg_extern]
    fn func_in_diff_schema() {}

    #[pg_extern(sql = false)]
    fn func_elided_from_schema() {}

    #[pg_extern(sql = generate_function)]
    fn func_generated_with_custom_sql() {}

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    pub struct TestType(pub u64);

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    #[pgx(sql = false)]
    pub struct ElidedType(pub u64);

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    #[pgx(sql = generate_type)]
    pub struct OtherType(pub u64);

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    #[pgx(sql = "CREATE TYPE test_schema.ManuallyRenderedType;")]
    pub struct OverriddenType(pub u64);

    fn generate_function(
        entity: &SqlGraphEntity,
        _context: &PgxSql,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        if let SqlGraphEntity::Function(ref func) = entity {
            Ok(format!("\
                CREATE OR REPLACE FUNCTION test_schema.\"func_generated_with_custom_name\"() RETURNS void\n\
                LANGUAGE c /* Rust */\n\
                AS 'MODULE_PATHNAME', '{unaliased_name}_wrapper';\
                ",
                unaliased_name = func.unaliased_name,
            ))
        } else {
            panic!("expected extern function entity, got {:?}", entity);
        }
    }

    fn generate_type(
        entity: &SqlGraphEntity,
        _context: &PgxSql,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        if let SqlGraphEntity::Type(ref ty) = entity {
            Ok(format!(
                "\n\
                CREATE TYPE test_schema.Custom{name};\
                ",
                name = ty.name,
            ))
        } else {
            panic!("expected type entity, got {:?}", entity);
        }
    }
}

#[pg_extern(schema = "test_schema")]
fn func_in_diff_schema2() {}

#[pg_extern]
fn type_in_diff_schema() -> test_schema::TestType {
    test_schema::TestType(1)
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_in_different_schema() {
        Spi::run("SELECT test_schema.func_in_diff_schema();");
    }

    #[pg_test]
    fn test_in_different_schema2() {
        Spi::run("SELECT test_schema.func_in_diff_schema2();");
    }

    #[pg_test]
    fn test_type_in_different_schema() {
        Spi::run("SELECT type_in_diff_schema();");
    }

    #[pg_test]
    fn elided_extern_is_elided() {
        // Validate that a function we know exists, exists
        let result: bool = Spi::get_one(
            "SELECT exists(SELECT 1 FROM pg_proc WHERE proname = 'func_in_diff_schema');",
        )
        .expect("expected result");
        assert_eq!(result, true);

        // Validate that the function we expect not to exist, doesn't
        let result: bool = Spi::get_one(
            "SELECT exists(SELECT 1 FROM pg_proc WHERE proname = 'func_elided_from_schema');",
        )
        .expect("expected result");
        assert_eq!(result, false);
    }

    #[pg_test]
    fn elided_type_is_elided() {
        // Validate that a type we know exists, exists
        let result: bool =
            Spi::get_one("SELECT exists(SELECT 1 FROM pg_type WHERE typname = 'testtype');")
                .expect("expected result");
        assert_eq!(result, true);

        // Validate that the type we expect not to exist, doesn't
        let result: bool =
            Spi::get_one("SELECT exists(SELECT 1 FROM pg_type WHERE typname = 'elidedtype');")
                .expect("expected result");
        assert_eq!(result, false);
    }

    #[pg_test]
    fn custom_to_sql_extern() {
        // Validate that the function we generated has the modifications we expect
        let result: bool = Spi::get_one("SELECT exists(SELECT 1 FROM pg_proc WHERE proname = 'func_generated_with_custom_name');").expect("expected result");
        assert_eq!(result, true);

        Spi::run("SELECT test_schema.func_generated_with_custom_name();");
    }

    #[pg_test]
    fn custom_to_sql_type() {
        // Validate that the type we generated has the expected modifications
        let result: bool =
            Spi::get_one("SELECT exists(SELECT 1 FROM pg_type WHERE typname = 'customothertype');")
                .expect("expected result");
        assert_eq!(result, true);
    }

    #[pg_test]
    fn custom_handwritten_to_sql_type() {
        // Validate that the SQL we provided was used
        let result: bool = Spi::get_one(
            "SELECT exists(SELECT 1 FROM pg_type WHERE typname = 'manuallyrenderedtype');",
        )
        .expect("expected result");
        assert_eq!(result, true);
    }
}
