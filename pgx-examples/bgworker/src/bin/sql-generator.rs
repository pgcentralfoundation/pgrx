fn main() -> Result<(), Box<dyn std::error::Error>>{
    bgworker::generate_sql()?.to_file("sql/bgworker.sql")
}