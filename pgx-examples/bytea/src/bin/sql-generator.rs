fn main() -> Result<(), Box<dyn std::error::Error>>{
    bytea::PgxSql::generate().to_file("sql/generated.sql")
}