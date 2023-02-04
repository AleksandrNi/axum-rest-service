use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

pub async fn run() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| panic!("RUST_LOG must be set!"));

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        // .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
