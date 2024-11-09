use crate::ping_uploader::MyHttpUploader;
use glean::{ClientInfoMetrics, ConfigurationBuilder};
use std::path::PathBuf;
use tempfile::Builder;


pub fn configure_glean() {
    let root = Builder::new().prefix("simple-db").tempdir().unwrap();
    let uploader = MyHttpUploader;
    let data_path: PathBuf = root.path().to_path_buf();
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.updater_report_sample")
        .with_server_endpoint("https://incoming.telemetry.mozilla.org")
        .with_use_core_mps(false)
        .with_uploader(uploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::set_log_pings(true);
    assert!(glean::set_debug_view_tag("cdupuis-updater"));
    glean::initialize(cfg, client_info);
}
