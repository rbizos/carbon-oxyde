use std::str;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;

use super::datapoint;

pub async fn accept_loop(bind_address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(bind_address).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut stream = BufReader::new(socket);
            loop {
                let mut line = String::new();
                stream.read_line(&mut line).await.unwrap();
                let dp = match datapoint::Datapoint::from_string(&line.trim_end()) {
                    Ok(v) => println!("{:?}", v),
                    Err(e) => println!("not a datapoint {}", e),
                };
            }
        });
    }
}
