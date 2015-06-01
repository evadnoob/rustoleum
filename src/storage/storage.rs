
extern crate serialize;

use std::env;
use std::collections::BTreeMap;
// use self::serialize::json::ToJson;
// use self::serialize::json::Json;
// use self::serialize::json;

#[derive(Debug)]
enum RepositoryType {
    Github,
}

#[derive(Decodable, Encodable)]
pub struct Repository {
    name: String,
    url: Option<String>,
    repo_type: RepositoryType
}

#[derive(Debug, Decodable, Encodable)]
pub struct Job {
    name: Option<String>,
    description: Option<String>,
    repository: Option<Repository>
}


#[derive(Debug)]
pub struct Storage {
    dirname: Option<String>,
    path: Option<String>,
    backend: Backend
}


// impl ToJson for Job {
//     fn to_json(&self) -> Json {
//         let mut d = BTreeMap::new();
//         d.insert("name".to_string(), self.name.to_json());
//         d.insert("description".to_string(), self.description.to_json());
//         d.insert("repository".to_string(), self.repository.unwrap().to_json());
//         Json::Object(d)
//     }
// }


impl Storage {

    pub fn new() -> Storage {
        let path = match env::current_exe() {
            Ok(exe_path) => exe_path.display().to_string(),
            Err(e) => format!("failed to get current exe path: {}", e).to_string(),
        };
        info!("{}", path);
        Storage {path: Some(path), dirname: None, backend: Backend::new()} 
    }
    
    pub fn bootstrap(&self) {
        info!("bootstrap done");        
    }

    pub fn path(&self) -> String  {
        //return format!("{:?}/{:?}", self.path.("."), self.dirname.or_else("data"));
        return format!("{:?}/{:?}", self.path.clone().unwrap_or("".to_string()), self.dirname.clone().unwrap_or(".data".to_string()));
    }
    
    pub fn exists(&self) -> bool {
        return false;
    }
    
    pub fn save(&self, job: Job) {
        self.backend.add(job)
    }

}


pub fn bootstrap() {
    let storage = Storage::new();
    storage.bootstrap();
    info!("storage path is {:?}, exists? {}", storage.path(), storage.exists());
}


#[derive(Debug)]
pub struct Backend;

impl Backend {
    pub fn new() -> Backend {
        Backend
    }

    pub fn add(&self, job: Job) {
        info!("adding...{}", job.to_json());
    }
}
