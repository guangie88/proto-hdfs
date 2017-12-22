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
pub struct CedeActiveRequestProto {
    // message fields
    millisToCede: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CedeActiveRequestProto {}

impl CedeActiveRequestProto {
    pub fn new() -> CedeActiveRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CedeActiveRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CedeActiveRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CedeActiveRequestProto,
        };
        unsafe {
            instance.get(CedeActiveRequestProto::new)
        }
    }

    // required uint32 millisToCede = 1;

    pub fn clear_millisToCede(&mut self) {
        self.millisToCede = ::std::option::Option::None;
    }

    pub fn has_millisToCede(&self) -> bool {
        self.millisToCede.is_some()
    }

    // Param is passed by value, moved
    pub fn set_millisToCede(&mut self, v: u32) {
        self.millisToCede = ::std::option::Option::Some(v);
    }

    pub fn get_millisToCede(&self) -> u32 {
        self.millisToCede.unwrap_or(0)
    }

    fn get_millisToCede_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.millisToCede
    }

    fn mut_millisToCede_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.millisToCede
    }
}

impl ::protobuf::Message for CedeActiveRequestProto {
    fn is_initialized(&self) -> bool {
        if self.millisToCede.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.millisToCede = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.millisToCede {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.millisToCede {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for CedeActiveRequestProto {
    fn new() -> CedeActiveRequestProto {
        CedeActiveRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CedeActiveRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "millisToCede",
                    CedeActiveRequestProto::get_millisToCede_for_reflect,
                    CedeActiveRequestProto::mut_millisToCede_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CedeActiveRequestProto>(
                    "CedeActiveRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CedeActiveRequestProto {
    fn clear(&mut self) {
        self.clear_millisToCede();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CedeActiveRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CedeActiveRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CedeActiveResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CedeActiveResponseProto {}

impl CedeActiveResponseProto {
    pub fn new() -> CedeActiveResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CedeActiveResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CedeActiveResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CedeActiveResponseProto,
        };
        unsafe {
            instance.get(CedeActiveResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CedeActiveResponseProto {
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

impl ::protobuf::MessageStatic for CedeActiveResponseProto {
    fn new() -> CedeActiveResponseProto {
        CedeActiveResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CedeActiveResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CedeActiveResponseProto>(
                    "CedeActiveResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CedeActiveResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CedeActiveResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CedeActiveResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GracefulFailoverRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GracefulFailoverRequestProto {}

impl GracefulFailoverRequestProto {
    pub fn new() -> GracefulFailoverRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GracefulFailoverRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GracefulFailoverRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GracefulFailoverRequestProto,
        };
        unsafe {
            instance.get(GracefulFailoverRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GracefulFailoverRequestProto {
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

impl ::protobuf::MessageStatic for GracefulFailoverRequestProto {
    fn new() -> GracefulFailoverRequestProto {
        GracefulFailoverRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GracefulFailoverRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GracefulFailoverRequestProto>(
                    "GracefulFailoverRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GracefulFailoverRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GracefulFailoverRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GracefulFailoverRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GracefulFailoverResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GracefulFailoverResponseProto {}

impl GracefulFailoverResponseProto {
    pub fn new() -> GracefulFailoverResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GracefulFailoverResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GracefulFailoverResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GracefulFailoverResponseProto,
        };
        unsafe {
            instance.get(GracefulFailoverResponseProto::new)
        }
    }
}

impl ::protobuf::Message for GracefulFailoverResponseProto {
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

impl ::protobuf::MessageStatic for GracefulFailoverResponseProto {
    fn new() -> GracefulFailoverResponseProto {
        GracefulFailoverResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GracefulFailoverResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GracefulFailoverResponseProto>(
                    "GracefulFailoverResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GracefulFailoverResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GracefulFailoverResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GracefulFailoverResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12ZKFCProtocol.proto\x12\rhadoop.common\"<\n\x16CedeActiveRequestPro\
    to\x12\"\n\x0cmillisToCede\x18\x01\x20\x02(\rR\x0cmillisToCede\"\x19\n\
    \x17CedeActiveResponseProto\"\x1e\n\x1cGracefulFailoverRequestProto\"\
    \x1f\n\x1dGracefulFailoverResponseProto2\xe1\x01\n\x13ZKFCProtocolServic\
    e\x12[\n\ncedeActive\x12%.hadoop.common.CedeActiveRequestProto\x1a&.hado\
    op.common.CedeActiveResponseProto\x12m\n\x10gracefulFailover\x12+.hadoop\
    .common.GracefulFailoverRequestProto\x1a,.hadoop.common.GracefulFailover\
    ResponseProtoB6\n\x1aorg.apache.hadoop.ha.protoB\x12ZKFCProtocolProtos\
    \xa0\x01\x01\x88\x01\x01J\xe4\r\n\x06\x12\x04\x18\0:\x01\n\x08\n\x01\x08\
    \x12\x03\x18\03\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\032\x83\x06*\n\
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
    \x07\x12\x03\x18\x162\n\x08\n\x01\x08\x12\x03\x19\03\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\x19\03\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x19\x07\
    \x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\
    \xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x07\x12\x03\x19\x1e2\n\x08\n\x01\x08\x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1a\x07\
    \x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\
    \xe7\x07\x02\x02\0\x01\x12\x03\x1a\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\
    \x03\x12\x03\x1a\x1f#\n\x08\n\x01\x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\
    \x07\x03\x12\x03\x1b\0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\
    \n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\x1b\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\
    \x03\x1b'+\n\x08\n\x01\x02\x12\x03\x1c\x08\x15\n\n\n\x02\x04\0\x12\x04\
    \x1e\0\x20\x01\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08\x1e\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x1f\x02#\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x1f\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1f\x0b\x11\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x1f\x12\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1f!\"\n\n\
    \n\x02\x04\x01\x12\x04\"\0#\x01\n\n\n\x03\x04\x01\x01\x12\x03\"\x08\x1f\
    \n\n\n\x02\x04\x02\x12\x04%\0&\x01\n\n\n\x03\x04\x02\x01\x12\x03%\x08$\n\
    \n\n\x02\x04\x03\x12\x04(\0)\x01\n\n\n\x03\x04\x03\x01\x12\x03(\x08%\nO\
    \n\x02\x06\0\x12\x04/\0:\x01\x1aC*\n\x20Protocol\x20provides\x20manual\
    \x20control\x20of\x20the\x20ZK\x20Failover\x20Controllers\n\n\n\n\x03\
    \x06\0\x01\x12\x03/\x08\x1b\np\n\x04\x06\0\x02\0\x12\x044\x025'\x1ab*\n\
    \x20Request\x20that\x20the\x20service\x20cede\x20its\x20active\x20state,\
    \x20and\x20quit\x20the\x20election\n\x20for\x20some\x20amount\x20of\x20t\
    ime\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x034\x06\x10\n\x0c\n\x05\x06\0\x02\
    \0\x02\x12\x034\x11'\n\x0c\n\x05\x06\0\x02\0\x03\x12\x035\x0e%\n\x0c\n\
    \x04\x06\0\x02\x01\x12\x048\x029-\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x038\
    \x06\x16\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x038\x173\n\x0c\n\x05\x06\0\
    \x02\x01\x03\x12\x039\x0e+\
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
