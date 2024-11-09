use crate::metrics::updater::{
    CompletionCheckObject, CompletionCheckObjectItemInstallationObject,
    CompletionCheckObjectItemUpdateObject,
};
use crate::metrics::updater_check;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::Builder;

mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
pub struct MyHttpUploader;

impl net::PingUploader for MyHttpUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        let mut req = ureq::post(&upload_request.url);
        for header in &upload_request.headers {
            req = req.set(&header.0, &header.1);
        }
        let res = req.send_bytes(&upload_request.body.as_slice());
        match res {
            Ok(response) => {
                println!("SUCCESS!!!!");
                return net::UploadResult::http_status(response.status() as i32)
            },
            Err(err) => {
                println!("Failure... {}", err.to_string());
                return net::UploadResult::http_status(400);
            }
        }
    }
}

pub fn report_state() {
    let report = CompletionCheckObject {
        profile_last_version: Some(String::from("hello there")),
        update: Some(CompletionCheckObjectItemUpdateObject {
            installed_ok: Some(true),
            rolled_back_ok: None,
            previous_version: Some(String::from("123")),
            update_version: Some(String::from("456")),
            is_patch: Some(true),
            time_since_update: Some(65536),
            mms_version: Some(String::from("11")),
            used_mms: Some(true),
        }),
        installation: Some(CompletionCheckObjectItemInstallationObject {
            is_shared: Some(true),
            launch_failed: Some(false),
            launch_succeeded: Some(false),
        }),
    };
    metrics::updater::completion_check.set(report);
}

pub fn send_ping() {
    updater_check.submit(None);
    thread::sleep(Duration::from_millis(1000));
    glean::shutdown();
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tautology() {
        assert_eq!(true, true);
    }
}
