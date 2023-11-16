use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

use json::JsonValue;

/**
 * Sends a json object through the TcpStream, by first creating the object, sending its size, and then the object.
 */
fn send_json_packet(s: &mut TcpStream, obj: JsonValue) {
    let strung = obj.dump();
    let pack_size = strung.len();

    s.write_all(&pack_size.to_be_bytes()).unwrap();
    s.write_all(strung.as_bytes()).unwrap();
}

/**
 * Receives a packet through the TcpStream, by first reading the size of the json packet, then reading the packet, then finally parsing the json packet.
 */
fn receive_json_packet(s: &mut TcpStream) -> JsonValue {
    let mut packet_size_buf: [u8; 2] = [0; 2];
    s.read_exact(&mut packet_size_buf).unwrap();

    let packet_size: u16 = u16::from_be_bytes(packet_size_buf);

    let mut packet_buf: Vec<u8> = vec![0; usize::from(packet_size)];

    s.read_exact(&mut packet_buf).unwrap();
    let packet: &str = str::from_utf8(&packet_buf).unwrap();

    json::parse(packet).unwrap()
}
