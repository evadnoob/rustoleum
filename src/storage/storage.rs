use std::env;
use rustc_serialize::json;
use std::fs;
use std::path::PathBuf;


    
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
    path: PathBuf,
    backend: Backend
}

impl Storage {

    pub fn new() -> Storage {
        // let path = match env::current_exe() {
        //     Ok(exe_path) => Some(exe_path),
        //     Err(e) => None,
        // };
        // let p = env::current_dir().unwrap();
        let path = env::current_exe().ok().unwrap();

        info!("{:?}", path);
        info!("{}", path.display());

        Storage {path: path, dirname: None, backend: Backend::new()}
    }
    
    pub fn bootstrap(&self) {
        info!("bootstrap done");        
    }

    pub fn path(&self) -> PathBuf  {

        let mut path = self.path.clone();
        path.push(".data");
        info!("{:?}", path);
        
        return path;
    }

    pub fn exists(&self) -> bool {
        info!("does file exist ? {:?}", self.path());
        return fs::metadata(self.path()).is_ok();
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

    if !storage.exists() {
        info!("creating storage directory {:?}", storage.path());
        let x = fs::create_dir_all(storage.path().as_path());
        info!("create file result {:?}", x);
        
    }

    
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
