use chrono::Local;
use crab_chat as lib;
use json::object;
use std::{io, net::TcpStream, process, thread};

fn main() {
    let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    connection_loop(connection);
}

fn connection_loop(stream: TcpStream) {
    println!("[START TYPING AND HIT ENTER TO SEND A MESSAGE]");
    let addr = stream.local_addr().unwrap();

    // copy stream and push it into a thread to handle getting input from user
    let mut stream_reader = stream.try_clone().unwrap();
    let reader = thread::spawn(move || loop {
        let mut msg = String::new();

        // read input from the user
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        let msg = msg.trim();

        // check if input is to exit, and leave the thread if so
        if msg == lib::EXIT_CODE {
            println!("[GOODBYE]");
            // The last message that any client sends to a server
            // should be of type "disconnection"
            lib::send_json_packet(&mut stream_reader, object! {kind: "disconnection"}).unwrap();
            break;
        };

        let local = Local::now().format("%H:%M:%S").to_string();

        // wrap message in a json object
        // currently, all authors are just the address of connection
        let obj = object! {
            author: format!("{addr}"),
            time: local,
            message: msg,
        };

        // try to send json object, fails if server/client exited already
        match lib::send_json_packet(&mut stream_reader, obj) {
            Ok(()) => (),
            Err(_) => break,
        };
    });

    // copy stream and move it into a thread to handle receiving incoming messages from the server
    let mut stream_listener = stream.try_clone().unwrap();
    let _listener = thread::spawn(move || loop {
        // endlessly try to get/print the next json packet from stream
        // fails if server/client exited already
        let obj = match lib::receive_json_packet(&mut stream_listener) {
            Ok(obj) => obj,
            Err(_) => break,
        };
        println!(
            "{}: {} says:\n\t\"{}\"",
            obj["time"], obj["author"], obj["message"]
        );
    });

    // hold until the input reader ends
    reader.join().unwrap();
}
