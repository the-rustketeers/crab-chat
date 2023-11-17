use crab_chat as lib;
use json::object;
use std::{
    net::{TcpListener, TcpStream},
    process,
};

const ERR: i32 = -1;

/**
 * Description: main program. takes exactly 1 command line argument.
 *   demonstrates binding, listening, and finally connecting to a socket.
 */
fn main() {
    //set up TcpListener, bind to port, and listen for connections
    let listener: TcpListener = TcpListener::bind(lib::ADDRESS).unwrap_or_else(|why| {
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
    lib::send_json_packet(s, name_prompt);
}
