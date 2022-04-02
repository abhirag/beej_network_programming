use std::os::unix::prelude::AsRawFd;

use nix::poll::{poll, PollFd, PollFlags};

fn main() {
    let mut pfds = [PollFd::new(0, PollFlags::POLLIN)];
    println!("Hit RETURN or wait 2.5 seconds for timeout");
    let num_events = poll(&mut pfds, 2500).expect("poll: poll failed");
    if num_events == 0 {
        println!("Poll timed out!");
    } else {
        let pollin_happened = pfds[0]
            .revents()
            .expect("poll: revents failed")
            .contains(PollFlags::POLLIN);
        if pollin_happened {
            println!("File descriptor {} is ready to read", pfds[0].as_raw_fd());
        } else {
            println!("Unexpected event occurred: {:?}", pfds[0].revents());
        }
    }
}
