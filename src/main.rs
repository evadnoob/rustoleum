extern crate postgres;
extern crate hyper;
extern crate docopt;
extern crate zmq;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libc;

mod agent;
mod x;
mod repl;
mod job;

use x::logging;
use docopt::Docopt;

static USAGE: &'static str = "
Publish cli.

Usage:
  buildr [options] agent [<args>...]
  buildr [options] repl

Options:
  --env=ARG     Set the environment where ENV is one of dev, qa, or prod
  -h --help     Show this screen.
  --version     Show version.

Some common buildr commands are:
    event <name>     Various commands related to events
    help 
See 'buildr help <command>' for more information on a specific command.
";

fn main() {
    
    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    info!("args: {:?}", args); 
    info!("arg vector: {:?}", args.get_vec("<args>"));

    if args.get_bool("agent")  { 
        agent::agent::do_server();
    }
    else if args.get_bool("repl") {
        repl::repl::start();
    }
}
