extern crate readline;

use std::process;
use job::jobs;
use agent::client;
use storage::storage;

pub fn start() {
    loop {
        match readline::readline("> ") {
            Ok(input) => {
                let input = input.replace("\n", "");
                if input.len() > 0 {
                    readline::add_history(input.as_ref());
                    println!("{:?}", input);

                    if "jobs" == input {
                        jobs::list();
                    }
                    else if "ping" == input {
                        client::ping(4);
                    }
                    else if "storage" == input {
                        let storage = storage::Storage::new();
                        storage.bootstrap();
                        
                    }
                    else if "exit" == input || "quit" == input {
                        process::exit(0);
                    }
                }
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
        
    }
}

