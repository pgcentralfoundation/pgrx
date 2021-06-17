fn main() -> Result<(), Box<dyn std::error::Error>>{
    custom_types::generate_sql()?.to_file("sql/custom_types.sql")
}