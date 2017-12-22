use super::error::Result;

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

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
