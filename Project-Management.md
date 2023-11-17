# Project Management

## 1. Members

A list of your team members with the project manager identified. Also include project aspect each team member will be responsible (client, server, or library code).

- Austin Swartley
- Evan Binkley
- Peter Schaefer

## 2. Communication Plan

- Snapchat, discord, and Github

## 3. Tasks

- Program design
  - JSON packet formatting and organization idea (Create format)
  - Create possible schedule and due dates.
  - Have consistent programming format practices for readability.
  - Create sequence diagram.
- Application Design (Writing)
  - Client (Peter Schaefer)
    - Basic nickname sending
    - JSON packet management and sending
    - Server-to-Client push receiving and printing.
  - Server (Austin Swartley & Evan Binkley)
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

- Client-side message and nickname sending
- Server-side message receiving
- Server-side message management and message pushing/updating
- User inputted program termination

## 6. Application Inputs and Outputs

- Client-side
  - User inputs username/nickname
  - User sends messages to terminal for processing
  - Client updates board of messages as server pushes and receives them
- Server-side
  - Server awaits port number for socket
  - Server processes information and pushes packet based data to clients based on receiving.
  - User can input termination signal for graceful close.

## 7. List of Shared Functionalities

- JSON packet formatting (May not be necessary for including)
- Encoding and decoding sent and received packets

## 8. Application Protocol

- Specifically designed JSON packets (Formatted in previously mentioned manner) in order to convey data sent from client to server, as well as server to client.

## 9. Sequence Diagram

- Found within Visio Sequence Diagram file.

## 10. Test Plan

- TBD
