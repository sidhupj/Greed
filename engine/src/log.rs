use std::io::stdout;

use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, prelude::*};

pub struct Log {
    _engine_guard: WorkerGuard,
    _app_guard: WorkerGuard,
}

impl Log {
    pub fn init_non_blocking_console() -> Self {
        let (engine_writer, engine_guard) = non_blocking(stdout());
        let (app_writer, app_guard) = non_blocking(stdout());

        let engine_layer = fmt::layer()
            .with_writer(engine_writer)
            .with_ansi(true)
            .with_filter(EnvFilter::new("engine=trace"));

        let app_layer = fmt::layer()
            .with_writer(app_writer)
            .with_ansi(true)
            .with_filter(EnvFilter::new("greed=trace"));

        tracing_subscriber::registry()
            .with(engine_layer)
            .with(app_layer)
            .init();

        Log {
            _engine_guard: engine_guard,
            _app_guard: app_guard,
        }
    }
}
