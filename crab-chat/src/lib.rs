use json::{object, JsonValue};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str, thread,
};

pub const ADDRESS: &str = "127.0.0.1:13579";
const EXIT_CODE: &str = "!!";

/**
 * Sends a json object through the TcpStream, by first creating the object, sending its size, and then the object.
 * NEW: this now returns a result, so it can be better handled in context
 */
pub fn send_json_packet(s: &mut TcpStream, obj: JsonValue) -> Result<(), JsonError> {
    let strung = obj.dump();
    // println!("Sent Packet: {}", strung);
    let pack_size = strung.len();

    match s.write_all(&pack_size.to_be_bytes()) {
        Err(_) => return Err(JsonError::ConnectionAborted),
        Ok(()) => (),
    };
    match s.write_all(strung.as_bytes()) {
        Err(_) => return Err(JsonError::ConnectionAborted),
        Ok(()) => (),
    };

    Ok(())
}

/**
 * Receives a packet through the TcpStream, by first reading the size of the json packet, then reading the packet, then finally parsing the json packet.
 * NEW: this now returns a result, so it can be better handled in context
 */
pub fn receive_json_packet(s: &mut TcpStream) -> Result<JsonValue, JsonError> {
    let mut packet_size_buf: [u8; 8] = [0; 8];
    match s.read_exact(&mut packet_size_buf) {
        Err(_) => return Err(JsonError::ConnectionAborted),
        Ok(_) => (),
    }

    let packet_size: usize = usize::from_be_bytes(packet_size_buf);

    let mut packet_buf: Vec<u8> = vec![0; usize::from(packet_size)];

    match s.read_exact(&mut packet_buf) {
        Err(_) => return Err(JsonError::ConnectionAborted),
        Ok(_) => (),
    }
    let packet: &str = str::from_utf8(&packet_buf).unwrap();
    // println!("Received Packet: {}", packet);

    Ok(json::parse(packet).unwrap())
}

pub enum JsonError {
    ConnectionAborted,
}

pub fn connection_loop(stream: TcpStream) {
    println!("[START TYPING AND HIT ENTER TO SEND A MESSAGE]");
    let addr = stream.local_addr().unwrap();

    // copy stream and push it into a thread to handle getting input from user
    let mut stream_reader = stream.try_clone().unwrap();
    let reader = thread::spawn(move || loop {
        let mut msg = String::new();

        // read input from the user
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        let msg = msg.trim();

        // check if input is to exit, and leave the thread if so
        if msg == EXIT_CODE {
            println!("[GOODBYE]");
            break;
        };

        // wrap message in a json object
        let obj = object! {
            author: format!("{addr}"),
            message: msg,
        };

        // try to send json object, fails if server/client exited already
        match send_json_packet(&mut stream_reader, obj) {
            Ok(()) => (),
            Err(_) => break,
        };
    });

    // copy stream and move it into a thread to handle receiving incoming messages from the server
    let mut stream_listener = stream.try_clone().unwrap();
    let _listener = thread::spawn(move || loop {
        // endlessly try to get/print the next json packet from stream
        // fails if server/client exited already
        let obj = match receive_json_packet(&mut stream_listener) {
            Ok(obj) => obj,
            Err(_) => break,
        };
        println!("{} says: \"{}\"", obj["author"], obj["message"]);
    });

    // hold until the input reader ends
    reader.join().unwrap();
}
