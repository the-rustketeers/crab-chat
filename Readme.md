# Crab-chat

## ID Block
### Program name
- **Crab-chat**

### Developers
- **Peter Schaefer**
- **Evan Binkley**
- **Austin Swartley**


## How to run/build Client and Server

*Must be in directory: ".../crab-chat/crab-chat/src/bin"

- **Client (run)**: `cargo run --bin client` 
- **Client (build)**: `cargo build --bin client`

- **Server (run)**: `cargo run --bin server` 
- **Server (build)**: `cargo run --bin server`

*(Builds will be located in the then-created folder named 'target')*

## Files/Folders
#### crab-chat
 - Folder in which all of the actual program files are held. (Code, libraries, etc.)
    #### src
    - Folder containing rust code only
        #### bin
        - Folder containing both the client and server source files
            ##### client.rs 
            - Source code for client program
            ##### server.rs
            - Source code for server program
        ##### lib.rs
        - Source code for shared functionalities between server and client source codes
    ##### cargo.toml
    - File that describes and manages external libraries for 'cargo' service to download and prepare
#### docs
- Folder to hold external documents pertaining to this project
#### Project-Management.md
- File containing the basic outline and premise of the project
#### Readme.md
- This file, containing the details of the project
#### Todo.md
- File containing list of items that are wanted to be done (Removable)

## Responsibilities
### Peter Schaefer
    - Structural setup and preperations
    - Implemented system for managing and pushing messages to all clients
    - Thought of and created JSON development for our project
    - Created basic programming format which was henceforth followed
    - Simplified files and helped to correct bad coding practices
### Evan Binkley
    - Implemented and developed nickname checking server-side
    - Implemented and developed colored names (with help from JSON formatting)
    - Found techniques for managing threads and using channels
    - Setup framework for basic signal handling
    - Document support and creations
    - Set up meeting times and coordinated team
### Austin Swartley
    - Designed high level flow of the project
    - Refined logic throughout the server
    - Helped find final idea for nickname and stream handling
    - Doucumented funtions


## Tasks and completion times

### Client
    - Basic nickname sending
        - 2-3 days
    - JSON packet management and sending
        - 1 week
    - Server-to-Client push receiving and printing.
        - 5 days
### Server
    - Basic server user setup
        - 1 day
    - Thread management for user connections
        - 2 days
    - Stream handling per thread, and JSON packet handling
        - 2 days (Same as 'Thread management')
    - Server-to-Client message pushing and updating.
        - 5 days (Same as client-side receiving)
    - Signal handling and graceful closing
        - Less than a day
### Library management
    - Library (lib.rs) update as per sufficient needs (When applicable)
        - The sending and receiving of json packets and it's setup:
            - 1 day

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
    JSON Object Makeup:
        author:
            Nickname (previously accepted) of client sending message, in string format. (Type String in rust)
        time:
            Time at the packets creation in (Hours:Minutes:Seconds), in string format. (Type String in rust)
        message: 
            Contents of the message in the form of a string. (Type str in rust)
        color:
            Three consecutive numbers 255 > x > 0 in a single string, seperated by spaces. (Type String in rust)
        kind (Optional, sent under specific circumstances):
            Contains specific packet type, signalling special behaviour by either server or client.
            In the form of a string. (Type str in rust)


    - When client connects, it's address is saved to a list of active clients in a scanning thread's memory.
      This removes the need for a "hello" message to be received.
    - When client disconnects, it will send a JSON object with type 'kind' of "disconnection", signalling it's removal
      from the active list of clients
    - After client successfully connects, they will select a username. This username is set as the "author" type
      of a JSON object, and the type 'kind' of "nick". 
        - The server will verify the uniqueness of the nickname it has received.
            - If unique, the server will send a JSON object back to the client with the type 'kind' of "okay".
              The client is now able to freely send and receive messages.
            - If not unique, the server will send a JSON object back to the client with the type 'kind' of "retry".
              This will start a loop that will make the client select a nickname not in use.
            - When the client selects a unique nickname, the client's nickname, IP address, and active 
              timestamp is recorded in a log file.
    - Sending messages
        - The user will enter a message through the client, which will then be sent to the server.
        - The user's messages will be recorded to a log file, which includes nickname, timestamp, and message content.
    - Receiving messages
        - The client has an actively listening thread that will print all messages received.
          This thread will also check for other types of things, such as:
            Not printing empty messages
            Other object types (Server disonnect, etc.)
        - This makes the messages a push-based service

## Assumptions
    - User nicknames are able to fit inside of size-string and do not contain
      values that would not be accepted in type 'string'. Also cannot contain "£".
      (Values should not contains arbitrary non-printable characters)
    - Messages sent must also be able to fit insize of size string and do not contain
      values that would not be accepted in type 'string'.
      (Values should not contains arbitrary non-printable characters)
    - Host machine is able to accept connections and gives access to ports.

## Discussion
    - Decision: JSON Object Packets
        - This was a rather unanimous vote that we would implement JSON Objects as our main
          packets sent and received in order to make the programming process ahead easier to
          manage and develop upon.
    - Major issue: Pushing to clients
        - We had come accross an issue where we found it difficult to manage every single
          connection with seperate threads and also send the messages the server had received to 
          all active clients. We had begun multiple development processes simultaneously to try to 
          resolve this issue.
        - Eventually, after team communication and discussion, Austin had discovered the current 
          method used that uses a single thread receiving from an mpsc channel to deal with 
          sending to all clients.
    - Minor issue: Updating list of clients after disconnection
        - At one point, the server program was not able to see if a client should be removed from 
          the list of active TcpStreams. 
        - This was a very simple fix of testing the connection, and returning it's result as a 
          form of appending approval/denial.
    - Major issue: Holding active nicknames
        - We had come accross an issue where we could not hold active nicknames throughout all of 
          our threads, also due to the complexity of the overall program at this point.
        - We initially tried to use a global struct that was able to hold the TcpStream as well as 
          the nickname per client. This idea had issues due to rust's lifetime errors per thread, 
          and was never able to be fully implemented.
        - After team discussion (and approval), we had decided to implement a system where the 
          names are written down to a .log file, labelled "active_nicks.log", and read/written 
          from there as a sort of multi-accessable variable for all threads.
        
## Status
    - client.rs
        - Works as expected, with no errors whatsoever.
        - Has fully implemented things expected in scope.
    - server.rs
        - Works as expected, with no errors whatsoever.
        - Has mostly implemented things expected in scope.
            - Is missing server shutdown timer. Not sure if able to implement.

(Not sure what else to add for "status", but there is possibly more.
Subject to updates and changes as the project further develops.)