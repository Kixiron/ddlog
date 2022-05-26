use ddlog_diagnostics::Interner;
use ddlog_driver::DDlogStatus;
use ddlog_lsp::{Backend, Session};
use ddlog_utils::Arc;
use std::{
    num::NonZeroUsize,
    panic,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
use tokio::{io, net::TcpListener, runtime::Builder};
use tower_lsp::{LspService, Server};

fn main() -> DDlogStatus {
    ddlog_driver::set_panic_hook();

    panic::catch_unwind(|| {
        ddlog_driver::set_logger();

        // Get the number of available threads
        let available_threads =
            thread::available_parallelism().unwrap_or(NonZeroUsize::new(8).unwrap());

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
            .expect("failed to build tokio runtime");

        runtime.block_on(async {
            // TODO: Better error handling?
            tracing::info!(address = "127.0.0.1:5007", "binding to tcp socket");
            let listener = TcpListener::bind("127.0.0.1:5007")
                .await
                .expect("failed to bind tcp listener to socket");

            tracing::info!(address = "127.0.0.1:5007", "accepting tcp connections");
            let (stream, _) = listener
                .accept()
                .await
                .expect("failed to accept tcp connection");
            let (read, write) = io::split(stream);

            tracing::info!("creating new ddlog backend");
            let session = Arc::new(Session::new(Interner::new()));
            let (service, messages) = LspService::new(|client| Backend::new(client, session));

            tracing::info!("spawning ddlog server");
            Server::new(read, write, messages).serve(service).await;
        })
    })
    .map_or(DDlogStatus::InternalError, |_| DDlogStatus::Success)
}
