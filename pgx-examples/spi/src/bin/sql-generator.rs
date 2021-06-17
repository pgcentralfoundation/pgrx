fn main() -> Result<(), Box<dyn std::error::Error>>{
    spi::generate_sql()?.to_file("sql/spi.sql")
}