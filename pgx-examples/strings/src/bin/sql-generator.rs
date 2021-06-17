fn main() -> Result<(), Box<dyn std::error::Error>>{
    strings::generate_sql()?.to_file("sql/arrays.sql")
}