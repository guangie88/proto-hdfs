// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RequestHeaderProto {
    // message fields
    methodName: ::protobuf::SingularField<::std::string::String>,
    declaringClassProtocolName: ::protobuf::SingularField<::std::string::String>,
    clientProtocolVersion: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestHeaderProto {}

impl RequestHeaderProto {
    pub fn new() -> RequestHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<RequestHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHeaderProto,
        };
        unsafe {
            instance.get(RequestHeaderProto::new)
        }
    }

    // required string methodName = 1;

    pub fn clear_methodName(&mut self) {
        self.methodName.clear();
    }

    pub fn has_methodName(&self) -> bool {
        self.methodName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_methodName(&mut self, v: ::std::string::String) {
        self.methodName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_methodName(&mut self) -> &mut ::std::string::String {
        if self.methodName.is_none() {
            self.methodName.set_default();
        }
        self.methodName.as_mut().unwrap()
    }

    // Take field
    pub fn take_methodName(&mut self) -> ::std::string::String {
        self.methodName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_methodName(&self) -> &str {
        match self.methodName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_methodName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.methodName
    }

    fn mut_methodName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.methodName
    }

    // required string declaringClassProtocolName = 2;

    pub fn clear_declaringClassProtocolName(&mut self) {
        self.declaringClassProtocolName.clear();
    }

    pub fn has_declaringClassProtocolName(&self) -> bool {
        self.declaringClassProtocolName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_declaringClassProtocolName(&mut self, v: ::std::string::String) {
        self.declaringClassProtocolName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_declaringClassProtocolName(&mut self) -> &mut ::std::string::String {
        if self.declaringClassProtocolName.is_none() {
            self.declaringClassProtocolName.set_default();
        }
        self.declaringClassProtocolName.as_mut().unwrap()
    }

    // Take field
    pub fn take_declaringClassProtocolName(&mut self) -> ::std::string::String {
        self.declaringClassProtocolName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_declaringClassProtocolName(&self) -> &str {
        match self.declaringClassProtocolName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_declaringClassProtocolName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.declaringClassProtocolName
    }

    fn mut_declaringClassProtocolName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.declaringClassProtocolName
    }

    // required uint64 clientProtocolVersion = 3;

    pub fn clear_clientProtocolVersion(&mut self) {
        self.clientProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_clientProtocolVersion(&self) -> bool {
        self.clientProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientProtocolVersion(&mut self, v: u64) {
        self.clientProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_clientProtocolVersion(&self) -> u64 {
        self.clientProtocolVersion.unwrap_or(0)
    }

    fn get_clientProtocolVersion_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.clientProtocolVersion
    }

    fn mut_clientProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.clientProtocolVersion
    }
}

impl ::protobuf::Message for RequestHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.methodName.is_none() {
            return false;
        }
        if self.declaringClassProtocolName.is_none() {
            return false;
        }
        if self.clientProtocolVersion.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.methodName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.declaringClassProtocolName)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.clientProtocolVersion = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.methodName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.declaringClassProtocolName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.clientProtocolVersion {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.methodName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.declaringClassProtocolName.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.clientProtocolVersion {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestHeaderProto {
    fn new() -> RequestHeaderProto {
        RequestHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "methodName",
                    RequestHeaderProto::get_methodName_for_reflect,
                    RequestHeaderProto::mut_methodName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "declaringClassProtocolName",
                    RequestHeaderProto::get_declaringClassProtocolName_for_reflect,
                    RequestHeaderProto::mut_declaringClassProtocolName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "clientProtocolVersion",
                    RequestHeaderProto::get_clientProtocolVersion_for_reflect,
                    RequestHeaderProto::mut_clientProtocolVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHeaderProto>(
                    "RequestHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHeaderProto {
    fn clear(&mut self) {
        self.clear_methodName();
        self.clear_declaringClassProtocolName();
        self.clear_clientProtocolVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ProtobufRpcEngine.proto\x12\rhadoop.common\"\xaa\x01\n\x12RequestH\
    eaderProto\x12\x1e\n\nmethodName\x18\x01\x20\x02(\tR\nmethodName\x12>\n\
    \x1adeclaringClassProtocolName\x18\x02\x20\x02(\tR\x1adeclaringClassProt\
    ocolName\x124\n\x15clientProtocolVersion\x18\x03\x20\x02(\x04R\x15client\
    ProtocolVersionB<\n\x1eorg.apache.hadoop.ipc.protobufB\x17ProtobufRpcEng\
    ineProtos\xa0\x01\x01J\xff\x15\n\x06\x12\x04\x1e\0B\x01\n\x08\n\x01\x08\
    \x12\x03\x1e\07\n\xa4\t\n\x04\x08\xe7\x07\0\x12\x03\x1e\07\x1a\xe3\x01*\
    \n\x20These\x20are\x20the\x20messages\x20used\x20by\x20Hadoop\x20RPC\x20\
    for\x20the\x20Rpc\x20Engine\x20Protocol\x20Buffer\n\x20to\x20marshal\x20\
    the\x20request\x20and\x20response\x20in\x20the\x20RPC\x20layer.\n\x20The\
    \x20messages\x20are\x20sent\x20in\x20addition\x20to\x20the\x20normal\x20\
    RPC\x20header\x20as\x20\n\x20defined\x20in\x20RpcHeader.proto\n2\x83\x06\
    *\n\x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF\
    )\x20under\x20one\n\x20or\x20more\x20contributor\x20license\x20agreement\
    s.\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\
    \x20work\x20for\x20additional\x20information\n\x20regarding\x20copyright\
    \x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20\
    you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\n\
    \x20\"License\");\x20you\x20may\x20not\x20use\x20this\x20file\x20except\
    \x20in\x20compliance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20o\
    btain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20\
    http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20\
    by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20softw\
    are\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\x20\
    on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20C\
    ONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\
    \x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20gover\
    ning\x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\
    \n2\xaa\x01*\n\x20These\x20.proto\x20interfaces\x20are\x20private\x20and\
    \x20stable.\n\x20Please\x20see\x20http://wiki.apache.org/hadoop/Compatib\
    ility\n\x20for\x20what\x20changes\x20are\x20allowed\x20for\x20a\x20*stab\
    le*\x20.proto\x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x1e\
    \x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x1e\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x1e\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\
    \x07\x12\x03\x1e\x166\n\x08\n\x01\x08\x12\x03\x1f\08\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\x1f\08\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x1f\x07\
    \x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x1f\x07\x1b\n\x0e\n\x07\x08\
    \xe7\x07\x01\x02\0\x01\x12\x03\x1f\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x07\x12\x03\x1f\x1e7\n\x08\n\x01\x08\x12\x03\x20\0,\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x20\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x20\x07$\
    \n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x20\x07$\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x20\x07$\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\
    \x03\x20'+\n\x08\n\x01\x02\x12\x03!\x08\x15\n\xdd\x02\n\x02\x04\0\x12\
    \x04+\0B\x01\x1a\xd0\x02*\n\x20This\x20message\x20is\x20the\x20header\
    \x20for\x20the\x20Protobuf\x20Rpc\x20Engine\n\x20when\x20sending\x20a\
    \x20RPC\x20request\x20from\x20\x20RPC\x20client\x20to\x20the\x20RPC\x20s\
    erver.\n\x20The\x20actual\x20request\x20(serialized\x20as\x20protobuf)\
    \x20follows\x20this\x20request.\n\n\x20No\x20special\x20header\x20is\x20\
    needed\x20for\x20the\x20Rpc\x20Response\x20for\x20Protobuf\x20Rpc\x20Eng\
    ine.\n\x20The\x20normal\x20RPC\x20response\x20header\x20(see\x20RpcHeade\
    r.proto)\x20are\x20sufficient.\x20\n\n\n\n\x03\x04\0\x01\x12\x03+\x08\
    \x1a\n&\n\x04\x04\0\x02\0\x12\x03-\x02!\x1a\x19*\x20Name\x20of\x20the\
    \x20RPC\x20method\x20\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03-\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03-\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03-\x12\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03-\x1f\x20\n\xd7\x05\n\
    \x04\x04\0\x02\x01\x12\x03>\x021\x1a\xc9\x05*\x20\n\x20RPCs\x20for\x20a\
    \x20particular\x20interface\x20(ie\x20protocol)\x20are\x20done\x20using\
    \x20a\n\x20IPC\x20connection\x20that\x20is\x20setup\x20using\x20rpcProxy\
    .\n\x20The\x20rpcProxy's\x20has\x20a\x20declared\x20protocol\x20name\x20\
    that\x20is\x20\n\x20sent\x20form\x20client\x20to\x20server\x20at\x20conn\
    ection\x20time.\x20\n\x20\n\x20Each\x20Rpc\x20call\x20also\x20sends\x20a\
    \x20protocol\x20name\x20\n\x20(called\x20declaringClassprotocolName).\
    \x20This\x20name\x20is\x20usually\x20the\x20same\n\x20as\x20the\x20conne\
    ction\x20protocol\x20name\x20except\x20in\x20some\x20cases.\x20\n\x20For\
    \x20example\x20metaProtocols\x20such\x20ProtocolInfoProto\x20which\x20ge\
    t\x20metainfo\n\x20about\x20the\x20protocol\x20reuse\x20the\x20connectio\
    n\x20but\x20need\x20to\x20indicate\x20that\n\x20the\x20actual\x20protoco\
    l\x20is\x20different\x20(i.e.\x20the\x20protocol\x20is\n\x20ProtocolInfo\
    Proto)\x20since\x20they\x20reuse\x20the\x20connection;\x20in\x20this\x20\
    case\n\x20the\x20declaringClassProtocolName\x20field\x20is\x20set\x20to\
    \x20the\x20ProtocolInfoProto\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03>\
    \x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03>\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03>\x12,\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03>/0\nE\n\
    \x04\x04\0\x02\x02\x12\x03A\x02,\x1a8*\x20protocol\x20version\x20of\x20c\
    lass\x20declaring\x20the\x20called\x20method\x20\n\x0c\n\x05\x04\0\x02\
    \x02\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03A\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03A\x12'\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03A*+\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
