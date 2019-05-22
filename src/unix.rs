#![cfg(not(windows))]

use super::Absolutize;

use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};

use path_dedot::{CWD, ParseDot};

impl Absolutize for Path {
    fn absolutize(&self) -> io::Result<PathBuf> {
        if self.is_absolute() {
            self.parse_dot()
        } else {
            let path = Path::join(&CWD, self);

            path.parse_dot()
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            if !path.starts_with(&virtual_root) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                if !path.starts_with(&virtual_root) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                return Ok(virtual_root);
            }
        }
    }
}