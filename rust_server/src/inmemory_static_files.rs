use log::{debug, error};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

pub struct InMemoryStaticFiles {
    base_dir: String,
    storage: HashMap<String, String>,
}

impl InMemoryStaticFiles {
    pub fn new(base_dir: &str) -> InMemoryStaticFiles {
        let mut storage: HashMap<String, String> = HashMap::new();
        for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let path_extension = path.extension().and_then(|s| s.to_str());
            let is_static = (path_extension == Some("js")) || (path_extension == Some("html"));

            if path.is_file() && is_static {
                let file_path = path.to_string_lossy().to_string();
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        storage.insert(file_path.clone(), contents.clone());
                    }
                    Err(e) => {
                        error!("Error reading file {:?}: {}", path, e);
                    }
                }
            }
        }
        return InMemoryStaticFiles {
            base_dir: base_dir.to_string(),
            storage,
        };
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let full_key = format!("{}/{}", self.base_dir, key);

        debug!("Static from memory: {}", full_key);

        let result = self.storage.get(&full_key).map(|x| x.clone());

        return result;
    }
}
