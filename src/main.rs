use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{unix::OwnedReadHalf, UnixListener},
    signal::unix::{signal, SignalKind},
};

#[tokio::main]
async fn main() {
    let listener = UnixListener::bind("/tmp/jay.socket").unwrap();

    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (mut read_half, mut write_half) = stream.into_split();

            // Process incoming requests on the read half
            let mut buffer = vec![0; 10];
            let read = match read_half.read_exact(&mut buffer).await {
                Ok(0) => 0,
                Ok(size) => size,
                Err(_) => 0,
            };

            if read > 0 {
                write_half
                    .write_all(&buffer)
                    .await
                    .expect("Write operation failed");
            }
        }
    });

    // register a signal handler for Ctrl+C or SIGTERM to initiate shutdown
    let mut signal = signal(SignalKind::interrupt()).unwrap();
    tokio::spawn(async move {
        signal.recv().await;
        println!("Shutting down...");

        // Shutdown the listener gracefully
    });
}
