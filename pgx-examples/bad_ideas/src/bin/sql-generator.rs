fn main() -> Result<(), Box<dyn std::error::Error>>{
    bad_ideas::generate_sql()?.to_file("sql/bad_ideas.sql")
}