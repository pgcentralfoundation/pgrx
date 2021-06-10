fn main() -> Result<(), Box<dyn std::error::Error>>{
    bad_ideas::PgxSql::generate().to_file("sql/generated.sql")
}