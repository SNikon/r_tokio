use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
    
            loop {
                tokio::select! {
                    bytes_read = reader.read_line(&mut line) => {
                        if bytes_read.unwrap() == 0 { break; }
    
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    rcv_data = rx.recv() => {
                        let msg = rcv_data.unwrap();
                        write.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
