//! Tracing levels
//! Trace - Only when I would be "tracing" the code and trying to find one part of a function specifically.
//! Debug - Information that is diagnostically helpful to people more than just developers (IT, sysadmins, etc.).
//! Info  - Generally useful information to log (service start/stop, configuration assumptions, etc).
//!         Info I want to always have available but usually don't care about under normal circumstances.
//!         This is my out-of-the-box config level.
//! Warn  - Anything that can potentially cause application oddities, but for which I am automatically recovering.
//!         (Such as switching from a primary to backup server, retrying an operation, missing secondary data, etc.)
//! Error - Any error which is fatal to the operation, but not the service or application (can't open a required file,
//!         missing data, etc.). These errors will force user (administrator, or direct user) intervention.
//!         These are usually reserved (in my apps) for incorrect connection strings, missing services, etc.

use tracing::level_filters::LevelFilter;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::{Registry, layer::SubscriberExt};

pub fn get_subscriber(filter: LevelFilter) -> impl Subscriber + Send + Sync {
    // Json Layer
    let json_formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(true)
        .with_span_list(true)
        .with_timer(ChronoUtc::rfc_3339())
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::io::stdout)
        .with_filter(filter);

    // Registry
    Registry::default().with(json_formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
