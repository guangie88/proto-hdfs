use error::Result;
use hadoop_common::{IpcConnectionContextProto, RequestHeaderProto,
                    RpcKindProto, RpcRequestHeaderProto,
                    RpcRequestHeaderProto_OperationProto, UserInformationProto};
use hadoop_hdfs::{GetFileInfoRequestProto, GetFileInfoResponseProto};
use protobuf::Message;
use rpc::make_rpc_packet;

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

fn new_request_header<S>(method_name: S) -> RequestHeaderProto
where
    S: Into<String>,
{
    let mut header = RequestHeaderProto::new();
    header.set_methodName(method_name.into());
    header.set_declaringClassProtocolName(PROTOCOL_CLASS.to_owned());
    header.set_clientProtocolVersion(PROTOCOL_CLASS_VERSION as u64);
    header
}

fn new_rpc_request_header<B>(id: i32, client_id: B) -> RpcRequestHeaderProto
where
    B: AsRef<[u8]>,
{
    let mut header = RpcRequestHeaderProto::new();
    header.set_rpcKind(RpcKindProto::RPC_PROTOCOL_BUFFER);
    header.set_rpcOp(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET);
    header.set_callId(id);
    header.set_clientId(client_id.as_ref().to_vec());
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
    fn write_request<S, M>(&mut self, method: S, req: M) -> Result<()>
    where
        S: AsRef<str>,
        M: Message,
    {
        let rrh = new_rpc_request_header(
            self.req_id.load(Ordering::Relaxed) as i32,
            &self.client_id,
        );

        let method = method.as_ref();
        let rh = new_request_header(method);

        let req_bytes = make_rpc_packet(
            vec![&rrh as &Message, &rh as &Message, &req].into_iter(),
        )?;

        Ok(self.stream.write_all(&req_bytes)?)
    }

    fn read_response<S>(&self, method: S, resp: &mut Message) -> Result<()>
    where
        S: AsRef<str>,
    {
        // TODO
        unimplemented!()
    }

    fn write_namenode_handshake(&mut self) -> Result<()> {
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
        let packet = make_rpc_packet(
            vec![&header as &Message, &context as &Message].into_iter(),
        )?;

        self.stream.write_all(&rpc_header)?;
        self.stream.write_all(packet.as_slice())?;
        Ok(())
    }

    fn resolve_connection(&self) -> Result<()> {
        // TODO
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

    pub fn execute<S, M>(
        &mut self,
        method: S,
        req: M,
        resp: &mut Message,
    ) -> Result<()>
    where
        S: AsRef<str>,
        M: Message,
    {
        let curr_req_id = self.req_id.fetch_add(1, Ordering::Relaxed);
        self.resolve_connection()?;

        let method = method.as_ref();
        self.write_request(method, req)?;
        self.read_response(method, resp)
    }
}
