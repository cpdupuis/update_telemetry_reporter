fn main() {
    println!("Hello, world!");
    let packet = collect_data();
    println!("version to install is {}", packet.update.version_to_install);
}



enum BasicStatus {
    Success,
    Error,
}

enum UpdateType {
    Patch,
    Full,
    Recovery
}

struct ProfileSample {
    last_version: Option<String>
}

struct UpdateSample {
    install_status: BasicStatus,
    rollback_status: Option<BasicStatus>,
    previous_installation_version: Option<String>,
    version_to_install: String,
    update_type: UpdateType,
    seconds_since_update: Option<u32>,
    using_maintenance_service: bool,
    maintenance_service_version: Option<String>
}

struct InstallationSample {
    is_shared: bool,
    launch_succeeded_since_last_update: bool,
    launch_failed_since_last_update: bool
}

struct TelemetryPacket {
    profile: ProfileSample,
    update: UpdateSample,
    installation: InstallationSample
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

fn collect_data() -> TelemetryPacket {
    TelemetryPacket {
        profile: collect_profile_sample(),
        update: collect_update_sample(),
        installation: collect_installation_sample()
    }
}
