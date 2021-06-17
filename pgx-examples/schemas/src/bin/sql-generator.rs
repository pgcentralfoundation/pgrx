fn main() -> Result<(), Box<dyn std::error::Error>>{
    schemas::generate_sql()?.to_file("sql/arrays.sql")
}