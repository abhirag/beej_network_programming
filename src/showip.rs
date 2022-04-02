// showip.rs -- show IP addresses for a host given on the command line

use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
    process,
};

fn main() {
    let mut ipver = "";
    const PORT: i32 = 80;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: showip hostname");
        process::exit(1);
    }

    let addrs: Vec<SocketAddr> = format!("{}:{}", args[1], PORT)
        .to_socket_addrs()
        .expect("unable to resolve hostname")
        .collect();

    println!("IP addresses for {}:\n", args[1]);
    for addr in addrs {
        if addr.is_ipv4() {
            ipver = "IPv4";
        }
        if addr.is_ipv6() {
            ipver = "IPv6";
        }
        println!("{}: {}", ipver, addr.ip());
    }
}
