use crab_chat::*;
use json::object;
use std::{
    env,
    iter::Iterator,
    net::{TcpListener, TcpStream},
    process,
};

const ERR: i32 = -1;

/**
 * Description: main program. takes exactly 1 command line argument.
 *              demonstrates binding, listening, and finally connecting to a socket.
 *              uses the provided port and hosts a server on localhost:port.
 * Argument:    <port>  :   a port number between 10000 and 65535.
 */
fn main() {
    // set up and confirm the command arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: ./server <port>\n\t[where 10000 < port < 65535]");
        process::exit(ERR);
    }

    let port: i32 = args[1].trim().parse().unwrap_or_else(|why| {
        eprintln!("ERROR: {why}");
        process::exit(ERR);
    });

    //set up TcpListener, bind to port, and listen for connections
    let listener: TcpListener =
        TcpListener::bind(format!("127.0.0.1:{port}")).unwrap_or_else(|why| {
            eprintln!("ERROR: {why}");
            process::exit(ERR);
        });

    println!(
        "INFO: Server started on and listening for connections on address {:?}",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("INFO: Got a connection from {:?}", s.peer_addr().unwrap());
                connection_loop(&mut s);
            }
            Err(why) => {
                eprintln!("ERROR: {why}");
                process::exit(ERR);
            }
        }
    }
}

fn connection_loop(s: &mut TcpStream) {
    let name_prompt = object! {
        message: "Please input a nickname: "
    };
    println!("{:#?}", name_prompt.dump());
    send_json_packet(s, name_prompt);
}
