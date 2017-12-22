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
pub struct RefreshServiceAclRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshServiceAclRequestProto {}

impl RefreshServiceAclRequestProto {
    pub fn new() -> RefreshServiceAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshServiceAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshServiceAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshServiceAclRequestProto,
        };
        unsafe {
            instance.get(RefreshServiceAclRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshServiceAclRequestProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RefreshServiceAclRequestProto {
    fn new() -> RefreshServiceAclRequestProto {
        RefreshServiceAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshServiceAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshServiceAclRequestProto>(
                    "RefreshServiceAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshServiceAclRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshServiceAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshServiceAclRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshServiceAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshServiceAclResponseProto {}

impl RefreshServiceAclResponseProto {
    pub fn new() -> RefreshServiceAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshServiceAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshServiceAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshServiceAclResponseProto,
        };
        unsafe {
            instance.get(RefreshServiceAclResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshServiceAclResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RefreshServiceAclResponseProto {
    fn new() -> RefreshServiceAclResponseProto {
        RefreshServiceAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshServiceAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshServiceAclResponseProto>(
                    "RefreshServiceAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshServiceAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshServiceAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshServiceAclResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(RefreshAuthorizationPolicyProtocol.proto\x12\rhadoop.common\"\x1f\n\
    \x1dRefreshServiceAclRequestProto\"\x20\n\x1eRefreshServiceAclResponsePr\
    oto2\x9d\x01\n)RefreshAuthorizationPolicyProtocolService\x12p\n\x11refre\
    shServiceAcl\x12,.hadoop.common.RefreshServiceAclRequestProto\x1a-.hadoo\
    p.common.RefreshServiceAclResponseProtoBR\n\x20org.apache.hadoop.securit\
    y.protoB(RefreshAuthorizationPolicyProtocolProtos\xa0\x01\x01\x88\x01\
    \x01J\xd5\x0c\n\x06\x12\x04\x18\03\x01\n\x08\n\x01\x08\x12\x03\x18\09\n\
    \xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\092\x83\x06*\n\x20Licensed\x20t\
    o\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\x20under\x20one\n\
    \x20or\x20more\x20contributor\x20license\x20agreements.\x20\x20See\x20th\
    e\x20NOTICE\x20file\n\x20distributed\x20with\x20this\x20work\x20for\x20a\
    dditional\x20information\n\x20regarding\x20copyright\x20ownership.\x20\
    \x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20you\x20under\x20th\
    e\x20Apache\x20License,\x20Version\x202.0\x20(the\n\x20\"License\");\x20\
    you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\n\
    \x20with\x20the\x20License.\x20\x20You\x20may\x20obtain\x20a\x20copy\x20\
    of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/\
    licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20la\
    w\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\
    \x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20\
    IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20A\
    NY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20Li\
    cense\x20for\x20the\x20specific\x20language\x20governing\x20permissions\
    \x20and\n\x20limitations\x20under\x20the\x20License.\n2\xaa\x01*\n\x20Th\
    ese\x20.proto\x20interfaces\x20are\x20private\x20and\x20stable.\n\x20Ple\
    ase\x20see\x20http://wiki.apache.org/hadoop/Compatibility\n\x20for\x20wh\
    at\x20changes\x20are\x20allowed\x20for\x20a\x20*stable*\x20.proto\x20int\
    erface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x18\x07\x13\n\r\n\x06\
    \x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\x18\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x18\x168\
    \n\x08\n\x01\x08\x12\x03\x19\0I\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x19\
    \0I\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x19\x07\x1b\n\r\n\x06\x08\
    \xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\
    \x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x19\x1e\
    H\n\x08\n\x01\x08\x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1a\
    \0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1a\x07\x1c\n\r\n\x06\x08\
    \xe7\x07\x02\x02\0\x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\
    \x01\x12\x03\x1a\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1a\x1f\
    #\n\x08\n\x01\x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1b\
    \0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\x1b\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1b'+\n\x08\n\
    \x01\x02\x12\x03\x1c\x08\x15\n-\n\x02\x04\0\x12\x04!\0\"\x01\x1a!*\n\x20\
    \x20Refresh\x20service\x20acl\x20request.\n\n\n\n\x03\x04\0\x01\x12\x03!\
    \x08%\n\x1d\n\x02\x04\x01\x12\x04'\0(\x01\x1a\x11*\n\x20void\x20response\
    \n\n\n\n\x03\x04\x01\x01\x12\x03'\x08&\n\\\n\x02\x06\0\x12\x04-\03\x01\
    \x1aP*\n\x20Protocol\x20which\x20is\x20used\x20to\x20refresh\x20the\x20a\
    uthorization\x20policy\x20in\x20use\x20currently.\n\n\n\n\x03\x06\0\x01\
    \x12\x03-\x081\nK\n\x04\x06\0\x02\0\x12\x041\x022.\x1a=*\n\x20Refresh\
    \x20the\x20service-level\x20authorization\x20policy\x20in-effect.\n\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x031\x06\x17\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x031\x185\n\x0c\n\x05\x06\0\x02\0\x03\x12\x032\x0e,\
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
