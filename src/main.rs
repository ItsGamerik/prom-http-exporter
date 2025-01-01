use std::{env, io, process::exit};

use config::{read_config, CONFIG_CELL};
use log::info;
use prom_response::create_http_response;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

mod config;
mod prom_response;
mod scrape;

use scrape::send_requests;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::new().parse_filters("info").init();
    let conf_path = parse_path();
    read_config(conf_path.unwrap_or("./config.toml".to_string()));

    let host = &CONFIG_CELL.get().unwrap().server.host;
    let port = CONFIG_CELL.get().unwrap().server.port;

    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    info!("bound to address {}:{}", host, port);

    for target in &CONFIG_CELL.get().unwrap().targets.hosts {
        info!("monitoring target {}", target);
    }

    loop {
        let (socket, _) = listener.accept().await?;
        handle_conn(socket).await;
    }
}

async fn handle_conn(mut stream: TcpStream) {
    info!("got request!");

    let buffer = BufReader::new(&mut stream);
    let _request = buffer
        .lines()
        .next_line()
        .await
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());

    let response_string = create_http_response(send_requests().await);

    stream.write_all(response_string.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn parse_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        let path = match arg.as_str().trim() {
            "--help" => {
                println!("usage: {} [PATH TO CONFIG]", args.first().unwrap());
                exit(0);
            }
            "-h" => {
                println!("usage: {} [PATH TO CONFIG]", args.first().unwrap());
                exit(0);
            }
            a => a,
        };
        info!("using path \"{}\"", path);
        Some(path.to_string())
    } else {
        None
    }
}
