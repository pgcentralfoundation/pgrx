fn main() -> Result<(), Box<dyn std::error::Error>>{
    strings::PgxSql::generate().to_file("sql/generated.sql")
}