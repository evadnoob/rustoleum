#![feature(custom_derive, plugin, macro_rules)]
//#![plugin(serde_macros)]
extern crate rustc_serialize;

extern crate hyper;
extern crate docopt;
extern crate zmq;
extern crate nanomsg;
extern crate git2;
extern crate serde;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libc;
extern crate nix;
extern crate glob;

pub mod cluster;
pub mod agent;
pub mod jobs;
pub mod storage;
mod logging;
mod repl;
mod jq;
mod help;

//use x::logging;
use docopt::Docopt;


static USAGE: &'static str = "
builder cli.

Usage:
  bldr agent [--port=<portnumber>] [<peers>...]
  bldr repl
  bldr storage init
  bldr storage show
  bldr storage show
  bldr add job <json>
  bldr -h | --help
  
Options: 
  -v --verbose 
  -h --help

Some common bldr commands are:
    agent (describe | help)     Various commands related to agent
    storage (init | | show | help)
    help 
See 'bldr help <command>' for more information on a specific command.
";

fn main() {
    
    let args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());
    //let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    
    println!("args: {:?}", args); 
    println!("arg vector: {:?}", args.get_vec("<args>"));
    
    match logging::logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    trace!("args: {:?}", args); 
    trace!("arg vector: {:?}", args.get_vec("<args>"));
    
    if args.get_bool("agent")  {
        let peers = args.get_vec("<peers>");
        
        // args[0] program name
        // args[1] self 
        for x in 1..peers.len() {
            println!("{} {}", x, peers[x]); // x: i32
        }
    
        //peers[0].as_ref(), args.iter().skip(1).collect::<Vec<_>>());
        
        info!("local: {}, peers: {:?}", args.get_str("<portnumber>"), args.get_vec("<peers>"));
        agent::start(peers);
    }
    else if args.get_bool("repl") {
        repl::start();
    }
    else if args.get_bool("storage") {
        let storage = storage::bootstrap();

        //let cmd = Command::new().args().exec();
        if args.get_bool("show") {
            storage.list();
        }
        
    }
    else if args.get_bool("add") && args.get_bool("job") {
        let storage = storage::bootstrap();
        // got an add command, what kind of add is it?
        // options discover add from json?
        // or assume a subsequent command to clarify type
        // right now assume sub-command for specific type of 'add'.
        let json = args.get_str("<json>");
        info!("json: {}", json);
        match jobs::from_raw_json(&storage, json) {
            Ok(_) => println!("got raw json ok."),
            Err(e) => panic!("uh oh {}", e)
        };
    }
}
