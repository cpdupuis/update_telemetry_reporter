use glean::net;
use log::{debug, error};

#[derive(Debug)]
pub struct MyHttpUploader;

impl net::PingUploader for MyHttpUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        let mut req = ureq::post(&upload_request.url);
        for header in &upload_request.headers {
            req = req.set(&header.0, &header.1);
        }
        let res = req.send_bytes(&upload_request.body.as_slice());
        match res {
            Ok(response) => {
                debug!("Successfully uploaded telemetry");
                return net::UploadResult::http_status(response.status() as i32)
            },
            Err(err) => {
                error!("Failed to upload telemetry: {}", err.to_string());
                return net::UploadResult::http_status(400);
            }
        }
    }
}