fn main() -> Result<(), Box<dyn std::error::Error>>{
    arrays::PgxSchema::generate().to_file("sql/generated.sql")
}