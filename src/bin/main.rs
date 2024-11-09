use update_telemetry_reporter;

fn main() {
    env_logger::init();
    update_telemetry_reporter::configure_glean();
    let num = update_telemetry_reporter::report_state();
    let mut answer = String::new();
    match num {
        None => answer.push_str("nothing"),
        Some(val) => answer.push_str(&val.to_string  ())
    }
    println!("The number is {}", &answer);
    update_telemetry_reporter::send_ping();
}
