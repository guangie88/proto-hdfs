use error::Result;
use hadoop_hdfs::GetFileInfoRequestProto;
use namenode::NamenodeConnection;

use std::net::ToSocketAddrs;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Metadata;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Permissions;

pub struct Client {
    namenode: NamenodeConnection,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Client> {
        Ok(Client {
            namenode: NamenodeConnection::new(addr)?,
        })
    }

    fn metadata<P: Into<String>>(&self, path: P) -> Result<Metadata> {
        let mut req = GetFileInfoRequestProto::new();
        req.set_src(path.into());
        let resp = self.namenode.execute("getFileInfo", &req)?;

        unimplemented!()
    }
}

impl Metadata {
    pub fn file_type(&self) -> FileType {
        unimplemented!()
    }

    pub fn is_dir(&self) -> bool {
        self.file_type().is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.file_type().is_file()
    }

    pub fn len(&self) -> u64 {
        unimplemented!()
    }

    pub fn permissions(&self) -> Permissions {
        unimplemented!()
    }

    pub fn modified(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    pub fn accessed(&self) -> Result<SystemTime> {
        unimplemented!()
    }

    pub fn created(&self) -> Result<SystemTime> {
        unimplemented!()
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        unimplemented!()
    }

    pub fn is_file(&self) -> bool {
        unimplemented!()
    }

    pub fn is_symlink(&self) -> bool {
        unimplemented!()
    }
}

impl Permissions {
    pub fn readonly(&self) -> bool {
        unimplemented!()
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("localhost:9000");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_metadata() {
        let client = Client::new("localhost:9000");
        assert!(client.is_ok());

        let client = client.unwrap();
        let metadata = client.metadata("/data/");

        assert!(metadata.is_ok());
    }
}
