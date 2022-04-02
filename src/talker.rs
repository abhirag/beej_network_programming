// talker.rs -- a datagram "client" demo

use std::{env, net::SocketAddr, process, str::FromStr};

use nix::{
    sys::socket::{self, sendto, AddressFamily, InetAddr, MsgFlags, SockAddr, SockFlag, SockType},
    unistd::close,
};

fn main() {
    const SERVERPORT: i32 = 4950;
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: talker hostname message");
        process::exit(1);
    }

    let std_sa = SocketAddr::from_str(format!("{}:{}", args[1], SERVERPORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket::socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("talker: socket failed");

    match sendto(sock, args[2].as_bytes(), &sock_addr, MsgFlags::empty()) {
        Ok(numbytes) => println!("talker: sent {} bytes to {}", numbytes, &sock_addr),
        Err(_) => panic!("talker: sendto failed"),
    }
    close(sock).expect("talker: close failed");
}
