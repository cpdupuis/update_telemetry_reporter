fn main() {
    println!("Hello, world!");
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

fn collect_data()  {

}