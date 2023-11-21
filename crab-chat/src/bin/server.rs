use crab_chat as lib;

use std::{net::TcpListener, process, thread};

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
            Ok(stream) => {
                println!("[CONNECTION FROM {:?}]", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    lib::connection_loop(stream);
                });
            }
            Err(why) => {
                eprintln!("ERROR: {why}");
                process::exit(ERR);
            }
        }
    }
}
