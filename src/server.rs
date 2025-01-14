use tokio::{
    io::{
            AsyncReadExt, // trait : kaykhelik t9ra data asynchronously mn files, Tcpstreams ... w tktebhoum f buffer
            AsyncWriteExt, // trait : kaykhelik tkteb data async l files, lTcpStreams ... yaeni tseftom
            BufReader, // struct : kat9ra data w katkhebiha eandou w kat9raha mn eandou blast mtakhoda mn source : ela 9bl lperformance, cnx t9ila, files kbaar ...
        },
    net::{
        TcpListener, // struct : li houwa server li kaytsenet ela l incomming connections
        TcpStream, // struct : hia single connection, mli kitconnecta l client dk TcpStream li houwa socket kithandla individually
        },
    sync::{
        mpsc, // multi-producers single-client : hadi channel bch kidwiw threads bintoum, hia li ghndwzo fiha messagat dl users baediyatom,
        Mutex, // mutex ela 9bl shared data bin threads iji thread i locki l mutex ikhdm beya i unlockeha ...
    }
    fs::File;
};
use std::{
    collections::HashMap,
    sync::Arc
};

/* hna definina type labels : unbounded yaeni l non limited capaty 
    hit fih channel eadiya kiji sender kib9a iseft mtln capacity 100, fch kiwsl 
    l 100 msgs sent and not received yaeni ba9in f channel mimknch tseft mzl.
    fch receiver ki receivi chi msg ktlibera dk l espace w i9dr sender ieawd iseft.
    ---------
    unbounded t9dr tb9a tsft 9dma bghiti gha hia l memoire t9dr taamr bzf
*/
type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

#[tokio::main]
async fn main() {
    // list dl users
    let clients: Arc<Mutex<HashMap<String, Tx>>> = Arc::new(Mutex::new(HashMap::new()));

    // ncreaw listener (server)
    let ip_port = "127.0.0.1:2356";
    let listener = TcpListener::bind(ip_port).await?;
    println!("Chat server is Live on {}", ip_port);

    loop {
        // fch itconnecta chi wahed accepteh
        let (socket, addr) = listener.accept().await;
        let addr = addr.to_string();
        // ----- add log here -----
        println!("New Connection: {}", addr);

        // cloni clients list
        let clients = clients.clone();

        // handli l connection
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, clients).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

struct CredentialsError;

impl std::fmt::Display for CredentialsError {
    f fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error in Credentials Function");
    }
}

impl Error for CredentialsError {}

async fn encrypt_pass<'a>(stored_pass: &'a str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from(stored_pass))
}


// async fn credentials(socket: TcpStream, addr: String, clients:Arc<Mutex<HashMap<String, Tx>>>) -> Result<TcpStream, Box<dyn std::error::Error>> {
async fn credentials(socket: TcpStream) -> Result<TcpStream, Box<dyn std::error::Error>> {
    
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    let mut buffer = vec![0;1024];
    match reader.read(&mut buffer).await {
        Ok(0) => {
            // ----- add log here -----
            println!("{} disconnected (credentials)", addr);
            return Err(Box::new(CredentialsError {}));
        }
        Ok(n) => {
            let username = String::from_utf8_lossy(&buffer[..n]).to_string();
            
            
            if username.starts_with("-b-e-g-i-n--u-s-e-r-n-a-m-e") {
                let db_as_file = File::open("credentials").await?;
                let mut lines = BufReader::new(db_as_file).lines();
                while let Some(line) = lines.next_line().await? {
                    if let Some((stored_username, stored_pass)) = line.split_once('_') {
                        if stored_username == username {
                            // send encrypted pass
                            let encrypted_pass = encrypt_pass(stored_pass).await?;
                            if write.write_all(encrypted_pass.as_bytes()).await.is_err() {
                                println!("error sending encrypted password");
                                return Err(Box::new(CredentialsError {}));  
                            }

                            for _ in 0..2 {
                                let mut buffer = vec![0;1024];
                                match reader.read(&mut buffer).await {
                                    Ok(0) => {
                                        println!("user closed connection after sending encrypted password");
                                        return Err(Box::new(CredentialsError {}));
                                    }
                                    Ok(n) => {
                                        let received_password = String::from_utf8_lossy(&buffer[..n]);
                                        if received_password == stored_pass {
                                            return Ok(socket);
                                        } else {
                                            if writer.write_all("incorrect".as_bytes()).await.is_err() {
                                                println!("error sending that pass is incorrect");
                                            };
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error reading password from user");
                                        return Err(Box::new(e));
                                    }
                                }
                            }
                        }
                    }
                    println!("user didn't provide correct credentials");
                    return Err(Box::new(CredentialsError {}));
                }
            }
        }
        Err(e) => {
            // ----- add log here -----
            println!("error reading from socket (credentials): {}", e);
            return Err(Box::new(e));
        }
    }     
}

async fn handle_client(
    socket: TcpStream,
    addr: String,
    clients: Arc<Mutex<HashMap<String, Tx>>>) 
    -> Result<(), Box<(), dyn std::error::Error>> {
         /* 
            socket.into_split(), kat9sem socket wla tcpstream l two parts: read, write
            bjojhom m implementying AsyncRead w AsyncWrite.
            yaeni t9der treceivi w tsendi fnfs lw9t yaeni concurrently b async
        */
        if let Some(sock) = credentials(socket).await {
            let (reader, mut writer) = sock.into_split();
        } else {
            println!("error credentials function");
            return Err(Box::new(CredentialsError {}))
        }
        let mut reader = BufReader::new(reader); // wrappina reader f BufReader li chraht lfo9

        // unbounded channel mafihach limit d lcapacity, yaeni t9der tsendi 9esdma bghiti
        // dmessagat fiha wakha receiver mayreceivihoumch t9dr tb9a tseft fiha
        let (tx, mut rx): (Tx, Rx) = mpsc::unbounded_channel();
        {
            let mut clients_lock = clients.lock().await;
            clients_lock.insert(addr.clone(), tx.clone());
        }

        let addr2 = addr.clone(); // cloni l addr hit anhtaj n printiha flekher
        let clients_for_read_task = clients.clone(); // hada hta houwa ghanhtaj npopi l address diali

        let read_task = tokio::spawn(async move {
            let mut buffer = vec![0, 1024];

            loop {
                match reader.read(&mut buffer).await { // reader: BufReader , rah m implemente trait dial AsyncReadExt dkchi elach read() method dial AsyncReadExt
                    Ok(0) => {
                        // ----- add log here -----
                        println!("{} disconnected", addr);
                        break;
                    }
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                        println!("{} >> {}", addr, message);

                        // broadcate lmessage l users li online
                        let clients_lock = clients_for_read_task.lock().await;
                        for (client_addr, tx) in clients_lock.iter() {
                            let _ = tx.send(format!("{} >> {}", addr, message));
                        }
                    }
                    Err(e) => {
                        // ----- add log here -----
                        println!("error reading from socket: {}", e);
                        break;
                    }
                }
            }
        });

        let write_task = tokio::spawn(async move {
            while let Some(msg) => rx.recv().await {
                if writer.write_all(msg.as_bytes()).await.is_err() {
                    break;
                }
            }
        });

        // wait for tasks to finish
        let _ = tokio::try_join!(read_task, write_task);

        // mli idecconecta chi user popih mn list
        {
            let mut clients_lock = clients.lock().await;
            clients_lock.remove(&addr2);
        }

        println!("Connection Closed: {}", addr2);
        Ok(())
    }