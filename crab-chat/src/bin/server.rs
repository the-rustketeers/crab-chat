use crab_chat as lib;
use json::{JsonValue, object};
use std::{
    time::Duration,
    net::{TcpListener, TcpStream},
    process,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    fs::{self, OpenOptions},
    io::Write,
};
use chrono::Local;

const ERR: i32 = -1;

/**
 * Main program. Hosts the Chat Server
 */
fn main() {
    let (json_producer, json_consumer) = mpsc::channel::<JsonValue>();
    let (stream_producer, stream_consumer) = mpsc::channel::<TcpStream>();

    let handler_producer = json_producer.clone();

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");

        match fs::remove_file("active_nicks.log") {
            Ok(()) => (),
            Err(why) => {
                println!("Unable to remove active_nicks.log: {}", why);
            },
        }

        let local = Local::now().format("%H:%M:%S").to_string();
        match handler_producer.send(
        object! {
        time: local,
        kind: "server_shutdown",
        author: "SERVER_HOST",
        color: "255 255 255",
        message: "The server will disconnect in 10 seconds..."}) {
            Ok(()) => (),
            Err(why) => {
                eprintln!("ERROR: {why}");
            }
        }
        // 10 second timer until shutdown. Can still send and receive messages.
        thread::sleep(Duration::from_millis(10000));  
        
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler.");

    //set up TcpListener, bind to port, and listen for connections
    let listener: TcpListener = TcpListener::bind(lib::ADDRESS).unwrap_or_else(|why| {
        eprintln!("ERROR: {why}");
        process::exit(ERR);
    });

    // Info message
    println!(
        "[SERVER STARTED AND LISTENING FOR CONNECTION ON {:?}]",
        listener.local_addr().unwrap()
    );

    // Set up mpsc channels to send to thread that handles pushing messages
    //let (json_producer, json_consumer) = mpsc::channel::<JsonValue>();
    //let (stream_producer, stream_consumer) = mpsc::channel::<TcpStream>();

    // Spawns new thread that handles fetching messages and pushing messages
    let fetcher = thread::spawn(move || {
        fetch_loop(json_consumer, stream_consumer);
    });

    // Main thread handles setting up new connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // log who connected to the server
                println!(
                    "[{:?} CONNECTED TO THE SERVER]",
                    stream.peer_addr().unwrap()
                );

                // Clone the producer, so it can forward messages to the fetcher
                let p = json_producer.clone();
                // Clone the stream, so the fetcher can send messages to it
                stream_producer.send(stream.try_clone().unwrap()).unwrap();

                // Spawn a new thread that handles listening to this connection
                thread::spawn(move || {
                    connection_loop(stream, p);
                });
            }
            Err(/*why*/_) => (),//eprintln!("[ERROR: {why}]"), // This code is removed as it prints when handling Ctrl+C
        }
    }


    // Ideally should wait until the fetcher thread ends before closing program.
    // Currently this place of code is never reached because the main thread
    // continuously just looks for possible connections
    fetcher.join().unwrap();
}

/**
 * This function takes a TcpStream and Producer part of a channel
 * and will forward any messages it receives through the channel
 */
