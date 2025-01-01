use crate::scrape::LocalResponse;

// # A metric indicating whether a server is online.
// # 1 means the server is online, 0 means it is offline.
// server_online{hostname="server1"} 1
// server_online{hostname="server2"} 0
pub fn create_http_response(statuses: Vec<LocalResponse>) -> String {
    let mut status_strings: Vec<String> = Vec::new();
    for status in statuses.iter() {
        if let Some(_code) = status.code {
            status_strings.push(prom_string(&status.endpoint, true));
        } else {
            status_strings.push(prom_string(&status.endpoint, false));
        }
    }

    form_response_bytes(status_strings)
}

fn prom_string(endpoint: &String, success: bool) -> String {
    let base_string = format!("server_status{{hostname=\"{}\"}} ", endpoint);

    let code = match success {
        true => "1",
        false => "0",
    };

    format!("{}{}", base_string, code)
}

fn form_response_bytes(strings: Vec<String>) -> String {
    let mut http_response: Vec<String> = vec![
        // header
        "HTTP/1.1 200 OK".to_string(),
        "Content-Type: text/plain; charset=utf-8".to_string(),
        // format!("Content-Length: {}", ) no
        "Connection: Close".to_string(),
        "".to_string(),
        "# HELP server_online Indicates whether the server is online (1 = online, 0 = offline)"
            .to_string(),
        "# TYPE server_online gauge".to_string(),
    ];

    strings.iter().for_each(|s| http_response.push(s.clone()));

    http_response.push("\n".to_string());
    http_response.join("\n")
}
