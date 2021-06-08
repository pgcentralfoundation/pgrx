fn main() -> Result<(), Box<dyn std::error::Error>>{
    strings::PgxSchema::generate().to_file("sql/generated.sql")
}