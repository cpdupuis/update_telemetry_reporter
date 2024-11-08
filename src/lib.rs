use std::path::PathBuf;
use tempfile::Builder;
use serde::{Deserialize, Serialize};
use flate2::read::GzDecoder;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder, ErrorType};
use crate::metrics::updater::{self, CompletionCheckObject,CompletionCheckObjectItemUpdateObject,CompletionCheckObjectItemInstallationObject};


mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
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
            used_mms: Some(true)
        }),
        installation: Some(CompletionCheckObjectItemInstallationObject {
            is_shared: Some(true),
            launch_failed: Some(false),
            launch_succeeded: Some(false)
        }),
    };
    metrics::updater::completion_check.set(report);
    Some(42)
}

pub fn configure_glean() {
    let root = Builder::new().prefix("simple-db").tempdir().unwrap();
    let data_path: PathBuf = root.path().to_path_buf();
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("localhost:3030/glean")
        .with_use_core_mps(false)
        .build();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tautology() {
        assert_eq!(true, true);
    }
}