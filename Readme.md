# Crab-chat

## ID Block

### Program name

- **Crab-chat**

### Developers

- **Peter Schaefer**
- **Evan Binkley**
- **Austin Swartley**

## Building & Running Client & Server

The following commands must be in directory `.../crab-chat/crab-chat`.
The builds will be located in the then-created folder named `.../crab-chat/target/debug`.

- **Build**
  - **Client**: `cargo build --bin client`
  - **Server**: `cargo build --bin server`

- **Run**
  - **Client**: `cargo run --bin client [SERVER IP #] [SERVER PORT #]`
  - **Server**: `cargo run --bin server [PORT #]`

Note: Due to the way that 'cargo' operates, signal handling will only operate correctly when building and running the executable file. This is of no fault to the program.

## Files/Folders

| File or Folder | Purpose |
| :-: | :-- |
| crab-chat | Folder in which all of the actual program files are held. (Code, Libraries, etc.) |
| src | Folder containing rust code only |
| bin | Folder containing both the client and server source files |
| docs | Folder to hold external documents pertaining to this project |
| client.rs | Source code for client program |
| server.rs | Source code for server program |
| lib.rs | Source code for shared functionalities between server and client source code |
| history.log | A file created or appended to upon running the server. Records server events in text format. (File created after program runs, created wherever the program has run from.) |
| active_nicks.log | A temporary file to store active nicknames server-side. (File created after program runs, and created where the program ran. Deleted after shutdown.) |
| cargo.toml | File that describes and manages external libraries for 'cargo' service to download and prepare |
| Project-Management.md | File containing the basic outline and premise of the project |
| Readme.md | This file, containing the details of the project |
| Todo.md | File containing list of items that are wanted to be done (Removable) |

## Responsibilities

### Peter Schaefer

- Structural setup and preparations
- Implemented system for managing and pushing messages to all clients
- Thought of and created JSON development for our project
- Created basic programming format which was henceforth followed
- Simplified files and helped to correct bad coding practices

### Evan Binkley

- Implemented and developed nickname checking server-side
- Implemented and developed colored names (with help from JSON formatting)
- Found techniques for managing threads and using channels
- Set up signal handling
- Document support and creations
- Set up meeting times and coordinated team

### Austin Swartley

- Designed high level flow of the project
- Refined logic throughout the server
- Helped find final idea for nickname and stream handling
- Documented functions

## Tasks & Completion Times

### Client

| Task | Approx. Completion Time |
| :-: | :-- |
| Basic nickname sending | 2-3 days |
| JSON packet management and sending | 1 week |
| Server-to-client push receiving and printing | 5 days |

### Server

| Task | Approx. Completion Time |
| :-: | :-- |
| Basic server user setup | 1 day |
| Thread management for user connections | 2 days |
| Stream handling per thread, and JSON packet handling | 2 days |
| Server-to-client message pushing and updating | 5 days |
| Signal handling and graceful closing | < 1 day |

### Library management

| Task | Approx. Completion Time |
| :-: | :-- |
| Set up | < 1 day |
| Sending and receiving json packets | 1 day |
| Extracting duplicated code into functions | 3 days |

## Updated Scope

- JSON Objects as packet types
- Client/Server nickname confirmations
- User nickname color choice
- Server-side message handling and pushing to all clients
- User inputted program termination, both server and client-side
- Logs sent user packets (Server-side)
- User receiving and sending messages
- Client listener for received messages
- Server managing thread for managing active connections and holding them in memory

## Protocol

### JSON Packet Makeup

| Component | Explanation | Type |
| :-: | :-- | :-- |
| '**author**' | Nickname (previously accepted) of client sending message, in string format. | Type `String` in rust |
| '**time**' | Time at the packets creation in (Hours:Minutes:Seconds), in string format. | Type `String` in rust |
| '**message**' | Contents of the message in the form of a string. | Type `str` in rust |
| '**color**' | Three consecutive integers in the range `[0, 255]` in a single string, separated by spaces. | Type `String` in rust |
| '**kind**' | (Optional, sent under specific circumstances) Contains specific packet type, signalling special behaviour by either server or client. | Type `str` in rust |

