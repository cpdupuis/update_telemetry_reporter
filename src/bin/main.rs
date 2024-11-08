use update_telemetry_reporter;

fn main() {
    update_telemetry_reporter::configure_glean();
    let num = update_telemetry_reporter::report_state();
    let mut answer = String::new();
    match num {
        None => answer.push_str("nothing"),
        Some(val) => answer.push_str(&val.to_string  ())
    }
    println!("The number is {}", &answer);
}
