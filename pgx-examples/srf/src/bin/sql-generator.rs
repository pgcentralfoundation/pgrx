fn main() -> Result<(), Box<dyn std::error::Error>>{
    srf::generate_sql()?.to_file("sql/srf.sql")
}