extern crate zmq;

use agent::cluster;
use storage::storage;
    
pub fn start() {
    
    cluster::join();
    storage::bootstrap();
    
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

