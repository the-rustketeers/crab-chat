## Protocol

### JSON Packet Makeup

| Component | Explanation | Type |
| :-: | :-- | :-- |
| '**author**' | Nickname (previously accepted) of client sending message, in string format. | Type `String` in rust |
| '**time**' | Time at the packets creation in (Hours:Minutes:Seconds), in string format. | Type `String` in rust |
| '**message**' | Contents of the message in the form of a string. | Type `str` in rust |
| '**color**' | Three consecutive numbers in the range `[0, 255]` in a single string, separated by spaces. | Type `String` in rust |
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