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
pub struct RefreshUserToGroupsMappingsRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshUserToGroupsMappingsRequestProto {}

impl RefreshUserToGroupsMappingsRequestProto {
    pub fn new() -> RefreshUserToGroupsMappingsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshUserToGroupsMappingsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshUserToGroupsMappingsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshUserToGroupsMappingsRequestProto,
        };
        unsafe {
            instance.get(RefreshUserToGroupsMappingsRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshUserToGroupsMappingsRequestProto {
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

impl ::protobuf::MessageStatic for RefreshUserToGroupsMappingsRequestProto {
    fn new() -> RefreshUserToGroupsMappingsRequestProto {
        RefreshUserToGroupsMappingsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshUserToGroupsMappingsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshUserToGroupsMappingsRequestProto>(
                    "RefreshUserToGroupsMappingsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshUserToGroupsMappingsRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshUserToGroupsMappingsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshUserToGroupsMappingsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshUserToGroupsMappingsResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshUserToGroupsMappingsResponseProto {}

impl RefreshUserToGroupsMappingsResponseProto {
    pub fn new() -> RefreshUserToGroupsMappingsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshUserToGroupsMappingsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshUserToGroupsMappingsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshUserToGroupsMappingsResponseProto,
        };
        unsafe {
            instance.get(RefreshUserToGroupsMappingsResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshUserToGroupsMappingsResponseProto {
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

impl ::protobuf::MessageStatic for RefreshUserToGroupsMappingsResponseProto {
    fn new() -> RefreshUserToGroupsMappingsResponseProto {
        RefreshUserToGroupsMappingsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshUserToGroupsMappingsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshUserToGroupsMappingsResponseProto>(
                    "RefreshUserToGroupsMappingsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshUserToGroupsMappingsResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshUserToGroupsMappingsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshUserToGroupsMappingsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshSuperUserGroupsConfigurationRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshSuperUserGroupsConfigurationRequestProto {}

impl RefreshSuperUserGroupsConfigurationRequestProto {
    pub fn new() -> RefreshSuperUserGroupsConfigurationRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshSuperUserGroupsConfigurationRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshSuperUserGroupsConfigurationRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshSuperUserGroupsConfigurationRequestProto,
        };
        unsafe {
            instance.get(RefreshSuperUserGroupsConfigurationRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshSuperUserGroupsConfigurationRequestProto {
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

impl ::protobuf::MessageStatic for RefreshSuperUserGroupsConfigurationRequestProto {
    fn new() -> RefreshSuperUserGroupsConfigurationRequestProto {
        RefreshSuperUserGroupsConfigurationRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshSuperUserGroupsConfigurationRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshSuperUserGroupsConfigurationRequestProto>(
                    "RefreshSuperUserGroupsConfigurationRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshSuperUserGroupsConfigurationRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshSuperUserGroupsConfigurationRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshSuperUserGroupsConfigurationRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshSuperUserGroupsConfigurationResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshSuperUserGroupsConfigurationResponseProto {}

impl RefreshSuperUserGroupsConfigurationResponseProto {
    pub fn new() -> RefreshSuperUserGroupsConfigurationResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshSuperUserGroupsConfigurationResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshSuperUserGroupsConfigurationResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshSuperUserGroupsConfigurationResponseProto,
        };
        unsafe {
            instance.get(RefreshSuperUserGroupsConfigurationResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshSuperUserGroupsConfigurationResponseProto {
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

impl ::protobuf::MessageStatic for RefreshSuperUserGroupsConfigurationResponseProto {
    fn new() -> RefreshSuperUserGroupsConfigurationResponseProto {
        RefreshSuperUserGroupsConfigurationResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshSuperUserGroupsConfigurationResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshSuperUserGroupsConfigurationResponseProto>(
                    "RefreshSuperUserGroupsConfigurationResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshSuperUserGroupsConfigurationResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshSuperUserGroupsConfigurationResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshSuperUserGroupsConfigurationResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!RefreshUserMappingsProtocol.proto\x12\rhadoop.common\")\n'RefreshUser\
    ToGroupsMappingsRequestProto\"*\n(RefreshUserToGroupsMappingsResponsePro\
    to\"1\n/RefreshSuperUserGroupsConfigurationRequestProto\"2\n0RefreshSupe\
    rUserGroupsConfigurationResponseProto2\xde\x02\n\"RefreshUserMappingsPro\
    tocolService\x12\x8e\x01\n\x1brefreshUserToGroupsMappings\x126.hadoop.co\
    mmon.RefreshUserToGroupsMappingsRequestProto\x1a7.hadoop.common.RefreshU\
    serToGroupsMappingsResponseProto\x12\xa6\x01\n#refreshSuperUserGroupsCon\
    figuration\x12>.hadoop.common.RefreshSuperUserGroupsConfigurationRequest\
    Proto\x1a?.hadoop.common.RefreshSuperUserGroupsConfigurationResponseProt\
    oBK\n\x20org.apache.hadoop.security.protoB!RefreshUserMappingsProtocolPr\
    otos\xa0\x01\x01\x88\x01\x01J\xf2\r\n\x06\x12\x04\x18\0E\x01\n\x08\n\x01\
    \x08\x12\x03\x18\09\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\092\x83\
    \x06*\n\x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20\
    (ASF)\x20under\x20one\n\x20or\x20more\x20contributor\x20license\x20agree\
    ments.\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20t\
    his\x20work\x20for\x20additional\x20information\n\x20regarding\x20copyri\
    ght\x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\
    \x20you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\
    \n\x20\"License\");\x20you\x20may\x20not\x20use\x20this\x20file\x20excep\
    t\x20in\x20compliance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20\
    obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\
    \x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\
    \x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20s\
    oftware\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\
    \x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\
    \x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20impli\
    ed.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20\
    governing\x20permissions\x20and\n\x20limitations\x20under\x20the\x20Lice\
    nse.\n2\xaa\x01*\n\x20These\x20.proto\x20interfaces\x20are\x20private\
    \x20and\x20stable.\n\x20Please\x20see\x20http://wiki.apache.org/hadoop/C\
    ompatibility\n\x20for\x20what\x20changes\x20are\x20allowed\x20for\x20a\
    \x20*stable*\x20.proto\x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\
    \x03\x18\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x13\n\x0e\
    \n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\x05\x08\xe7\
    \x07\0\x07\x12\x03\x18\x168\n\x08\n\x01\x08\x12\x03\x19\0B\n\x0b\n\x04\
    \x08\xe7\x07\x01\x12\x03\x19\0B\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\
    \x19\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\
    \x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x01\x07\x12\x03\x19\x1eA\n\x08\n\x01\x08\x12\x03\x1a\0$\n\x0b\n\x04\
    \x08\xe7\x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\
    \x1a\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07\x1c\n\x0e\n\
    \x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1a\x07\x1c\n\x0c\n\x05\x08\xe7\
    \x07\x02\x03\x12\x03\x1a\x1f#\n\x08\n\x01\x08\x12\x03\x1b\0,\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03\x1b\0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\
    \x1b\x07$\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x1b\x07$\n\x0e\n\x07\
    \x08\xe7\x07\x03\x02\0\x01\x12\x03\x1b\x07$\n\x0c\n\x05\x08\xe7\x07\x03\
    \x03\x12\x03\x1b'+\n\x08\n\x01\x02\x12\x03\x1c\x08\x15\n8\n\x02\x04\0\
    \x12\x04!\0\"\x01\x1a,*\n\x20\x20Refresh\x20user\x20to\x20group\x20mappi\
    ngs\x20request.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08/\n\x1d\n\x02\x04\x01\
    \x12\x04'\0(\x01\x1a\x11*\n\x20void\x20response\n\n\n\n\x03\x04\x01\x01\
    \x12\x03'\x080\n8\n\x02\x04\x02\x12\x04-\0.\x01\x1a,*\n\x20Refresh\x20su\
    peruser\x20configuration\x20request.\n\n\n\n\x03\x04\x02\x01\x12\x03-\
    \x087\n\x1d\n\x02\x04\x03\x12\x043\04\x01\x1a\x11*\n\x20void\x20response\
    \n\n\n\n\x03\x04\x03\x01\x12\x033\x088\n6\n\x02\x06\0\x12\x049\0E\x01\
    \x1a**\n\x20Protocol\x20to\x20refresh\x20the\x20user\x20mappings.\n\n\n\
    \n\x03\x06\0\x01\x12\x039\x08*\n1\n\x04\x06\0\x02\0\x12\x04=\x02>8\x1a#*\
    \n\x20Refresh\x20user\x20to\x20group\x20mappings.\n\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03=\x06!\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03=\"I\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03>\x0e6\n5\n\x04\x06\0\x02\x01\x12\x04C\x02D@\
    \x1a'*\n\x20Refresh\x20superuser\x20proxy\x20group\x20list.\n\n\x0c\n\
    \x05\x06\0\x02\x01\x01\x12\x03C\x06)\n\x0c\n\x05\x06\0\x02\x01\x02\x12\
    \x03C*Y\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03D\x0e>\
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
