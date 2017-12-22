use super::error::Result;

use std::net::SocketAddr;

struct Client {}

impl Client {
    fn new<A: AsRef<SocketAddr>>(addr: A) -> Result<Client> {
        Ok(Client {})
    }
}
