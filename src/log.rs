use bevy::prelude::*;

pub struct CustomLogPlugin;

use bevy::log::tracing_subscriber::Layer;

use bevy::log::tracing::{level_filters::LevelFilter, subscriber::set_global_default};
use std::sync::OnceLock;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::layer::SubscriberExt;

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

impl Plugin for CustomLogPlugin {
    fn build(&self, app: &mut App) {
        use bevy::log::tracing_subscriber::Registry;
        let finished_subscriber;
        let subscriber = Registry::default();

        let file_appender = rolling::hourly("logs", "app.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let _ = LOG_GUARD.set(guard);
        let file_layer = bevy::log::tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking)
            .with_file(true)
            .with_line_number(true)
            .with_filter(LevelFilter::TRACE)
            .boxed();

        let subscriber = subscriber.with(file_layer);

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_file(true)
            .with_line_number(true)
            .with_filter(LevelFilter::INFO)
            .boxed();

        let subscriber = subscriber.with(fmt_layer);
        finished_subscriber = subscriber;

        let already_set = set_global_default(finished_subscriber).is_err();
    }
}
