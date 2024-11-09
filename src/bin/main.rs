use update_telemetry_reporter;

fn main() {
    env_logger::init();
    update_telemetry_reporter::configure_glean();
    update_telemetry_reporter::report_state();
    update_telemetry_reporter::send_ping();
}
