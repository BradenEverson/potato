use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use potato::{message::PotatoMessage, service::PotatoService};
use std::sync::mpsc::channel;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to default");

    println!("🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔");
    println!("🥔🥔   WELCOME TO THE POTATO CANNON SERVER  🥔🥔");
    println!("🥔🥔   listening on http://localhost:8080   🥔🥔");
    println!("🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔🥔");

    let (sender, receiver) = channel();
    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");

            let io = TokioIo::new(socket);

            let sender = sender.clone();
            tokio::spawn(async move {
                let service = PotatoService::with(sender.clone());
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    });

    while let Ok(message) = receiver.recv() {
        match message {
            PotatoMessage::Shoot => {
                println!("Releasing potato payload");
            }
        }
    }
}
