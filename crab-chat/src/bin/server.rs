use std::{
    env,
    io::{Read, Write},
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
                send_words(&mut s);
            }
            Err(why) => {
                eprintln!("ERROR: {why}");
                process::exit(ERR);
            }
        }
    }
}

/**
 * Description: sends 1 to 10 random word packets to a client.
 *              each word packet has two parts:
 *                  the length of the string, a u16 in big endian (network) byte order
 *                  the ASCII characters, a sequence of bytes
 */
fn send_words(s: &mut TcpStream) {
    s.write_all("Please choose a nickname: ".as_bytes())
        .unwrap();

    let mut name = String::new();
    s.read_to_string(&mut name).unwrap();
    println!("{}", name);
}
