// selectserver.rs -- a cheezy multiperson chat server

use std::{net::SocketAddr, str::FromStr};

use nix::{
    sys::{
        select::{select, FdSet},
        socket::{
            accept, bind, getpeername, listen, recv, send, setsockopt, socket, sockopt,
            AddressFamily, InetAddr, MsgFlags, SockAddr, SockFlag, SockType,
        },
    },
    unistd::close,
};

fn get_listener_socket() -> i32 {
    const PORT: i32 = 9034;
    let std_sa = SocketAddr::from_str(format!("0.0.0.0:{}", PORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("selectserver: socket failed");

    setsockopt(sock, sockopt::ReuseAddr, &true).expect("selectserver: setsockopt failed");

    bind(sock, &sock_addr).expect("selectserver: bind failed");

    listen(sock, 10).expect("selectserver: listen failed");

    sock
}

fn main() {
    let mut readfds;
    let mut master = FdSet::new();
    let mut buf = [0; 256];
    let listener = get_listener_socket();
    master.insert(listener);
    let mut fdmax = listener;
    loop {
        readfds = master;
        select(fdmax + 1, &mut readfds, None, None, None).expect("selectserver: select failed");
        for i in 0..=fdmax {
            if readfds.contains(i) {
                if i == listener {
                    let sock;
                    match accept(listener) {
                        Ok(s) => {
                            sock = s;
                            master.insert(sock);
                            if sock > fdmax {
                                fdmax = sock;
                            }
                            if let Ok(saddr) = getpeername(sock) {
                                println!(
                                    "selectserver: new connection from {} on socket {}",
                                    saddr, sock
                                );
                            }
                        }
                        _ => panic!("selectserver: accept failed"),
                    }
                } else {
                    match recv(i, &mut buf, MsgFlags::empty()) {
                        Ok(numbytes) if numbytes == 0 => {
                            println!("selectserver: socket {} hung up", i);
                            close(i).expect("selectserver: close failed");
                            master.remove(i);
                        }
                        Ok(_) => {
                            for j in 0..=fdmax {
                                if master.contains(j) && j != listener && j != i {
                                    send(j, &buf, MsgFlags::empty())
                                        .expect("selectserver: send failed");
                                }
                            }
                        }
                        _ => {
                            close(i).expect("selectserver: close failed");
                            master.remove(i);
                            panic!("selectserver: recv failed");
                        }
                    }
                }
            }
        }
    }
}
