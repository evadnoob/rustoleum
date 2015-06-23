use std::env;
use rustc_serialize::json;
use std::fs;
use std::path::PathBuf;
use git2::Repository;
use std::fs::File;
use std::io::prelude::*;
use glob::glob;

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
enum RepositoryType {
    Github,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct RepositoryDescriptor {
    name: String,
    url: Option<String>,
    repo_type: RepositoryType
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
pub struct Job {
    name: Option<String>,
    description: Option<String>,
    repository: Option<RepositoryDescriptor>
}


#[derive(Debug)]
pub struct Storage {
    git_local_repo_path: PathBuf,
    path: PathBuf,
    
}

impl Storage {

    pub fn new() -> Storage {
        let path = env::current_exe().ok().unwrap().parent().unwrap().to_path_buf();

        info!("{}", path.display());

        let mut git_local_repo_path = path.clone();
        git_local_repo_path.push(".data/");

        Storage {path: path, git_local_repo_path: git_local_repo_path}  
    }
    
    pub fn bootstrap(&self) {
        info!("does file exist ? {:?}", self.path);
        //let mut git_local_repo_path = self.path.clone();
        //git_local_repo_path.push(".data/");
        if !self.exists() {
            info!("creating storage directory {:?}", self.git_local_repo_path);
            let x = fs::create_dir_all(self.git_local_repo_path.as_path());
            info!("create file result {:?}", x);
            
            match Repository::init(self.git_local_repo_path.as_path()) {
                Ok(_) => info!("repo initialized successfully" ),
                Err(e) => panic!("failed to init: {}", e),
            };
        }
        else {
            info!("skipped repo initialization, directory already exists.");
        }

        let repo = match Repository::open(self.git_local_repo_path.as_path()) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };
        info!("opened repo {:?}", repo.path());
        
        let statuses = repo.statuses(None);
        info!("statuses {}", statuses.unwrap().len());
        info!("bootstrap done");        
    }

    pub fn path(&self) -> PathBuf  {
        let mut path = self.path.clone();
        path.push(".data");
        return path;
    }

    pub fn exists(&self) -> bool {
        return fs::metadata(self.path()).is_ok();
    }
    
    pub fn save(&self, job: Job) {
        let job_as_json = json::as_pretty_json(&job);
        let mut job_as_json_path = PathBuf::from(self.git_local_repo_path.clone());
        job_as_json_path.push("job1.json");
        info!("adding...{}", job_as_json);
        info!("job_as_json_path {}", job_as_json_path.display());

        match File::create(job_as_json_path.as_path()) {
            Ok(ref mut file) => {
                info!("created file, ready for writing");
                write!(file, "{}", job_as_json)
            },
            Err(e) => panic!("unable to create file for writing {}", e)
        };
    }

    pub fn list(&self) {
        let path = self.git_local_repo_path.to_string_lossy();
        info!("path {}", path);
        // path.push_str("*");
        // info!("path {}", path);
        for entry in glob(&format!("{}/*", path)).unwrap() {
            match entry {
                Ok(p) => info!("{:?}", p),

                // if the path matched but was unreadable,
                // thereby preventing its contents from matching
                Err(e) => println!("{:?}", e),
            }
        }
    }
}


pub fn bootstrap() -> Storage {
    let storage = Storage::new();

    storage.bootstrap();
    storage.save(Job{
        name: Some("test".to_string()),
        description: Some("dsc".to_string()),
        repository: None });
    return storage;
}
