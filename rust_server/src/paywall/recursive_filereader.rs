use std::fs;

use log::error;
use walkdir::WalkDir;

pub struct PathAndFile<T> {
    pub file_path: String,
    pub file_content: T,
}

pub trait RecursiveFileReader<T> {
    fn get_paths_and_files(&self) -> Vec<PathAndFile<T>>;
}

pub struct RecursiveFileReaderString<'a> {
    base_dir: &'a str,
    file_extensions: Vec<&'a str>,
}

impl<'a> RecursiveFileReaderString<'a> {
    pub fn new(base_dir: &'a str, file_extensions: Vec<&'a str>) -> RecursiveFileReaderString<'a> {
        return RecursiveFileReaderString {
            base_dir,
            file_extensions,
        };
    }
}

impl<'a> RecursiveFileReader<String> for RecursiveFileReaderString<'a> {
    fn get_paths_and_files(&self) -> Vec<PathAndFile<String>> {
        let mut result: Vec<PathAndFile<String>> = Vec::new();

        for entry in WalkDir::new(self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|s| s.to_str().map(|x| self.file_extensions.contains(&x)))
                    .unwrap_or_else(|| false)
            //if file extension can be converted to str
            //(`Some(&str)`), check if extension is contained
            //in `file_extensions`. If extension cannot be
            //converted, the check fails immediately
            {
                match fs::read_to_string(path) {
                    Ok(file_content) => {
                        let file_path = path
                            .to_string_lossy()
                            .to_string()
                            .replace(self.base_dir, ""); //base dir is only necessary to read from
                                                         //disk
                        let path_and_file = PathAndFile {
                            file_path,
                            file_content,
                        };
                        result.push(path_and_file);
                    }
                    Err(e) => error!("Error reading file {:?}: {}", path, e),
                }
            }
        }

        return result;
    }
}
