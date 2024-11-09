mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

mod ping_uploader;

pub mod glean_config;

pub mod glean_telemetry;
