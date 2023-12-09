# Things that we would like to do

- [x] I would like to add a packet-type option to our json packets, so that the client/server can deal with types more robustly.

- [x] We need to modify the fetch loop so that it doesn't use SO much of the CPU. On my machine it uses 12% of the CPU to do, most of the time, absolutely nothing. I think it would make sense to just have a sleep call at the end of each loop to limit how many times it can loop per second. The only thing is that this might affect performance if a large amount of clients are connected to the server. But it should be fine.

- [x] We need to make the client ask for the user's nickname and their preferred color of text.

- [x] Print out colored text according to client json requests

- [x] I would love to have the server output its logs to a file that it creates at the start of the session, which appends the shutdown and startup time of the server.

- [x] The server must be able to gracefully shutdown. If the server receives a CTRL-c to kill it, the server should send a message to all clients indicating it will be shutting down in x seconds, then wait x seconds and gracefully close. This should be done by using a signal handler for the SIGINT signal. (Done! WILL ONLY WORK ON BUILDS! Cargo run interferes with shutdown processes.)

- [x] Change the server and the client to take command line arguments for the address

- [x] Update readme file according to the project's specifications

- [x] Have a list of nicknames so that the server can approve nickname requests. When a client disconnects, it sends it's nickname so it can be removed from the list of nicknames.

- [ ] Have a `format_JSON_packet()` function that returns a string to reduce code duplication
