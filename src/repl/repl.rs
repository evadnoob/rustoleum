
use std::io;
use std::io::prelude::*;
use std::process;
use job::jobs;
use agent::client;

pub fn start() {
    let mut stdout = io::stdout();
    loop {

        write!(&mut stdout, "builr> ");
        stdout.flush();
        
        let input = &mut String::new();
        match io::stdin().read_line(input) {
            Ok(x)=> {
                let input = input.replace("\n", "");
                if input.len() > 0 {
                    println!("{:?} {:?}", input, x);

                    if "jobs" == input {
                        jobs::list();
                    }
                    else if "ping" == input {
                        client::ping(4);
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

