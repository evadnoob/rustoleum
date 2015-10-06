#![allow(unused_must_use)]

extern crate nanomsg;
extern crate time;
use nanomsg::{Socket, Protocol, Error};
use std::thread::sleep_ms;
use std::{str, process, thread};
use std::collections::HashMap;
use nix::sys::signal;

pub fn participate(host: &str, peers: Vec<String>) {

    // TODO: prevent calling particpate twice in the same process.    
    let mut endpoints: HashMap<String, nanomsg::endpoint::Endpoint> = HashMap::new();
    
    println!("peers: {:?}", peers);
    let mut socket = match Socket::new(Protocol::Bus) {
        Ok(socket) => socket,
        Err(err) => panic!("{}", err)
    };

    setup_signal_handler();
    
    socket.bind(host).unwrap();

    //
    // connect to peers
    //
    for peer in peers {
        println!("connecting to peer {}", peer);
         match socket.connect(&peer) {
            Ok(ep) => {
                endpoints.insert(peer.clone(), ep);
                println!("connected to {}", peer);
            },
            Err(err) => panic!("Failed to connect socket: {}", err)
        };
    }

    info!("endpoints: {:?}", endpoints.len());
    sleep_ms(10);

    match socket.nb_write(format!(" {} join ", host).as_bytes()) {
        Ok(_) => println!("sent ok."),
        Err(err) => assert_eq!(err, Error::TryAgain)
    }

    let start = time::now();
    info!("{:?}", start.to_timespec());
    //let (tx, rx) = mpsc::channel();
    
    thread::spawn (move || {
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
                    
                },
                Err(Error::TryAgain) => {
                    // println!("Nothing to be read for the moment ...");
                    // here we can use the CPU for something else and try again later
                },
                Err(err) => panic!("Problem while reading: {}", err)
                    
            };

            // let now = time::now().to_timespec();
            // if (now.sec - start.to_timespec().sec > 5) {
            //     break;
            // }
            
        }
        //info!("cluster msg loop finished.");
        //endpoint.shutdown();
    }).join();

}


pub fn ping(count: i32) {
    info!("ping {}", count);
}

extern fn handle_sigint(_: i32) {
    Socket::terminate();
    sleep_ms(100);
    process::exit(0);
}

// fn setup_signal_handler(f: &Fn(i32)) {
//     println!("setting up signal handler");
//     let sig_action = signal::SigAction::new(f /*handle_sigint*/,
//                                             signal::SockFlag::empty(),
//                                             signal::SigSet::empty());
//     unsafe {
//         signal::sigaction(signal::SIGINT, &sig_action);
//     }
// }


fn setup_signal_handler() {
    println!("setting up signal handler");
    let sig_action = signal::SigAction::new(handle_sigint,
                                            signal::SockFlag::empty(),
                                            signal::SigSet::empty());
    unsafe {
        signal::sigaction(signal::SIGINT, &sig_action);
    }
}

