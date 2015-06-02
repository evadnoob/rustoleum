

use std::env;
use std::collections::BTreeMap;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
enum RepositoryType {
    Github,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct Repository {
    name: String,
    url: Option<String>,
    repo_type: RepositoryType
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
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
    storage.save(Job{
        name: Some("test".to_string()),
        description: Some("dsc".to_string()),
        repository: Some(Repository{name: "test".to_string(),
                        url: Some("http://github.com/evadnoob".to_string()),
                        repo_type: RepositoryType::Github})});
}


#[derive(Debug)]
pub struct Backend;

impl Backend {
    pub fn new() -> Backend {
        Backend
    }

    pub fn add(&self, job: Job) {
        
        info!("adding...{}", json::as_pretty_json(&job));
    }
}
