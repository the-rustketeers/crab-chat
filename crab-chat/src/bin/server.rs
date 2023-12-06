/*
Authors:        Peter Schaefer, Evan Binkley, and Austin Swartley
Creation Date:  11/21-12/5
Due Date:       12/14/23 @ 10:00am
Course:         CsC328-020
Professor name: Dr. Schwesinger
Assignment:     Final Project
Filename:       server.rs
Purpose:        This is the server for a server-client project.
                This program will collect messages sent by clients
                and distribute them to all connected client
                programs.
*/
use chrono::Local;
use crab_chat as lib;
use json::{object, JsonValue};
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    net::{TcpListener, TcpStream},
    process,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::Duration,
};

/// Function name:      main
/// Description:        Hosts and executes program.
/// Parameters:         None
/// Return Value:       None
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Please input the correct number of arguments...
        Usage: ./server [PORT #]"
        );
        process::exit(0);
    }
    let address: String = format!("127.0.0.1:{}", args[1]);

    let (json_producer, json_consumer) = mpsc::channel::<JsonValue>();
    let (stream_producer, stream_consumer) = mpsc::channel::<TcpStream>();

    let handler_producer = json_producer.clone();

    // This code will execute when the server shuts down
    ctrlc::set_handler(move || {
        println!("\n[COMMENCING SERVER SHUTDOWN]");

        // remove runtime file(s)
        match fs::remove_file("active_nicks.log") {
            Ok(()) => (),
            Err(_) => (), // This does not need to print.
        }

        lib::log_to_file(
            &format!(
                "\n[SERVER SHUTDOWN @ {}]\n",
                Local::now().format("%H:%M:%S").to_string()
            ),
            "history.log",
        );

        // send a final message to all clients that the server is shutting down
        handler_producer
            .send(shutdown_json(Local::now().format("%H:%M:%S").to_string()))
            .unwrap_or_else(|why| {
                eprintln!("[ERROR: {why}");
            });
        // 10 second timer until shutdown. Can still send and receive messages.
        thread::sleep(lib::SHUTDOWN_TIME);

        process::exit(0);
    })
    .expect("[Error setting Ctrl-C handler]");

    // Prints to log file when server has started up.
    lib::log_to_file(
        &format!(
            "\n[SERVER STARTUP @ {}]\n",
            Local::now().format("%H:%M:%S").to_string()
        ),
        "history.log",
    );

    //set up TcpListener, bind to port, and listen for connections
    let listener: TcpListener = TcpListener::bind(address).unwrap_or_else(|why| {
        eprintln!("[ERROR: {why}]");
        process::exit(0);
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
            Err(/*why*/ _) => (),
            //eprintln!("[ERROR: {why}]"), // This code is removed as it prints when handling Ctrl+C
        }
    }

    // Ideally should wait until the fetcher thread ends before closing program.
    // Currently this place of code is never reached because the main thread
    // continuously just looks for possible connections
    fetcher.join().unwrap();
}

