use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let addr = "127.0.0.1:2356";
    let stream = TcpStream::connect(addr).await?;
    println!("Connected to the server at {}", addr);

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // nspawni task li ghyhandli messagat li ghywslo nchaalah
    let read_task = tokio::spawn(async move {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer).await {
                Ok(0) => {
                    println!("Server down.");
                }
                Ok(n) => {
                    // let received_message = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", buffer);
                }
                Err(e) => {
                    println!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    let write_task = tokio::spawn(async move {
        let mut stdin = io::BufReader::new(io::stdin());
        let mut input = String::new();
        loop {
            input.clear();
            if stdin.read_line(&mut input).await.is_err() {
                println!("Error sending from stdin");
                break;
            }
            if writer.write_all(input.as_bytes()).await.is_err() {
                println!("Failed to send message.");
                break;
            }
        }
    });

    let _ = tokio::try_join!(read_task, write_task);
    Ok(())
}