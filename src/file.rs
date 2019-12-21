use std::fs::File as FsFile;
use std::io;

use crate::ReadWrite;

pub struct File {
    path: String,
    once: bool,
    count: i32,
}

impl ReadWrite for FsFile {}

impl File {
    pub fn new(path: String, once: bool) -> File {
        File {
            path,
            once,
            count: 0,
        }
    }
}

impl Iterator for File {
    type Item = Result<Box<dyn ReadWrite>, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.once {
            if self.count > 0 {
                return None;
            }

            self.count += 1
        }

        match FsFile::open(&self.path) {
            Ok(file) => Some(Ok(Box::new(file))),
            Err(err) => {
                if let io::ErrorKind::NotFound = err.kind() {
                    match FsFile::create(&self.path) {
                        Ok(f) => return Some(Ok(Box::new(f))),
                        Err(err) => return Some(Err(err))
                    }
                }

                return Some(Err(err));
            }
        }
    }
}