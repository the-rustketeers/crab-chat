/*
Authors:        Peter Schaefer, Evan Binkley, and Austin Swartley
Creation Date:  11/21-12/5
Due Date:       12/14/23 @ 10:00am
Course:         CsC328-020
Professor name: Dr. Schwesinger
Assignment:     Final Project
Filename:       lib.rs
Purpose:        This is the library for shared code between the client and server programs.
*/
use json::JsonValue;
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

// pub const ADDRESS: &str = "127.0.0.1:13579"; dead code
pub const EXIT_CODE: &str = "!!"; // command to be typed to exit from client side program (via command)
pub const ACTIVE_NICKNAME_FILE: &str = "active_nicks.log";
pub const SHUTDOWN_TIME: Duration = Duration::from_secs(3);

/// Function name:      send_json_packet
/// Description:        Sends a JSON packet to the desired stream
/// Parameters:         s: &mut TcpStream | The stream to send the JSON packet to
///                     obj: JsonValue | The JSON packet to be sent
/// Return Value:       Result<(), JsonError> | An Ok()/Err() result to determine the function's success in use cases
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

/// Function name:      receive_json_packet   
/// Description:        Receives a packet through the given TcpStream, reading size, then content, and finally parsing the packet itself
/// Parameters:         s: &mut TcpStream | The stream to receive a packet from
/// Return Value:       Result<JsonValue, JsonError> | An Ok()/Err() result to determine the function's success, and upon success, return the JSON object retrieved.
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
///Type JsonError for error types
pub enum JsonError {
    ConnectionAborted,
}

/// Function name:      log_json_packet
/// Description:        To simply print the given JSON object, for logging purposes and debug server-side
/// Parameters:         obj: &JsonValue | The JSON object to be printed
/// Return Value:       None
pub fn log_json_packet(obj: &JsonValue) {
    println!("{:?}", obj);
}
