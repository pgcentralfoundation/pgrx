use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

#[pg_extern]
fn normalize_phrase(input: &str) -> String {
    std::thread::sleep_ms(1);
    input
        .split_whitespace()
        .map(|word| word.trim_matches(|ch: char| !ch.is_alphanumeric()).to_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(feature = "pg_bench")]
#[pg_schema]
mod benches {
    use pgrx::prelude::*;
    use pgrx_bench::{black_box, BatchSize, Bencher};

    fn prepare_spi_fixture() {
        Spi::run(
            "CREATE UNLOGGED TABLE IF NOT EXISTS bench_sink (
                value integer NOT NULL
            )",
        )
        .unwrap();
        Spi::run("TRUNCATE bench_sink").unwrap();
    }

    #[pg_bench]
    fn bench_normalize_phrase(b: &mut Bencher) {
        let input = "The QUICK, Brown fox jumped over the lazy dog";
        b.iter(|| black_box(crate::normalize_phrase(black_box(input))));
    }

    #[pg_bench(
        setup = prepare_spi_fixture,
        transaction = "subtransaction_per_batch",
        sample_size = 50,
        measurement_time_ms = 2_000
    )]
    fn bench_spi_insert_batch(b: &mut Bencher) {
        b.iter_batched(
            || (0..32).collect::<Vec<i32>>(),
            |values| {
                for value in values {
                    Spi::run(&format!("INSERT INTO bench_sink VALUES ({value})")).unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    }
}
