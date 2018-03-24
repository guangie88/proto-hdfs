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
pub struct GetGroupsForUserRequestProto {
    // message fields
    user: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGroupsForUserRequestProto {}

impl GetGroupsForUserRequestProto {
    pub fn new() -> GetGroupsForUserRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGroupsForUserRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetGroupsForUserRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGroupsForUserRequestProto,
        };
        unsafe {
            instance.get(GetGroupsForUserRequestProto::new)
        }
    }

    // required string user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut ::std::string::String {
        if self.user.is_none() {
            self.user.set_default();
        }
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> ::std::string::String {
        self.user.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user(&self) -> &str {
        match self.user.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_user_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.user
    }

    fn mut_user_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.user
    }
}

impl ::protobuf::Message for GetGroupsForUserRequestProto {
    fn is_initialized(&self) -> bool {
        if self.user.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user)?;
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
        if let Some(ref v) = self.user.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.user.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for GetGroupsForUserRequestProto {
    fn new() -> GetGroupsForUserRequestProto {
        GetGroupsForUserRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGroupsForUserRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user",
                    GetGroupsForUserRequestProto::get_user_for_reflect,
                    GetGroupsForUserRequestProto::mut_user_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGroupsForUserRequestProto>(
                    "GetGroupsForUserRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGroupsForUserRequestProto {
    fn clear(&mut self) {
        self.clear_user();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetGroupsForUserRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGroupsForUserRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetGroupsForUserResponseProto {
    // message fields
    groups: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGroupsForUserResponseProto {}

impl GetGroupsForUserResponseProto {
    pub fn new() -> GetGroupsForUserResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGroupsForUserResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetGroupsForUserResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGroupsForUserResponseProto,
        };
        unsafe {
            instance.get(GetGroupsForUserResponseProto::new)
        }
    }

    // repeated string groups = 1;

    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    // Param is passed by value, moved
    pub fn set_groups(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.groups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_groups(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.groups
    }

    // Take field
    pub fn take_groups(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.groups, ::protobuf::RepeatedField::new())
    }

    pub fn get_groups(&self) -> &[::std::string::String] {
        &self.groups
    }

    fn get_groups_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.groups
    }

    fn mut_groups_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.groups
    }
}

impl ::protobuf::Message for GetGroupsForUserResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.groups)?;
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
        for value in &self.groups {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.groups {
            os.write_string(1, &v)?;
        };
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

impl ::protobuf::MessageStatic for GetGroupsForUserResponseProto {
    fn new() -> GetGroupsForUserResponseProto {
        GetGroupsForUserResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGroupsForUserResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "groups",
                    GetGroupsForUserResponseProto::get_groups_for_reflect,
                    GetGroupsForUserResponseProto::mut_groups_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGroupsForUserResponseProto>(
                    "GetGroupsForUserResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGroupsForUserResponseProto {
    fn clear(&mut self) {
        self.clear_groups();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetGroupsForUserResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGroupsForUserResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dGetUserMappingsProtocol.proto\x12\rhadoop.common\"2\n\x1cGetGroups\
    ForUserRequestProto\x12\x12\n\x04user\x18\x01\x20\x02(\tR\x04user\"7\n\
    \x1dGetGroupsForUserResponseProto\x12\x16\n\x06groups\x18\x01\x20\x03(\t\
    R\x06groups2\x8f\x01\n\x1eGetUserMappingsProtocolService\x12m\n\x10getGr\
    oupsForUser\x12+.hadoop.common.GetGroupsForUserRequestProto\x1a,.hadoop.\
    common.GetGroupsForUserResponseProtoBD\n\x1dorg.apache.hadoop.tools.prot\
    oB\x1dGetUserMappingsProtocolProtos\xa0\x01\x01\x88\x01\x01J\xbb\r\n\x06\
    \x12\x04\x18\06\x01\n\x08\n\x01\x08\x12\x03\x18\06\n\xbe\x07\n\x04\x08\
    \xe7\x07\0\x12\x03\x18\062\x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\
    \x20Software\x20Foundation\x20(ASF)\x20under\x20one\n\x20or\x20more\x20c\
    ontributor\x20license\x20agreements.\x20\x20See\x20the\x20NOTICE\x20file\
    \n\x20distributed\x20with\x20this\x20work\x20for\x20additional\x20inform\
    ation\n\x20regarding\x20copyright\x20ownership.\x20\x20The\x20ASF\x20lic\
    enses\x20this\x20file\n\x20to\x20you\x20under\x20the\x20Apache\x20Licens\
    e,\x20Version\x202.0\x20(the\n\x20\"License\");\x20you\x20may\x20not\x20\
    use\x20this\x20file\x20except\x20in\x20compliance\n\x20with\x20the\x20Li\
    cense.\x20\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\
    \x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\
    \n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n2\xaa\x01*\n\x20These\x20.proto\x20\
    interfaces\x20are\x20private\x20and\x20stable.\n\x20Please\x20see\x20htt\
    p://wiki.apache.org/hadoop/Compatibility\n\x20for\x20what\x20changes\x20\
    are\x20allowed\x20for\x20a\x20*stable*\x20.proto\x20interface.\n\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x18\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x18\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\
    \x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x18\x165\n\x08\n\x01\x08\x12\
    \x03\x19\0>\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x19\0>\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x19\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x19\x1e=\n\x08\n\x01\x08\
    \x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x1a\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1a\
    \x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1a\x1f#\n\x08\n\x01\
    \x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1b\0,\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\n\r\n\x06\x08\xe7\x07\x03\x02\
    \0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x1b\
    \x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1b'+\n\x08\n\x01\x02\x12\
    \x03\x1c\x08\x15\n-\n\x02\x04\0\x12\x04!\0#\x01\x1a!*\n\x20\x20Get\x20gr\
    oups\x20for\x20user\x20request.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08$\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\"\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\"\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\"\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\"\x12\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\"\
    \x19\x1a\n(\n\x02\x04\x01\x12\x04(\0*\x01\x1a\x1c*\n\x20Response\x20for\
    \x20get\x20groups.\n\n\n\n\x03\x04\x01\x01\x12\x03(\x08%\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03)\x02\x1d\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03)\x02\n\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03)\x12\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03)\x1b\x1c\n\
    4\n\x02\x06\0\x12\x040\06\x01\x1a(*\n\x20Protocol\x20which\x20maps\x20us\
    ers\x20to\x20groups.\n\n\n\n\x03\x06\0\x01\x12\x030\x08&\nD\n\x04\x06\0\
    \x02\0\x12\x044\x025-\x1a6*\n\x20Get\x20the\x20groups\x20which\x20are\
    \x20mapped\x20to\x20the\x20given\x20user.\n\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x034\x06\x16\n\x0c\n\x05\x06\0\x02\0\x02\x12\x034\x173\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x035\x0e+\
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
