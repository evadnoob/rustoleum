use logging::logging;
use docopt::Docopt;




    static USAGE: &'static str = "
Publish cli.

Usage:
  buildr [options] agent [<args>...]
  buildr [options] repl
  buildr [options] storage (init | show)

Options: -v, --verbose 
         -h, --help

Some common buildr commands are:
    agent (describe | help)     Various commands related to agent
    storage (init | help)
    help 
See 'buildr help <command>' for more information on a specific command.
";


//
pub fn help() {
    println!("help....");
   
    let args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).version(Some("0.0.1".to_string())).parse())
        .unwrap_or_else(|e| e.exit());

    
    trace!("args: {:?}", args); 
    trace!("arg vector: {:?}", args.get_vec("<args>"));

    
}