### JSON Usage Outline

- When client connects, its address is saved to a list of active clients in a scanning thread's memory. This removes the need for a "hello" message to be received.
- When client disconnects, it will send a JSON object with type 'kind' of "disconnection", signalling it's removal from the active list of clients
- After client successfully connects, they will select a username. This username is set as the "author" type of a JSON object, and the type 'kind' of "nick".
  - The server will verify the uniqueness of the nickname it has received.
    - If unique, the server will send a JSON object back to the client with the type 'kind' of "okay". The client is now able to freely send and receive messages.
    - If not unique, the server will send a JSON object back to the client with the type 'kind' of "retry". This will start a loop that will make the client select a nickname not in use.
    - When the client selects a unique nickname, the client's nickname, IP address, and active timestamp is recorded in a log file.
- **Sending messages**
  - The user will enter a message through the client, which will then be sent to the server.
  - The user's messages will be recorded to a log file, which includes nickname, timestamp, and message content.
- **Receiving messages**
  - The client has an actively listening thread that will print all messages received.
    - This thread will also check for other types of things, such as:
      - Not printing messages with no type 'kind' of "message".
      - Other object types (Server disconnect, etc.)
    - This makes the messages a push-based service

These protocol specs are also found in ProtocolSpecs.md

## Assumptions

- **User nicknames**
  - User nicknames are assumed to be able to fit inside of a rust `String`.
  - User nicknames do not contain invalid unicode characters.
  - User nicknames cannot contain the newline character, `\n`.
  - Values should not contain arbitrary, non-printable characters.
- **Messages**
  - Messages are assumed to be able to fit inside of a rust `String`.
  - Messages do not contain invalid unicode characters.
  - Values should not contain arbitrary non-printable characters.
- **Host Machine**
  - Host machine is able to accept connections and gives access to ports.

## Discussion

- **Decision: JSON Object Packets**
  - This was a rather unanimous vote that we would implement JSON Objects as our main packets sent and received in order to make the programming process ahead easier to manage and develop upon.
- **Major issue: Pushing to clients**
  - We had come across an issue where we found it difficult to manage every single connection with separate threads and also send the messages the server had received to all active clients. We had begun multiple development processes simultaneously to try to resolve this issue.
  - Eventually, after team communication and discussion, Austin had discovered the current method used that uses a single thread receiving from an mpsc channel to deal with sending to all clients.
- **Minor issue: Updating list of clients after disconnection**
  - At one point, the server program was not able to see if a client should be removed from the list of active TcpStreams.
  - This was a very simple fix of testing the connection, and returning it's result as a form of appending approval/denial.
- **Major issue: Holding active nicknames**
  - We had come across an issue where we could not hold active nicknames throughout all of our threads, also due to the complexity of the overall program at this point.
  - We initially tried to use a global struct that was able to hold the TcpStream as well as the nickname per client. This idea had issues due to rust's lifetime errors per thread, and was never able to be fully implemented.
  - After team discussion (and approval), we had decided to implement a system where the names are written down to a .log file, labelled "active_nicks.log", and read/written from there as a sort of multi-accessible variable for all threads.
- **Minor issue: SIGINT handling**
  - We had come across and issue where we were not able to properly wait 3 seconds to handle SIGINT in the server-side of the program.
  - After research, it would appear the 'cargo run' command built into rust runs the program through an emulator.
  - The solution ended up to run the built product instead of through the emulator, which overrode the SIGINT sent through the terminal.

## Status

- client.rs
  - Works as expected, with one known issue:
    - The client receives the message it sends.
      - We would like to have the program not echo the user's input. However, we have encountered Rust limitations regarding this. So instead of removing the ability to see text being entered in order to accomplish this, we simply leave the echo'd text in the terminal, and let the client receive the formatted text.
  - Has fully implemented things expected in scope.
- server.rs
  - Works as expected, with no known errors.
  - Has fully implemented things expected in scope.
