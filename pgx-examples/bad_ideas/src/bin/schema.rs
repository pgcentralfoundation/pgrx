fn main() -> Result<(), Box<dyn std::error::Error>>{
    bad_ideas::PgxSchema::generate().to_file("sql/generated.sql")
}