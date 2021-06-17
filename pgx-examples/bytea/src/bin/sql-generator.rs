fn main() -> Result<(), Box<dyn std::error::Error>>{
    bytea::generate_sql()?.to_file("sql/bytea.sql")
}