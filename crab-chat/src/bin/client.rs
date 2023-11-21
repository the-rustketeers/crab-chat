// CHANGES MADE FOR TESTING PURPOSES ONLY!

use crab_chat as lib;
use std::{net::TcpStream, process, time};

fn main() {
    let mut connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    send_loop(&mut connection);
    let n_s: String = connection_loop(&mut connection);
    println!("{}", n_s);
    std::thread::sleep(time::Duration::from_millis(1000)); // TESTING: Used to test concurrency in sending and receiving despite one concurrent server program.
    send_loop(&mut connection);
}

/// FOR COMMIT: modified to print packet message literal for ease of development (personally helps me only, I understand it isn't practical)
fn connection_loop(stream: &mut TcpStream) -> String {
    let obj = lib::receive_json_packet(stream);
    obj["message"].to_string()
}

/// send_loop is equivalent to server-side's sending.
fn send_loop(s: &mut TcpStream) {
    let name_prompt = json::object! {
        message: "Test message from client!"
    };
    println!("{:#?}", name_prompt.dump());
    lib::send_json_packet(s, name_prompt);
}
