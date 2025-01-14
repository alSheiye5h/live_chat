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
}