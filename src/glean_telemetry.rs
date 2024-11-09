use crate::metrics::updater::{
    CompletionCheckObject, CompletionCheckObjectItemInstallationObject,
    CompletionCheckObjectItemUpdateObject,
};
use crate::metrics;


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
    metrics::updater_check.submit(None);
}