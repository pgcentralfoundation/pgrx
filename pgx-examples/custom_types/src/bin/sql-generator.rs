fn main() -> Result<(), Box<dyn std::error::Error>>{
    schemas::PgxSql::generate().to_file("sql/generated.sql")
}