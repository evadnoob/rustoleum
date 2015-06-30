extern crate hyper;
extern crate docopt;
extern crate zmq;
extern crate nanomsg;
extern crate rustc_serialize;
extern crate git2;

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
  bldr agent [<args>...]
  bldr repl
  bldr storage init
  bldr storage show
  bldr storage show
  bldr job add <json>
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
    
    println!("args: {:?}", args); 
    println!("arg vector: {:?}", args.get_vec("<args>"));


    
    match logging::logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    trace!("args: {:?}", args); 
    trace!("arg vector: {:?}", args.get_vec("<args>"));
    
    if args.get_bool("agent")  { 
        agent::start();
    }
    else if args.get_bool("repl") {
        repl::start();
    }
    else if args.get_bool("storage") {
        let storage = storage::bootstrap();
        storage.list();
    }
}
