extern crate readline;

use std::process;
use job::jobs;
use agent::client;
use storage::storage;
use help::help;

pub fn start() {
    loop {
        match readline::readline(">>> ") {
            Ok(input) => {
                let input = input.replace("\n", "");
                if input.len() > 0 {
                    readline::add_history(input.as_ref());
                    println!("{:?}", input);
                    if "help" == input {
                        help::help();
                    }
                    else if "jobs" == input {
                        jobs::list();
                    }
                    else if "ping" == input {
                        client::ping(4);
                    }
                    else if "storage" == input {
                        let storage = storage::bootstrap();
                        storage.list();
                        
                    }
                    else if "exit" == input || "quit" == input {
                        process::exit(0);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                //panic!("{}", e);
            }
        }
        
    }
}

