fn main() -> Result<(), Box<dyn std::error::Error>>{
    srf::PgxSql::generate().to_file("sql/generated.sql")
}