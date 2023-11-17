use crab_chat as lib;
use std::{net::TcpStream, process};

fn main() {
    let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    connection_loop(connection);
}

fn connection_loop(mut stream: TcpStream) {
    let obj = lib::receive_json_packet(&mut stream);
    println!("{:#?}", obj);
}
