use std::env;

#[derive(Debug)]
struct Storage {
    dirname: Option<String>,
    path: Option<String>
}

impl Storage {

    fn new() -> Storage {
        let path = match env::current_exe() {
            Ok(exe_path) => exe_path.display().to_string(),
            Err(e) => format!("failed to get current exe path: {}", e).to_string(),
        };
        info!("{}", path);
        Storage {path: Some(path), dirname: None} 
    }
    
    fn bootstrap(&self) {
        info!("bootstrap done");        
    }

    fn path(&self) -> String  {
        //return format!("{:?}/{:?}", self.path.("."), self.dirname.or_else("data"));
        return format!("{:?}/{:?}", self.path, self.dirname);
    }
    
    fn exists(&self) -> bool {
        return false;
        
    }
}

pub fn bootstrap() -> String {
    // determine if local repo/cache exists

    let storage = Storage::new();
    storage.bootstrap();
    info!("storage path is {:?}, exists? {}", storage.path(), storage.exists());
    
    return "hello".to_string();
}
