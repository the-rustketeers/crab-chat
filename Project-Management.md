# Project Management

## 1. Members

A list of your team members with the project manager identified. Also include project aspect each team member will be responsible (client, server, or library code).

- Austin Swartley
- Evan Binkley
- Peter Schaefer

## 2. Communication Plan

- Snapchat and Github

## 3. Tasks

- Program design
  - JSON packet formatting and organization idea (Create format)
  - Create possible schedule and due dates.
  - Have consistent programming format practices for readability.
  - Create sequence diagram.
- Application Design (Writing)
  - Client
    - Basic nickname sending
    - JSON packet management and sending
    - Server-to-Client push receiving and printing.
  - Server
    - Basic server user setup (Executable startup process)
    - Thread management for user connections
    - Stream handling per thread, and JSON packet handling
    - Server-to-Client message pushing and updating.
    - Signal handling and graceful closing
  - Library management
    - Library (lib.rs) update as per sufficient needs (When applicable)

## 4. Programming Languages

- The client-side program for our project will be written and running in Rust.
- The server-side program for our project will also be written and running using Rust.
- The library for both programs will also be written using Rust, and shared between both programs for what is needed.

## 5. Project Requirements

- Client-side nickname sending
- Client-side message sending
- Client-side SIGINT handling
- User can choose color of nickname
- Server-side nickname checking/handling
- Server-side message receiving
- Server-side message management and message pushing / updating
- Server-side message and event logging
- Server-side graceful program termination
- Client taking Ip/Port #’s in CLA
- Server taking Port # in CLA

## 6. Application Inputs and Outputs

- Client-side
  - User inputs username/nickname (UTF-8 text restrictions)
  - User sends messages to terminal for processing (UTF-8 text restrictions)
  - Client updates board of messages as server pushes and receives them (UTF-8 text restrictions)
- Server-side
  - Server awaits port number for socket
  - Server processes information and pushes packet based data to clients based on receiving. (Packets received can be UTF-8, but are expected to be ASCII restrained in terms of content)
  - User can input termination signal for graceful close.

## 7. List of Shared Functionalities

- JSON packet formatting (May not be necessary for including)
- Encoding and decoding sent and received packets

## 8. Application Protocol

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
    - The nickname of the client will be in the type 'kind' of "author".
    - The message data will be in type 'kind' of "message".
    - Color data will be included in the type 'kind' of "color", formatted in the format above.
    - Time data will be included in type 'kind' of "time", as formatted above.
  - The user's messages will be recorded to a log file, which includes nickname, timestamp, and message content.
- **Receiving messages**
  - The client has an actively listening thread that will print all messages received.
    - This thread will also check for other types of things, such as:
      - Not printing messages with no type 'kind' of "message".
      - Other object types (Server disconnect, etc.)
    - This makes the messages a push-based service

## 9. Sequence Diagram

- See Docs folder, "Visio Program Sequence Diagram"

## 10. Test Plan

|Test Case Number|Brief Description|Expected Results|Tester Name|Date Tested|Actual Results|Success or Fail|Correction|
|:----|:----|:----|:----|:----|:----|:----|:----|
|1|Nickname taken|Error message stating nickname is taken and then ask for a new name.|Evan|12/8/23|Message states nickname is taken. Asks for new nickname|Success|N/A|
|2|Send a message|Push a message from the server to all connected clients|Austin|12/8/23|Message received by all active clients.|Success|N/A|
|3|Close a client|User inputs the close command and then their client session will be closed and cleaned up on server side, also should work with Ctrl+C|Evan |12/8/23|Terminal closes and cleans up server-side. Functional with SIGINT|Success|N/A|
|4|Close server|Should inform all clients upon closing, then close after X time.|Evan|12/8/23|Server sends message to all active clients upon receiving SIGINT, waits X amount of time, then closes. (server-only)|Success|N/A|
|5|Empty Message|If a user sends an empty message, then the message should still send to the server and all the other users.|Austin|12/8/23|An empty message is sent to active clients.|Success|N/A|
|6|User name color selection|User can select color for their nickname to display to clients when messaging.|Evan|12/8/23|Users can see colored nicknames in terminals when receiving other clients messages (based on their color choice), and if the choice is invalid, the program will simply shut down.|Success |N/A|
|7|Large amount of users all inputting messages.|If there are many users all sending messages at once, we would like to see no felt drop in performance.|Evan|12/7/23|Attempted to access multiple terminals at same time and send messages rapidly. Saw no difference in performance.|Success|N/A|
|8|Server logs messages and events|Server should print events and user messages to a log file.|Evan|12/8/23|Log file is created and appended to upon new messages or events caught by server.|Success|N/A|
|9|Client message sending|Client should accept (expected) characters and send to server.|Austin|12/8/23|Expected characters received by server and pushed to other active clients.|Success|N/A|
|10|Server-side nickname handling|Server should be able to access a table of active nicknames and refuse requests for taken ones|Evan|12/8/23|Server recognizes active nicknames from file table of nicknames and refuses new requests for ones already taken.|Success|N/A|
|11|Server-side message receiving|Server receives messages sent by any active client.|Austin|12/8/23|Server receives all messages. Evident by fact that multiple clients can receive clients message|Success|N/A|
|12|Client-side message receiving|Client able to receive messages sent by other clients.|Austin|12/6/23|Client is able to receive and print all messages received by active clients connect to same server |Success|N/A|
|13|Client taking bad IP addresses or ports|If bad IP or port is given, program refuses connection and exits, giving reason|Austin|12/6/23|Client refuses connection because of rust unwrap, error printed|Success |N/A|
|14|Server taking bad port #|If bad port is given in cla, server refuses to start, prints error for reason|Austin|12/6/23|Server refuses to start because of rust unwrap error checking, error printed|Success|N/A|
