extern crate zmq;

use agent::cluster;
use storage::storage;
use nix::sys::signal;

pub fn start() {
    setup_signal_handler();
    cluster::join();
    storage::bootstrap();

    info!("0MQ version: {:?}", zmq::version());
    let mut context = zmq::Context::new();

    let mut responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        info!("Received {}", msg.as_str().unwrap());
        responder.send_str("Yes she does", 0).unwrap();
        //thread::sleep_ms(1000);
    }
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
