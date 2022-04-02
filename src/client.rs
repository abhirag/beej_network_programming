// client.rs -- a stream socket client demo

use std::{env, net::SocketAddr, process, str::FromStr};

use nix::{
    sys::socket::{
        self, connect, recv, AddressFamily, InetAddr, MsgFlags, SockAddr, SockFlag, SockType,
    },
    unistd::close,
};

fn main() {
    const PORT: i32 = 3490;
    const MAXDATASIZE: usize = 100;
    let mut buf = [0; MAXDATASIZE];
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: client hostname");
        process::exit(1);
    }

    let std_sa = SocketAddr::from_str(format!("{}:{}", args[1], PORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket::socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("server: socket failed");

    connect(sock, &sock_addr).expect("client: connect failed");

    println!("client: connecting to {}", &sock_addr);
    match recv(sock, &mut buf, MsgFlags::empty()) {
        Ok(numbytes) => println!(
            "client: received '{}'",
            String::from_utf8(buf[..numbytes].to_vec()).expect("client: string conversion failed")
        ),
        _ => panic!("client: recv failed"),
    }
    close(sock).expect("client: close failed");
}
