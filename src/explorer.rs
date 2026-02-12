use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Explorer {
    pub path: PathBuf,
}

impl Explorer {
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path.to_path_buf();
    }
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn list_dict(&self) -> Vec<String> {
        let mut entries = Vec::new();

        if let Ok(dir) = fs::read_dir(&self.path) {
            for entry in dir {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push(name.to_string());
                    }
                }
            }
        }

        entries
    }
}
