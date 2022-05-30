mod panic;
mod status;

pub use status::DDlogStatus;

use anyhow::{Context, Result};
use atty::Stream;
use human_panic::setup_panic;
use std::io;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, Registry};
use tracing_tree::HierarchicalLayer;

/// Setup the lsp's logger to take log levels from the `DDLOG_LOG` env var, see
/// [`EnvFilter`] for more info on log directives
pub fn set_logger() -> Result<()> {
    let env = EnvFilter::from_env("DDLOG_LOG");
    let fmt = HierarchicalLayer::new(2)
        // Write logs to stderr
        .with_writer(io::stderr)
        .with_thread_names(true)
        .with_thread_ids(false)
        // Don't use ansi codes if we're not printing to a console
        .with_ansi(atty::is(Stream::Stdout));

    let registry = Registry::default().with(env).with(fmt);
    tracing::subscriber::set_global_default(registry)
        .context("failed to set logging registry, maybe `ddlog_driver::set_logger()` was called multiple times?")?;

    tracing::info!("logging hook has been set");

    Ok(())
}

/// Set up a panic hook to give human-friendly errors when panics occur
// TODO: Implement this manually so we can make the message more relevant
//       as well as use the `DDLOG_BACKTRACE` env var instead of `RUST_BACKTRACE`
pub fn set_panic_hook() {
    setup_panic!(Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    });
}
