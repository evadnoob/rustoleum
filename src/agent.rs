
use cluster;
use storage;
use nix::sys::signal;

pub fn start(port: &str, peers: Vec<&str>) {
    setup_signal_handler();

    cluster::participate(port, peers);
}

extern fn handle_sigint(_:i32) {
    println!("Interrupted!");
    //panic!();
}


fn setup_signal_handler() {
    unsafe {
        
        let sig_action = signal::SigAction::new(handle_sigint,
                                                signal::SockFlag::empty(),
                                                signal::SigSet::empty());
        signal::sigaction(signal::SIGINT, &sig_action);
    }
}

