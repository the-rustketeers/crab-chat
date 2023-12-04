use json::JsonValue;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub const ADDRESS: &str = "127.0.0.1:13579";
pub const EXIT_CODE: &str = "!!";
pub const ACTIVE_NICKNAME_FILE: &str = "active_nicks.log";

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
    let packet: &str = std::str::from_utf8(&packet_buf).unwrap();
    // println!("Received Packet: {}", packet);

    Ok(json::parse(packet).unwrap())
}

#[derive(Debug)]
pub enum JsonError {
    ConnectionAborted,
}

pub fn log_json_packet(obj: &JsonValue) {
    println!("{:?}", obj);
}
