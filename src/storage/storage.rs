use std::env;

#[derive(Debug)]
struct Storage {
    dirname: Option<String>,
    path: Option<String>,
    backend: backend::git::Backend
}

impl Storage {

    fn new() -> Storage {
        let path = match env::current_exe() {
            Ok(exe_path) => exe_path.display().to_string(),
            Err(e) => format!("failed to get current exe path: {}", e).to_string(),
        };
        info!("{}", path);
        Storage {path: Some(path), dirname: None, backend: backend::git::Backend::new()} 
    }
    
    fn bootstrap(&self) {
        info!("bootstrap done");        
    }

    fn path(&self) -> String  {
        //return format!("{:?}/{:?}", self.path.("."), self.dirname.or_else("data"));
        return format!("{:?}/{:?}", self.path.clone().unwrap_or("".to_string()), self.dirname.clone().unwrap_or(".data".to_string()));
    }
    
    fn exists(&self) -> bool {
        return false;
    }

    // fn to_string(&self) -> str {
    //     return self.path.unwrap_or("");
    // }
}


pub fn bootstrap() {
    // determine if local repo/cache exists

    let storage = Storage::new();
    storage.bootstrap();
    info!("storage path is {:?}, exists? {}", storage.path(), storage.exists());
}


mod backend {
    pub mod git {

        #[derive(Debug)]
        pub struct Backend;

        impl Backend {
            pub fn new() -> Backend {
                Backend
            }
        }
    }
}
