// listener.rs -- a datagram sockets "server" demo

use std::{net::SocketAddr, str::FromStr};

use nix::{
    sys::socket::{self, bind, recvfrom, AddressFamily, InetAddr, SockAddr, SockFlag, SockType},
    unistd::close,
};

fn main() {
    const MYPORT: i32 = 4950;
    const MAXBUFLEN: usize = 100;
    let mut buf = [0; MAXBUFLEN];
    let std_sa = SocketAddr::from_str(format!("0.0.0.0:{}", MYPORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket::socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("listener: socket failed");

    if bind(sock, &sock_addr).is_err() {
        close(sock).expect("listener: close failed");
        panic!("listener: bind failed");
    }

    println!("listener: waiting to recvfrom...");
    match recvfrom(sock, &mut buf) {
        Ok((numbytes, s)) => {
            if let Some(saddr) = s {
                println!("listener: got packet from {}", saddr);
            }
            println!("listener: packet is {} bytes long", numbytes);
            println!(
                "listener: packet contains '{}'",
                String::from_utf8(buf[..numbytes].to_vec())
                    .expect("listener: string conversion failed")
            )
        }
        _ => panic!("listener: recvfrom failed"),
    }
    close(sock).expect("listener: close failed");
}
