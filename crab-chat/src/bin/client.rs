use chrono::Local;
use crab_chat as lib;
use json::object;
use colored::Colorize;
use std::{io, net::TcpStream, process, thread};

fn main() {

    let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });

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

        let num_test = val.trim().parse::<i16>();
        match num_test {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Please input proper values when signing in. Shutting down...");
                process::exit(0);
            },
        }

        if(val.trim().parse::<i16>().unwrap() > 255) || (val.trim().parse::<i16>().unwrap() < 0) {
            eprintln!("Please input proper values when signing in. Shutting down...");
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

    /*let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });*/

    let mut nick_change: String = user_info[0].clone();
    let nick_connection = &mut connection.try_clone().unwrap();
    let mut nick_obj = object! {
        author: user_info[0].clone(),
        kind: "nick",
    };

    loop {
        match lib::send_json_packet(nick_connection, nick_obj.clone()) {
            Ok(()) => (),
            Err(_) => {
                println!("Error sending nickname request...");
                process::exit(0);
            },
        };

        let rec = match lib::receive_json_packet(nick_connection) {
            Ok(obj) => obj,
            Err(_) => break,
        };

        if rec["kind"] == "retry" {
            nick_change = String::new();
            println!("Nickname unavailable. Try again:");
            io::stdin()
                .read_line(&mut nick_change)
                .expect("Could not read user input");
            
            let trimmer = nick_change.clone();
            let trimmed = trimmer.trim();

            nick_obj["author"] = json::JsonValue::String(trimmed.to_string());
            continue;
        } else {
            println!("Nickname accepted. Start chatting!");
            user_info[0] = nick_change.trim().to_string();
            break;
        }
    }

    let mut handler_connection = connection.try_clone().unwrap();
    let handler_copy = user_info.clone();

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        match lib::send_json_packet(&mut handler_connection, object! {kind: "disconnection", author: handler_copy[0].to_string()}) {
            Ok(()) => (),
            Err(_) => {
                println!("[GOODBYE]");
                process::exit(0);
            }
        }
        println!("[GOODBYE]");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler.");

    connection_loop(connection, user_info.clone());
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
            lib::send_json_packet(&mut stream_reader, object! {kind: "disconnection", author: user[0].to_string()}).unwrap();
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
        let mut clrval: Vec<&str> = vec![];
        if !obj["color"].is_null() { 
            clrval = obj["color"].as_str().unwrap().split(" ").collect(); // clrval[0] is R, clrval[1] is G, clrval[2] is B.
        } else {
            clrval.push("255");
            clrval.push("255");
            clrval.push("255");
        }
        if obj["kind"] == "server_shutdown" {
            (); // Server will shutdown in 10 seconds. Client recognizes this.
        }
        if obj["message"].is_null() {
            continue;
        }
        println!(
            "{}: {} says:\n\t\"{}\"",
            obj["time"], obj["author"].to_string().truecolor(clrval[0].parse::<u8>().unwrap(), clrval[1].parse::<u8>().unwrap(), clrval[2].parse::<u8>().unwrap()), obj["message"]
        );
    });

    // hold until the input reader ends
    reader.join().unwrap();
}
