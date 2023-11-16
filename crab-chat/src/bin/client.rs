use crab_chat::*;
use std::{env::args, iter::Iterator, net::TcpStream, process};

fn main() {
    let cmd_args: Vec<String> = args().collect();
    if cmd_args.len() != 3 {
        eprintln!("Usage: client <host> <port>");
        process::exit(1);
    }
    let host: &String = &cmd_args[1];
    let port: i32 = cmd_args[2].trim().parse().unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    let connection: TcpStream = TcpStream::connect(format!("{host}:{port}")).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    connection_loop(connection);
}

fn connection_loop(mut stream: TcpStream) {
    let obj = receive_json_packet(&mut stream);
    println!("{:#?}", obj);
    loop {}
}
