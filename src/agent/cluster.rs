extern crate nix;

//todo:
// ** determine if running alone, or find a leader via discovery
// ** if leader cannot be discovered, become leader.
// ** set leader and leader status via TLS, so that other threads can easily access cluster.


pub fn join() {
    let mut name = [0u8; 256];
    let mut buf = &mut name[..];
    
//TODO: this doesn't work, not sure why....    
    let hostname = match nix::unistd::gethostname(buf) {
        Ok(x) => {
            println!("x: {:?}", x);
            return x;
        },
        _ => println!("uh oh")
    };

    //info!("name {:?}", name); 
    info!("hostname {:?}", hostname);
    //info!("hostname {:?}, buf: {}", name, String::from_utf8(&buf).unwrap());
    //info!("{:?}", nix::unistd::gethostname(&vec));
    info!("joining cluster");
    info!("no leader discovered, becoming leader");
    broadcast("<hostname:port> joined...".to_string());
    
}

pub fn broadcast(message: String) {
    info!("broadcasting: {}", message);
}