/// Function name:      connection_loop
/// Description:        Takes a TcpStream and JSON channel producer, and forwards messages it receives through the channel.
/// Parameters:         mut listener: TcpStream | Actively connected client stream
///                     json_producer: mpsc::Sender<JsonValue> | Producer end of channel to be able to send to thread to handle incoming messages
/// Return Value:       None
fn connection_loop(mut listener: TcpStream, json_producer: mpsc::Sender<JsonValue>) {
    loop {
        let obj = match lib::receive_json_packet(&mut listener) {
            Ok(obj) => obj,
            Err(lib::JsonError::ConnectionAborted) => break,
        };

        if obj["kind"] == "disconnection" {
            // read the current list of nicknames
            let mut nicks: Vec<String> = fs::read_to_string("active_nicks.log")
                .expect("Failed to read active_nicks.log")
                .split("\n")
                .map(|s| s.to_string())
                .collect();

            // remove the disconnected client nickname
            nicks.retain(|x| *x != obj["author"].to_string());
            println!("{:?}", nicks);

            // remove the old nickname file
            // @Ediblelnk: not exactly sure why this is necessary
            match fs::remove_file("active_nicks.log") {
                Ok(()) => (),
                Err(why) => {
                    println!("Unable to remove active_nicks.log: {}", why);
                }
            }

            // recreate nickname file
            lib::log_to_file(&nicks.join("\n"), "active_nicks.log");

            // open the logfile and log the disconnection
            lib::log_to_file(
                &format!(
                    "\n{} with nickname \"{}\" has disconnected @ {}\n",
                    listener.peer_addr().unwrap(),
                    obj["author"],
                    Local::now().format("%H:%M:%S").to_string()
                ),
                "history.log",
            );

            println!(
                "[{:?} DISCONNECTED FROM THE SERVER]",
                listener.peer_addr().unwrap()
            );
            break;
        }

        // someone is trying is set their nickname
        if obj["kind"].to_string() == "nick" {
            // read the current nicknames from file
            let mut file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open("active_nicks.log")
                .unwrap();
            let nicks: Vec<String> = fs::read_to_string("active_nicks.log")
                .expect("Failed to read active_nicks.log")
                .split("\n")
                .map(|s| s.to_string())
                .collect();

            // if the requested nickname is already found in the nickname file tell the client to retry
            if nicks.iter().any(|e| e == obj["author"].as_str().unwrap()) {
                lib::send_json_packet(&mut listener, object! {kind: "retry"}).unwrap();
                println!("Retried!\n");
                print!("\n{:?}\n", nicks);
                continue;
            } else {
                // the client's nickname is unique
                let mut name = obj["author"].to_string();
                name.push_str("\n");
                file.write_all(name.as_bytes())
                    .expect("Write to active_nicks.log failed.");
                print!("\n{:?}\n", nicks);

                lib::send_json_packet(&mut listener, object! {kind: "okay"}).unwrap();

                lib::log_to_file(
                    &format!(
                        "\n{} has selected \"{}\" for their nickname @ {}\n",
                        listener.peer_addr().unwrap(),
                        obj["author"],
                        Local::now().format("%H:%M:%S").to_string()
                    ),
                    "history.log",
                );
                continue;
            }
        }

        match json_producer.send(obj) {
            Ok(()) => (),
            Err(why) => {
                eprintln!("[ERROR: {why}]");
                break;
            }
        };
    }
}

/// Function name:      fetch_loop
/// Description:        Takes messages from JSON consumer channel end and forwards message to all active clients
/// Parameters:         json_consumer: Receiver<JsonValue> | JSON consumer end of channel, to grab and send messages received from clients
///                     stream_consumer: Receiver<TcpStream> | stream consumer end of channel, to grab and store active clients
/// Return Value:       None
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
        thread::sleep(Duration::from_millis(10)); // To limit CPU usage based on time
    }
}

/// Function name:      push_to_clients
/// Description:        Loops through all active clients in a given list and attempts to send a packet to each
/// Parameters:         client_list: &mut Vec<TcpStream> | List of clients to loop through and send packets to
///                     obj: JsonValue | JSON object to be sent to active clients
/// Return Value:       revised_client_list: Vec<TcpStream> | The edited list of clients, as if to change list upon client disconnection
fn push_to_clients(client_list: &mut Vec<TcpStream>, obj: JsonValue) -> Vec<TcpStream> {
    let mut revised_client_list: Vec<TcpStream> = vec![];
    lib::log_to_file(&lib::stringify_json_packet(&obj), "history.log");
    for i in 0..client_list.len() {
        match lib::send_json_packet(&mut client_list[i], obj.clone()) {
            Ok(()) => revised_client_list.push(client_list[i].try_clone().unwrap()), // Appends to new list if stream = no error. List is then returned.
            Err(_) => (),
        }
    }
    revised_client_list // returned list
}

/// Function name:      shutdown_json
/// Description:        Returns a JSON packet after receiving current time, used for shutdown.
/// Parameters:         local: String | String of current time.
/// Return Value:       JsonValue | JSON object containing shutdown information
fn shutdown_json(local: String) -> JsonValue {
    object! {
    time: local,
    kind: "server_shutdown",
    author: "SERVER_HOST",
    color: "255 255 255",
    message: "The server will disconnect in 3 seconds..."}
}
