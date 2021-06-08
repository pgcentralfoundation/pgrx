fn main() -> Result<(), Box<dyn std::error::Error>>{
    shmem::PgxSchema::generate().to_file("sql/generated.sql")
}