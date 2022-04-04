# Socket Programing in Rust
Code from Beej's Guide to Network Programming ported to Rust

## Contents
- [showip.rs](src/showip.rs) - show IP addresses for a host given on the command line
- [client.rs](src/client.rs) - stream socket client demo
- [server.rs](src/server.rs) - stream socket server demo
- [poll.rs](src/poll.rs) - demo of `poll()`
- [pollserver.rs](src/pollserver.rs) - multiperson chat server using `poll()` 
- [select.rs](src/select.rs) - demo of `select()`
- [selectserver.rs](src/selectserver.rs) - multiperson chat server using `select()`
- [talker.rs](src/talker.rs) - datagram client demo
- [listener.rs](src/listener.rs) - datagram sockets server demo
- [broadcaster.rs](src/broadcaster.rs) - datagram client like talker.rs, except this one can broadcast
