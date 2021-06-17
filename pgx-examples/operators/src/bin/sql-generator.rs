fn main() -> Result<(), Box<dyn std::error::Error>>{
    operators::generate_sql()?.to_file("sql/operators.sql")
}