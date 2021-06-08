fn main() -> Result<(), Box<dyn std::error::Error>>{
    srf::PgxSchema::generate().to_file("sql/generated.sql")
}