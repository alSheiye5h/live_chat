use tokio {
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};
use std::{collections::HashMap, sync::Arc};

type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // list dial l users .
    let clients: Arc<Mutex<HashMap<String, Tx>>> = Arc::new(Mutex::new(HashMap::new()));

    // ncreaw listener (server)
    let ip_port = "127.0.0.1:2356";
    let listener = TcpListener::bind(ip_port).await?;
    println!("Chat server is running on {}", ip_port);

    loop {
        // fch itconnecta chi whd acceptih
        let (socket, addr) = listener.accept().await?;
        let addr = addr.to_string();
        println!("New connection: {}", addr);

        // cloni clients list
        let clients = clients.clone();

        // lgreen thread (async task) li ghayhandli l connection    
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, clients).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    addr: String,
    clients: Arc<Mutex<HashMap<String, Tx>>>) 
    -> Result<(), Box<dyn std::error::Error>> {
        /* 
            socket.into_split(), kat9sem socket wla tcpstream l two parts: read, write
            bjojhom m implementying AsyncRead w AsyncWrite.
            yaeni t9der treceivi w tsendi fnfs lw9t yaeni concurrently b async
        */
        let (reader, mut writer) = socket.into_split();
        let reader = BufReader::new(reader);

        // unbounded channel mafihach limit d lcapacity, yaeni t9der tsendi 9esdma bghiti
        // dmessagat fiha wakha receiver mayreceivihoumch t9dr tb9a tseft fiha
        let (tx, mut rx): (Tx, Rx) = mpsc::unbounded_channel();
        {
            let mut clients_lock = clients.lock().await;
            clients_lock.insert(addr.clone(), tx.clone());
        }

        // 
        let read_task = tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            // loop {
            //     match reader.read(&mut buffer).await {
            //         Ok(0) => {
            //             println!("{} disconnected", addr);
            //         }
            //         Ok(n) => {
            //             let message = String::from_utf8_lossy(&buffer[..n]).to_string();
            //             println!("{} >> {}", addr, message);

            //             // broadcaste lmessage lclients lokhrin
            //             let clients_lock = clients.lock().await;
            //             for (client_addr, tx) in clients_lock.iter() {
            //                 if client_addr != &addr {

            //                 }
            //             }
            //         }
            //     }
            }
        })
}
