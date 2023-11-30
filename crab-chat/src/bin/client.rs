use chrono::Local;
use crab_chat as lib;
use json::object;
use colored::Colorize;
use std::{io, net::TcpStream, process, thread};

fn main() {
    let mut user_info: Vec<String> = vec![]; // Vector that contains user information. [0] is name, [1], [2], and [3] are R, G, and B, respectively for name color
    let mut user_input: String = String::new();

    println!("Please input a username:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");

    user_info.push(user_input.clone().trim().to_string());
    user_input = String::new();

    println!("Please input an RGB color combination from 0-255 for your name,\n(Example: 255 255 255):");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");

    let temp = user_input.split(" "); // Temp value to hold split info with an interator

    let mut iter: i8 = 0;
    for val in temp {
        if(val.trim().parse::<i16>().unwrap() > 255) || (val.trim().parse::<i16>().unwrap() < 0) {
            println!("Please input proper values when signing in. Shutting down...");
            process::exit(0);
        }
        user_info.push(val.trim().to_string());
        iter += 1;
        if iter > 2 {
            break;
        }
    }
    if iter != 3 {
        println!("Please input proper values when signing in. Shutting down...");
        process::exit(0);
    }


    let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    connection_loop(connection, user_info);
}

fn connection_loop(stream: TcpStream, user: Vec<String>) {
    println!("[START TYPING AND HIT ENTER TO SEND A MESSAGE]");

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
            author: format!("{}", user[0]),
            time: local,
            message: msg,
            color: format!("{} {} {}", user[1], user[2], user[3]),
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
        let clrval: Vec<&str> = obj["color"].as_str().unwrap().split(" ").collect(); // clrval[0] is R, clrval[1] is G, clrval[2] is B.
        println!(
            "{}: {} says:\n\t\"{}\"",
            obj["time"], obj["author"].to_string().truecolor(clrval[0].parse::<u8>().unwrap(), clrval[1].parse::<u8>().unwrap(), clrval[2].parse::<u8>().unwrap()), obj["message"]
        );
    });

    // hold until the input reader ends
    reader.join().unwrap();
}
