fn main() -> Result<(), Box<dyn std::error::Error>>{
    operators::PgxSchema::generate().to_file("sql/generated.sql")
}