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
pub struct RefreshCallQueueRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshCallQueueRequestProto {}

impl RefreshCallQueueRequestProto {
    pub fn new() -> RefreshCallQueueRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshCallQueueRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshCallQueueRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshCallQueueRequestProto,
        };
        unsafe {
            instance.get(RefreshCallQueueRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshCallQueueRequestProto {
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

impl ::protobuf::MessageStatic for RefreshCallQueueRequestProto {
    fn new() -> RefreshCallQueueRequestProto {
        RefreshCallQueueRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshCallQueueRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshCallQueueRequestProto>(
                    "RefreshCallQueueRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshCallQueueRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshCallQueueRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshCallQueueRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshCallQueueResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshCallQueueResponseProto {}

impl RefreshCallQueueResponseProto {
    pub fn new() -> RefreshCallQueueResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshCallQueueResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshCallQueueResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshCallQueueResponseProto,
        };
        unsafe {
            instance.get(RefreshCallQueueResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshCallQueueResponseProto {
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

impl ::protobuf::MessageStatic for RefreshCallQueueResponseProto {
    fn new() -> RefreshCallQueueResponseProto {
        RefreshCallQueueResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshCallQueueResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshCallQueueResponseProto>(
                    "RefreshCallQueueResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshCallQueueResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshCallQueueResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshCallQueueResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eRefreshCallQueueProtocol.proto\x12\rhadoop.common\"\x1e\n\x1cRefre\
    shCallQueueRequestProto\"\x1f\n\x1dRefreshCallQueueResponseProto2\x90\
    \x01\n\x1fRefreshCallQueueProtocolService\x12m\n\x10refreshCallQueue\x12\
    +.hadoop.common.RefreshCallQueueRequestProto\x1a,.hadoop.common.RefreshC\
    allQueueResponseProtoBC\n\x1borg.apache.hadoop.ipc.protoB\x1eRefreshCall\
    QueueProtocolProtos\xa0\x01\x01\x88\x01\x01J\x95\x0c\n\x06\x12\x04\x18\0\
    3\x01\n\x08\n\x01\x08\x12\x03\x18\04\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\
    \x03\x18\042\x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\x20Software\
    \x20Foundation\x20(ASF)\x20under\x20one\n\x20or\x20more\x20contributor\
    \x20license\x20agreements.\x20\x20See\x20the\x20NOTICE\x20file\n\x20dist\
    ributed\x20with\x20this\x20work\x20for\x20additional\x20information\n\
    \x20regarding\x20copyright\x20ownership.\x20\x20The\x20ASF\x20licenses\
    \x20this\x20file\n\x20to\x20you\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\n\x20\"License\");\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\n\x20with\x20the\x20Lice\
    nse.\x20\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\
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
    \x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x18\x163\n\x08\n\x01\x08\x12\
    \x03\x19\0?\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x19\0?\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x19\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x19\x1e>\n\x08\n\x01\x08\
    \x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x1a\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1a\
    \x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1a\x1f#\n\x08\n\x01\
    \x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1b\0,\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\n\r\n\x06\x08\xe7\x07\x03\x02\
    \0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x1b\
    \x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1b'+\n\x08\n\x01\x02\x12\
    \x03\x1c\x08\x15\n+\n\x02\x04\0\x12\x04!\0\"\x01\x1a\x1f*\n\x20\x20Refre\
    sh\x20callqueue\x20request.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08$\n\x1e\n\
    \x02\x04\x01\x12\x04'\0(\x01\x1a\x12*\n\x20void\x20response.\n\n\n\n\x03\
    \x04\x01\x01\x12\x03'\x08%\n@\n\x02\x06\0\x12\x04-\03\x01\x1a4*\n\x20Pro\
    tocol\x20which\x20is\x20used\x20to\x20refresh\x20the\x20callqueue.\n\n\n\
    \n\x03\x06\0\x01\x12\x03-\x08'\n(\n\x04\x06\0\x02\0\x12\x041\x022-\x1a\
    \x1a*\n\x20Refresh\x20the\x20callqueue.\n\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x031\x06\x16\n\x0c\n\x05\x06\0\x02\0\x02\x12\x031\x173\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x032\x0e+\
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
