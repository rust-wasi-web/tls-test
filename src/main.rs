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

#![feature(cfg_target_thread_local)]

use std::cell::Cell;
use std::env;
use std::env::consts::{ARCH, FAMILY, OS};

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
    println!("arch={ARCH} family={FAMILY} os={OS}");
    println!("target_thread_local={}", cfg!(target_thread_local));
    println!();

    TLS.set(DropMsg("main"));

    println!("thread start");
    match std::thread::Builder::new().spawn(thread_fn) {
        Ok(hnd) => {
            hnd.join().unwrap();
            println!("thread joined");
        }
        Err(err) => println!("cannot start thread: {err}"),
    }

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
