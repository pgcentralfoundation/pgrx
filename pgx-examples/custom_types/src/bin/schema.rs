fn main() -> Result<(), Box<dyn std::error::Error>>{
    schemas::PgxSchema::generate().to_file("sql/generated.sql")
}