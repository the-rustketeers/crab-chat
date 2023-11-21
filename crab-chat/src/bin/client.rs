use crab_chat as lib;
use std::{net::TcpStream, process};

fn main() {
    let connection: TcpStream = TcpStream::connect(lib::ADDRESS).unwrap_or_else(|e| {
        eprintln!("Error: {e}.");
        process::exit(1);
    });
    lib::connection_loop(connection);
}
