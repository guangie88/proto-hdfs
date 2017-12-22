use std;

pub struct HdfsError;

pub type Result<T> = std::result::Result<T, HdfsError>;
