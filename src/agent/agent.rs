extern crate zmq;

use agent::cluster;
use std::thread;


pub fn do_server() {
    
    cluster::join();
    
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

// pub fn do_client() {
//     cluster::join();

//     let mut context = zmq::Context::new();
//     let mut requester = context.socket(zmq::REQ).unwrap();

//     assert!(requester.connect("tcp://localhost:5555").is_ok());

//     let mut msg = zmq::Message::new().unwrap();
//     let mut x = 0;
    
//     loop {
        
//         info!("Sending Hello {}", x);
//         requester.send(b"Zoe Rocks!!", 0).unwrap();

//         requester.recv(&mut msg, 0).unwrap();
//         info!("Received {}: {}", msg.as_str().unwrap(), x);
//         x += 1;
//         thread::sleep_ms(1000);
//     }
    
// }

