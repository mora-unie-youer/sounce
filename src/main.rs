use anyhow::Result;
use tracing::{debug, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    // Fetching env-filter
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .with_env_var("SOUNCE_LOG")
        .from_env_lossy();

    // Setting up subscriber that prints formatted traces
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    debug!("Logging was initialized successfully");

    Ok(())
}
