// This project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! This module checks libs store in this node.
//! In the cache directory.

use std::fs;
use std::io;

pub struct Cache {
    file_path: &'static str,
}

impl Cache {
    pub fn new(file_path: &'static str) -> Result<Cache, io::Error> {
        Cache::extists_or_create(file_path)?;
        Ok(Cache { file_path })
    }

    fn extists_or_create(file_path: &'static str) -> Result<(), io::Error> {
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.into_iter() {
                if let Ok(dir) = entry {
                    if dir.file_name().into_string().unwrap() == file_path.to_owned()
                        && dir.file_type().unwrap().is_dir()
                    {
                        return Ok(());
                    }
                }
            }
        }
        // Not exists.
        match fs::create_dir("./".to_string() + file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
