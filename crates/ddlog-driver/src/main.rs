use anyhow::{Context, Result};
use ddlog_diagnostics::Interner;
use ddlog_driver::DDlogStatus;
use ddlog_lsp::{Backend, Session};
use ddlog_utils::Arc;
use std::{
    convert::identity,
    num::NonZeroUsize,
    panic,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
use tokio::{io, net::TcpListener, runtime::Builder};
use tower_lsp::{LspService, Server};

/// The address of the socket the LSP will bind to
const SOCKET_ADDRESS: &str = "127.0.0.1:5007";

fn main() -> DDlogStatus {
    panic::catch_unwind(|| -> Result<()> {
        ddlog_driver::set_panic_hook();

        ddlog_driver::set_logger()?;

        // Get the number of available threads
        let available_threads =
            thread::available_parallelism().unwrap_or(NonZeroUsize::new(8).expect("8 is not zero"));

        // Build our tokio runtime
        tracing::info!("creating tokio runtime with {available_threads} worker threads");
        let runtime = Builder::new_multi_thread()
            .worker_threads(available_threads.get())
            .thread_name_fn(|| {
                static WORKER_ID: AtomicUsize = AtomicUsize::new(0);

                format!("ddlog-worker-{}", WORKER_ID.fetch_add(1, Ordering::Relaxed))
            })
            .thread_stack_size(3 * 1024 * 1024)
            .build()
            .context("failed to build tokio runtime")?;

        runtime.block_on(async {
            tracing::info!("binding server to tcp socket {SOCKET_ADDRESS}");
            let listener = TcpListener::bind(SOCKET_ADDRESS).await.with_context(|| {
                format!("failed to bind tcp listener to socket {SOCKET_ADDRESS}")
            })?;

            tracing::info!("accepting tcp connections from {SOCKET_ADDRESS}");
            let (stream, _) = listener.accept().await.with_context(|| {
                format!("failed to accept tcp connection from {SOCKET_ADDRESS}")
            })?;
            let (read, write) = io::split(stream);

            tracing::info!("creating new ddlog backend");
            let session = Arc::new(Session::new(Interner::new()));
            let (service, messages) = LspService::new(|client| Backend::new(client, session));

            tracing::info!("spawning ddlog server");
            Server::new(read, write, messages).serve(service).await;

            Ok(())
        })
    })
    .map_err(|error| {
        if let Some(message) = error.downcast_ref::<&str>() {
            anyhow::anyhow!("a panic occurred within the main runtime: {message}")
        } else if let Some(message) = error.downcast_ref::<String>() {
            anyhow::anyhow!("a panic occurred within the main runtime: {message}")
        } else {
            anyhow::anyhow!("a panic occurred within the main runtime")
        }
    })
    .and_then(identity)
    // Note: We don't do anything with the error (like printing it out)
    // because panics cause the panic hook to be called regardless of whether
    // the panic is later caught, so all of our relevant panic information
    // and reporting has already happened at this point. All we do here
    // is map panics to the proper exit code
    .map_or(DDlogStatus::InternalError, |()| DDlogStatus::Success)
}
