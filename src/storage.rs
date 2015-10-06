
extern crate git2;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use git2::{Repository, Error, StatusOptions};
use std::fs::File;
use std::io::prelude::*;
use glob::glob;
//use serde::{json, de, ser};
use rustc_serialize::json;

const REPO_DIR: &'static str = "bldr-repo-data";

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[derive_deserialize]
enum RepositoryType {
    Github,
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[derive_deserialize]
pub struct RepositoryDescriptor {
    name: Option<String>,
    url: Option<String>,
    repo_type: RepositoryType
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[derive_deserialize]
pub struct Job {
    name: String,
    description: Option<String>,
    repository: RepositoryDescriptor
}

#[derive(RustcEncodable, RustcDecodable, Debug, Clone)]
//#[derive(Debug)]
pub struct Storage {
    git_local_repo_path: PathBuf,
    path: PathBuf,
}

impl Storage {

    pub fn new() -> Storage {

        let cwd = env::current_dir().unwrap();
        info!("The current directory is {}", cwd.display());

        let mut git_local_repo_path = cwd.clone(); //path.clone();
        git_local_repo_path.push(REPO_DIR);

        Storage {path: cwd, git_local_repo_path: git_local_repo_path}  
    }
    
    pub fn bootstrap(&self) {
        info!("does file exist ? {:?}", self.path);
        if !self.exists() {
            info!("creating storage directory {:?}", self.git_local_repo_path);
            let x = fs::create_dir_all(self.git_local_repo_path.as_path());
            info!("create file result {:?}", x);
            
            match Repository::init(self.git_local_repo_path.as_path()) {
                Ok(_) => {
                    info!("repo initialized successfully" );
                    info!("Created empty initial commit"); 
                    
                },
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
        match self.create_initial_commit(&repo) {
            Ok(_) => println!("initial commit created ok."),
            Err(e) => panic!("failed to create initial commit {}", e)
        };

        
        let statuses = repo.statuses(None);
        info!("statuses {}", statuses.unwrap().len());
        info!("bootstrap done");        
    }

    fn path(&self) -> PathBuf  {
        let mut path = self.path.clone();
        path.push(REPO_DIR);
        return path;
    }
    
    fn show_statuses(&self, repo: &Repository) {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false).include_untracked(true);
        //let statuses = try!(repo.statuses(None));
        match repo.statuses(Some(&mut opts)) {
            Ok(statuses) => {
                for entry in statuses.iter().filter(|e| e.status() != git2::STATUS_CURRENT) {
                    let path = entry.path();
                    info!("path: {}", path.unwrap_or(""));
                    
                    let status = match entry.status() {
                        s if s.contains(git2::STATUS_INDEX_NEW) => "new file: ",
                        s if s.contains(git2::STATUS_INDEX_MODIFIED) => "modified: ",
                        s if s.contains(git2::STATUS_INDEX_DELETED) => "deleted: ",
                        s if s.contains(git2::STATUS_INDEX_RENAMED) => "renamed: ",
                        s if s.contains(git2::STATUS_INDEX_TYPECHANGE) => "typechange:",
                        s if s.contains(git2::STATUS_WT_NEW) => "wt new: ",
                        _ => continue,
                    };
                    
                    info!("{} {}", path.unwrap_or(""), status);
                }

                info!("status length {}", statuses.len());
            },
            _ => panic!("failed to get statuses")
        }
    }

    fn stage(&self, repo: &Repository, path: &Path) {
        info!("stage");
        let mut index = match repo.index() {
            Ok(index) => index,
            Err(e) => panic!("failed to get index {}", e)
        };
        
        //index.add_path(path);
        let path_spec = vec!("*.json");

        let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
            let status = repo.status_file(path).unwrap();
            let ret = if status.contains(git2::STATUS_WT_MODIFIED) ||
                status.contains(git2::STATUS_WT_NEW) {
                    info!("add '{}'", path.display());
                    0
                } else {
                    1
                };

            ret
        };
        
        let cb = cb as &mut git2::IndexMatchedPath;
        
        match index.add_all(path_spec.iter(), git2::ADD_DEFAULT, Some(cb)) {
            Ok(_) => info!("add_all ok."),
            Err(e) => panic!("error performing index.add_all {}", e)
        };
        
        info!("added {:?}", path);
        self.show_statuses(&repo);

        match index.write() {
            Ok(_) => info!("index write ok."),
            Err(e) => panic!("Error: failed to write index {}", e)
        };
    }

