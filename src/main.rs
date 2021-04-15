mod datapoint;
mod input;
mod tests;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    return input::accept_loop("127.0.0.1:6142").await;
}
