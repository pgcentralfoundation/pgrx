use ::pgx_utils::pg_inventory::{
    tracing_error::ErrorLayer,
    tracing,
    tracing_subscriber::{self, util::SubscriberInitExt, layer::SubscriberExt, EnvFilter},
    color_eyre,
    eyre,
};
use std::env;

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

    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or("./sql/bgworker.sql".into());
    if args.next().is_some() {
        return Err(eyre::eyre!("Only accepts one argument, the destination path."));
    }

    tracing::info!(path = %path, "Writing SQL.");
    bgworker::generate_sql()?.to_file(path)?;
    Ok(())
}