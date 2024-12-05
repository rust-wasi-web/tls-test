//! Test program for TLS destructors.
//!
//! Expected output when run without arguments:
//!
//! thread start
//! thread exit
//! dropping: thread
//! thread joined
//! main exit
//! dropping: main
//!
//!
//! Expected output when run with `--thread-exit`:
//!
//! thread start
//! thread exiting process
//! dropping: thread
//!

use std::cell::Cell;
use std::env;

pub struct DropMsg(&'static str);

impl Drop for DropMsg {
    fn drop(&mut self) {
        println!("dropping: {}", self.0);
    }
}

thread_local! {
    pub static TLS: Cell<DropMsg> = Cell::new(DropMsg("init"));
}

fn main() {
    TLS.set(DropMsg("main"));

    println!("thread start");
    let hnd = std::thread::spawn(thread_fn);
    hnd.join().unwrap();
    println!("thread joined");

    println!("main exit");
}

pub fn thread_fn() {
    TLS.set(DropMsg("thread"));

    if env::args().any(|arg| arg == "--thread-exit") {
        println!("thread exiting process");
        std::process::exit(0);
    }

    println!("thread exit");
}
