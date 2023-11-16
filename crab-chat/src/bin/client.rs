use std::{env::args, io::Read, iter::Iterator, net::TcpStream, process, str};

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
    print_word_packets(connection);
}

fn print_word_packets(mut stream: TcpStream) {
    loop {}
}
