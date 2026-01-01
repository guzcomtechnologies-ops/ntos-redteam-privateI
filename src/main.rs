use blake3;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    println!("NTOS organelle + QRNG WS server on :8080");

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind socket");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("QRNG client connected: {}", addr);
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(stream: TcpStream) {
    let ws = accept_async(stream)
        .await
        .expect("WebSocket handshake failed");

    let (mut write, _read) = ws.split();

    loop {
        let seed = b"qrng_ntos_2026";
        let mut hasher = blake3::Hasher::new();
        hasher.update(seed);
        let bits = hasher.finalize().as_bytes().to_vec();

        // FIXED: Vec<u8> -> Bytes
        if write.send(Message::Binary(bits.into())).await.is_err() {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
