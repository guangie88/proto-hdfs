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
pub struct UserInformationProto {
    // message fields
    effectiveUser: ::protobuf::SingularField<::std::string::String>,
    realUser: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserInformationProto {}

impl UserInformationProto {
    pub fn new() -> UserInformationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserInformationProto {
        static mut instance: ::protobuf::lazy::Lazy<UserInformationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserInformationProto,
        };
        unsafe {
            instance.get(UserInformationProto::new)
        }
    }

    // optional string effectiveUser = 1;

    pub fn clear_effectiveUser(&mut self) {
        self.effectiveUser.clear();
    }

    pub fn has_effectiveUser(&self) -> bool {
        self.effectiveUser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effectiveUser(&mut self, v: ::std::string::String) {
        self.effectiveUser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_effectiveUser(&mut self) -> &mut ::std::string::String {
        if self.effectiveUser.is_none() {
            self.effectiveUser.set_default();
        }
        self.effectiveUser.as_mut().unwrap()
    }

    // Take field
    pub fn take_effectiveUser(&mut self) -> ::std::string::String {
        self.effectiveUser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_effectiveUser(&self) -> &str {
        match self.effectiveUser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_effectiveUser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.effectiveUser
    }

    fn mut_effectiveUser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.effectiveUser
    }

    // optional string realUser = 2;

    pub fn clear_realUser(&mut self) {
        self.realUser.clear();
    }

    pub fn has_realUser(&self) -> bool {
        self.realUser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_realUser(&mut self, v: ::std::string::String) {
        self.realUser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_realUser(&mut self) -> &mut ::std::string::String {
        if self.realUser.is_none() {
            self.realUser.set_default();
        }
        self.realUser.as_mut().unwrap()
    }

    // Take field
    pub fn take_realUser(&mut self) -> ::std::string::String {
        self.realUser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_realUser(&self) -> &str {
        match self.realUser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_realUser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.realUser
    }

    fn mut_realUser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.realUser
    }
}

impl ::protobuf::Message for UserInformationProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.effectiveUser)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.realUser)?;
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
        if let Some(ref v) = self.effectiveUser.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.realUser.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.effectiveUser.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.realUser.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for UserInformationProto {
    fn new() -> UserInformationProto {
        UserInformationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserInformationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "effectiveUser",
                    UserInformationProto::get_effectiveUser_for_reflect,
                    UserInformationProto::mut_effectiveUser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "realUser",
                    UserInformationProto::get_realUser_for_reflect,
                    UserInformationProto::mut_realUser_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserInformationProto>(
                    "UserInformationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserInformationProto {
    fn clear(&mut self) {
        self.clear_effectiveUser();
        self.clear_realUser();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserInformationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserInformationProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IpcConnectionContextProto {
    // message fields
    userInfo: ::protobuf::SingularPtrField<UserInformationProto>,
    protocol: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IpcConnectionContextProto {}

impl IpcConnectionContextProto {
    pub fn new() -> IpcConnectionContextProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IpcConnectionContextProto {
        static mut instance: ::protobuf::lazy::Lazy<IpcConnectionContextProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IpcConnectionContextProto,
        };
        unsafe {
            instance.get(IpcConnectionContextProto::new)
        }
    }

    // optional .hadoop.common.UserInformationProto userInfo = 2;

    pub fn clear_userInfo(&mut self) {
        self.userInfo.clear();
    }

    pub fn has_userInfo(&self) -> bool {
        self.userInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userInfo(&mut self, v: UserInformationProto) {
        self.userInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userInfo(&mut self) -> &mut UserInformationProto {
        if self.userInfo.is_none() {
            self.userInfo.set_default();
        }
        self.userInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_userInfo(&mut self) -> UserInformationProto {
        self.userInfo.take().unwrap_or_else(|| UserInformationProto::new())
    }

    pub fn get_userInfo(&self) -> &UserInformationProto {
        self.userInfo.as_ref().unwrap_or_else(|| UserInformationProto::default_instance())
    }

    fn get_userInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<UserInformationProto> {
        &self.userInfo
    }

    fn mut_userInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UserInformationProto> {
        &mut self.userInfo
    }

    // optional string protocol = 3;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol(&mut self) -> &mut ::std::string::String {
        if self.protocol.is_none() {
            self.protocol.set_default();
        }
        self.protocol.as_mut().unwrap()
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::string::String {
        self.protocol.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_protocol(&self) -> &str {
        match self.protocol.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_protocol_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.protocol
    }
}

impl ::protobuf::Message for IpcConnectionContextProto {
    fn is_initialized(&self) -> bool {
        for v in &self.userInfo {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.userInfo)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol)?;
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
        if let Some(ref v) = self.userInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.userInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.protocol.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for IpcConnectionContextProto {
    fn new() -> IpcConnectionContextProto {
        IpcConnectionContextProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<IpcConnectionContextProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserInformationProto>>(
                    "userInfo",
                    IpcConnectionContextProto::get_userInfo_for_reflect,
                    IpcConnectionContextProto::mut_userInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    IpcConnectionContextProto::get_protocol_for_reflect,
                    IpcConnectionContextProto::mut_protocol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IpcConnectionContextProto>(
                    "IpcConnectionContextProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IpcConnectionContextProto {
    fn clear(&mut self) {
        self.clear_userInfo();
        self.clear_protocol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IpcConnectionContextProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IpcConnectionContextProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aIpcConnectionContext.proto\x12\rhadoop.common\"X\n\x14UserInformat\
    ionProto\x12$\n\reffectiveUser\x18\x01\x20\x01(\tR\reffectiveUser\x12\
    \x1a\n\x08realUser\x18\x02\x20\x01(\tR\x08realUser\"x\n\x19IpcConnection\
    ContextProto\x12?\n\x08userInfo\x18\x02\x20\x01(\x0b2#.hadoop.common.Use\
    rInformationProtoR\x08userInfo\x12\x1a\n\x08protocol\x18\x03\x20\x01(\tR\
    \x08protocolB?\n\x1eorg.apache.hadoop.ipc.protobufB\x1aIpcConnectionCont\
    extProtos\xa0\x01\x01J\xb4\x0f\n\x06\x12\x04\x18\01\x01\n\x08\n\x01\x08\
    \x12\x03\x18\07\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\072\x83\x06*\n\
    \x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\
    \x20under\x20one\n\x20or\x20more\x20contributor\x20license\x20agreements\
    .\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\
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
    le*\x20.proto\x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x18\
    \x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\
    \x07\x12\x03\x18\x166\n\x08\n\x01\x08\x12\x03\x19\0;\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\x19\0;\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x19\x07\
    \x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\
    \xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x07\x12\x03\x19\x1e:\n\x08\n\x01\x08\x12\x03\x1a\0,\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x1a\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1a\x07$\
    \n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07$\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x1a\x07$\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\
    \x03\x1a'+\n\x08\n\x01\x02\x12\x03\x1b\x08\x15\n`\n\x02\x04\0\x12\x04\
    \x20\0#\x01\x1aT*\n\x20Spec\x20for\x20UserInformationProto\x20is\x20spec\
    ified\x20in\x20ProtoUtil#makeIpcConnectionContext\n\n\n\n\x03\x04\0\x01\
    \x12\x03\x20\x08\x1c\n\x0b\n\x04\x04\0\x02\0\x12\x03!\x02$\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03!\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03!\x0b\
    \x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03!\x12\x1f\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03!\"#\n\x0b\n\x04\x04\0\x02\x01\x12\x03\"\x02\x1f\n\x0c\n\
    \x05\x04\0\x02\x01\x04\x12\x03\"\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\"\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\"\x12\x1a\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\"\x1d\x1e\n\x9d\x01\n\x02\x04\x01\x12\x04\
    )\01\x01\x1a\x90\x01*\n\x20The\x20connection\x20context\x20is\x20sent\
    \x20as\x20part\x20of\x20the\x20connection\x20establishment.\n\x20It\x20e\
    stablishes\x20the\x20context\x20for\x20ALL\x20Rpc\x20calls\x20within\x20\
    the\x20connection.\n\n\n\n\x03\x04\x01\x01\x12\x03)\x08!\n|\n\x04\x04\
    \x01\x02\0\x12\x03,\x02-\x1ao\x20UserInfo\x20beyond\x20what\x20is\x20det\
    ermined\x20as\x20part\x20of\x20security\x20handshake\x20\n\x20at\x20conn\
    ection\x20time\x20(kerberos,\x20tokens\x20etc).\n\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03,\x0b\x1f\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03,\x20(\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03,+,\nd\n\x04\x04\x01\x02\x01\x12\x030\x02\x1f\x1aW\x20Protoc\
    ol\x20name\x20for\x20next\x20rpc\x20layer.\n\x20The\x20client\x20created\
    \x20a\x20proxy\x20with\x20this\x20protocol\x20name\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x030\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x030\x12\x1a\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x030\x1d\x1e\
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
