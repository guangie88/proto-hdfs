use super::error::Result;

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::time::SystemTime;

const RPC_VERSION: u8 = 0x09;
const SERVICE_CLASS: u8 = 0x0;
const AUTH_PROTOCOL: u8 = 0x0;
const PROTOCOL_CLASS: &str = "org.apache.hadoop.hdfs.protocol.ClientProtocol";
const PROTOCOL_CLASS_VERSION: u8 = 1;
const HANDSHAKE_CALL_ID: i8 = -3;
const STANDBY_EXCEPTION_CLASS: &str = "org.apache.hadoop.ipc.StandbyException";

pub struct Client {
    stream: TcpStream,
}

#[derive(Clone, Debug)]
pub struct Metadata;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Permissions;

impl Client {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Client> {
        let mut stream = TcpStream::connect(addr)?;

        let rpc_header = [
            b'h',
            b'r',
            b'p',
            b'c',
            RPC_VERSION,
            SERVICE_CLASS,
            AUTH_PROTOCOL,
        ];

        stream.write_all(&rpc_header)?;

        Ok(Client { stream: stream })
    }

    fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
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
    fn test_client() {
        let client = Client::new("localhost:9000");
        assert!(client.is_ok());
    }
}
