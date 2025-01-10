// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use update_telemetry_reporter;
fn main() {
    update_telemetry_reporter::glean_config::configure_glean();
    update_telemetry_reporter::glean_telemetry::report_state();
    update_telemetry_reporter::glean_telemetry::send_ping();
    update_telemetry_reporter::glean_config::shutdown_glean();
}
