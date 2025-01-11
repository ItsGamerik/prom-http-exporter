use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use log::{error, info};
use reqwest::StatusCode;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::config::CONFIG_CELL;

#[derive(Debug)]
struct LocalRequest {
    handle: JoinHandle<Result<reqwest::Response, reqwest::Error>>,
    endpoint: String,
}

#[derive(Debug, Clone)]
pub struct LocalResponse {
    /// none -> error
    /// some -> success?
    pub code: Option<StatusCode>,
    pub endpoint: String,
}

pub fn init() -> Arc<RwLock<Vec<LocalResponse>>> {
    let result = Arc::new(RwLock::new(Vec::new()));
    let request_loop = infinite_loop(result.clone());

    tokio::spawn(request_loop);

    Arc::clone(&result)
}

pub async fn get_results(results: Arc<RwLock<Vec<LocalResponse>>>) -> Vec<LocalResponse>{
    // dbg!("awaiting lock (get)");
    let data = results.read().await;
    // dbg!("cleared lock");

    let clone2 = data.clone();
    drop(data);
    clone2
}

async fn infinite_loop(results: Arc<RwLock<Vec<LocalResponse>>>) {
    loop {
        let clone = Arc::clone(&results);
        send_requests(&clone).await;
        drop(clone);
        sleep(Duration::from_secs(
            CONFIG_CELL.get().unwrap().server.scrape_velocity,
        ));
    }
}

async fn send_requests(thing: &Arc<RwLock<Vec<LocalResponse>>>) {
    info!("start sending requests");
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


    // dbg!("awaiting lock (set)");
    let mut lock = thing.write().await;

    *lock = results;

    // dbg!("cleared lock");
}
