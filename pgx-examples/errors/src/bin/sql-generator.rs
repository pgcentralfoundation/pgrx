fn main() -> Result<(), Box<dyn std::error::Error>>{
    errors::PgxSql::generate().to_file("sql/generated.sql")
}