fn connection_loop(mut listener: TcpStream, json_producer: mpsc::Sender<JsonValue>) {

    //let mut temp_nicks: Vec<String> = vec![];
    loop {
        let obj = match lib::receive_json_packet(&mut listener) {
            Ok(obj) => obj,
            Err(lib::JsonError::ConnectionAborted) => break,
        };

        lib::log_json_packet(&obj);

        if obj["kind"] == "disconnection" {
            let mut nicks: Vec<String> = fs::read_to_string("active_nicks.log").expect("Failed to read active_nicks.log").split("£").map(|s| s.to_string()).collect();
            //let index = nicks.iter().position(|x| *x == obj["author"].to_string());
            //nicks.remove(index);
            nicks.retain(|x| *x != obj["author"].to_string());
            println!("{:?}", nicks);
            match fs::remove_file("active_nicks.log") {
                Ok(()) => (),
                Err(why) => {
                    println!("Unable to remove active_nicks.log: {}", why);
                },
            }
            let mut file = OpenOptions::new().read(true).append(true).create(true).open("active_nicks.log").unwrap();

            file.write_all(nicks.join("£").as_bytes()).expect("Wite to active_nicks.log failed.");
            drop(file); // drops file from scope, forcing flush

            let mut logfile = OpenOptions::new().read(true).append(true).create(true).open("history.log").unwrap();
                logfile.write_all(format!("\n{} with nickname \"{}\" has disconnected @ {}\n", listener.peer_addr().unwrap(), obj["author"], Local::now().format("%H:%M:%S").to_string()).as_bytes()).expect("Write to history.log failed.");

            println!(
                "[{:?} DISCONNECTED FROM THE SERVER]",
                listener.peer_addr().unwrap()
            );
            break;
        }

        if obj["kind"].to_string() == "nick" {
            let mut file = OpenOptions::new().read(true).append(true).create(true).open("active_nicks.log").unwrap();
            let nicks: Vec<String> = fs::read_to_string("active_nicks.log").expect("Failed to read active_nicks.log").split("£").map(|s| s.to_string()).collect();
                if nicks.iter().any(|e| e==obj["author"].as_str().unwrap()) {
                lib::send_json_packet(&mut listener, object! {kind: "retry"}).unwrap();
                println!("Retried!\n");
                print!("\n{:?}\n", nicks);
                continue;
            } else {
                let mut name = obj["author"].to_string();
                name.push_str("£");
                file.write_all(name.as_bytes()).expect("Write to active_nicks.log failed.");
                print!("\n{:?}\n", nicks);
                lib::send_json_packet(&mut listener, object! {kind: "okay"}).unwrap();
                let mut logfile = OpenOptions::new().read(true).append(true).create(true).open("history.log").unwrap();
                logfile.write_all(format!("\n{} has selected \"{}\" for their nickname @ {}\n", listener.peer_addr().unwrap(), obj["author"], Local::now().format("%H:%M:%S").to_string()).as_bytes()).expect("Write to history.log failed.");
                continue;
            }
        }

        match json_producer.send(obj) {
            Ok(()) => (),
            Err(why) => {
                eprintln!("ERROR: {why}");
                break;
            }
        };
    }
}

/**
 * Takes any messages that the json consumer gets and forwards it to all client
 * streams currently seen as connected
 */
fn fetch_loop(json_consumer: Receiver<JsonValue>, stream_consumer: Receiver<TcpStream>) {
    // This holds all of the client streams to try to send to
    // As more clients connect this grows and shrinks
    let mut client_list: Vec<TcpStream> = vec![];

    // this loop handles probing for new client streams, new json packets sent
    loop {
        // see if a client stream is available, if so add it to list
        match stream_consumer.try_recv() {
            Ok(stream) => client_list.push(stream),
            Err(why) => match why {
                TryRecvError::Disconnected => {
                    eprint!("[FATAL ERROR: {why}]");
                    break;
                }
                TryRecvError::Empty => (),
            },
        };

        // see if a json packet is available, if so forward it to clients
        match json_consumer.try_recv() {
            Ok(obj) => client_list = push_to_clients(&mut client_list, obj),
            Err(why) => match why {
                TryRecvError::Disconnected => {
                    eprint!("[FATAL ERROR: {why}]");
                    break;
                }
                TryRecvError::Empty => (),
            },
        };
        thread::sleep(Duration::from_millis(10));
    }
}

/**
 * This function just loops through all clients in the client list and attempts
 * to send a json packet to them.
 */
fn push_to_clients(client_list: &mut Vec<TcpStream>, obj: JsonValue) -> Vec<TcpStream> {
    let mut new_list: Vec<TcpStream> = vec![];
    let mut logfile = OpenOptions::new().read(true).append(true).create(true).open("history.log").unwrap();
    logfile.write_all(format!("{}: {} says:\n\t\"{}\"\n",obj["time"], obj["author"], obj["message"]).as_bytes()).expect("Write to history.log failed.");
    for i in 0..client_list.len() {
        match lib::send_json_packet(&mut client_list[i], obj.clone()) {
            Ok(()) => new_list.push(client_list[i].try_clone().unwrap()), // Appends to new list if stream = no error. List is then returned.
            Err(_) => (), // println!("[{:?} HAS BEEN REMOVED FROM THE LIST OF ACTIVE CLIENTS]", client_list[i].peer_addr().unwrap()), // Not needed, helped to make sure
        }
    }
    new_list // returned list
}