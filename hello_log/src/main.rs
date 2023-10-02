use slog::{Level, Drain, Logger};

fn main() {
    let log_level = Level::Trace;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = Logger::root(drain, slog::o!());

    // Log a trace-level message
    slog::trace!(logger, "This is a trace message");

    // Log messages at other log levels
    slog::info!(logger, "This is an info message");
    slog::warn!(logger, "This is a warning message");
    slog::error!(logger, "This is an error message");
}
