// CHANGES MADE FOR TESTING PURPOSES ONLY!

use crab_chat as lib;
use json::object;
use std::{
    net::{TcpListener, TcpStream},
    process,
    time,
};
use crossbeam::{self, channel::at};

const ERR: i32 = -1;

/**
 * Description: main program. takes exactly 1 command line argument.
 *   demonstrates binding, listening, and finally connecting to a socket.
 */
fn main() {

    let tpool = lib::ThreadPool::new(10);
    // Size basically means nothing here, but would be halved due to double thread usage.
    // There is most certainly a better way to handle threads, I'm sure.

    //set up TcpListener, bind to port, and listen for connections
    let listener: TcpListener = TcpListener::bind(lib::ADDRESS).unwrap_or_else(|why| {
        eprintln!("ERROR: {why}");
        process::exit(ERR);
    });

    println!(
        "INFO: Server started on and listening for connections on address {:?}",
        listener.local_addr().unwrap()
    );

    let (tx, rx) = crossbeam::channel::unbounded();
    

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("INFO: Got a connection from {:?}", s.peer_addr().unwrap());
                let mut readstream = s.try_clone().expect("Cannot clone stream");
                let mut sendstream = s.try_clone().expect("Cannot clone stream");
                // NOTE FOR ADDITION!: move is used for the closure in order to kill stream after use.
                // stream is now OWNED by thread and will terminate after scope is lost. (End of thread execution)
                // This is why stream is cloned for read purposes. Both streams identical.


                let thread_tx2 = tx.clone();
                let thread_rx = rx.clone();
                let mut send_string: String;                    // Two declared strings to be used for the reading.
                let mut prev_string: String = "".to_string();

                tpool.execute(move || {

                    loop {
                        std::thread::sleep(time::Duration::from_millis(100)); // Loops through indefinitely; an attempt to read from channel across multiple threads.
                        match thread_rx.recv() {                              // This match case may not be entirely useful yet, I'm not sure.
                            Ok(send_string) => {                          // Right now the main issue is getting the information from the out-end of the channel
                                if (send_string == prev_string) {                 // to be read from every thread. The information DOES NOT stay there, and I'm done trying to
                                    println!("Same string!");                     // figure out how to force it to read it. For now.
                                    continue;
                                }
                                if (send_string != prev_string) {
                                    connection_loop(&mut sendstream, &send_string);
                                    prev_string = send_string.clone();
                                    thread_tx2.send(send_string).unwrap();
                                }
                            },
                            Err(_) => {
                                continue;
                            }
                        }


                        //let send_string = thread_rx.recv().unwrap();
                        //connection_loop(&mut sendstream, &send_string);

                        // ISSUE: Due to the fact that s is being moved, it will be killed after connection_loop finishes.
                        // Because I suspect the server will only ever send messages after receiving them (For pushing)
                        // This (in theory) should not be a problem. There are solutions to this- should it be one, though.
                    }
                });
                
                let thread_tx = tx.clone(); // Channel clone, for the move in thread

                tpool.execute(move || {

                    loop {
                        let n_s: String = receiver_loop(&mut readstream);
                        // NOTE FOR ADDITION!: prints out values so is readable server-side to prove thread is working properly.
                        println!("{}", n_s);
                        // THIS WILL PANIC! After the client's connection is finished sending ALL PACKETS PLANNED!
                        thread_tx.send(n_s).unwrap();
                    }
                    // Because it is a thread, this is fine (enough), but not good.
                });

            }

            Err(why) => {
                eprintln!("ERROR: {why}");
                process::exit(ERR);
            }
        }
    }
}

fn connection_loop(s: &mut TcpStream, snd: &String) {
    let name_prompt = object! {
        message: snd.as_str()
    };
    println!("{:#?}", name_prompt.dump());
    lib::send_json_packet(s, name_prompt);
}

/// server-side receiving and printing. only prints message for testing purposes.
fn receiver_loop(s: &mut TcpStream) -> String {
    let obj = lib::receive_json_packet(s);
    obj["message"].to_string()
}
