use nix::sys::{
    select::{select, FdSet},
    time::{TimeVal, TimeValLike},
};

fn main() {
    const STDIN: i32 = 0;
    let mut readfds = FdSet::new();
    readfds.insert(STDIN);
    let mut timeout = TimeVal::milliseconds(2500);
    select(STDIN + 1, &mut readfds, None, None, &mut timeout).expect("select failed");
    if readfds.contains(STDIN) {
        println!("A key was pressed!");
    } else {
        println!("Timed out.");
    }
}
