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
pub struct GenericRefreshRequestProto {
    // message fields
    identifier: ::protobuf::SingularField<::std::string::String>,
    args: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GenericRefreshRequestProto {}

impl GenericRefreshRequestProto {
    pub fn new() -> GenericRefreshRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GenericRefreshRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GenericRefreshRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenericRefreshRequestProto,
        };
        unsafe {
            instance.get(GenericRefreshRequestProto::new)
        }
    }

    // optional string identifier = 1;

    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: ::std::string::String) {
        self.identifier = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut ::std::string::String {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> ::std::string::String {
        self.identifier.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_identifier(&self) -> &str {
        match self.identifier.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_identifier_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.identifier
    }

    fn mut_identifier_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.identifier
    }

    // repeated string args = 2;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_args(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.args, ::protobuf::RepeatedField::new())
    }

    pub fn get_args(&self) -> &[::std::string::String] {
        &self.args
    }

    fn get_args_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.args
    }

    fn mut_args_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }
}

impl ::protobuf::Message for GenericRefreshRequestProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.identifier)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.args)?;
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
        if let Some(ref v) = self.identifier.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.args {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.identifier.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.args {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for GenericRefreshRequestProto {
    fn new() -> GenericRefreshRequestProto {
        GenericRefreshRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GenericRefreshRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identifier",
                    GenericRefreshRequestProto::get_identifier_for_reflect,
                    GenericRefreshRequestProto::mut_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "args",
                    GenericRefreshRequestProto::get_args_for_reflect,
                    GenericRefreshRequestProto::mut_args_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenericRefreshRequestProto>(
                    "GenericRefreshRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GenericRefreshRequestProto {
    fn clear(&mut self) {
        self.clear_identifier();
        self.clear_args();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenericRefreshRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenericRefreshRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenericRefreshResponseProto {
    // message fields
    exitStatus: ::std::option::Option<i32>,
    userMessage: ::protobuf::SingularField<::std::string::String>,
    senderName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GenericRefreshResponseProto {}

impl GenericRefreshResponseProto {
    pub fn new() -> GenericRefreshResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GenericRefreshResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GenericRefreshResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenericRefreshResponseProto,
        };
        unsafe {
            instance.get(GenericRefreshResponseProto::new)
        }
    }

    // optional int32 exitStatus = 1;

    pub fn clear_exitStatus(&mut self) {
        self.exitStatus = ::std::option::Option::None;
    }

    pub fn has_exitStatus(&self) -> bool {
        self.exitStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exitStatus(&mut self, v: i32) {
        self.exitStatus = ::std::option::Option::Some(v);
    }

    pub fn get_exitStatus(&self) -> i32 {
        self.exitStatus.unwrap_or(0)
    }

    fn get_exitStatus_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.exitStatus
    }

    fn mut_exitStatus_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.exitStatus
    }

    // optional string userMessage = 2;

    pub fn clear_userMessage(&mut self) {
        self.userMessage.clear();
    }

    pub fn has_userMessage(&self) -> bool {
        self.userMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userMessage(&mut self, v: ::std::string::String) {
        self.userMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userMessage(&mut self) -> &mut ::std::string::String {
        if self.userMessage.is_none() {
            self.userMessage.set_default();
        }
        self.userMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_userMessage(&mut self) -> ::std::string::String {
        self.userMessage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_userMessage(&self) -> &str {
        match self.userMessage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_userMessage_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.userMessage
    }

    fn mut_userMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.userMessage
    }

    // optional string senderName = 3;

    pub fn clear_senderName(&mut self) {
        self.senderName.clear();
    }

    pub fn has_senderName(&self) -> bool {
        self.senderName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_senderName(&mut self, v: ::std::string::String) {
        self.senderName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_senderName(&mut self) -> &mut ::std::string::String {
        if self.senderName.is_none() {
            self.senderName.set_default();
        }
        self.senderName.as_mut().unwrap()
    }

    // Take field
    pub fn take_senderName(&mut self) -> ::std::string::String {
        self.senderName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_senderName(&self) -> &str {
        match self.senderName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_senderName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.senderName
    }

    fn mut_senderName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.senderName
    }
}

impl ::protobuf::Message for GenericRefreshResponseProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.exitStatus = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.userMessage)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.senderName)?;
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
        if let Some(v) = self.exitStatus {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.userMessage.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.senderName.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.exitStatus {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.userMessage.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.senderName.as_ref() {
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

impl ::protobuf::MessageStatic for GenericRefreshResponseProto {
    fn new() -> GenericRefreshResponseProto {
        GenericRefreshResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GenericRefreshResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "exitStatus",
                    GenericRefreshResponseProto::get_exitStatus_for_reflect,
                    GenericRefreshResponseProto::mut_exitStatus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userMessage",
                    GenericRefreshResponseProto::get_userMessage_for_reflect,
                    GenericRefreshResponseProto::mut_userMessage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "senderName",
                    GenericRefreshResponseProto::get_senderName_for_reflect,
                    GenericRefreshResponseProto::mut_senderName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenericRefreshResponseProto>(
                    "GenericRefreshResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GenericRefreshResponseProto {
    fn clear(&mut self) {
        self.clear_exitStatus();
        self.clear_userMessage();
        self.clear_senderName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenericRefreshResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenericRefreshResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenericRefreshResponseCollectionProto {
    // message fields
    responses: ::protobuf::RepeatedField<GenericRefreshResponseProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GenericRefreshResponseCollectionProto {}

impl GenericRefreshResponseCollectionProto {
    pub fn new() -> GenericRefreshResponseCollectionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GenericRefreshResponseCollectionProto {
        static mut instance: ::protobuf::lazy::Lazy<GenericRefreshResponseCollectionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenericRefreshResponseCollectionProto,
        };
        unsafe {
            instance.get(GenericRefreshResponseCollectionProto::new)
        }
    }

    // repeated .hadoop.common.GenericRefreshResponseProto responses = 1;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<GenericRefreshResponseProto>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<GenericRefreshResponseProto> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<GenericRefreshResponseProto> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[GenericRefreshResponseProto] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<GenericRefreshResponseProto> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GenericRefreshResponseProto> {
        &mut self.responses
    }
}

impl ::protobuf::Message for GenericRefreshResponseCollectionProto {
    fn is_initialized(&self) -> bool {
        for v in &self.responses {
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
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
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
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.responses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for GenericRefreshResponseCollectionProto {
    fn new() -> GenericRefreshResponseCollectionProto {
        GenericRefreshResponseCollectionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GenericRefreshResponseCollectionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GenericRefreshResponseProto>>(
                    "responses",
                    GenericRefreshResponseCollectionProto::get_responses_for_reflect,
                    GenericRefreshResponseCollectionProto::mut_responses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenericRefreshResponseCollectionProto>(
                    "GenericRefreshResponseCollectionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GenericRefreshResponseCollectionProto {
    fn clear(&mut self) {
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenericRefreshResponseCollectionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenericRefreshResponseCollectionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cGenericRefreshProtocol.proto\x12\rhadoop.common\"P\n\x1aGenericRef\
    reshRequestProto\x12\x1e\n\nidentifier\x18\x01\x20\x01(\tR\nidentifier\
    \x12\x12\n\x04args\x18\x02\x20\x03(\tR\x04args\"\x7f\n\x1bGenericRefresh\
    ResponseProto\x12\x1e\n\nexitStatus\x18\x01\x20\x01(\x05R\nexitStatus\
    \x12\x20\n\x0buserMessage\x18\x02\x20\x01(\tR\x0buserMessage\x12\x1e\n\n\
    senderName\x18\x03\x20\x01(\tR\nsenderName\"q\n%GenericRefreshResponseCo\
    llectionProto\x12H\n\tresponses\x18\x01\x20\x03(\x0b2*.hadoop.common.Gen\
    ericRefreshResponseProtoR\tresponses2\x8b\x01\n\x1dGenericRefreshProtoco\
    lService\x12j\n\x07refresh\x12).hadoop.common.GenericRefreshRequestProto\
    \x1a4.hadoop.common.GenericRefreshResponseCollectionProtoBA\n\x1borg.apa\
    che.hadoop.ipc.protoB\x1cGenericRefreshProtocolProtos\xa0\x01\x01\x88\
    \x01\x01J\xe4\x10\n\x06\x12\x04\x18\0<\x01\n\x08\n\x01\x08\x12\x03\x18\0\
    4\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\042\x83\x06*\n\x20Licensed\
    \x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\x20under\x20o\
    ne\n\x20or\x20more\x20contributor\x20license\x20agreements.\x20\x20See\
    \x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\x20work\x20fo\
    r\x20additional\x20information\n\x20regarding\x20copyright\x20ownership.\
    \x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20you\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\n\x20\"License\"\
    );\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compli\
    ance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20obtain\x20a\x20co\
    py\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apach\
    e.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\
    \x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distrib\
    uted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\
    \x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n2\xaa\x01*\n\
    \x20These\x20.proto\x20interfaces\x20are\x20private\x20and\x20stable.\n\
    \x20Please\x20see\x20http://wiki.apache.org/hadoop/Compatibility\n\x20fo\
    r\x20what\x20changes\x20are\x20allowed\x20for\x20a\x20*stable*\x20.proto\
    \x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x18\x07\x13\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x18\
    \x163\n\x08\n\x01\x08\x12\x03\x19\0=\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x19\0=\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x19\x07\x1b\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\
    \x19\x1e<\n\x08\n\x01\x08\x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\
    \x03\x1a\0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1a\x07\x1c\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\xe7\x07\
    \x02\x02\0\x01\x12\x03\x1a\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\
    \x03\x1a\x1f#\n\x08\n\x01\x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\x07\x03\
    \x12\x03\x1b\0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\n\r\n\
    \x06\x08\xe7\x07\x03\x02\0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\x07\x03\
    \x02\0\x01\x12\x03\x1b\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1b'\
    +\n\x08\n\x01\x02\x12\x03\x1c\x08\x15\n!\n\x02\x04\0\x12\x04!\0$\x01\x1a\
    \x15*\n\x20\x20Refresh\x20request.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08\"\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\"\x04#\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\"\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\"\r\x13\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\"\x14\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\"\
    !\"\n\x0b\n\x04\x04\0\x02\x01\x12\x03#\x04\x1d\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03#\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03#\r\x13\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03#\x14\x18\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03#\x1b\x1c\n9\n\x02\x04\x01\x12\x04)\0-\x01\x1a-*\n\x20A\x20singl\
    e\x20response\x20from\x20a\x20refresh\x20handler.\n\n\n\n\x03\x04\x01\
    \x01\x12\x03)\x08#\n)\n\x04\x04\x01\x02\0\x12\x03*\x04\"\"\x1c\x20unix\
    \x20exit\x20status\x20to\x20return\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03*\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03*\r\x12\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03*\x13\x1d\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03*\x20!\n*\n\x04\x04\x01\x02\x01\x12\x03+\x04$\"\x1d\x20to\x20be\x20d\
    isplayed\x20to\x20the\x20user\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03+\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03+\r\x13\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03+\x14\x1f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03+\"#\n.\n\x04\x04\x01\x02\x02\x12\x03,\x04#\"!\x20which\x20handler\
    \x20sent\x20this\x20message\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03,\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03,\r\x13\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03,\x14\x1e\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03,!\"\nC\n\x02\x04\x02\x12\x042\04\x01\x1a7*\n\x20Collection\x20of\
    \x20responses\x20from\x20zero\x20or\x20more\x20handlers.\n\n\n\n\x03\x04\
    \x02\x01\x12\x032\x08-\n\x0b\n\x04\x04\x02\x02\0\x12\x033\x047\n\x0c\n\
    \x05\x04\x02\x02\0\x04\x12\x033\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x06\
    \x12\x033\r(\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x033)2\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03356\nK\n\x02\x06\0\x12\x049\0<\x01\x1a?*\n\x20Prot\
    ocol\x20which\x20is\x20used\x20to\x20refresh\x20a\x20user-specified\x20f\
    eature.\n\n\n\n\x03\x06\0\x01\x12\x039\x08%\n\x0c\n\x04\x06\0\x02\0\x12\
    \x04:\x02;5\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03:\x06\r\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03:\x0e(\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03;\x0e3\
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
