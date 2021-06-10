fn main() -> Result<(), Box<dyn std::error::Error>>{
    spi::PgxSql::generate().to_file("sql/generated.sql")
}