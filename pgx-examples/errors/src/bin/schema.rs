fn main() -> Result<(), Box<dyn std::error::Error>>{
    errors::PgxSchema::generate().to_file("sql/generated.sql")
}