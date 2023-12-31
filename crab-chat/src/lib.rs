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
use colored::Colorize;
use json::JsonValue;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
    vec,
};

// pub const ADDRESS: &str = "127.0.0.1:13579"; dead code
pub const EXIT_CODE: &str = "!!"; // command to be typed to exit from client side program (via command)
pub const ACTIVE_NICKNAME_FILE: &str = "active_nicks.log";
pub const SHUTDOWN_TIME: Duration = Duration::from_secs(3);
pub const USER_NAME_WIDTH: usize = 20;

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

/// Function name:      stringify_json_packet
/// Description:        To simply convert JSON object to a string
/// Parameters:         obj: &JsonValue | The JSON object to be converted to a string
/// Return Value:       String | The value of the JSON object in string format
pub fn stringify_json_packet(obj: &JsonValue, true_color: bool) -> String {
    let mut colors: Vec<&str> = vec![];
    if !obj["color"].is_null() {
        colors = obj["color"].as_str().unwrap().split(" ").collect();
    } else {
        colors.push("255");
        colors.push("255");
        colors.push("255");
    }

    if true_color {
        format!(
            "{}: {:<USER_NAME_WIDTH$} \n        \"{}\"",
            obj["time"],
            obj["author"].to_string().truecolor(
                colors[0].parse::<u8>().unwrap(),
                colors[1].parse::<u8>().unwrap(),
                colors[2].parse::<u8>().unwrap()
            ),
            obj["message"]
        )
    } else {
        format!(
            "{}: {:<USER_NAME_WIDTH$} \n        \"{}\"\n",
            obj["time"], obj["author"], obj["message"]
        )
    }
}

/// Function name:      log_to_file
/// Description:        To log the data passed in to a file name also given.
/// Parameters:         data: &String | The information to be printed to the file
///                     filename: &'static str | The filename to log the data to
/// Return Value:       None
pub fn log_to_file(data: &String, filename: &'static str) {
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap()
        .write_all(data.as_bytes())
        .unwrap_or_else(|_| {
            eprintln!("[ERROR: Write to '{}' failed]", filename);
        });
}

/// Function name:      get_rgb
/// Description:        takes a string input and outputs the corresponding RGB value
/// Parameters:         color: String | The string of a color
/// Returns:            Result: either a vec![R,G,B] or GetRgbError
pub fn get_rgb(mut color: String) -> Result<Vec<u8>, GetRgbError> {
    color = color.trim().to_lowercase();

    match color.as_str() {
        "red" => Ok(vec![255, 0, 0]),
        "orange" => Ok(vec![255, 165, 0]),
        "yellow" => Ok(vec![255, 255, 0]),
        "green" => Ok(vec![0, 255, 0]),
        "cyan" => Ok(vec![0, 255, 255]),
        "blue" => Ok(vec![0, 0, 255]),
        "magenta" => Ok(vec![255, 0, 255]),
        "white" => Ok(vec![255, 255, 255]),
        "black" => Ok(vec![0, 0, 0]),
        _ => Err(GetRgbError),
    }
}

#[derive(Debug)]
pub struct GetRgbError;

/// Struct containing user information for the client
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub name: String,
    pub color: Vec<u8>,
}
