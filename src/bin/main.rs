use update_telemetry_reporter::collect_data;

fn main() {
    println!("Hello, world!");
    let packet = collect_data();
    println!("version to install is {}", packet.update.version_to_install);
}
