use std::net::TcpListener;

use http_home::run;
use http_home::home;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4083")?;
    run(listener, home::Home::restore())?.await
}
