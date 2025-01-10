// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glean::net;
use minreq;
#[derive(Debug)]
pub struct MyHttpUploader;

impl net::PingUploader for MyHttpUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        let header_vec: Vec<(String,String)> = upload_request.headers;
        let request = minreq::post(&upload_request.url)
            .with_headers(header_vec)
            .with_body(upload_request.body);
        let res = request.send();

        match res {
            Ok(response) => {
                return net::UploadResult::http_status(response.status_code);
            },
            Err(err) => {
                panic!("Failed to upload telemetry: {}", err.to_string());
            }
        }
    }
}
