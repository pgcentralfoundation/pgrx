fn main() -> Result<(), Box<dyn std::error::Error>>{
    bgworker::PgxSchema::generate().to_file("sql/generated.sql")
}