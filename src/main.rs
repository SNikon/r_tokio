use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, addr) = listener.accept().await.unwrap();

    let (read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 { break; }
        write.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
