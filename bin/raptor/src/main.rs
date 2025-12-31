
use nice_common::tracing::init_trace_logging;
use raptor::CONFIG;
fn main() -> anyhow::Result<()> {
    let _guard = init_trace_logging(&CONFIG.log)?;
    tracing::info!("raptor initialized");
    Ok(())
}
