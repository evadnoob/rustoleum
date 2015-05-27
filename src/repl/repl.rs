extern crate readline;

use std::io;
use std::io::prelude::*;
use job::jobs;
use agent::client;


pub fn start() {
    //let mut stdout = io::stdout(); 
    loop {

        //write!(&mut stdout, "builr> ");
        //stdout.flush();
        
        //let input = &mut String::new();
        //match io::stdin().read_line(input) {
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
                }
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
        
    }
}

