use http::StatusCode;
use log::error;
use tokio::task::JoinHandle;

use crate::config::CONFIG_CELL;

#[derive(Debug)]
struct LocalRequest {
    handle: JoinHandle<Result<reqwest::Response, reqwest::Error>>,
    endpoint: String,
}

#[derive(Debug)]
pub struct LocalResponse {
    /// none -> error
    /// some -> success?
    pub code: Option<StatusCode>,
    pub endpoint: String,
}

pub async fn send_requests() -> Vec<LocalResponse> {
    let mut handles: Vec<LocalRequest> = Vec::new();
    for endpoint in &CONFIG_CELL.get().unwrap().targets.hosts {
        let request = reqwest::Client::builder()
            .danger_accept_invalid_certs(CONFIG_CELL.get().unwrap().server.accept_invalid_certs)
            .build()
            .unwrap()
            .get(endpoint)
            .send();

        let localrequest = LocalRequest {
            endpoint: endpoint.clone(),
            handle: tokio::task::spawn(request),
        };

        handles.push(localrequest);
    }

    let mut results: Vec<LocalResponse> = Vec::new();

    for i in handles {
        let response = match i.handle.await.unwrap() {
            Ok(r) => {
                LocalResponse {
                    code: Some(r.status()),
                    endpoint: i.endpoint,
                    // error: None,
                }
            }
            Err(e) => {
                error!("an error occurred: {e}");
                LocalResponse {
                    code: None,
                    endpoint: i.endpoint,
                    // error: Some(Box::from(e)),
                }
            }
        };

        results.push(response);
    }

    results
}