    fn commit(&self, repo: &Repository) {
        info!("commit");
        let mut index = repo.index().unwrap();
        let id = index.write_tree().unwrap();
        
        let head = repo.refname_to_id("HEAD").unwrap();
        //info!("HEAD: {}", head);
        match repo.find_commit(head) {
            Ok(commit) => {
                info!("commit: {}", commit.id());
                // let parents = commit.parents().len();
                // info!("HEAD: {:?}, parents: {}", id, parents);

                let tree = repo.find_tree(id).unwrap();
                let sig = repo.signature().unwrap();
                repo.commit(Some("HEAD"), &sig, &sig, "automated commit", &tree, &[&commit]).unwrap();
            },
            Err(e) => panic!("error {}", e)
        }
    }


    /// Unlike regular "git init", this example shows how to create an initial empty
    /// commit in the repository. This is the helper function that does that.
    fn create_initial_commit(&self, repo: &Repository) -> Result<(), Error> {
        // First use the config to initialize a commit signature for the user.
        let sig = try!(repo.signature());

        // Now let's create an empty tree for this commit
        let tree_id = {
            let mut index = try!(repo.index());

            // Outside of this example, you could call index.add_path()
            // here to put actual files into the index. For our purposes, we'll
            // leave it empty for now.

            try!(index.write_tree())
        };

        let tree = try!(repo.find_tree(tree_id));

        // Ready to create the initial commit.
        //
        // Normally creating a commit would involve looking up the current HEAD
        // commit and making that be the parent of the initial commit, but here this
        // is the first commit so there will be no parent.
        try!(repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]));

        Ok(())
    }
    

    
    fn exists(&self) -> bool {
        return fs::metadata(self.path()).is_ok();
    }
    
    pub fn save(&self, job: Job) {
        info!("save");
        //let job_as_json = json::to_string_pretty(&job).unwrap();
        let job_as_json = json::as_pretty_json(&job);
        let mut job_as_json_path = PathBuf::from(self.git_local_repo_path.clone());
        job_as_json_path.push(format!("job_{}.json", job.name));
        info!("adding...{}", job_as_json);
        info!("job_as_json_path {}", job_as_json_path.display());

        match File::create(job_as_json_path.as_path()) {
            Ok(ref mut file) => {
                info!("created file, ready for writing");
                write!(file, "{}", job_as_json)
            },
            Err(e) => panic!("unable to create file for writing {}", e)
        };

        //let repo = self.repo();
        let repo = match Repository::open(self.git_local_repo_path.as_path()) {
            Ok(x) => x,
            Err(e) => panic!("failed to init: {}", e),
        };
        info!("opened repo {:?}", repo.path());

        // http://stackoverflow.com/questions/27672722/libgit2-commit-example
        // Get the index and write it to a tree */
        // git_repository_index(&index, repo);
        // git_index_write_tree(tree, index);
        // git_reference_name_to_id(parent_id, repo, "HEAD");
        // git_commit_lookup(&parent, repo, parent_id);
        

        self.show_statuses(&repo);
        
        self.stage(&repo, job_as_json_path.as_path());
        
        self.commit(&repo);
        
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
                Err(e) => println!("{:?}", e)
            }
        }
    }
}


pub fn bootstrap() -> Storage {
    let storage = Storage::new();

    storage.bootstrap();
    return storage;
}
