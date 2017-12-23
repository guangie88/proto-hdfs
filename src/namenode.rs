use super::error::Result;
use super::hadoop_common::{IpcConnectionContextProto, RpcKindProto,
                           RpcRequestHeaderProto,
                           RpcRequestHeaderProto_OperationProto,
                           UserInformationProto};
use super::hadoop_hdfs::{GetFileInfoRequestProto, GetFileInfoResponseProto};

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicIsize, Ordering};

const RPC_VERSION: u8 = 0x09;
const SERVICE_CLASS: u8 = 0x0;
const AUTH_PROTOCOL: u8 = 0x0;
const PROTOCOL_CLASS: &str = "org.apache.hadoop.hdfs.protocol.ClientProtocol";
const PROTOCOL_CLASS_VERSION: u8 = 1;
const HANDSHAKE_CALL_ID: i8 = -3;
const STANDBY_EXCEPTION_CLASS: &str = "org.apache.hadoop.ipc.StandbyException";

pub struct NamenodeConnection {
    stream: TcpStream,
    user: String,
    client_id: String,
    req_id: AtomicIsize,
}

fn new_rpc_request_header(id: i32, client_id: &str) -> RpcRequestHeaderProto {
    let mut header = RpcRequestHeaderProto::new();
    header.set_rpcKind(RpcKindProto::RPC_PROTOCOL_BUFFER);
    header.set_rpcOp(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET);
    header.set_callId(id);
    header.set_clientId(client_id.as_bytes().to_vec());
    header
}

fn new_connection_context(user: &str) -> IpcConnectionContextProto {
    let mut user_info = UserInformationProto::new();
    user_info.set_effectiveUser(user.to_owned());

    let mut context = IpcConnectionContextProto::new();
    context.set_userInfo(user_info);
    context.set_protocol(PROTOCOL_CLASS.to_owned());
    context
}

impl NamenodeConnection {
    fn write_request(
        &self,
        method: &str,
        req_id: isize,
        req: &GetFileInfoRequestProto,
    ) -> Result<()> {
        unimplemented!()
    }

    fn read_response(&self, method: &str) -> Result<GetFileInfoResponseProto> {
        unimplemented!()
    }

    fn write_namenode_handshake(&self) -> Result<()> {
        let rpc_header = [
            b'h',
            b'r',
            b'p',
            b'c',
            RPC_VERSION,
            SERVICE_CLASS,
            AUTH_PROTOCOL,
        ];

        let header =
            new_rpc_request_header(HANDSHAKE_CALL_ID as i32, &self.client_id);

        let context = new_connection_context(&self.user);

        unimplemented!()
    }

    fn resolve_connection(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<NamenodeConnection> {
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

        Ok(NamenodeConnection {
            stream: stream,
            user: "user".to_owned(),
            client_id: "proto-hdfs-client".to_owned(),
            req_id: AtomicIsize::new(0),
        })
    }

    pub fn execute(
        &self,
        method: &str,
        req: &GetFileInfoRequestProto,
    ) -> Result<GetFileInfoResponseProto> {
        let curr_req_id = self.req_id.fetch_add(1, Ordering::Relaxed);

        self.resolve_connection()?;
        self.write_request(method, curr_req_id, req)?;
        self.read_response(method)
    }
}
