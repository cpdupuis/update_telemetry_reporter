use crate::metrics::updater::{
    CompletionCheckObject, CompletionCheckObjectItemInstallationObject,
    CompletionCheckObjectItemUpdateObject,
};
use crate::metrics::updater_check;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder, ErrorType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::Builder;

mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}


/// A simple mechanism to upload pings over HTTPS.
#[derive(Debug)]
pub struct MyHttpUploader {
    hello_there: String
}

impl net::PingUploader for MyHttpUploader {
    /// Uploads a ping to a server.
    ///
    /// # Arguments
    ///
    /// * `upload_request` - the requested upload.
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        // haha. You thought this was an uploader
        println!("hello_there: {}", self.hello_there);
        net::UploadResult::http_status(200)
    }
}


pub fn report_state() -> Option<i32> {
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
    Some(42)
}

pub fn send_ping() {

    /*updater_check.test_before_next_submit(move |reason| {
        println!("yippee");
        match reason {
            None => assert!(false),
            Some(val) => println!("reason is {}", val)
        }
    });
    let metric_value = metrics::updater::completion_check.test_get_value("updater-check");
    match metric_value {
        None => assert!(false),
        Some(str) => println!("Metric: {}", str)
    }
*/
    updater_check.submit(Some("because"));
    thread::sleep(Duration::from_millis(1000));
    glean::shutdown();
}

pub fn configure_glean() {
    let root = Builder::new().prefix("simple-db").tempdir().unwrap();
    match root.path().to_str() {
        Some(str) => println!("This is root dir: {}", str),
        None => println!("No root"),
    }
    let uploader = MyHttpUploader {
        hello_there: String::from("General Kenobi")
    };
    let data_path: PathBuf = root.path().to_path_buf();
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.updater_report_sample")
        .with_server_endpoint("https://incoming.telemetry.mozilla.org")
        .with_use_core_mps(true)
        .with_uploader(uploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: Some(String::from("pre-alpha")),
        locale: Some(String::from("en-us")),
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
