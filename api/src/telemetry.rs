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
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{Registry, layer::SubscriberExt};

pub fn get_subscriber<Sink1>(filter: LevelFilter, sink: Sink1) -> impl Subscriber + Send + Sync
where
    Sink1: for<'a> MakeWriter<'a> + Clone + Send + Sync + 'static,
{
    // Layer
    let layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(sink)
        .with_target(false)
        .with_ansi(true)
        .with_timer(ChronoLocal::new("%H:%M:%S%.3f".into()))
        .with_filter(filter);

    // Registry
    Registry::default().with(layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
