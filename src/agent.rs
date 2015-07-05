
use cluster;
use storage;
use nix::sys::signal;

pub fn start(peers: Vec<&str>) {
    setup_signal_handler();

    for x in 1..peers.len() {
      println!("{} {}", x, peers[x]); // x: i32
    }
    //peers[0].as_ref(), args.iter().skip(1).collect::<Vec<_>>());
     
    //cluster::participate(peers[0].as_ref(), peers.iter().skip(1).collect::<Vec<_>>());
    cluster::participate(peers[0].as_ref(), peers.iter().skip(1).map(|s| s.to_owned()).collect::<Vec<_>>());
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

