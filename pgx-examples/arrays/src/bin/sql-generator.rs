fn main() -> Result<(), Box<dyn std::error::Error>>{
    arrays::generate_sql()?.to_file("sql/arrays.sql")
}