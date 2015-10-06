#![allow(unused_must_use)]
extern crate nix;
use std::thread::sleep_ms;
use std::process;
use nix::sys::signal;

pub fn main() {
    println!("setting up signal handler");
    let sig_action = signal::SigAction::new(handle_sigint,
                                            signal::SockFlag::empty(),
                                            signal::SigSet::empty());
    unsafe {

        signal::sigaction(signal::SIGINT, &sig_action);
    }

    loop {
      sleep_ms(50);
    }
}

extern fn handle_sigint(_: i32) {
    println!("got sigint");
    sleep_ms(100);
    process::exit(0);
}

