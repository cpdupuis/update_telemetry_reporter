mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

pub enum BasicStatus {
    Success,
    Error,
}

pub enum UpdateType {
    Patch,
    Full,
    Recovery
}

pub struct ProfileSample {
    pub last_version: Option<String>
}

pub struct UpdateSample {
    pub install_status: BasicStatus,
    pub rollback_status: Option<BasicStatus>,
    pub previous_installation_version: Option<String>,
    pub version_to_install: String,
    pub update_type: UpdateType,
    pub seconds_since_update: Option<u32>,
    pub using_maintenance_service: bool,
    pub maintenance_service_version: Option<String>
}

struct InstallationSample {
    pub is_shared: bool,
    pub launch_succeeded_since_last_update: bool,
    pub launch_failed_since_last_update: bool
}

pub struct TelemetryPacket {
    pub profile: ProfileSample,
    pub update: UpdateSample,
    pub installation: InstallationSample
}

fn collect_profile_sample() -> ProfileSample {
    ProfileSample {
        last_version: Some(String::from("1.2.3.4"))
    }
}

fn collect_update_sample() -> UpdateSample {
    UpdateSample {
        install_status: BasicStatus::Success,
        rollback_status: None,
        previous_installation_version: Some(String::from("0.0.0.1")),
        version_to_install: String::from("2.0.0.0"),
        update_type: UpdateType::Patch,
        seconds_since_update: Some(1234u32),
        using_maintenance_service: true,
        maintenance_service_version: Some(String::from("1.1.1.1"))
    }
}

fn collect_installation_sample() -> InstallationSample {
    InstallationSample {
        is_shared: true,
        launch_failed_since_last_update: false,
        launch_succeeded_since_last_update: true
    }
}

pub fn collect_data() -> TelemetryPacket {
    TelemetryPacket {
        profile: collect_profile_sample(),
        update: collect_update_sample(),
        installation: collect_installation_sample()
    }
}

pub fn report_state() -> Option<i32> {
    use crate::metrics::updater::{CompletionCheckObject,CompletionCheckObjectItemUpdateObject,CompletionCheckObjectItemInstallationObject};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_data() {
        let telemetry_packet = collect_data();
        assert_eq!(true, telemetry_packet.update.using_maintenance_service);
    }
}