
pub fn join() {
    info!("joining cluster");
    //todo:
    // * determine if running alone, or find a leader via discovery
    // ** if leader cannot be discovered, become leader.
    // ** set leader and leader status via TLS, so that other threads can easily access cluster.


    info!("no leader discovered, becoming leader");
    broadcast("<hostname:port> joined...".to_string());
    
}

pub fn broadcast(message: String) {
    info!("broadcasting: {}", message);
}
