#![feature(custom_derive)]
    
//use serde::{json, de, ser};
use storage::{Job, Storage};
use rustc_serialize::json;

pub fn list() {
    info!("doing list");
}

pub fn from_raw_json(storage: &Storage, raw_json: &str) -> Result<(), &'static str> {
    info!("from_raw_json: {}", raw_json);
     // macro_rules! fs_try {
     //    ($e:expr) => (match $e { Ok(e) => e, Err(..) => return Ok(None) })
     // }
    
    //let mut f = BufReader::new(fs_try!(File::open(dep_info)));
    let decoded: Job = json::decode(&raw_json).unwrap();
    
    info!("decoded {:?}", decoded);

    storage.save(decoded);
    Ok(())
}
