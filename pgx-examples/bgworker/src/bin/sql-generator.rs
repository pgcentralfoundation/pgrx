fn main() -> Result<(), Box<dyn std::error::Error>>{
    bgworker::PgxSql::generate().to_file("sql/generated.sql")
}