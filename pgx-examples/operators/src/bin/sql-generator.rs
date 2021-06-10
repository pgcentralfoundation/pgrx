fn main() -> Result<(), Box<dyn std::error::Error>>{
    operators::PgxSql::generate().to_file("sql/generated.sql")
}