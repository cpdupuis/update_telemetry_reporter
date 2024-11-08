use update_telemetry_reporter;

fn main() {
    println!("Hello, world!");
    let packet = update_telemetry_reporter::collect_data();
    println!("version to install is {}", packet.update.version_to_install);
    let num = update_telemetry_reporter::report_state();
    let mut answer = String::new();
    match num {
        None => answer.push_str("nothing"),
        Some(val) => answer.push_str(&val.to_string  ())
    }
    println!("The number is {}", &answer);
}
