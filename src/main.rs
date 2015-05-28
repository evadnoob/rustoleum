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
  --env=ARG       Set the environment where ENV is one of dev, qa, or prod

Some common buildr commands are:
    agent init     Various commands related to agent
    help 
See 'buildr help <command>' for more information on a specific command.
";

fn main() {
    
    let args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    match logging::init() {
        Err(e) => println!("Unable to initialize logging system: {}", e),
        _ => {}
    }
    
    trace!("args: {:?}", args); 
    trace!("arg vector: {:?}", args.get_vec("<args>"));
    
    if args.get_bool("agent")  { 
        agent::agent::start();
    }
    else if args.get_bool("repl") {
        repl::repl::start();
    }

    
}
