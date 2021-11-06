use slog;
use slog_term;
use slog_async;

use slog::{Drain, Logger, o};

/// Create a logger for structured logging
/// This logger logs to std err
pub fn create_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

	Logger::root(drain, o!())
}