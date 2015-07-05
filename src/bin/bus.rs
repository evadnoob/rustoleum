#![allow(unused_must_use)]

extern crate nanomsg;
use nanomsg::{Socket, Protocol, Error};
use std::thread::sleep_ms;
use std::str;
fn participate(host: &str, peers: Vec<&String>) {

    // println!("{:?}", peers);
    // let mut socket = match Socket::new(Protocol::Bus) {
    //     Ok(socket) => socket,
    //     Err(err) => panic!("{}", err)
    // };

    // let mut endpoint = socket.bind(host).unwrap();
    
    // // match socket.bind(host) {
    // //     Ok(endpoint) => {
    // //         println!("socket bind successfully.");
    // //     },
    // //     Err(err) => panic!("failed to bind socket {}", err)
    // // }

    // //
    // // connect to peers
    // //
    // for peer in peers {
    //     print!("connecting to peer {}", peer);
    //     let endpoint = match socket.connect(peer) {
    //         Ok(ep) => {
    //             println!("connected to {}", peer);
    //         },
    //         Err(err) => panic!("Failed to connect socket: {}", err)
    //     };
    // }
    
    sleep_ms(10);


    match socket.nb_write(format!(" {} join ", host).as_bytes()) {
        Ok(_) => println!("sent ok."),
        Err(err) => assert_eq!(err, Error::TryAgain)
    }
    
    
    loop {
        let mut buffer = Vec::new();
        match socket.nb_read_to_end(&mut buffer) {
            Ok(_) => { 
                println!("Read message {} !", buffer.len()); 

                let s = match str::from_utf8(&buffer) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("{:?}", s);

                
                // here we can process the message stored in `msg`
            },
            Err(Error::TryAgain) => {
                //println!("Nothing to be read for the moment ...");
                // here we can use the CPU for something else and try again later
            },
            Err(err) => panic!("Problem while reading: {}", err)
        };
    }
    
    // TODO: hook to SIGINT handler  
    endpoint.shutdown();
}

// bus 'tcp://0.0.0.0:4445' 'tcp://0.0.0.0:4444' 'tcp://0.0.0.0:4446'
// 2 tcp://0.0.0.0:4444
// 3 tcp://0.0.0.0:4446
// ["tcp://0.0.0.0:4444", "tcp://0.0.0.0:4446"]
// connecting to peer tcp://0.0.0.0:4444connected to tcp://0.0.0.0:4444
// connecting to peer tcp://0.0.0.0:4446connected to tcp://0.0.0.0:4446
// sent ok.
fn main() {

    let args: Vec<_> = std::env::args().collect();

    // args[0] program name
    // args[1] self 
    for x in 2..args.len() {
        println!("{} {}", x, args[x]); // x: i32
    }
    
    participate(args[1].as_ref(), args.iter().skip(2).collect::<Vec<_>>());

}
