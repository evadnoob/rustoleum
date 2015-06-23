#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use std::thread::sleep_ms;

fn participate(host: &str, peers: Vec<&String>) {

    println!("{:?}", peers);
    let mut socket = match Socket::new(Protocol::Bus) {
        Ok(socket) => socket,
        Err(err) => panic!("{}", err)
    };

    match socket.bind(host) {
        Ok(endpoint) => {
            println!("socket bind successfully.");
        },
        Err(err) => panic!("failed to bind socket {}", err)
    }

    //
    // connect to peers
    //
    for peer in peers {
        println!("connecting to peer {}", peer);
        let endpoint = match socket.connect(peer) {
            Ok(ep) => {
                println!("connected to {}", peer);
            },
            Err(err) => panic!("Failed to connect socket: {}", err)
        };
    }

    
    

    
    sleep_ms(10);
    
}

fn main() {

    let args: Vec<_> = std::env::args().collect();

    // args[0] program name
    // args[1] self 
    for x in 2..args.len() {
        println!("{} {}", x, args[x]); // x: i32
    }
    
    participate(args[1].as_ref(), args.iter().skip(2).collect::<Vec<_>>());
}
