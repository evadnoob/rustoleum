#![feature(custom_derive)]
    
use serde::{json, de, ser};
use storage::Job;

pub fn list() {
    info!("doing list");
}

pub fn from_raw_json(raw_json: &str) -> Result<(), &'static str> {
    info!("from_raw_json: {}", raw_json);
    let job: Job = json::from_str(raw_json).unwrap();
    Ok(())
}
