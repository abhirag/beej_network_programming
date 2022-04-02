// pollserver.rs -- a cheezy multiperson chat server

use std::{net::SocketAddr, os::unix::prelude::AsRawFd, str::FromStr};

use nix::{
    poll::{poll, PollFd, PollFlags},
    sys::socket::{
        self, accept, bind, getpeername, listen, recv, send, setsockopt, sockopt, AddressFamily,
        InetAddr, MsgFlags, SockAddr, SockFlag, SockType,
    },
    unistd::close,
};

fn get_listener_socket() -> i32 {
    const PORT: i32 = 9034;
    let std_sa = SocketAddr::from_str(format!("0.0.0.0:{}", PORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket::socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("pollserver: socket failed");

    setsockopt(sock, sockopt::ReuseAddr, &true).expect("server: setsockopt failed");

    bind(sock, &sock_addr).expect("pollserver: bind failed");

    listen(sock, 10).expect("pollserver: listen failed");

    sock
}

fn add_to_pfds(pfds: &mut Vec<PollFd>, sock: i32, pfds_len: &mut usize) {
    let pfd = PollFd::new(sock, PollFlags::POLLIN);
    pfds.push(pfd);
    *pfds_len += 1;
}

fn del_from_pfds(pfds: &mut Vec<PollFd>, i: usize, pfds_len: &mut usize) {
    pfds.remove(i);
    *pfds_len -= 1;
}

fn main() {
    let mut pfds_len = 0;
    let mut buf = [0; 256];
    let listener = get_listener_socket();
    let mut pfds = vec![PollFd::new(listener, PollFlags::POLLIN)];
    pfds_len += 1;
    loop {
        poll(&mut pfds, -1).expect("pollserver: poll failed");
        for i in 0..pfds_len {
            if pfds[i]
                .revents()
                .expect("pollserver: revents failed")
                .contains(PollFlags::POLLIN)
            {
                if pfds[i].as_raw_fd() == listener {
                    let sock;
                    match accept(listener) {
                        Ok(s) => {
                            sock = s;
                            add_to_pfds(&mut pfds, sock, &mut pfds_len);
                            if let Ok(saddr) = getpeername(sock) {
                                println!(
                                    "pollserver: new connection from {} on socket {}",
                                    saddr, sock
                                );
                            }
                        }
                        _ => panic!("pollserver: accept failed"),
                    }
                } else {
                    match recv(pfds[i].as_raw_fd(), &mut buf, MsgFlags::empty()) {
                        Ok(numbytes) if numbytes == 0 => {
                            println!("pollserver: socket {} hung up", pfds[i].as_raw_fd());
                            close(pfds[i].as_raw_fd()).expect("pollserver: close failed");
                            del_from_pfds(&mut pfds, i, &mut pfds_len);
                        }
                        Ok(_) => {
                            for j in 0..pfds_len {
                                let dest_fd = pfds[j].as_raw_fd();
                                if dest_fd != listener && dest_fd != pfds[i].as_raw_fd() {
                                    send(dest_fd, &buf, MsgFlags::empty())
                                        .expect("pollserver: send failed");
                                }
                            }
                        }
                        _ => {
                            close(pfds[i].as_raw_fd()).expect("pollserver: close failed");
                            del_from_pfds(&mut pfds, i, &mut pfds_len);
                            panic!("pollserver: recv failed");
                        }
                    }
                }
            }
        }
    }
}
