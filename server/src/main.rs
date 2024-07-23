use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
//The imports bring in functionality needed for I/O operations, network communication, and multi-threading.
//ErrorKind is used for matching against specific I/O errors.
//Read and Write traits are used to handle reading from and writing to the TCP streams.
//TcpListener is used to accept new connections.
//mpsc provides channels for inter-thread communication.
//thread is used to spawn and manage threads.

const LOCAL: &str="127.0.0.1:6000";
const MSG_SIZE: usize= 32;
//LOCAL is the address and port where the server will listen for incoming connections.
//MSG_SIZE is a predefined size for messages, which helps in managing the buffer size for reading data.

//Allow loop to rest...break loop
//initialize sleep function
fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}
//main function
fn main() -> ! {
    println!("Welcome to Rust!");

//Bind TcpListener to local address
let server = TcpListener::bind(LOCAL).expect("Could not bind");
server.set_nonblocking(true).expect("Failed to initialize non-blocking");

//create channel for inter-thread communication
let mut clients = vec![];
let (tx, rx) = mpsc::channel::<String>();
loop { 
    match server.accept() {
        Ok((mut socket, addr)) => {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff=vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("Received message from {}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_)=> {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
                //call the sleep function
                sleep();
            });
        }
        _ => (),
    }
    //what will happen when te server receives the message
    if let Ok(msg) = rx.try_recv() {
        clients = clients.into_iter().filter_map(|mut client| {
            let mut buff = msg.clone().into_bytes(); //convert message into bytes
            buff.resize(MSG_SIZE, 0);

            client.write_all(&buff).map(|_| client).ok()
        }).collect::<Vec<_>>();
    }
    //call the sleep function again
    sleep();
}

}
