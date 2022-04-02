// server.rs -- a stream socket server demo

use libc::_exit;
use nix::{
    errno::Errno,
    sys::{
        signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal},
        socket::{
            self, accept, bind, getpeername, listen, send, setsockopt, sockopt, AddressFamily,
            InetAddr, MsgFlags, SockAddr, SockFlag, SockType,
        },
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{close, fork, ForkResult},
};
use std::{net::SocketAddr, str::FromStr};
use Signal::SIGCHLD;

extern "C" fn sigchld_handler(_signal: libc::c_int) {
    while let Ok(WaitStatus::StillAlive) = waitpid(None, Some(WaitPidFlag::WNOHANG)) {}
}

fn main() {
    const PORT: i32 = 3490;
    const BACKLOG: usize = 10;
    let std_sa = SocketAddr::from_str(format!("0.0.0.0:{}", PORT).as_str()).unwrap();
    let inet_addr = InetAddr::from_std(&std_sa);
    let sock_addr = SockAddr::new_inet(inet_addr);

    let sock = socket::socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("server: socket failed");

    setsockopt(sock, sockopt::ReuseAddr, &true).expect("server: setsockopt failed");

    bind(sock, &sock_addr).expect("server: bind failed");

    listen(sock, BACKLOG).expect("server: listen failed");

    let handler = SigHandler::Handler(sigchld_handler);
    let sa = SigAction::new(handler, SaFlags::empty(), SigSet::empty());
    unsafe {
        sigaction(SIGCHLD, &sa).expect("server: sigaction failed");
    };

    println!("server: waiting for connections...");
    loop {
        let session_sock;
        loop {
            match accept(sock) {
                Err(Errno::EINTR) => continue,
                Ok(s) => {
                    session_sock = s;
                    if let Ok(saddr) = getpeername(session_sock) {
                        println!("server: got connection from {}", saddr);
                    }
                    break;
                }
                _ => panic!("server: accept failed"),
            }
        }
        match unsafe { fork() }.expect("server: fork failed") {
            ForkResult::Child => {
                close(sock).expect("server: close failed");
                send(session_sock, "Hello, World!".as_bytes(), MsgFlags::empty())
                    .expect("server: send failed");
                close(session_sock).expect("server: close failed");
                unsafe { _exit(0) }
            }
            _ => close(session_sock).expect("server: close failed"),
        }
    }
}
