fn main() -> Result<(), Box<dyn std::error::Error>>{
    spi::PgxSchema::generate().to_file("sql/generated.sql")
}