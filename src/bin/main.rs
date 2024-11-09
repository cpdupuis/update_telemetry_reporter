use update_telemetry_reporter;
fn main() {
    env_logger::init();
    update_telemetry_reporter::glean_config::configure_glean();
    update_telemetry_reporter::glean_telemetry::report_state();
    update_telemetry_reporter::glean_telemetry::send_ping();
    update_telemetry_reporter::glean_config::shutdown_glean();
}
