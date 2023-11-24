// CHANGES MADE FOR TESTING PURPOSES ONLY!
use crab_chat as lib;
use std::{net::TcpStream, process, time, io, thread,};

fn main() {
    
    let mut connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });

    let mut stream_sender = connection.try_clone().unwrap();
    let mut stream_reader = connection.try_clone().unwrap();

    thread::spawn(move || loop { // Spawns thread to read in received information
        let n_s: String = rec_loop(&mut stream_reader);
        println!("{}", n_s);
    });

    loop { // Loops main to send and get use input
        let mut msg = String::new();
        
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read input");

        let msg = msg.trim();

        if msg == "!!" {
            println!("[GOODBYE]");
            break;
        };

        send_loop(&mut stream_sender, msg.to_string());
    };
}

/// FOR COMMIT: modified to print packet message literal for ease of development (personally helps me only, I understand it isn't practical)
fn rec_loop(stream: &mut TcpStream) -> String {
    let obj = lib::receive_json_packet(stream);
    obj["message"].to_string()
}

/// send_loop is equivalent to server-side's sending.
fn send_loop(s: &mut TcpStream, msg: String) {
    let name_prompt = json::object! {
        message: msg.as_str()
    };
    lib::send_json_packet(s, name_prompt);
}
