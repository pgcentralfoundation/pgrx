fn main() -> Result<(), Box<dyn std::error::Error>>{
    arrays::PgxSql::generate().to_file("sql/generated.sql")
}