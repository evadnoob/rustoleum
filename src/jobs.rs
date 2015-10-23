use storage::{Job, Storage};
use rustc_serialize::json;


pub fn from_raw_json(storage: &Storage, raw_json: &str) -> Result<Job, &'static str> {
    info!("from_raw_json: {}", raw_json);
    
    let decoded: Job = json::decode(&raw_json).unwrap();
    
    info!("decoded {:?}", decoded);

    Ok(decoded)
}

pub fn list(storage: &Storage) {
    storage.list();
}
