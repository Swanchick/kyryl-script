use std::io;
use std::path::{absolute, Path, PathBuf};

const KS_FILE_FORMAT: &str = ".ks";


#[derive(Debug)]
pub struct KsPath {
    path: PathBuf
}

impl KsPath {
    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn new() -> KsPath {
        KsPath { 
            path: PathBuf::new() 
        }
    }

    pub fn from(path: &str) -> io::Result<KsPath> {
        let absolute_path = absolute(path)?;

        Ok(KsPath {
            path: absolute_path
        })
    }

    pub fn from_path(path: &Path) -> KsPath {
        KsPath {
            path: path.to_path_buf()
        }
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        let filename = self.get_filename();

        if let Some(filename) = filename {
            let mut filename = filename.to_string();

            if filename.ends_with(KS_FILE_FORMAT) {
                self.path.is_file()
            }  else {
                filename.push_str(KS_FILE_FORMAT);
                let mut parent_path = self.parent();
                
                parent_path.push(filename);

                parent_path.is_file()
            }
        } else {
            false
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn parent(&self) -> KsPath {
        let parent = self.path.parent().unwrap();

        KsPath::from_path(parent)
    }

    pub fn get_filename(&self) -> Option<&str> {
        let filename = self.path.file_name();

        if let Some(filename) = filename {
            filename.to_str()
        } else {
            None
        }
    }

    pub fn push(&mut self, path: String) {
        self.path.push(path);
    }

    pub fn join(&mut self, path: &str) {
        self.path.push(path);
    }
}