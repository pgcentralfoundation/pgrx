fn main() -> Result<(), Box<dyn std::error::Error>>{
    shmem::generate_sql()?.to_file("sql/shmem.sql")
}