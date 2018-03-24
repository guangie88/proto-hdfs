use std;

#[derive(Debug)]
pub enum HdfsError {
    IO(std::io::Error),
}

impl From<std::io::Error> for HdfsError {
    fn from(e: std::io::Error) -> Self {
        HdfsError::IO(e)
    }
}

pub type Result<T> = std::result::Result<T, HdfsError>;
