fn main() -> Result<(), Box<dyn std::error::Error>>{
    bytea::PgxSchema::generate().to_file("sql/generated.sql")
}