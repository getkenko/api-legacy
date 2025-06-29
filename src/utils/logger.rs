use tracing::Level;
use tracing_subscriber::{fmt::{self, time::ChronoLocal}, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_logger(level: Level) {
    let fmt_layer = fmt::layer()
        .with_ansi(true)
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_timer(ChronoLocal::new("%a, %b %d, %H:%M:%S%.3f".to_string()));

    // TODO: json file logging

    let filter = EnvFilter::from_default_env().add_directive(level.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}