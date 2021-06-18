use ::pgx_utils::pg_inventory::{
    tracing_error::ErrorLayer,
    tracing_subscriber::{self, util::SubscriberInitExt, layer::SubscriberExt, fmt, EnvFilter, Registry},
    color_eyre,
};

fn main() -> color_eyre::Result<()> {
    // Initialize tracing with tracing-error.
    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    color_eyre::install()?;

    shmem::generate_sql()?.to_file("sql/shmem.sql")?;
    Ok(())
}