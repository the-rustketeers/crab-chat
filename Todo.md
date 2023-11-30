# Things that we would like to do

- [x] I would like to add a packet-type option to our json packets, so that the client/server can deal with types more robustly.

- [ ] We need to modify the fetch loop so that it doesn't use SO much of the CPU. On my machine it uses 12% of the CPU to do, most of the time, absolutely nothing. I think it would make sense to just have a sleep call at the end of each loop to limit how many times it can loop per second. The only thing is that this might affect performance if a large amount of clients are connected to the server. But it should be fine.

- [ ] We need to make the client ask for the user's nickname and their preferred color of text.

- [ ] I would love to have the server output its logs to a file that it creates at the start of the session, which the name could have something to do with the current time.
