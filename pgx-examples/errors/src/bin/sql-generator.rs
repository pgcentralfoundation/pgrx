fn main() -> Result<(), Box<dyn std::error::Error>>{
    errors::generate_sql()?.to_file("sql/arrays.sql")
}