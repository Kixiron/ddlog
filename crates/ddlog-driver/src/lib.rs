mod status;

pub use status::DDlogStatus;

use human_panic::setup_panic;
use tracing_subscriber::{filter::EnvFilter, fmt};

/// Setup the lsp's logger to take log levels from the `DDLOG_LOG` env var, see
/// [`EnvFilter`] for more info on log directives
pub fn set_logger() {
    use tracing_subscriber::prelude::*;

    let env = EnvFilter::from_env("DDLOG_LOG");
    let formatter = fmt::layer().with_thread_names(true);

    tracing_subscriber::registry()
        .with(env)
        .with(formatter)
        .init();

    tracing::info!("logging hook has been set");
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
