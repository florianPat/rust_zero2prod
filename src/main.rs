use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind to random port");

    zero2prod::run(listener)?.await
}
