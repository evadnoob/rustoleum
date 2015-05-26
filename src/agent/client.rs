extern crate zmq;

use agent::cluster;
use std::thread;

pub fn ping(count: i32) {
    cluster::join();

    let mut context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    //let mut x = 0;
    
    for x in 0..count {
        
        info!("sending ping {}", x);
        requester.send(b"Zoe Rocks!!", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        info!("Received {}: {}", msg.as_str().unwrap(), x);
        //x += 1;
        thread::sleep_ms(1000);
    }
    
}



