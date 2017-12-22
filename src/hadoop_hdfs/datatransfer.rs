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

use hadoop_common::security;

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct DataTransferEncryptorMessageProto {
    // message fields
    status: ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    message: ::protobuf::SingularField<::std::string::String>,
    cipherOption: ::protobuf::RepeatedField<super::hdfs::CipherOptionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataTransferEncryptorMessageProto {}

impl DataTransferEncryptorMessageProto {
    pub fn new() -> DataTransferEncryptorMessageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferEncryptorMessageProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferEncryptorMessageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferEncryptorMessageProto,
        };
        unsafe {
            instance.get(DataTransferEncryptorMessageProto::new)
        }
    }

    // required .hadoop.hdfs.DataTransferEncryptorMessageProto.DataTransferEncryptorStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: DataTransferEncryptorMessageProto_DataTransferEncryptorStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
        self.status.unwrap_or(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        &mut self.status
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // repeated .hadoop.hdfs.CipherOptionProto cipherOption = 4;

    pub fn clear_cipherOption(&mut self) {
        self.cipherOption.clear();
    }

    // Param is passed by value, moved
    pub fn set_cipherOption(&mut self, v: ::protobuf::RepeatedField<super::hdfs::CipherOptionProto>) {
        self.cipherOption = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cipherOption(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &mut self.cipherOption
    }

    // Take field
    pub fn take_cipherOption(&mut self) -> ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        ::std::mem::replace(&mut self.cipherOption, ::protobuf::RepeatedField::new())
    }

    pub fn get_cipherOption(&self) -> &[super::hdfs::CipherOptionProto] {
        &self.cipherOption
    }

    fn get_cipherOption_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &self.cipherOption
    }

    fn mut_cipherOption_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &mut self.cipherOption
    }
}

impl ::protobuf::Message for DataTransferEncryptorMessageProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.cipherOption {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cipherOption)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.cipherOption {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.cipherOption {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DataTransferEncryptorMessageProto {
    fn new() -> DataTransferEncryptorMessageProto {
        DataTransferEncryptorMessageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferEncryptorMessageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>>(
                    "status",
                    DataTransferEncryptorMessageProto::get_status_for_reflect,
                    DataTransferEncryptorMessageProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    DataTransferEncryptorMessageProto::get_payload_for_reflect,
                    DataTransferEncryptorMessageProto::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    DataTransferEncryptorMessageProto::get_message_for_reflect,
                    DataTransferEncryptorMessageProto::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::CipherOptionProto>>(
                    "cipherOption",
                    DataTransferEncryptorMessageProto::get_cipherOption_for_reflect,
                    DataTransferEncryptorMessageProto::mut_cipherOption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferEncryptorMessageProto>(
                    "DataTransferEncryptorMessageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferEncryptorMessageProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_payload();
        self.clear_message();
        self.clear_cipherOption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataTransferEncryptorMessageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataTransferEncryptorMessageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    SUCCESS = 0,
    ERROR_UNKNOWN_KEY = 1,
    ERROR = 2,
}

impl ::protobuf::ProtobufEnum for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        match value {
            0 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS),
            1 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR_UNKNOWN_KEY),
            2 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataTransferEncryptorMessageProto_DataTransferEncryptorStatus] = &[
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS,
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR_UNKNOWN_KEY,
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataTransferEncryptorMessageProto_DataTransferEncryptorStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
}

impl ::protobuf::reflect::ProtobufValue for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BaseHeaderProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<security::TokenProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BaseHeaderProto {}

impl BaseHeaderProto {
    pub fn new() -> BaseHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BaseHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<BaseHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BaseHeaderProto,
        };
        unsafe {
            instance.get(BaseHeaderProto::new)
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::hdfs::ExtendedBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::hdfs::ExtendedBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::hdfs::ExtendedBlockProto {
        self.block.take().unwrap_or_else(|| super::hdfs::ExtendedBlockProto::new())
    }

    pub fn get_block(&self) -> &super::hdfs::ExtendedBlockProto {
        self.block.as_ref().unwrap_or_else(|| super::hdfs::ExtendedBlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto> {
        &mut self.block
    }

    // optional .hadoop.common.TokenProto token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: security::TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut security::TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> security::TokenProto {
        self.token.take().unwrap_or_else(|| security::TokenProto::new())
    }

    pub fn get_token(&self) -> &security::TokenProto {
        self.token.as_ref().unwrap_or_else(|| security::TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<security::TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<security::TokenProto> {
        &mut self.token
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 3;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for BaseHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.token {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.traceInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.traceInfo)?;
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for BaseHeaderProto {
    fn new() -> BaseHeaderProto {
        BaseHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BaseHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    BaseHeaderProto::get_block_for_reflect,
                    BaseHeaderProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<security::TokenProto>>(
                    "token",
                    BaseHeaderProto::get_token_for_reflect,
                    BaseHeaderProto::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    BaseHeaderProto::get_traceInfo_for_reflect,
                    BaseHeaderProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BaseHeaderProto>(
                    "BaseHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BaseHeaderProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BaseHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BaseHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataTransferTraceInfoProto {
    // message fields
    traceId: ::std::option::Option<u64>,
    parentId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataTransferTraceInfoProto {}

impl DataTransferTraceInfoProto {
    pub fn new() -> DataTransferTraceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferTraceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferTraceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferTraceInfoProto,
        };
        unsafe {
            instance.get(DataTransferTraceInfoProto::new)
        }
    }

    // required uint64 traceId = 1;

    pub fn clear_traceId(&mut self) {
        self.traceId = ::std::option::Option::None;
    }

    pub fn has_traceId(&self) -> bool {
        self.traceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceId(&mut self, v: u64) {
        self.traceId = ::std::option::Option::Some(v);
    }

    pub fn get_traceId(&self) -> u64 {
        self.traceId.unwrap_or(0)
    }

    fn get_traceId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.traceId
    }

    fn mut_traceId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.traceId
    }

    // required uint64 parentId = 2;

    pub fn clear_parentId(&mut self) {
        self.parentId = ::std::option::Option::None;
    }

    pub fn has_parentId(&self) -> bool {
        self.parentId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parentId(&mut self, v: u64) {
        self.parentId = ::std::option::Option::Some(v);
    }

    pub fn get_parentId(&self) -> u64 {
        self.parentId.unwrap_or(0)
    }

    fn get_parentId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.parentId
    }

    fn mut_parentId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.parentId
    }
}

impl ::protobuf::Message for DataTransferTraceInfoProto {
    fn is_initialized(&self) -> bool {
        if self.traceId.is_none() {
            return false;
        }
        if self.parentId.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.traceId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.parentId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.traceId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.parentId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.traceId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.parentId {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for DataTransferTraceInfoProto {
    fn new() -> DataTransferTraceInfoProto {
        DataTransferTraceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferTraceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "traceId",
                    DataTransferTraceInfoProto::get_traceId_for_reflect,
                    DataTransferTraceInfoProto::mut_traceId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parentId",
                    DataTransferTraceInfoProto::get_parentId_for_reflect,
                    DataTransferTraceInfoProto::mut_parentId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferTraceInfoProto>(
                    "DataTransferTraceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferTraceInfoProto {
    fn clear(&mut self) {
        self.clear_traceId();
        self.clear_parentId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataTransferTraceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataTransferTraceInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientOperationHeaderProto {
    // message fields
    baseHeader: ::protobuf::SingularPtrField<BaseHeaderProto>,
    clientName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientOperationHeaderProto {}

impl ClientOperationHeaderProto {
    pub fn new() -> ClientOperationHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientOperationHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientOperationHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientOperationHeaderProto,
        };
        unsafe {
            instance.get(ClientOperationHeaderProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto baseHeader = 1;

    pub fn clear_baseHeader(&mut self) {
        self.baseHeader.clear();
    }

    pub fn has_baseHeader(&self) -> bool {
        self.baseHeader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseHeader(&mut self, v: BaseHeaderProto) {
        self.baseHeader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseHeader(&mut self) -> &mut BaseHeaderProto {
        if self.baseHeader.is_none() {
            self.baseHeader.set_default();
        }
        self.baseHeader.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseHeader(&mut self) -> BaseHeaderProto {
        self.baseHeader.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_baseHeader(&self) -> &BaseHeaderProto {
        self.baseHeader.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_baseHeader_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.baseHeader
    }

    fn mut_baseHeader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.baseHeader
    }

    // required string clientName = 2;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName(&mut self) -> &mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        }
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName(&self) -> &str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientName
    }

    fn mut_clientName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientName
    }
}

impl ::protobuf::Message for ClientOperationHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.baseHeader.is_none() {
            return false;
        }
        if self.clientName.is_none() {
            return false;
        }
        for v in &self.baseHeader {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.baseHeader)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientName)?;
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
        if let Some(ref v) = self.baseHeader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.clientName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.baseHeader.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.clientName.as_ref() {
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

impl ::protobuf::MessageStatic for ClientOperationHeaderProto {
    fn new() -> ClientOperationHeaderProto {
        ClientOperationHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientOperationHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "baseHeader",
                    ClientOperationHeaderProto::get_baseHeader_for_reflect,
                    ClientOperationHeaderProto::mut_baseHeader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientName",
                    ClientOperationHeaderProto::get_clientName_for_reflect,
                    ClientOperationHeaderProto::mut_clientName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientOperationHeaderProto>(
                    "ClientOperationHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientOperationHeaderProto {
    fn clear(&mut self) {
        self.clear_baseHeader();
        self.clear_clientName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientOperationHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientOperationHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CachingStrategyProto {
    // message fields
    dropBehind: ::std::option::Option<bool>,
    readahead: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CachingStrategyProto {}

impl CachingStrategyProto {
    pub fn new() -> CachingStrategyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CachingStrategyProto {
        static mut instance: ::protobuf::lazy::Lazy<CachingStrategyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CachingStrategyProto,
        };
        unsafe {
            instance.get(CachingStrategyProto::new)
        }
    }

    // optional bool dropBehind = 1;

    pub fn clear_dropBehind(&mut self) {
        self.dropBehind = ::std::option::Option::None;
    }

    pub fn has_dropBehind(&self) -> bool {
        self.dropBehind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dropBehind(&mut self, v: bool) {
        self.dropBehind = ::std::option::Option::Some(v);
    }

    pub fn get_dropBehind(&self) -> bool {
        self.dropBehind.unwrap_or(false)
    }

    fn get_dropBehind_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.dropBehind
    }

    fn mut_dropBehind_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.dropBehind
    }

    // optional int64 readahead = 2;

    pub fn clear_readahead(&mut self) {
        self.readahead = ::std::option::Option::None;
    }

    pub fn has_readahead(&self) -> bool {
        self.readahead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readahead(&mut self, v: i64) {
        self.readahead = ::std::option::Option::Some(v);
    }

    pub fn get_readahead(&self) -> i64 {
        self.readahead.unwrap_or(0)
    }

    fn get_readahead_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.readahead
    }

    fn mut_readahead_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.readahead
    }
}

impl ::protobuf::Message for CachingStrategyProto {
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
                    let tmp = is.read_bool()?;
                    self.dropBehind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.readahead = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dropBehind {
            my_size += 2;
        }
        if let Some(v) = self.readahead {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dropBehind {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.readahead {
            os.write_int64(2, v)?;
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

impl ::protobuf::MessageStatic for CachingStrategyProto {
    fn new() -> CachingStrategyProto {
        CachingStrategyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CachingStrategyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "dropBehind",
                    CachingStrategyProto::get_dropBehind_for_reflect,
                    CachingStrategyProto::mut_dropBehind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "readahead",
                    CachingStrategyProto::get_readahead_for_reflect,
                    CachingStrategyProto::mut_readahead_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CachingStrategyProto>(
                    "CachingStrategyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CachingStrategyProto {
    fn clear(&mut self) {
        self.clear_dropBehind();
        self.clear_readahead();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CachingStrategyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CachingStrategyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpReadBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    offset: ::std::option::Option<u64>,
    len: ::std::option::Option<u64>,
    sendChecksums: ::std::option::Option<bool>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpReadBlockProto {}

impl OpReadBlockProto {
    pub fn new() -> OpReadBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReadBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReadBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReadBlockProto,
        };
        unsafe {
            instance.get(OpReadBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
    }

    // required uint64 offset = 2;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    fn get_offset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.offset
    }

    // required uint64 len = 3;

    pub fn clear_len(&mut self) {
        self.len = ::std::option::Option::None;
    }

    pub fn has_len(&self) -> bool {
        self.len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_len(&mut self, v: u64) {
        self.len = ::std::option::Option::Some(v);
    }

    pub fn get_len(&self) -> u64 {
        self.len.unwrap_or(0)
    }

    fn get_len_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.len
    }

    fn mut_len_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.len
    }

    // optional bool sendChecksums = 4;

    pub fn clear_sendChecksums(&mut self) {
        self.sendChecksums = ::std::option::Option::None;
    }

    pub fn has_sendChecksums(&self) -> bool {
        self.sendChecksums.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sendChecksums(&mut self, v: bool) {
        self.sendChecksums = ::std::option::Option::Some(v);
    }

    pub fn get_sendChecksums(&self) -> bool {
        self.sendChecksums.unwrap_or(true)
    }

    fn get_sendChecksums_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sendChecksums
    }

    fn mut_sendChecksums_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sendChecksums
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 5;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy(&mut self) -> &mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        }
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy(&self) -> &CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }

    fn get_cachingStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<CachingStrategyProto> {
        &self.cachingStrategy
    }

    fn mut_cachingStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CachingStrategyProto> {
        &mut self.cachingStrategy
    }
}

impl ::protobuf::Message for OpReadBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.offset.is_none() {
            return false;
        }
        if self.len.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cachingStrategy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.len = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.sendChecksums = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cachingStrategy)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.len {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sendChecksums {
            my_size += 2;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.offset {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.len {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.sendChecksums {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for OpReadBlockProto {
    fn new() -> OpReadBlockProto {
        OpReadBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReadBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpReadBlockProto::get_header_for_reflect,
                    OpReadBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    OpReadBlockProto::get_offset_for_reflect,
                    OpReadBlockProto::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "len",
                    OpReadBlockProto::get_len_for_reflect,
                    OpReadBlockProto::mut_len_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sendChecksums",
                    OpReadBlockProto::get_sendChecksums_for_reflect,
                    OpReadBlockProto::mut_sendChecksums_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CachingStrategyProto>>(
                    "cachingStrategy",
                    OpReadBlockProto::get_cachingStrategy_for_reflect,
                    OpReadBlockProto::mut_cachingStrategy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReadBlockProto>(
                    "OpReadBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReadBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_offset();
        self.clear_len();
        self.clear_sendChecksums();
        self.clear_cachingStrategy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpReadBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpReadBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChecksumProto {
    // message fields
    field_type: ::std::option::Option<super::hdfs::ChecksumTypeProto>,
    bytesPerChecksum: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChecksumProto {}

impl ChecksumProto {
    pub fn new() -> ChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumProto,
        };
        unsafe {
            instance.get(ChecksumProto::new)
        }
    }

    // required .hadoop.hdfs.ChecksumTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::hdfs::ChecksumTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> super::hdfs::ChecksumTypeProto {
        self.field_type.unwrap_or(super::hdfs::ChecksumTypeProto::CHECKSUM_NULL)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &mut self.field_type
    }

    // required uint32 bytesPerChecksum = 2;

    pub fn clear_bytesPerChecksum(&mut self) {
        self.bytesPerChecksum = ::std::option::Option::None;
    }

    pub fn has_bytesPerChecksum(&self) -> bool {
        self.bytesPerChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerChecksum(&mut self, v: u32) {
        self.bytesPerChecksum = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerChecksum(&self) -> u32 {
        self.bytesPerChecksum.unwrap_or(0)
    }

    fn get_bytesPerChecksum_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bytesPerChecksum
    }

    fn mut_bytesPerChecksum_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bytesPerChecksum
    }
}

impl ::protobuf::Message for ChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.bytesPerChecksum.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.bytesPerChecksum {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.bytesPerChecksum {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for ChecksumProto {
    fn new() -> ChecksumProto {
        ChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::ChecksumTypeProto>>(
                    "type",
                    ChecksumProto::get_field_type_for_reflect,
                    ChecksumProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerChecksum",
                    ChecksumProto::get_bytesPerChecksum_for_reflect,
                    ChecksumProto::mut_bytesPerChecksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumProto>(
                    "ChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChecksumProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_bytesPerChecksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpWriteBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>,
    source: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto>,
    stage: ::std::option::Option<OpWriteBlockProto_BlockConstructionStage>,
    pipelineSize: ::std::option::Option<u32>,
    minBytesRcvd: ::std::option::Option<u64>,
    maxBytesRcvd: ::std::option::Option<u64>,
    latestGenerationStamp: ::std::option::Option<u64>,
    requestedChecksum: ::protobuf::SingularPtrField<ChecksumProto>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    storageType: ::std::option::Option<super::hdfs::StorageTypeProto>,
    targetStorageTypes: ::std::vec::Vec<super::hdfs::StorageTypeProto>,
    allowLazyPersist: ::std::option::Option<bool>,
    pinning: ::std::option::Option<bool>,
    targetPinnings: ::std::vec::Vec<bool>,
    storageId: ::protobuf::SingularField<::std::string::String>,
    targetStorageIds: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpWriteBlockProto {}

impl OpWriteBlockProto {
    pub fn new() -> OpWriteBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpWriteBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpWriteBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpWriteBlockProto,
        };
        unsafe {
            instance.get(OpWriteBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets(&self) -> &[super::hdfs::DatanodeInfoProto] {
        &self.targets
    }

    fn get_targets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &self.targets
    }

    fn mut_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // optional .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: super::hdfs::DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut super::hdfs::DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> super::hdfs::DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::new())
    }

    pub fn get_source(&self) -> &super::hdfs::DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::default_instance())
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &mut self.source
    }

    // required .hadoop.hdfs.OpWriteBlockProto.BlockConstructionStage stage = 4;

    pub fn clear_stage(&mut self) {
        self.stage = ::std::option::Option::None;
    }

    pub fn has_stage(&self) -> bool {
        self.stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: OpWriteBlockProto_BlockConstructionStage) {
        self.stage = ::std::option::Option::Some(v);
    }

    pub fn get_stage(&self) -> OpWriteBlockProto_BlockConstructionStage {
        self.stage.unwrap_or(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND)
    }

    fn get_stage_for_reflect(&self) -> &::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        &self.stage
    }

    fn mut_stage_for_reflect(&mut self) -> &mut ::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        &mut self.stage
    }

    // required uint32 pipelineSize = 5;

    pub fn clear_pipelineSize(&mut self) {
        self.pipelineSize = ::std::option::Option::None;
    }

    pub fn has_pipelineSize(&self) -> bool {
        self.pipelineSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pipelineSize(&mut self, v: u32) {
        self.pipelineSize = ::std::option::Option::Some(v);
    }

    pub fn get_pipelineSize(&self) -> u32 {
        self.pipelineSize.unwrap_or(0)
    }

    fn get_pipelineSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pipelineSize
    }

    fn mut_pipelineSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pipelineSize
    }

    // required uint64 minBytesRcvd = 6;

    pub fn clear_minBytesRcvd(&mut self) {
        self.minBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_minBytesRcvd(&self) -> bool {
        self.minBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minBytesRcvd(&mut self, v: u64) {
        self.minBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_minBytesRcvd(&self) -> u64 {
        self.minBytesRcvd.unwrap_or(0)
    }

    fn get_minBytesRcvd_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.minBytesRcvd
    }

    fn mut_minBytesRcvd_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.minBytesRcvd
    }

    // required uint64 maxBytesRcvd = 7;

    pub fn clear_maxBytesRcvd(&mut self) {
        self.maxBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_maxBytesRcvd(&self) -> bool {
        self.maxBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxBytesRcvd(&mut self, v: u64) {
        self.maxBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_maxBytesRcvd(&self) -> u64 {
        self.maxBytesRcvd.unwrap_or(0)
    }

    fn get_maxBytesRcvd_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.maxBytesRcvd
    }

    fn mut_maxBytesRcvd_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.maxBytesRcvd
    }

    // required uint64 latestGenerationStamp = 8;

    pub fn clear_latestGenerationStamp(&mut self) {
        self.latestGenerationStamp = ::std::option::Option::None;
    }

    pub fn has_latestGenerationStamp(&self) -> bool {
        self.latestGenerationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestGenerationStamp(&mut self, v: u64) {
        self.latestGenerationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_latestGenerationStamp(&self) -> u64 {
        self.latestGenerationStamp.unwrap_or(0)
    }

    fn get_latestGenerationStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.latestGenerationStamp
    }

    fn mut_latestGenerationStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.latestGenerationStamp
    }

    // required .hadoop.hdfs.ChecksumProto requestedChecksum = 9;

    pub fn clear_requestedChecksum(&mut self) {
        self.requestedChecksum.clear();
    }

    pub fn has_requestedChecksum(&self) -> bool {
        self.requestedChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestedChecksum(&mut self, v: ChecksumProto) {
        self.requestedChecksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_requestedChecksum(&mut self) -> &mut ChecksumProto {
        if self.requestedChecksum.is_none() {
            self.requestedChecksum.set_default();
        }
        self.requestedChecksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_requestedChecksum(&mut self) -> ChecksumProto {
        self.requestedChecksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_requestedChecksum(&self) -> &ChecksumProto {
        self.requestedChecksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    fn get_requestedChecksum_for_reflect(&self) -> &::protobuf::SingularPtrField<ChecksumProto> {
        &self.requestedChecksum
    }

    fn mut_requestedChecksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChecksumProto> {
        &mut self.requestedChecksum
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 10;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy(&mut self) -> &mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        }
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy(&self) -> &CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }

    fn get_cachingStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<CachingStrategyProto> {
        &self.cachingStrategy
    }

    fn mut_cachingStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CachingStrategyProto> {
        &mut self.cachingStrategy
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 11;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: super::hdfs::StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> super::hdfs::StorageTypeProto {
        self.storageType.unwrap_or(super::hdfs::StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<super::hdfs::StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::StorageTypeProto> {
        &mut self.storageType
    }

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 12;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<super::hdfs::StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes(&self) -> &[super::hdfs::StorageTypeProto] {
        &self.targetStorageTypes
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // optional bool allowLazyPersist = 13;

    pub fn clear_allowLazyPersist(&mut self) {
        self.allowLazyPersist = ::std::option::Option::None;
    }

    pub fn has_allowLazyPersist(&self) -> bool {
        self.allowLazyPersist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowLazyPersist(&mut self, v: bool) {
        self.allowLazyPersist = ::std::option::Option::Some(v);
    }

    pub fn get_allowLazyPersist(&self) -> bool {
        self.allowLazyPersist.unwrap_or(false)
    }

    fn get_allowLazyPersist_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allowLazyPersist
    }

    fn mut_allowLazyPersist_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allowLazyPersist
    }

    // optional bool pinning = 14;

    pub fn clear_pinning(&mut self) {
        self.pinning = ::std::option::Option::None;
    }

    pub fn has_pinning(&self) -> bool {
        self.pinning.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pinning(&mut self, v: bool) {
        self.pinning = ::std::option::Option::Some(v);
    }

    pub fn get_pinning(&self) -> bool {
        self.pinning.unwrap_or(false)
    }

    fn get_pinning_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pinning
    }

    fn mut_pinning_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pinning
    }

    // repeated bool targetPinnings = 15;

    pub fn clear_targetPinnings(&mut self) {
        self.targetPinnings.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetPinnings(&mut self, v: ::std::vec::Vec<bool>) {
        self.targetPinnings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetPinnings(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.targetPinnings
    }

    // Take field
    pub fn take_targetPinnings(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.targetPinnings, ::std::vec::Vec::new())
    }

    pub fn get_targetPinnings(&self) -> &[bool] {
        &self.targetPinnings
    }

    fn get_targetPinnings_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.targetPinnings
    }

    fn mut_targetPinnings_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.targetPinnings
    }

    // optional string storageId = 16;

    pub fn clear_storageId(&mut self) {
        self.storageId.clear();
    }

    pub fn has_storageId(&self) -> bool {
        self.storageId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageId(&mut self, v: ::std::string::String) {
        self.storageId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageId(&mut self) -> &mut ::std::string::String {
        if self.storageId.is_none() {
            self.storageId.set_default();
        }
        self.storageId.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageId(&mut self) -> ::std::string::String {
        self.storageId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageId(&self) -> &str {
        match self.storageId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageId
    }

    fn mut_storageId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageId
    }

    // repeated string targetStorageIds = 17;

    pub fn clear_targetStorageIds(&mut self) {
        self.targetStorageIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageIds(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.targetStorageIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageIds(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.targetStorageIds
    }

    // Take field
    pub fn take_targetStorageIds(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.targetStorageIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_targetStorageIds(&self) -> &[::std::string::String] {
        &self.targetStorageIds
    }

    fn get_targetStorageIds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.targetStorageIds
    }

    fn mut_targetStorageIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.targetStorageIds
    }
}

impl ::protobuf::Message for OpWriteBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.stage.is_none() {
            return false;
        }
        if self.pipelineSize.is_none() {
            return false;
        }
        if self.minBytesRcvd.is_none() {
            return false;
        }
        if self.maxBytesRcvd.is_none() {
            return false;
        }
        if self.latestGenerationStamp.is_none() {
            return false;
        }
        if self.requestedChecksum.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targets {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.requestedChecksum {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cachingStrategy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.stage = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.pipelineSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.minBytesRcvd = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.maxBytesRcvd = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.latestGenerationStamp = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.requestedChecksum)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cachingStrategy)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allowLazyPersist = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.pinning = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.targetPinnings)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageId)?;
                },
                17 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.targetStorageIds)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.stage {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(v) = self.pipelineSize {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.minBytesRcvd {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.maxBytesRcvd {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.latestGenerationStamp {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.requestedChecksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(11, v);
        }
        for value in &self.targetStorageTypes {
            my_size += ::protobuf::rt::enum_size(12, *value);
        };
        if let Some(v) = self.allowLazyPersist {
            my_size += 2;
        }
        if let Some(v) = self.pinning {
            my_size += 2;
        }
        my_size += 2 * self.targetPinnings.len() as u32;
        if let Some(ref v) = self.storageId.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        for value in &self.targetStorageIds {
            my_size += ::protobuf::rt::string_size(17, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.targets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.source.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.stage {
            os.write_enum(4, v.value())?;
        }
        if let Some(v) = self.pipelineSize {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.minBytesRcvd {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.maxBytesRcvd {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.latestGenerationStamp {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.requestedChecksum.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(11, v.value())?;
        }
        for v in &self.targetStorageTypes {
            os.write_enum(12, v.value())?;
        };
        if let Some(v) = self.allowLazyPersist {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.pinning {
            os.write_bool(14, v)?;
        }
        for v in &self.targetPinnings {
            os.write_bool(15, *v)?;
        };
        if let Some(ref v) = self.storageId.as_ref() {
            os.write_string(16, &v)?;
        }
        for v in &self.targetStorageIds {
            os.write_string(17, &v)?;
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

impl ::protobuf::MessageStatic for OpWriteBlockProto {
    fn new() -> OpWriteBlockProto {
        OpWriteBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpWriteBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpWriteBlockProto::get_header_for_reflect,
                    OpWriteBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "targets",
                    OpWriteBlockProto::get_targets_for_reflect,
                    OpWriteBlockProto::mut_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "source",
                    OpWriteBlockProto::get_source_for_reflect,
                    OpWriteBlockProto::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OpWriteBlockProto_BlockConstructionStage>>(
                    "stage",
                    OpWriteBlockProto::get_stage_for_reflect,
                    OpWriteBlockProto::mut_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pipelineSize",
                    OpWriteBlockProto::get_pipelineSize_for_reflect,
                    OpWriteBlockProto::mut_pipelineSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "minBytesRcvd",
                    OpWriteBlockProto::get_minBytesRcvd_for_reflect,
                    OpWriteBlockProto::mut_minBytesRcvd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "maxBytesRcvd",
                    OpWriteBlockProto::get_maxBytesRcvd_for_reflect,
                    OpWriteBlockProto::mut_maxBytesRcvd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "latestGenerationStamp",
                    OpWriteBlockProto::get_latestGenerationStamp_for_reflect,
                    OpWriteBlockProto::mut_latestGenerationStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChecksumProto>>(
                    "requestedChecksum",
                    OpWriteBlockProto::get_requestedChecksum_for_reflect,
                    OpWriteBlockProto::mut_requestedChecksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CachingStrategyProto>>(
                    "cachingStrategy",
                    OpWriteBlockProto::get_cachingStrategy_for_reflect,
                    OpWriteBlockProto::mut_cachingStrategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "storageType",
                    OpWriteBlockProto::get_storageType_for_reflect,
                    OpWriteBlockProto::mut_storageType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "targetStorageTypes",
                    OpWriteBlockProto::get_targetStorageTypes_for_reflect,
                    OpWriteBlockProto::mut_targetStorageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allowLazyPersist",
                    OpWriteBlockProto::get_allowLazyPersist_for_reflect,
                    OpWriteBlockProto::mut_allowLazyPersist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pinning",
                    OpWriteBlockProto::get_pinning_for_reflect,
                    OpWriteBlockProto::mut_pinning_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "targetPinnings",
                    OpWriteBlockProto::get_targetPinnings_for_reflect,
                    OpWriteBlockProto::mut_targetPinnings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageId",
                    OpWriteBlockProto::get_storageId_for_reflect,
                    OpWriteBlockProto::mut_storageId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "targetStorageIds",
                    OpWriteBlockProto::get_targetStorageIds_for_reflect,
                    OpWriteBlockProto::mut_targetStorageIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpWriteBlockProto>(
                    "OpWriteBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpWriteBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_source();
        self.clear_stage();
        self.clear_pipelineSize();
        self.clear_minBytesRcvd();
        self.clear_maxBytesRcvd();
        self.clear_latestGenerationStamp();
        self.clear_requestedChecksum();
        self.clear_cachingStrategy();
        self.clear_storageType();
        self.clear_targetStorageTypes();
        self.clear_allowLazyPersist();
        self.clear_pinning();
        self.clear_targetPinnings();
        self.clear_storageId();
        self.clear_targetStorageIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpWriteBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpWriteBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OpWriteBlockProto_BlockConstructionStage {
    PIPELINE_SETUP_APPEND = 0,
    PIPELINE_SETUP_APPEND_RECOVERY = 1,
    DATA_STREAMING = 2,
    PIPELINE_SETUP_STREAMING_RECOVERY = 3,
    PIPELINE_CLOSE = 4,
    PIPELINE_CLOSE_RECOVERY = 5,
    PIPELINE_SETUP_CREATE = 6,
    TRANSFER_RBW = 7,
    TRANSFER_FINALIZED = 8,
}

impl ::protobuf::ProtobufEnum for OpWriteBlockProto_BlockConstructionStage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        match value {
            0 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND),
            1 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND_RECOVERY),
            2 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::DATA_STREAMING),
            3 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_STREAMING_RECOVERY),
            4 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE),
            5 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE_RECOVERY),
            6 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE),
            7 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_RBW),
            8 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_FINALIZED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OpWriteBlockProto_BlockConstructionStage] = &[
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::DATA_STREAMING,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_STREAMING_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
            OpWriteBlockProto_BlockConstructionStage::TRANSFER_RBW,
            OpWriteBlockProto_BlockConstructionStage::TRANSFER_FINALIZED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OpWriteBlockProto_BlockConstructionStage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OpWriteBlockProto_BlockConstructionStage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OpWriteBlockProto_BlockConstructionStage {
}

impl ::protobuf::reflect::ProtobufValue for OpWriteBlockProto_BlockConstructionStage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpTransferBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>,
    targetStorageTypes: ::std::vec::Vec<super::hdfs::StorageTypeProto>,
    targetStorageIds: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpTransferBlockProto {}

impl OpTransferBlockProto {
    pub fn new() -> OpTransferBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpTransferBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpTransferBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpTransferBlockProto,
        };
        unsafe {
            instance.get(OpTransferBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets(&self) -> &[super::hdfs::DatanodeInfoProto] {
        &self.targets
    }

    fn get_targets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &self.targets
    }

    fn mut_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 3;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<super::hdfs::StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes(&self) -> &[super::hdfs::StorageTypeProto] {
        &self.targetStorageTypes
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // repeated string targetStorageIds = 4;

    pub fn clear_targetStorageIds(&mut self) {
        self.targetStorageIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageIds(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.targetStorageIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageIds(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.targetStorageIds
    }

    // Take field
    pub fn take_targetStorageIds(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.targetStorageIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_targetStorageIds(&self) -> &[::std::string::String] {
        &self.targetStorageIds
    }

    fn get_targetStorageIds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.targetStorageIds
    }

    fn mut_targetStorageIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.targetStorageIds
    }
}

impl ::protobuf::Message for OpTransferBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targets {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.targetStorageIds)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.targetStorageTypes {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.targetStorageIds {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.targets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.targetStorageTypes {
            os.write_enum(3, v.value())?;
        };
        for v in &self.targetStorageIds {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for OpTransferBlockProto {
    fn new() -> OpTransferBlockProto {
        OpTransferBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpTransferBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpTransferBlockProto::get_header_for_reflect,
                    OpTransferBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "targets",
                    OpTransferBlockProto::get_targets_for_reflect,
                    OpTransferBlockProto::mut_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "targetStorageTypes",
                    OpTransferBlockProto::get_targetStorageTypes_for_reflect,
                    OpTransferBlockProto::mut_targetStorageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "targetStorageIds",
                    OpTransferBlockProto::get_targetStorageIds_for_reflect,
                    OpTransferBlockProto::mut_targetStorageIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpTransferBlockProto>(
                    "OpTransferBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpTransferBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_targetStorageTypes();
        self.clear_targetStorageIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpTransferBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpTransferBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpReplaceBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    delHint: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto>,
    storageType: ::std::option::Option<super::hdfs::StorageTypeProto>,
    storageId: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpReplaceBlockProto {}

impl OpReplaceBlockProto {
    pub fn new() -> OpReplaceBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReplaceBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReplaceBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReplaceBlockProto,
        };
        unsafe {
            instance.get(OpReplaceBlockProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }

    // required string delHint = 2;

    pub fn clear_delHint(&mut self) {
        self.delHint.clear();
    }

    pub fn has_delHint(&self) -> bool {
        self.delHint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delHint(&mut self, v: ::std::string::String) {
        self.delHint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delHint(&mut self) -> &mut ::std::string::String {
        if self.delHint.is_none() {
            self.delHint.set_default();
        }
        self.delHint.as_mut().unwrap()
    }

    // Take field
    pub fn take_delHint(&mut self) -> ::std::string::String {
        self.delHint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_delHint(&self) -> &str {
        match self.delHint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_delHint_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.delHint
    }

    fn mut_delHint_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.delHint
    }

    // required .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: super::hdfs::DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut super::hdfs::DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> super::hdfs::DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::new())
    }

    pub fn get_source(&self) -> &super::hdfs::DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::default_instance())
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &mut self.source
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 4;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: super::hdfs::StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> super::hdfs::StorageTypeProto {
        self.storageType.unwrap_or(super::hdfs::StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<super::hdfs::StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::StorageTypeProto> {
        &mut self.storageType
    }

    // optional string storageId = 5;

    pub fn clear_storageId(&mut self) {
        self.storageId.clear();
    }

    pub fn has_storageId(&self) -> bool {
        self.storageId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageId(&mut self, v: ::std::string::String) {
        self.storageId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageId(&mut self) -> &mut ::std::string::String {
        if self.storageId.is_none() {
            self.storageId.set_default();
        }
        self.storageId.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageId(&mut self) -> ::std::string::String {
        self.storageId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageId(&self) -> &str {
        match self.storageId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageId
    }

    fn mut_storageId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageId
    }
}

impl ::protobuf::Message for OpReplaceBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.delHint.is_none() {
            return false;
        }
        if self.source.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.delHint)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageId)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.delHint.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.storageId.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.delHint.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.source.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.storageId.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for OpReplaceBlockProto {
    fn new() -> OpReplaceBlockProto {
        OpReplaceBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReplaceBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpReplaceBlockProto::get_header_for_reflect,
                    OpReplaceBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "delHint",
                    OpReplaceBlockProto::get_delHint_for_reflect,
                    OpReplaceBlockProto::mut_delHint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "source",
                    OpReplaceBlockProto::get_source_for_reflect,
                    OpReplaceBlockProto::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "storageType",
                    OpReplaceBlockProto::get_storageType_for_reflect,
                    OpReplaceBlockProto::mut_storageType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageId",
                    OpReplaceBlockProto::get_storageId_for_reflect,
                    OpReplaceBlockProto::mut_storageId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReplaceBlockProto>(
                    "OpReplaceBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReplaceBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_delHint();
        self.clear_source();
        self.clear_storageType();
        self.clear_storageId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpReplaceBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpReplaceBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpCopyBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpCopyBlockProto {}

impl OpCopyBlockProto {
    pub fn new() -> OpCopyBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpCopyBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpCopyBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpCopyBlockProto,
        };
        unsafe {
            instance.get(OpCopyBlockProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }
}

impl ::protobuf::Message for OpCopyBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for OpCopyBlockProto {
    fn new() -> OpCopyBlockProto {
        OpCopyBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpCopyBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpCopyBlockProto::get_header_for_reflect,
                    OpCopyBlockProto::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpCopyBlockProto>(
                    "OpCopyBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpCopyBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpCopyBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpCopyBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpBlockChecksumProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpBlockChecksumProto {}

impl OpBlockChecksumProto {
    pub fn new() -> OpBlockChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumProto,
        };
        unsafe {
            instance.get(OpBlockChecksumProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }
}

impl ::protobuf::Message for OpBlockChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for OpBlockChecksumProto {
    fn new() -> OpBlockChecksumProto {
        OpBlockChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpBlockChecksumProto::get_header_for_reflect,
                    OpBlockChecksumProto::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumProto>(
                    "OpBlockChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpBlockChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpBlockChecksumProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpBlockGroupChecksumProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    datanodes: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto>,
    blockTokens: ::protobuf::RepeatedField<security::TokenProto>,
    ecPolicy: ::protobuf::SingularPtrField<super::hdfs::ErasureCodingPolicyProto>,
    blockIndices: ::std::vec::Vec<u32>,
    requestedNumBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpBlockGroupChecksumProto {}

impl OpBlockGroupChecksumProto {
    pub fn new() -> OpBlockGroupChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockGroupChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockGroupChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockGroupChecksumProto,
        };
        unsafe {
            instance.get(OpBlockGroupChecksumProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }

    // required .hadoop.hdfs.DatanodeInfosProto datanodes = 2;

    pub fn clear_datanodes(&mut self) {
        self.datanodes.clear();
    }

    pub fn has_datanodes(&self) -> bool {
        self.datanodes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanodes(&mut self, v: super::hdfs::DatanodeInfosProto) {
        self.datanodes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanodes(&mut self) -> &mut super::hdfs::DatanodeInfosProto {
        if self.datanodes.is_none() {
            self.datanodes.set_default();
        }
        self.datanodes.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanodes(&mut self) -> super::hdfs::DatanodeInfosProto {
        self.datanodes.take().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::new())
    }

    pub fn get_datanodes(&self) -> &super::hdfs::DatanodeInfosProto {
        self.datanodes.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::default_instance())
    }

    fn get_datanodes_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &self.datanodes
    }

    fn mut_datanodes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &mut self.datanodes
    }

    // repeated .hadoop.common.TokenProto blockTokens = 3;

    pub fn clear_blockTokens(&mut self) {
        self.blockTokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockTokens(&mut self, v: ::protobuf::RepeatedField<security::TokenProto>) {
        self.blockTokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blockTokens(&mut self) -> &mut ::protobuf::RepeatedField<security::TokenProto> {
        &mut self.blockTokens
    }

    // Take field
    pub fn take_blockTokens(&mut self) -> ::protobuf::RepeatedField<security::TokenProto> {
        ::std::mem::replace(&mut self.blockTokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_blockTokens(&self) -> &[security::TokenProto] {
        &self.blockTokens
    }

    fn get_blockTokens_for_reflect(&self) -> &::protobuf::RepeatedField<security::TokenProto> {
        &self.blockTokens
    }

    fn mut_blockTokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<security::TokenProto> {
        &mut self.blockTokens
    }

    // required .hadoop.hdfs.ErasureCodingPolicyProto ecPolicy = 4;

    pub fn clear_ecPolicy(&mut self) {
        self.ecPolicy.clear();
    }

    pub fn has_ecPolicy(&self) -> bool {
        self.ecPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicy(&mut self, v: super::hdfs::ErasureCodingPolicyProto) {
        self.ecPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicy(&mut self) -> &mut super::hdfs::ErasureCodingPolicyProto {
        if self.ecPolicy.is_none() {
            self.ecPolicy.set_default();
        }
        self.ecPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicy(&mut self) -> super::hdfs::ErasureCodingPolicyProto {
        self.ecPolicy.take().unwrap_or_else(|| super::hdfs::ErasureCodingPolicyProto::new())
    }

    pub fn get_ecPolicy(&self) -> &super::hdfs::ErasureCodingPolicyProto {
        self.ecPolicy.as_ref().unwrap_or_else(|| super::hdfs::ErasureCodingPolicyProto::default_instance())
    }

    fn get_ecPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::ErasureCodingPolicyProto> {
        &self.ecPolicy
    }

    fn mut_ecPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::ErasureCodingPolicyProto> {
        &mut self.ecPolicy
    }

    // repeated uint32 blockIndices = 5;

    pub fn clear_blockIndices(&mut self) {
        self.blockIndices.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockIndices(&mut self, v: ::std::vec::Vec<u32>) {
        self.blockIndices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blockIndices(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.blockIndices
    }

    // Take field
    pub fn take_blockIndices(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.blockIndices, ::std::vec::Vec::new())
    }

    pub fn get_blockIndices(&self) -> &[u32] {
        &self.blockIndices
    }

    fn get_blockIndices_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.blockIndices
    }

    fn mut_blockIndices_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.blockIndices
    }

    // required uint64 requestedNumBytes = 6;

    pub fn clear_requestedNumBytes(&mut self) {
        self.requestedNumBytes = ::std::option::Option::None;
    }

    pub fn has_requestedNumBytes(&self) -> bool {
        self.requestedNumBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestedNumBytes(&mut self, v: u64) {
        self.requestedNumBytes = ::std::option::Option::Some(v);
    }

    pub fn get_requestedNumBytes(&self) -> u64 {
        self.requestedNumBytes.unwrap_or(0)
    }

    fn get_requestedNumBytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.requestedNumBytes
    }

    fn mut_requestedNumBytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.requestedNumBytes
    }
}

impl ::protobuf::Message for OpBlockGroupChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.datanodes.is_none() {
            return false;
        }
        if self.ecPolicy.is_none() {
            return false;
        }
        if self.requestedNumBytes.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.datanodes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blockTokens {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ecPolicy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.datanodes)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blockTokens)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ecPolicy)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.blockIndices)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.requestedNumBytes = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.datanodes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.blockTokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.ecPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.blockIndices {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.requestedNumBytes {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.datanodes.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.blockTokens {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.ecPolicy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.blockIndices {
            os.write_uint32(5, *v)?;
        };
        if let Some(v) = self.requestedNumBytes {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for OpBlockGroupChecksumProto {
    fn new() -> OpBlockGroupChecksumProto {
        OpBlockGroupChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockGroupChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpBlockGroupChecksumProto::get_header_for_reflect,
                    OpBlockGroupChecksumProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfosProto>>(
                    "datanodes",
                    OpBlockGroupChecksumProto::get_datanodes_for_reflect,
                    OpBlockGroupChecksumProto::mut_datanodes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<security::TokenProto>>(
                    "blockTokens",
                    OpBlockGroupChecksumProto::get_blockTokens_for_reflect,
                    OpBlockGroupChecksumProto::mut_blockTokens_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ErasureCodingPolicyProto>>(
                    "ecPolicy",
                    OpBlockGroupChecksumProto::get_ecPolicy_for_reflect,
                    OpBlockGroupChecksumProto::mut_ecPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "blockIndices",
                    OpBlockGroupChecksumProto::get_blockIndices_for_reflect,
                    OpBlockGroupChecksumProto::mut_blockIndices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "requestedNumBytes",
                    OpBlockGroupChecksumProto::get_requestedNumBytes_for_reflect,
                    OpBlockGroupChecksumProto::mut_requestedNumBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockGroupChecksumProto>(
                    "OpBlockGroupChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockGroupChecksumProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_datanodes();
        self.clear_blockTokens();
        self.clear_ecPolicy();
        self.clear_blockIndices();
        self.clear_requestedNumBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpBlockGroupChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpBlockGroupChecksumProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmIdProto {
    // message fields
    hi: ::std::option::Option<i64>,
    lo: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmIdProto {}

impl ShortCircuitShmIdProto {
    pub fn new() -> ShortCircuitShmIdProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmIdProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmIdProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmIdProto,
        };
        unsafe {
            instance.get(ShortCircuitShmIdProto::new)
        }
    }

    // required int64 hi = 1;

    pub fn clear_hi(&mut self) {
        self.hi = ::std::option::Option::None;
    }

    pub fn has_hi(&self) -> bool {
        self.hi.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hi(&mut self, v: i64) {
        self.hi = ::std::option::Option::Some(v);
    }

    pub fn get_hi(&self) -> i64 {
        self.hi.unwrap_or(0)
    }

    fn get_hi_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.hi
    }

    fn mut_hi_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.hi
    }

    // required int64 lo = 2;

    pub fn clear_lo(&mut self) {
        self.lo = ::std::option::Option::None;
    }

    pub fn has_lo(&self) -> bool {
        self.lo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lo(&mut self, v: i64) {
        self.lo = ::std::option::Option::Some(v);
    }

    pub fn get_lo(&self) -> i64 {
        self.lo.unwrap_or(0)
    }

    fn get_lo_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.lo
    }

    fn mut_lo_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.lo
    }
}

impl ::protobuf::Message for ShortCircuitShmIdProto {
    fn is_initialized(&self) -> bool {
        if self.hi.is_none() {
            return false;
        }
        if self.lo.is_none() {
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
                    let tmp = is.read_int64()?;
                    self.hi = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hi {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lo {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hi {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.lo {
            os.write_int64(2, v)?;
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

impl ::protobuf::MessageStatic for ShortCircuitShmIdProto {
    fn new() -> ShortCircuitShmIdProto {
        ShortCircuitShmIdProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmIdProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "hi",
                    ShortCircuitShmIdProto::get_hi_for_reflect,
                    ShortCircuitShmIdProto::mut_hi_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lo",
                    ShortCircuitShmIdProto::get_lo_for_reflect,
                    ShortCircuitShmIdProto::mut_lo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmIdProto>(
                    "ShortCircuitShmIdProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmIdProto {
    fn clear(&mut self) {
        self.clear_hi();
        self.clear_lo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmIdProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmIdProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmSlotProto {
    // message fields
    shmId: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    slotIdx: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmSlotProto {}

impl ShortCircuitShmSlotProto {
    pub fn new() -> ShortCircuitShmSlotProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmSlotProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmSlotProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmSlotProto,
        };
        unsafe {
            instance.get(ShortCircuitShmSlotProto::new)
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmIdProto shmId = 1;

    pub fn clear_shmId(&mut self) {
        self.shmId.clear();
    }

    pub fn has_shmId(&self) -> bool {
        self.shmId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shmId(&mut self, v: ShortCircuitShmIdProto) {
        self.shmId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shmId(&mut self) -> &mut ShortCircuitShmIdProto {
        if self.shmId.is_none() {
            self.shmId.set_default();
        }
        self.shmId.as_mut().unwrap()
    }

    // Take field
    pub fn take_shmId(&mut self) -> ShortCircuitShmIdProto {
        self.shmId.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_shmId(&self) -> &ShortCircuitShmIdProto {
        self.shmId.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }

    fn get_shmId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &self.shmId
    }

    fn mut_shmId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &mut self.shmId
    }

    // required int32 slotIdx = 2;

    pub fn clear_slotIdx(&mut self) {
        self.slotIdx = ::std::option::Option::None;
    }

    pub fn has_slotIdx(&self) -> bool {
        self.slotIdx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotIdx(&mut self, v: i32) {
        self.slotIdx = ::std::option::Option::Some(v);
    }

    pub fn get_slotIdx(&self) -> i32 {
        self.slotIdx.unwrap_or(0)
    }

    fn get_slotIdx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slotIdx
    }

    fn mut_slotIdx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slotIdx
    }
}

impl ::protobuf::Message for ShortCircuitShmSlotProto {
    fn is_initialized(&self) -> bool {
        if self.shmId.is_none() {
            return false;
        }
        if self.slotIdx.is_none() {
            return false;
        }
        for v in &self.shmId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shmId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.slotIdx = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.shmId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.slotIdx {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.shmId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.slotIdx {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for ShortCircuitShmSlotProto {
    fn new() -> ShortCircuitShmSlotProto {
        ShortCircuitShmSlotProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmSlotProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmIdProto>>(
                    "shmId",
                    ShortCircuitShmSlotProto::get_shmId_for_reflect,
                    ShortCircuitShmSlotProto::mut_shmId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slotIdx",
                    ShortCircuitShmSlotProto::get_slotIdx_for_reflect,
                    ShortCircuitShmSlotProto::mut_slotIdx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmSlotProto>(
                    "ShortCircuitShmSlotProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmSlotProto {
    fn clear(&mut self) {
        self.clear_shmId();
        self.clear_slotIdx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmSlotProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmSlotProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpRequestShortCircuitAccessProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    maxVersion: ::std::option::Option<u32>,
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    supportsReceiptVerification: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpRequestShortCircuitAccessProto {}

impl OpRequestShortCircuitAccessProto {
    pub fn new() -> OpRequestShortCircuitAccessProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpRequestShortCircuitAccessProto {
        static mut instance: ::protobuf::lazy::Lazy<OpRequestShortCircuitAccessProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpRequestShortCircuitAccessProto,
        };
        unsafe {
            instance.get(OpRequestShortCircuitAccessProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }

    // required uint32 maxVersion = 2;

    pub fn clear_maxVersion(&mut self) {
        self.maxVersion = ::std::option::Option::None;
    }

    pub fn has_maxVersion(&self) -> bool {
        self.maxVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxVersion(&mut self, v: u32) {
        self.maxVersion = ::std::option::Option::Some(v);
    }

    pub fn get_maxVersion(&self) -> u32 {
        self.maxVersion.unwrap_or(0)
    }

    fn get_maxVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.maxVersion
    }

    fn mut_maxVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.maxVersion
    }

    // optional .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 3;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId(&mut self) -> &mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        }
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId(&self) -> &ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    fn get_slotId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &self.slotId
    }

    fn mut_slotId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &mut self.slotId
    }

    // optional bool supportsReceiptVerification = 4;

    pub fn clear_supportsReceiptVerification(&mut self) {
        self.supportsReceiptVerification = ::std::option::Option::None;
    }

    pub fn has_supportsReceiptVerification(&self) -> bool {
        self.supportsReceiptVerification.is_some()
    }

    // Param is passed by value, moved
    pub fn set_supportsReceiptVerification(&mut self, v: bool) {
        self.supportsReceiptVerification = ::std::option::Option::Some(v);
    }

    pub fn get_supportsReceiptVerification(&self) -> bool {
        self.supportsReceiptVerification.unwrap_or(false)
    }

    fn get_supportsReceiptVerification_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.supportsReceiptVerification
    }

    fn mut_supportsReceiptVerification_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.supportsReceiptVerification
    }
}

impl ::protobuf::Message for OpRequestShortCircuitAccessProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.maxVersion.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.slotId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maxVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.slotId)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.supportsReceiptVerification = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.maxVersion {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.slotId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.supportsReceiptVerification {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.maxVersion {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.slotId.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.supportsReceiptVerification {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for OpRequestShortCircuitAccessProto {
    fn new() -> OpRequestShortCircuitAccessProto {
        OpRequestShortCircuitAccessProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpRequestShortCircuitAccessProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpRequestShortCircuitAccessProto::get_header_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maxVersion",
                    OpRequestShortCircuitAccessProto::get_maxVersion_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_maxVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmSlotProto>>(
                    "slotId",
                    OpRequestShortCircuitAccessProto::get_slotId_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_slotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "supportsReceiptVerification",
                    OpRequestShortCircuitAccessProto::get_supportsReceiptVerification_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_supportsReceiptVerification_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpRequestShortCircuitAccessProto>(
                    "OpRequestShortCircuitAccessProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpRequestShortCircuitAccessProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_maxVersion();
        self.clear_slotId();
        self.clear_supportsReceiptVerification();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpRequestShortCircuitAccessProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpRequestShortCircuitAccessProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReleaseShortCircuitAccessRequestProto {
    // message fields
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleaseShortCircuitAccessRequestProto {}

impl ReleaseShortCircuitAccessRequestProto {
    pub fn new() -> ReleaseShortCircuitAccessRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessRequestProto,
        };
        unsafe {
            instance.get(ReleaseShortCircuitAccessRequestProto::new)
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 1;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId(&mut self) -> &mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        }
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId(&self) -> &ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    fn get_slotId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &self.slotId
    }

    fn mut_slotId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &mut self.slotId
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessRequestProto {
    fn is_initialized(&self) -> bool {
        if self.slotId.is_none() {
            return false;
        }
        for v in &self.slotId {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.traceInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.slotId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.traceInfo)?;
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
        if let Some(ref v) = self.slotId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.slotId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessRequestProto {
    fn new() -> ReleaseShortCircuitAccessRequestProto {
        ReleaseShortCircuitAccessRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmSlotProto>>(
                    "slotId",
                    ReleaseShortCircuitAccessRequestProto::get_slotId_for_reflect,
                    ReleaseShortCircuitAccessRequestProto::mut_slotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    ReleaseShortCircuitAccessRequestProto::get_traceInfo_for_reflect,
                    ReleaseShortCircuitAccessRequestProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessRequestProto>(
                    "ReleaseShortCircuitAccessRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessRequestProto {
    fn clear(&mut self) {
        self.clear_slotId();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReleaseShortCircuitAccessRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReleaseShortCircuitAccessResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleaseShortCircuitAccessResponseProto {}

impl ReleaseShortCircuitAccessResponseProto {
    pub fn new() -> ReleaseShortCircuitAccessResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessResponseProto,
        };
        unsafe {
            instance.get(ReleaseShortCircuitAccessResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
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

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessResponseProto {
    fn new() -> ReleaseShortCircuitAccessResponseProto {
        ReleaseShortCircuitAccessResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ReleaseShortCircuitAccessResponseProto::get_status_for_reflect,
                    ReleaseShortCircuitAccessResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ReleaseShortCircuitAccessResponseProto::get_error_for_reflect,
                    ReleaseShortCircuitAccessResponseProto::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessResponseProto>(
                    "ReleaseShortCircuitAccessResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReleaseShortCircuitAccessResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmRequestProto {
    // message fields
    clientName: ::protobuf::SingularField<::std::string::String>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmRequestProto {}

impl ShortCircuitShmRequestProto {
    pub fn new() -> ShortCircuitShmRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmRequestProto,
        };
        unsafe {
            instance.get(ShortCircuitShmRequestProto::new)
        }
    }

    // required string clientName = 1;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName(&mut self) -> &mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        }
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName(&self) -> &str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientName
    }

    fn mut_clientName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientName
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for ShortCircuitShmRequestProto {
    fn is_initialized(&self) -> bool {
        if self.clientName.is_none() {
            return false;
        }
        for v in &self.traceInfo {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.traceInfo)?;
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
        if let Some(ref v) = self.clientName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clientName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ShortCircuitShmRequestProto {
    fn new() -> ShortCircuitShmRequestProto {
        ShortCircuitShmRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientName",
                    ShortCircuitShmRequestProto::get_clientName_for_reflect,
                    ShortCircuitShmRequestProto::mut_clientName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    ShortCircuitShmRequestProto::get_traceInfo_for_reflect,
                    ShortCircuitShmRequestProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmRequestProto>(
                    "ShortCircuitShmRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmRequestProto {
    fn clear(&mut self) {
        self.clear_clientName();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmResponseProto {}

impl ShortCircuitShmResponseProto {
    pub fn new() -> ShortCircuitShmResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmResponseProto,
        };
        unsafe {
            instance.get(ShortCircuitShmResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }

    // optional .hadoop.hdfs.ShortCircuitShmIdProto id = 3;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ShortCircuitShmIdProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ShortCircuitShmIdProto {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ShortCircuitShmIdProto {
        self.id.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_id(&self) -> &ShortCircuitShmIdProto {
        self.id.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &mut self.id
    }
}

impl ::protobuf::Message for ShortCircuitShmResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.id {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.id)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.id.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ShortCircuitShmResponseProto {
    fn new() -> ShortCircuitShmResponseProto {
        ShortCircuitShmResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ShortCircuitShmResponseProto::get_status_for_reflect,
                    ShortCircuitShmResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ShortCircuitShmResponseProto::get_error_for_reflect,
                    ShortCircuitShmResponseProto::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmIdProto>>(
                    "id",
                    ShortCircuitShmResponseProto::get_id_for_reflect,
                    ShortCircuitShmResponseProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmResponseProto>(
                    "ShortCircuitShmResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PacketHeaderProto {
    // message fields
    offsetInBlock: ::std::option::Option<i64>,
    seqno: ::std::option::Option<i64>,
    lastPacketInBlock: ::std::option::Option<bool>,
    dataLen: ::std::option::Option<i32>,
    syncBlock: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PacketHeaderProto {}

impl PacketHeaderProto {
    pub fn new() -> PacketHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PacketHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<PacketHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PacketHeaderProto,
        };
        unsafe {
            instance.get(PacketHeaderProto::new)
        }
    }

    // required sfixed64 offsetInBlock = 1;

    pub fn clear_offsetInBlock(&mut self) {
        self.offsetInBlock = ::std::option::Option::None;
    }

    pub fn has_offsetInBlock(&self) -> bool {
        self.offsetInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offsetInBlock(&mut self, v: i64) {
        self.offsetInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_offsetInBlock(&self) -> i64 {
        self.offsetInBlock.unwrap_or(0)
    }

    fn get_offsetInBlock_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.offsetInBlock
    }

    fn mut_offsetInBlock_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.offsetInBlock
    }

    // required sfixed64 seqno = 2;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.seqno
    }

    // required bool lastPacketInBlock = 3;

    pub fn clear_lastPacketInBlock(&mut self) {
        self.lastPacketInBlock = ::std::option::Option::None;
    }

    pub fn has_lastPacketInBlock(&self) -> bool {
        self.lastPacketInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastPacketInBlock(&mut self, v: bool) {
        self.lastPacketInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_lastPacketInBlock(&self) -> bool {
        self.lastPacketInBlock.unwrap_or(false)
    }

    fn get_lastPacketInBlock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.lastPacketInBlock
    }

    fn mut_lastPacketInBlock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.lastPacketInBlock
    }

    // required sfixed32 dataLen = 4;

    pub fn clear_dataLen(&mut self) {
        self.dataLen = ::std::option::Option::None;
    }

    pub fn has_dataLen(&self) -> bool {
        self.dataLen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataLen(&mut self, v: i32) {
        self.dataLen = ::std::option::Option::Some(v);
    }

    pub fn get_dataLen(&self) -> i32 {
        self.dataLen.unwrap_or(0)
    }

    fn get_dataLen_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dataLen
    }

    fn mut_dataLen_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dataLen
    }

    // optional bool syncBlock = 5;

    pub fn clear_syncBlock(&mut self) {
        self.syncBlock = ::std::option::Option::None;
    }

    pub fn has_syncBlock(&self) -> bool {
        self.syncBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncBlock(&mut self, v: bool) {
        self.syncBlock = ::std::option::Option::Some(v);
    }

    pub fn get_syncBlock(&self) -> bool {
        self.syncBlock.unwrap_or(false)
    }

    fn get_syncBlock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.syncBlock
    }

    fn mut_syncBlock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.syncBlock
    }
}

impl ::protobuf::Message for PacketHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.offsetInBlock.is_none() {
            return false;
        }
        if self.seqno.is_none() {
            return false;
        }
        if self.lastPacketInBlock.is_none() {
            return false;
        }
        if self.dataLen.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed64()?;
                    self.offsetInBlock = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed64()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.lastPacketInBlock = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed32()?;
                    self.dataLen = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.syncBlock = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.offsetInBlock {
            my_size += 9;
        }
        if let Some(v) = self.seqno {
            my_size += 9;
        }
        if let Some(v) = self.lastPacketInBlock {
            my_size += 2;
        }
        if let Some(v) = self.dataLen {
            my_size += 5;
        }
        if let Some(v) = self.syncBlock {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offsetInBlock {
            os.write_sfixed64(1, v)?;
        }
        if let Some(v) = self.seqno {
            os.write_sfixed64(2, v)?;
        }
        if let Some(v) = self.lastPacketInBlock {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.dataLen {
            os.write_sfixed32(4, v)?;
        }
        if let Some(v) = self.syncBlock {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for PacketHeaderProto {
    fn new() -> PacketHeaderProto {
        PacketHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PacketHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "offsetInBlock",
                    PacketHeaderProto::get_offsetInBlock_for_reflect,
                    PacketHeaderProto::mut_offsetInBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "seqno",
                    PacketHeaderProto::get_seqno_for_reflect,
                    PacketHeaderProto::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "lastPacketInBlock",
                    PacketHeaderProto::get_lastPacketInBlock_for_reflect,
                    PacketHeaderProto::mut_lastPacketInBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed32>(
                    "dataLen",
                    PacketHeaderProto::get_dataLen_for_reflect,
                    PacketHeaderProto::mut_dataLen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "syncBlock",
                    PacketHeaderProto::get_syncBlock_for_reflect,
                    PacketHeaderProto::mut_syncBlock_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PacketHeaderProto>(
                    "PacketHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PacketHeaderProto {
    fn clear(&mut self) {
        self.clear_offsetInBlock();
        self.clear_seqno();
        self.clear_lastPacketInBlock();
        self.clear_dataLen();
        self.clear_syncBlock();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PacketHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PipelineAckProto {
    // message fields
    seqno: ::std::option::Option<i64>,
    reply: ::std::vec::Vec<Status>,
    downstreamAckTimeNanos: ::std::option::Option<u64>,
    flag: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PipelineAckProto {}

impl PipelineAckProto {
    pub fn new() -> PipelineAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PipelineAckProto {
        static mut instance: ::protobuf::lazy::Lazy<PipelineAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PipelineAckProto,
        };
        unsafe {
            instance.get(PipelineAckProto::new)
        }
    }

    // required sint64 seqno = 1;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.seqno
    }

    // repeated .hadoop.hdfs.Status reply = 2;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::std::vec::Vec<Status>) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reply(&mut self) -> &mut ::std::vec::Vec<Status> {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::std::vec::Vec<Status> {
        ::std::mem::replace(&mut self.reply, ::std::vec::Vec::new())
    }

    pub fn get_reply(&self) -> &[Status] {
        &self.reply
    }

    fn get_reply_for_reflect(&self) -> &::std::vec::Vec<Status> {
        &self.reply
    }

    fn mut_reply_for_reflect(&mut self) -> &mut ::std::vec::Vec<Status> {
        &mut self.reply
    }

    // optional uint64 downstreamAckTimeNanos = 3;

    pub fn clear_downstreamAckTimeNanos(&mut self) {
        self.downstreamAckTimeNanos = ::std::option::Option::None;
    }

    pub fn has_downstreamAckTimeNanos(&self) -> bool {
        self.downstreamAckTimeNanos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downstreamAckTimeNanos(&mut self, v: u64) {
        self.downstreamAckTimeNanos = ::std::option::Option::Some(v);
    }

    pub fn get_downstreamAckTimeNanos(&self) -> u64 {
        self.downstreamAckTimeNanos.unwrap_or(0u64)
    }

    fn get_downstreamAckTimeNanos_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.downstreamAckTimeNanos
    }

    fn mut_downstreamAckTimeNanos_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.downstreamAckTimeNanos
    }

    // repeated uint32 flag = 4;

    pub fn clear_flag(&mut self) {
        self.flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: ::std::vec::Vec<u32>) {
        self.flag = v;
    }

    // Mutable pointer to the field.
    pub fn mut_flag(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.flag
    }

    // Take field
    pub fn take_flag(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.flag, ::std::vec::Vec::new())
    }

    pub fn get_flag(&self) -> &[u32] {
        &self.flag
    }

    fn get_flag_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.flag
    }

    fn mut_flag_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.flag
    }
}

impl ::protobuf::Message for PipelineAckProto {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
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
                    let tmp = is.read_sint64()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.reply)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.downstreamAckTimeNanos = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.flag)?;
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
        if let Some(v) = self.seqno {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        for value in &self.reply {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if let Some(v) = self.downstreamAckTimeNanos {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.flag.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.flag);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            os.write_sint64(1, v)?;
        }
        for v in &self.reply {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.downstreamAckTimeNanos {
            os.write_uint64(3, v)?;
        }
        if !self.flag.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.flag))?;
            for v in &self.flag {
                os.write_uint32_no_tag(*v)?;
            };
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

impl ::protobuf::MessageStatic for PipelineAckProto {
    fn new() -> PipelineAckProto {
        PipelineAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PipelineAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "seqno",
                    PipelineAckProto::get_seqno_for_reflect,
                    PipelineAckProto::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "reply",
                    PipelineAckProto::get_reply_for_reflect,
                    PipelineAckProto::mut_reply_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "downstreamAckTimeNanos",
                    PipelineAckProto::get_downstreamAckTimeNanos_for_reflect,
                    PipelineAckProto::mut_downstreamAckTimeNanos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flag",
                    PipelineAckProto::get_flag_for_reflect,
                    PipelineAckProto::mut_flag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PipelineAckProto>(
                    "PipelineAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PipelineAckProto {
    fn clear(&mut self) {
        self.clear_seqno();
        self.clear_reply();
        self.clear_downstreamAckTimeNanos();
        self.clear_flag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PipelineAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PipelineAckProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadOpChecksumInfoProto {
    // message fields
    checksum: ::protobuf::SingularPtrField<ChecksumProto>,
    chunkOffset: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadOpChecksumInfoProto {}

impl ReadOpChecksumInfoProto {
    pub fn new() -> ReadOpChecksumInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadOpChecksumInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ReadOpChecksumInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadOpChecksumInfoProto,
        };
        unsafe {
            instance.get(ReadOpChecksumInfoProto::new)
        }
    }

    // required .hadoop.hdfs.ChecksumProto checksum = 1;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: ChecksumProto) {
        self.checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut ChecksumProto {
        if self.checksum.is_none() {
            self.checksum.set_default();
        }
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ChecksumProto {
        self.checksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_checksum(&self) -> &ChecksumProto {
        self.checksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<ChecksumProto> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChecksumProto> {
        &mut self.checksum
    }

    // required uint64 chunkOffset = 2;

    pub fn clear_chunkOffset(&mut self) {
        self.chunkOffset = ::std::option::Option::None;
    }

    pub fn has_chunkOffset(&self) -> bool {
        self.chunkOffset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunkOffset(&mut self, v: u64) {
        self.chunkOffset = ::std::option::Option::Some(v);
    }

    pub fn get_chunkOffset(&self) -> u64 {
        self.chunkOffset.unwrap_or(0)
    }

    fn get_chunkOffset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.chunkOffset
    }

    fn mut_chunkOffset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.chunkOffset
    }
}

impl ::protobuf::Message for ReadOpChecksumInfoProto {
    fn is_initialized(&self) -> bool {
        if self.checksum.is_none() {
            return false;
        }
        if self.chunkOffset.is_none() {
            return false;
        }
        for v in &self.checksum {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksum)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.chunkOffset = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.chunkOffset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.checksum.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.chunkOffset {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for ReadOpChecksumInfoProto {
    fn new() -> ReadOpChecksumInfoProto {
        ReadOpChecksumInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadOpChecksumInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChecksumProto>>(
                    "checksum",
                    ReadOpChecksumInfoProto::get_checksum_for_reflect,
                    ReadOpChecksumInfoProto::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chunkOffset",
                    ReadOpChecksumInfoProto::get_chunkOffset_for_reflect,
                    ReadOpChecksumInfoProto::mut_chunkOffset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadOpChecksumInfoProto>(
                    "ReadOpChecksumInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadOpChecksumInfoProto {
    fn clear(&mut self) {
        self.clear_checksum();
        self.clear_chunkOffset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadOpChecksumInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadOpChecksumInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockOpResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    firstBadLink: ::protobuf::SingularField<::std::string::String>,
    checksumResponse: ::protobuf::SingularPtrField<OpBlockChecksumResponseProto>,
    readOpChecksumInfo: ::protobuf::SingularPtrField<ReadOpChecksumInfoProto>,
    message: ::protobuf::SingularField<::std::string::String>,
    shortCircuitAccessVersion: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockOpResponseProto {}

impl BlockOpResponseProto {
    pub fn new() -> BlockOpResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockOpResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockOpResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockOpResponseProto,
        };
        unsafe {
            instance.get(BlockOpResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string firstBadLink = 2;

    pub fn clear_firstBadLink(&mut self) {
        self.firstBadLink.clear();
    }

    pub fn has_firstBadLink(&self) -> bool {
        self.firstBadLink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstBadLink(&mut self, v: ::std::string::String) {
        self.firstBadLink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firstBadLink(&mut self) -> &mut ::std::string::String {
        if self.firstBadLink.is_none() {
            self.firstBadLink.set_default();
        }
        self.firstBadLink.as_mut().unwrap()
    }

    // Take field
    pub fn take_firstBadLink(&mut self) -> ::std::string::String {
        self.firstBadLink.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firstBadLink(&self) -> &str {
        match self.firstBadLink.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_firstBadLink_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.firstBadLink
    }

    fn mut_firstBadLink_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.firstBadLink
    }

    // optional .hadoop.hdfs.OpBlockChecksumResponseProto checksumResponse = 3;

    pub fn clear_checksumResponse(&mut self) {
        self.checksumResponse.clear();
    }

    pub fn has_checksumResponse(&self) -> bool {
        self.checksumResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumResponse(&mut self, v: OpBlockChecksumResponseProto) {
        self.checksumResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksumResponse(&mut self) -> &mut OpBlockChecksumResponseProto {
        if self.checksumResponse.is_none() {
            self.checksumResponse.set_default();
        }
        self.checksumResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksumResponse(&mut self) -> OpBlockChecksumResponseProto {
        self.checksumResponse.take().unwrap_or_else(|| OpBlockChecksumResponseProto::new())
    }

    pub fn get_checksumResponse(&self) -> &OpBlockChecksumResponseProto {
        self.checksumResponse.as_ref().unwrap_or_else(|| OpBlockChecksumResponseProto::default_instance())
    }

    fn get_checksumResponse_for_reflect(&self) -> &::protobuf::SingularPtrField<OpBlockChecksumResponseProto> {
        &self.checksumResponse
    }

    fn mut_checksumResponse_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OpBlockChecksumResponseProto> {
        &mut self.checksumResponse
    }

    // optional .hadoop.hdfs.ReadOpChecksumInfoProto readOpChecksumInfo = 4;

    pub fn clear_readOpChecksumInfo(&mut self) {
        self.readOpChecksumInfo.clear();
    }

    pub fn has_readOpChecksumInfo(&self) -> bool {
        self.readOpChecksumInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readOpChecksumInfo(&mut self, v: ReadOpChecksumInfoProto) {
        self.readOpChecksumInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readOpChecksumInfo(&mut self) -> &mut ReadOpChecksumInfoProto {
        if self.readOpChecksumInfo.is_none() {
            self.readOpChecksumInfo.set_default();
        }
        self.readOpChecksumInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_readOpChecksumInfo(&mut self) -> ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.take().unwrap_or_else(|| ReadOpChecksumInfoProto::new())
    }

    pub fn get_readOpChecksumInfo(&self) -> &ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.as_ref().unwrap_or_else(|| ReadOpChecksumInfoProto::default_instance())
    }

    fn get_readOpChecksumInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<ReadOpChecksumInfoProto> {
        &self.readOpChecksumInfo
    }

    fn mut_readOpChecksumInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReadOpChecksumInfoProto> {
        &mut self.readOpChecksumInfo
    }

    // optional string message = 5;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional uint32 shortCircuitAccessVersion = 6;

    pub fn clear_shortCircuitAccessVersion(&mut self) {
        self.shortCircuitAccessVersion = ::std::option::Option::None;
    }

    pub fn has_shortCircuitAccessVersion(&self) -> bool {
        self.shortCircuitAccessVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortCircuitAccessVersion(&mut self, v: u32) {
        self.shortCircuitAccessVersion = ::std::option::Option::Some(v);
    }

    pub fn get_shortCircuitAccessVersion(&self) -> u32 {
        self.shortCircuitAccessVersion.unwrap_or(0)
    }

    fn get_shortCircuitAccessVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.shortCircuitAccessVersion
    }

    fn mut_shortCircuitAccessVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.shortCircuitAccessVersion
    }
}

impl ::protobuf::Message for BlockOpResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.checksumResponse {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.readOpChecksumInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firstBadLink)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksumResponse)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.readOpChecksumInfo)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.shortCircuitAccessVersion = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.firstBadLink.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.checksumResponse.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.readOpChecksumInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.shortCircuitAccessVersion {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.firstBadLink.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.checksumResponse.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.readOpChecksumInfo.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.shortCircuitAccessVersion {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for BlockOpResponseProto {
    fn new() -> BlockOpResponseProto {
        BlockOpResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockOpResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    BlockOpResponseProto::get_status_for_reflect,
                    BlockOpResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "firstBadLink",
                    BlockOpResponseProto::get_firstBadLink_for_reflect,
                    BlockOpResponseProto::mut_firstBadLink_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpBlockChecksumResponseProto>>(
                    "checksumResponse",
                    BlockOpResponseProto::get_checksumResponse_for_reflect,
                    BlockOpResponseProto::mut_checksumResponse_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReadOpChecksumInfoProto>>(
                    "readOpChecksumInfo",
                    BlockOpResponseProto::get_readOpChecksumInfo_for_reflect,
                    BlockOpResponseProto::mut_readOpChecksumInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    BlockOpResponseProto::get_message_for_reflect,
                    BlockOpResponseProto::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shortCircuitAccessVersion",
                    BlockOpResponseProto::get_shortCircuitAccessVersion_for_reflect,
                    BlockOpResponseProto::mut_shortCircuitAccessVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockOpResponseProto>(
                    "BlockOpResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockOpResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_firstBadLink();
        self.clear_checksumResponse();
        self.clear_readOpChecksumInfo();
        self.clear_message();
        self.clear_shortCircuitAccessVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockOpResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockOpResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientReadStatusProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientReadStatusProto {}

impl ClientReadStatusProto {
    pub fn new() -> ClientReadStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientReadStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientReadStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientReadStatusProto,
        };
        unsafe {
            instance.get(ClientReadStatusProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for ClientReadStatusProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for ClientReadStatusProto {
    fn new() -> ClientReadStatusProto {
        ClientReadStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientReadStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ClientReadStatusProto::get_status_for_reflect,
                    ClientReadStatusProto::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientReadStatusProto>(
                    "ClientReadStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientReadStatusProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientReadStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientReadStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DNTransferAckProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DNTransferAckProto {}

impl DNTransferAckProto {
    pub fn new() -> DNTransferAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DNTransferAckProto {
        static mut instance: ::protobuf::lazy::Lazy<DNTransferAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DNTransferAckProto,
        };
        unsafe {
            instance.get(DNTransferAckProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for DNTransferAckProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for DNTransferAckProto {
    fn new() -> DNTransferAckProto {
        DNTransferAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DNTransferAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    DNTransferAckProto::get_status_for_reflect,
                    DNTransferAckProto::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DNTransferAckProto>(
                    "DNTransferAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DNTransferAckProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DNTransferAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DNTransferAckProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpBlockChecksumResponseProto {
    // message fields
    bytesPerCrc: ::std::option::Option<u32>,
    crcPerBlock: ::std::option::Option<u64>,
    md5: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    crcType: ::std::option::Option<super::hdfs::ChecksumTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpBlockChecksumResponseProto {}

impl OpBlockChecksumResponseProto {
    pub fn new() -> OpBlockChecksumResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumResponseProto,
        };
        unsafe {
            instance.get(OpBlockChecksumResponseProto::new)
        }
    }

    // required uint32 bytesPerCrc = 1;

    pub fn clear_bytesPerCrc(&mut self) {
        self.bytesPerCrc = ::std::option::Option::None;
    }

    pub fn has_bytesPerCrc(&self) -> bool {
        self.bytesPerCrc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerCrc(&mut self, v: u32) {
        self.bytesPerCrc = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerCrc(&self) -> u32 {
        self.bytesPerCrc.unwrap_or(0)
    }

    fn get_bytesPerCrc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bytesPerCrc
    }

    fn mut_bytesPerCrc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bytesPerCrc
    }

    // required uint64 crcPerBlock = 2;

    pub fn clear_crcPerBlock(&mut self) {
        self.crcPerBlock = ::std::option::Option::None;
    }

    pub fn has_crcPerBlock(&self) -> bool {
        self.crcPerBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcPerBlock(&mut self, v: u64) {
        self.crcPerBlock = ::std::option::Option::Some(v);
    }

    pub fn get_crcPerBlock(&self) -> u64 {
        self.crcPerBlock.unwrap_or(0)
    }

    fn get_crcPerBlock_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.crcPerBlock
    }

    fn mut_crcPerBlock_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.crcPerBlock
    }

    // required bytes md5 = 3;

    pub fn clear_md5(&mut self) {
        self.md5.clear();
    }

    pub fn has_md5(&self) -> bool {
        self.md5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_md5(&mut self, v: ::std::vec::Vec<u8>) {
        self.md5 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_md5(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.md5.is_none() {
            self.md5.set_default();
        }
        self.md5.as_mut().unwrap()
    }

    // Take field
    pub fn take_md5(&mut self) -> ::std::vec::Vec<u8> {
        self.md5.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_md5(&self) -> &[u8] {
        match self.md5.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_md5_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.md5
    }

    fn mut_md5_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.md5
    }

    // optional .hadoop.hdfs.ChecksumTypeProto crcType = 4;

    pub fn clear_crcType(&mut self) {
        self.crcType = ::std::option::Option::None;
    }

    pub fn has_crcType(&self) -> bool {
        self.crcType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcType(&mut self, v: super::hdfs::ChecksumTypeProto) {
        self.crcType = ::std::option::Option::Some(v);
    }

    pub fn get_crcType(&self) -> super::hdfs::ChecksumTypeProto {
        self.crcType.unwrap_or(super::hdfs::ChecksumTypeProto::CHECKSUM_NULL)
    }

    fn get_crcType_for_reflect(&self) -> &::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &self.crcType
    }

    fn mut_crcType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &mut self.crcType
    }
}

impl ::protobuf::Message for OpBlockChecksumResponseProto {
    fn is_initialized(&self) -> bool {
        if self.bytesPerCrc.is_none() {
            return false;
        }
        if self.crcPerBlock.is_none() {
            return false;
        }
        if self.md5.is_none() {
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
                    self.bytesPerCrc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.crcPerBlock = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.md5)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crcType = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.bytesPerCrc {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.crcPerBlock {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.md5.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(v) = self.crcType {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bytesPerCrc {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.crcPerBlock {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.md5.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(v) = self.crcType {
            os.write_enum(4, v.value())?;
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

impl ::protobuf::MessageStatic for OpBlockChecksumResponseProto {
    fn new() -> OpBlockChecksumResponseProto {
        OpBlockChecksumResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerCrc",
                    OpBlockChecksumResponseProto::get_bytesPerCrc_for_reflect,
                    OpBlockChecksumResponseProto::mut_bytesPerCrc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "crcPerBlock",
                    OpBlockChecksumResponseProto::get_crcPerBlock_for_reflect,
                    OpBlockChecksumResponseProto::mut_crcPerBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "md5",
                    OpBlockChecksumResponseProto::get_md5_for_reflect,
                    OpBlockChecksumResponseProto::mut_md5_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::ChecksumTypeProto>>(
                    "crcType",
                    OpBlockChecksumResponseProto::get_crcType_for_reflect,
                    OpBlockChecksumResponseProto::mut_crcType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumResponseProto>(
                    "OpBlockChecksumResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumResponseProto {
    fn clear(&mut self) {
        self.clear_bytesPerCrc();
        self.clear_crcPerBlock();
        self.clear_md5();
        self.clear_crcType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpBlockChecksumResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpBlockChecksumResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpCustomProto {
    // message fields
    customId: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpCustomProto {}

impl OpCustomProto {
    pub fn new() -> OpCustomProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpCustomProto {
        static mut instance: ::protobuf::lazy::Lazy<OpCustomProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpCustomProto,
        };
        unsafe {
            instance.get(OpCustomProto::new)
        }
    }

    // required string customId = 1;

    pub fn clear_customId(&mut self) {
        self.customId.clear();
    }

    pub fn has_customId(&self) -> bool {
        self.customId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_customId(&mut self, v: ::std::string::String) {
        self.customId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_customId(&mut self) -> &mut ::std::string::String {
        if self.customId.is_none() {
            self.customId.set_default();
        }
        self.customId.as_mut().unwrap()
    }

    // Take field
    pub fn take_customId(&mut self) -> ::std::string::String {
        self.customId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_customId(&self) -> &str {
        match self.customId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_customId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.customId
    }

    fn mut_customId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.customId
    }
}

impl ::protobuf::Message for OpCustomProto {
    fn is_initialized(&self) -> bool {
        if self.customId.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.customId)?;
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
        if let Some(ref v) = self.customId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.customId.as_ref() {
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

impl ::protobuf::MessageStatic for OpCustomProto {
    fn new() -> OpCustomProto {
        OpCustomProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpCustomProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "customId",
                    OpCustomProto::get_customId_for_reflect,
                    OpCustomProto::mut_customId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpCustomProto>(
                    "OpCustomProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpCustomProto {
    fn clear(&mut self) {
        self.clear_customId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpCustomProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpCustomProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    SUCCESS = 0,
    ERROR = 1,
    ERROR_CHECKSUM = 2,
    ERROR_INVALID = 3,
    ERROR_EXISTS = 4,
    ERROR_ACCESS_TOKEN = 5,
    CHECKSUM_OK = 6,
    ERROR_UNSUPPORTED = 7,
    OOB_RESTART = 8,
    OOB_RESERVED1 = 9,
    OOB_RESERVED2 = 10,
    OOB_RESERVED3 = 11,
    IN_PROGRESS = 12,
    ERROR_BLOCK_PINNED = 13,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::SUCCESS),
            1 => ::std::option::Option::Some(Status::ERROR),
            2 => ::std::option::Option::Some(Status::ERROR_CHECKSUM),
            3 => ::std::option::Option::Some(Status::ERROR_INVALID),
            4 => ::std::option::Option::Some(Status::ERROR_EXISTS),
            5 => ::std::option::Option::Some(Status::ERROR_ACCESS_TOKEN),
            6 => ::std::option::Option::Some(Status::CHECKSUM_OK),
            7 => ::std::option::Option::Some(Status::ERROR_UNSUPPORTED),
            8 => ::std::option::Option::Some(Status::OOB_RESTART),
            9 => ::std::option::Option::Some(Status::OOB_RESERVED1),
            10 => ::std::option::Option::Some(Status::OOB_RESERVED2),
            11 => ::std::option::Option::Some(Status::OOB_RESERVED3),
            12 => ::std::option::Option::Some(Status::IN_PROGRESS),
            13 => ::std::option::Option::Some(Status::ERROR_BLOCK_PINNED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::SUCCESS,
            Status::ERROR,
            Status::ERROR_CHECKSUM,
            Status::ERROR_INVALID,
            Status::ERROR_EXISTS,
            Status::ERROR_ACCESS_TOKEN,
            Status::CHECKSUM_OK,
            Status::ERROR_UNSUPPORTED,
            Status::OOB_RESTART,
            Status::OOB_RESERVED1,
            Status::OOB_RESERVED2,
            Status::OOB_RESERVED3,
            Status::IN_PROGRESS,
            Status::ERROR_BLOCK_PINNED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ShortCircuitFdResponse {
    DO_NOT_USE_RECEIPT_VERIFICATION = 0,
    USE_RECEIPT_VERIFICATION = 1,
}

impl ::protobuf::ProtobufEnum for ShortCircuitFdResponse {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ShortCircuitFdResponse> {
        match value {
            0 => ::std::option::Option::Some(ShortCircuitFdResponse::DO_NOT_USE_RECEIPT_VERIFICATION),
            1 => ::std::option::Option::Some(ShortCircuitFdResponse::USE_RECEIPT_VERIFICATION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ShortCircuitFdResponse] = &[
            ShortCircuitFdResponse::DO_NOT_USE_RECEIPT_VERIFICATION,
            ShortCircuitFdResponse::USE_RECEIPT_VERIFICATION,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ShortCircuitFdResponse>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ShortCircuitFdResponse", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ShortCircuitFdResponse {
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitFdResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12datatransfer.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.proto\x1a\nh\
    dfs.proto\"\xcd\x02\n!DataTransferEncryptorMessageProto\x12b\n\x06status\
    \x18\x01\x20\x02(\x0e2J.hadoop.hdfs.DataTransferEncryptorMessageProto.Da\
    taTransferEncryptorStatusR\x06status\x12\x18\n\x07payload\x18\x02\x20\
    \x01(\x0cR\x07payload\x12\x18\n\x07message\x18\x03\x20\x01(\tR\x07messag\
    e\x12B\n\x0ccipherOption\x18\x04\x20\x03(\x0b2\x1e.hadoop.hdfs.CipherOpt\
    ionProtoR\x0ccipherOption\"L\n\x1bDataTransferEncryptorStatus\x12\x0b\n\
    \x07SUCCESS\x10\0\x12\x15\n\x11ERROR_UNKNOWN_KEY\x10\x01\x12\t\n\x05ERRO\
    R\x10\x02\"\xc0\x01\n\x0fBaseHeaderProto\x125\n\x05block\x18\x01\x20\x02\
    (\x0b2\x1f.hadoop.hdfs.ExtendedBlockProtoR\x05block\x12/\n\x05token\x18\
    \x02\x20\x01(\x0b2\x19.hadoop.common.TokenProtoR\x05token\x12E\n\ttraceI\
    nfo\x18\x03\x20\x01(\x0b2'.hadoop.hdfs.DataTransferTraceInfoProtoR\ttrac\
    eInfo\"R\n\x1aDataTransferTraceInfoProto\x12\x18\n\x07traceId\x18\x01\
    \x20\x02(\x04R\x07traceId\x12\x1a\n\x08parentId\x18\x02\x20\x02(\x04R\
    \x08parentId\"z\n\x1aClientOperationHeaderProto\x12<\n\nbaseHeader\x18\
    \x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProtoR\nbaseHeader\x12\x1e\
    \n\nclientName\x18\x02\x20\x02(\tR\nclientName\"T\n\x14CachingStrategyPr\
    oto\x12\x1e\n\ndropBehind\x18\x01\x20\x01(\x08R\ndropBehind\x12\x1c\n\tr\
    eadahead\x18\x02\x20\x01(\x03R\treadahead\"\xf6\x01\n\x10OpReadBlockProt\
    o\x12?\n\x06header\x18\x01\x20\x02(\x0b2'.hadoop.hdfs.ClientOperationHea\
    derProtoR\x06header\x12\x16\n\x06offset\x18\x02\x20\x02(\x04R\x06offset\
    \x12\x10\n\x03len\x18\x03\x20\x02(\x04R\x03len\x12*\n\rsendChecksums\x18\
    \x04\x20\x01(\x08:\x04trueR\rsendChecksums\x12K\n\x0fcachingStrategy\x18\
    \x05\x20\x01(\x0b2!.hadoop.hdfs.CachingStrategyProtoR\x0fcachingStrategy\
    \"o\n\rChecksumProto\x122\n\x04type\x18\x01\x20\x02(\x0e2\x1e.hadoop.hdf\
    s.ChecksumTypeProtoR\x04type\x12*\n\x10bytesPerChecksum\x18\x02\x20\x02(\
    \rR\x10bytesPerChecksum\"\xb3\t\n\x11OpWriteBlockProto\x12?\n\x06header\
    \x18\x01\x20\x02(\x0b2'.hadoop.hdfs.ClientOperationHeaderProtoR\x06heade\
    r\x128\n\x07targets\x18\x02\x20\x03(\x0b2\x1e.hadoop.hdfs.DatanodeInfoPr\
    otoR\x07targets\x126\n\x06source\x18\x03\x20\x01(\x0b2\x1e.hadoop.hdfs.D\
    atanodeInfoProtoR\x06source\x12K\n\x05stage\x18\x04\x20\x02(\x0e25.hadoo\
    p.hdfs.OpWriteBlockProto.BlockConstructionStageR\x05stage\x12\"\n\x0cpip\
    elineSize\x18\x05\x20\x02(\rR\x0cpipelineSize\x12\"\n\x0cminBytesRcvd\
    \x18\x06\x20\x02(\x04R\x0cminBytesRcvd\x12\"\n\x0cmaxBytesRcvd\x18\x07\
    \x20\x02(\x04R\x0cmaxBytesRcvd\x124\n\x15latestGenerationStamp\x18\x08\
    \x20\x02(\x04R\x15latestGenerationStamp\x12H\n\x11requestedChecksum\x18\
    \t\x20\x02(\x0b2\x1a.hadoop.hdfs.ChecksumProtoR\x11requestedChecksum\x12\
    K\n\x0fcachingStrategy\x18\n\x20\x01(\x0b2!.hadoop.hdfs.CachingStrategyP\
    rotoR\x0fcachingStrategy\x12E\n\x0bstorageType\x18\x0b\x20\x01(\x0e2\x1d\
    .hadoop.hdfs.StorageTypeProto:\x04DISKR\x0bstorageType\x12M\n\x12targetS\
    torageTypes\x18\x0c\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageTypeProtoR\x12t\
    argetStorageTypes\x121\n\x10allowLazyPersist\x18\r\x20\x01(\x08:\x05fals\
    eR\x10allowLazyPersist\x12\x1f\n\x07pinning\x18\x0e\x20\x01(\x08:\x05fal\
    seR\x07pinning\x12&\n\x0etargetPinnings\x18\x0f\x20\x03(\x08R\x0etargetP\
    innings\x12\x1c\n\tstorageId\x18\x10\x20\x01(\tR\tstorageId\x12*\n\x10ta\
    rgetStorageIds\x18\x11\x20\x03(\tR\x10targetStorageIds\"\x88\x02\n\x16Bl\
    ockConstructionStage\x12\x19\n\x15PIPELINE_SETUP_APPEND\x10\0\x12\"\n\
    \x1ePIPELINE_SETUP_APPEND_RECOVERY\x10\x01\x12\x12\n\x0eDATA_STREAMING\
    \x10\x02\x12%\n!PIPELINE_SETUP_STREAMING_RECOVERY\x10\x03\x12\x12\n\x0eP\
    IPELINE_CLOSE\x10\x04\x12\x1b\n\x17PIPELINE_CLOSE_RECOVERY\x10\x05\x12\
    \x19\n\x15PIPELINE_SETUP_CREATE\x10\x06\x12\x10\n\x0cTRANSFER_RBW\x10\
    \x07\x12\x16\n\x12TRANSFER_FINALIZED\x10\x08\"\x8c\x02\n\x14OpTransferBl\
    ockProto\x12?\n\x06header\x18\x01\x20\x02(\x0b2'.hadoop.hdfs.ClientOpera\
    tionHeaderProtoR\x06header\x128\n\x07targets\x18\x02\x20\x03(\x0b2\x1e.h\
    adoop.hdfs.DatanodeInfoProtoR\x07targets\x12M\n\x12targetStorageTypes\
    \x18\x03\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageTypeProtoR\x12targetStorag\
    eTypes\x12*\n\x10targetStorageIds\x18\x04\x20\x03(\tR\x10targetStorageId\
    s\"\x82\x02\n\x13OpReplaceBlockProto\x124\n\x06header\x18\x01\x20\x02(\
    \x0b2\x1c.hadoop.hdfs.BaseHeaderProtoR\x06header\x12\x18\n\x07delHint\
    \x18\x02\x20\x02(\tR\x07delHint\x126\n\x06source\x18\x03\x20\x02(\x0b2\
    \x1e.hadoop.hdfs.DatanodeInfoProtoR\x06source\x12E\n\x0bstorageType\x18\
    \x04\x20\x01(\x0e2\x1d.hadoop.hdfs.StorageTypeProto:\x04DISKR\x0bstorage\
    Type\x12\x1c\n\tstorageId\x18\x05\x20\x01(\tR\tstorageId\"H\n\x10OpCopyB\
    lockProto\x124\n\x06header\x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHea\
    derProtoR\x06header\"L\n\x14OpBlockChecksumProto\x124\n\x06header\x18\
    \x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProtoR\x06header\"\xe2\x02\
    \n\x19OpBlockGroupChecksumProto\x124\n\x06header\x18\x01\x20\x02(\x0b2\
    \x1c.hadoop.hdfs.BaseHeaderProtoR\x06header\x12=\n\tdatanodes\x18\x02\
    \x20\x02(\x0b2\x1f.hadoop.hdfs.DatanodeInfosProtoR\tdatanodes\x12;\n\x0b\
    blockTokens\x18\x03\x20\x03(\x0b2\x19.hadoop.common.TokenProtoR\x0bblock\
    Tokens\x12A\n\x08ecPolicy\x18\x04\x20\x02(\x0b2%.hadoop.hdfs.ErasureCodi\
    ngPolicyProtoR\x08ecPolicy\x12\"\n\x0cblockIndices\x18\x05\x20\x03(\rR\
    \x0cblockIndices\x12,\n\x11requestedNumBytes\x18\x06\x20\x02(\x04R\x11re\
    questedNumBytes\"8\n\x16ShortCircuitShmIdProto\x12\x0e\n\x02hi\x18\x01\
    \x20\x02(\x03R\x02hi\x12\x0e\n\x02lo\x18\x02\x20\x02(\x03R\x02lo\"o\n\
    \x18ShortCircuitShmSlotProto\x129\n\x05shmId\x18\x01\x20\x02(\x0b2#.hado\
    op.hdfs.ShortCircuitShmIdProtoR\x05shmId\x12\x18\n\x07slotIdx\x18\x02\
    \x20\x02(\x05R\x07slotIdx\"\x80\x02\n\x20OpRequestShortCircuitAccessProt\
    o\x124\n\x06header\x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProto\
    R\x06header\x12\x1e\n\nmaxVersion\x18\x02\x20\x02(\rR\nmaxVersion\x12=\n\
    \x06slotId\x18\x03\x20\x01(\x0b2%.hadoop.hdfs.ShortCircuitShmSlotProtoR\
    \x06slotId\x12G\n\x1bsupportsReceiptVerification\x18\x04\x20\x01(\x08:\
    \x05falseR\x1bsupportsReceiptVerification\"\xad\x01\n%ReleaseShortCircui\
    tAccessRequestProto\x12=\n\x06slotId\x18\x01\x20\x02(\x0b2%.hadoop.hdfs.\
    ShortCircuitShmSlotProtoR\x06slotId\x12E\n\ttraceInfo\x18\x02\x20\x01(\
    \x0b2'.hadoop.hdfs.DataTransferTraceInfoProtoR\ttraceInfo\"k\n&ReleaseSh\
    ortCircuitAccessResponseProto\x12+\n\x06status\x18\x01\x20\x02(\x0e2\x13\
    .hadoop.hdfs.StatusR\x06status\x12\x14\n\x05error\x18\x02\x20\x01(\tR\
    \x05error\"\x84\x01\n\x1bShortCircuitShmRequestProto\x12\x1e\n\nclientNa\
    me\x18\x01\x20\x02(\tR\nclientName\x12E\n\ttraceInfo\x18\x02\x20\x01(\
    \x0b2'.hadoop.hdfs.DataTransferTraceInfoProtoR\ttraceInfo\"\x96\x01\n\
    \x1cShortCircuitShmResponseProto\x12+\n\x06status\x18\x01\x20\x02(\x0e2\
    \x13.hadoop.hdfs.StatusR\x06status\x12\x14\n\x05error\x18\x02\x20\x01(\t\
    R\x05error\x123\n\x02id\x18\x03\x20\x01(\x0b2#.hadoop.hdfs.ShortCircuitS\
    hmIdProtoR\x02id\"\xbc\x01\n\x11PacketHeaderProto\x12$\n\roffsetInBlock\
    \x18\x01\x20\x02(\x10R\roffsetInBlock\x12\x14\n\x05seqno\x18\x02\x20\x02\
    (\x10R\x05seqno\x12,\n\x11lastPacketInBlock\x18\x03\x20\x02(\x08R\x11las\
    tPacketInBlock\x12\x18\n\x07dataLen\x18\x04\x20\x02(\x0fR\x07dataLen\x12\
    #\n\tsyncBlock\x18\x05\x20\x01(\x08:\x05falseR\tsyncBlock\"\xa6\x01\n\
    \x10PipelineAckProto\x12\x14\n\x05seqno\x18\x01\x20\x02(\x12R\x05seqno\
    \x12)\n\x05reply\x18\x02\x20\x03(\x0e2\x13.hadoop.hdfs.StatusR\x05reply\
    \x129\n\x16downstreamAckTimeNanos\x18\x03\x20\x01(\x04:\x010R\x16downstr\
    eamAckTimeNanos\x12\x16\n\x04flag\x18\x04\x20\x03(\rR\x04flagB\x02\x10\
    \x01\"s\n\x17ReadOpChecksumInfoProto\x126\n\x08checksum\x18\x01\x20\x02(\
    \x0b2\x1a.hadoop.hdfs.ChecksumProtoR\x08checksum\x12\x20\n\x0bchunkOffse\
    t\x18\x02\x20\x02(\x04R\x0bchunkOffset\"\xec\x02\n\x14BlockOpResponsePro\
    to\x12+\n\x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.StatusR\x06sta\
    tus\x12\"\n\x0cfirstBadLink\x18\x02\x20\x01(\tR\x0cfirstBadLink\x12U\n\
    \x10checksumResponse\x18\x03\x20\x01(\x0b2).hadoop.hdfs.OpBlockChecksumR\
    esponseProtoR\x10checksumResponse\x12T\n\x12readOpChecksumInfo\x18\x04\
    \x20\x01(\x0b2$.hadoop.hdfs.ReadOpChecksumInfoProtoR\x12readOpChecksumIn\
    fo\x12\x18\n\x07message\x18\x05\x20\x01(\tR\x07message\x12<\n\x19shortCi\
    rcuitAccessVersion\x18\x06\x20\x01(\rR\x19shortCircuitAccessVersion\"D\n\
    \x15ClientReadStatusProto\x12+\n\x06status\x18\x01\x20\x02(\x0e2\x13.had\
    oop.hdfs.StatusR\x06status\"A\n\x12DNTransferAckProto\x12+\n\x06status\
    \x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.StatusR\x06status\"\xae\x01\n\x1c\
    OpBlockChecksumResponseProto\x12\x20\n\x0bbytesPerCrc\x18\x01\x20\x02(\r\
    R\x0bbytesPerCrc\x12\x20\n\x0bcrcPerBlock\x18\x02\x20\x02(\x04R\x0bcrcPe\
    rBlock\x12\x10\n\x03md5\x18\x03\x20\x02(\x0cR\x03md5\x128\n\x07crcType\
    \x18\x04\x20\x01(\x0e2\x1e.hadoop.hdfs.ChecksumTypeProtoR\x07crcType\"+\
    \n\rOpCustomProto\x12\x1a\n\x08customId\x18\x01\x20\x02(\tR\x08customId*\
    \x8c\x02\n\x06Status\x12\x0b\n\x07SUCCESS\x10\0\x12\t\n\x05ERROR\x10\x01\
    \x12\x12\n\x0eERROR_CHECKSUM\x10\x02\x12\x11\n\rERROR_INVALID\x10\x03\
    \x12\x10\n\x0cERROR_EXISTS\x10\x04\x12\x16\n\x12ERROR_ACCESS_TOKEN\x10\
    \x05\x12\x0f\n\x0bCHECKSUM_OK\x10\x06\x12\x15\n\x11ERROR_UNSUPPORTED\x10\
    \x07\x12\x0f\n\x0bOOB_RESTART\x10\x08\x12\x11\n\rOOB_RESERVED1\x10\t\x12\
    \x11\n\rOOB_RESERVED2\x10\n\x12\x11\n\rOOB_RESERVED3\x10\x0b\x12\x0f\n\
    \x0bIN_PROGRESS\x10\x0c\x12\x16\n\x12ERROR_BLOCK_PINNED\x10\r*[\n\x16Sho\
    rtCircuitFdResponse\x12#\n\x1fDO_NOT_USE_RECEIPT_VERIFICATION\x10\0\x12\
    \x1c\n\x18USE_RECEIPT_VERIFICATION\x10\x01B>\n%org.apache.hadoop.hdfs.pr\
    otocol.protoB\x12DataTransferProtos\xa0\x01\x01J\xefc\n\x07\x12\x05\x1b\
    \0\xc1\x02\x01\n\x08\n\x01\x08\x12\x03\x1b\0>\n\xbd\x08\n\x04\x08\xe7\
    \x07\0\x12\x03\x1b\0>2\x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\x20\
    Software\x20Foundation\x20(ASF)\x20under\x20one\n\x20or\x20more\x20contr\
    ibutor\x20license\x20agreements.\x20\x20See\x20the\x20NOTICE\x20file\n\
    \x20distributed\x20with\x20this\x20work\x20for\x20additional\x20informat\
    ion\n\x20regarding\x20copyright\x20ownership.\x20\x20The\x20ASF\x20licen\
    ses\x20this\x20file\n\x20to\x20you\x20under\x20the\x20Apache\x20License,\
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
    are\x20allowed\x20for\x20a\x20*stable*\x20.proto\x20interface.\n2}\x20Th\
    is\x20file\x20contains\x20protocol\x20buffers\x20that\x20are\x20used\x20\
    to\x20transfer\x20data\n\x20to\x20and\x20from\x20the\x20datanode,\x20as\
    \x20well\x20as\x20between\x20datanodes.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x1b\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x1b\x07\x13\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x1b\x07\x13\n\x0c\n\x05\x08\
    \xe7\x07\0\x07\x12\x03\x1b\x16=\n\x08\n\x01\x08\x12\x03\x1c\03\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\x1c\03\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\x1c\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x1c\x07\x1b\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x1c\x07\x1b\n\x0c\n\x05\x08\
    \xe7\x07\x01\x07\x12\x03\x1c\x1e2\n\x08\n\x01\x08\x12\x03\x1d\0,\n\x0b\n\
    \x04\x08\xe7\x07\x02\x12\x03\x1d\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\
    \x03\x1d\x07$\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1d\x07$\n\x0e\n\
    \x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1d\x07$\n\x0c\n\x05\x08\xe7\x07\
    \x02\x03\x12\x03\x1d'+\n\x08\n\x01\x02\x12\x03\x1e\x08\x13\n\t\n\x02\x03\
    \0\x12\x03\x20\x07\x17\n\t\n\x02\x03\x01\x12\x03!\x07\x13\n\n\n\x02\x04\
    \0\x12\x04#\0-\x01\n\n\n\x03\x04\0\x01\x12\x03#\x08)\n\x0c\n\x04\x04\0\
    \x04\0\x12\x04$\x02(\x03\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03$\x07\"\n\r\
    \n\x06\x04\0\x04\0\x02\0\x12\x03%\x04\x10\n\x0e\n\x07\x04\0\x04\0\x02\0\
    \x01\x12\x03%\x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03%\x0e\x0f\
    \n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03&\x04\x1a\n\x0e\n\x07\x04\0\x04\0\
    \x02\x01\x01\x12\x03&\x04\x15\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\
    \x03&\x18\x19\n\r\n\x06\x04\0\x04\0\x02\x02\x12\x03'\x04\x0e\n\x0e\n\x07\
    \x04\0\x04\0\x02\x02\x01\x12\x03'\x04\t\n\x0e\n\x07\x04\0\x04\0\x02\x02\
    \x02\x12\x03'\x0c\r\n\x0b\n\x04\x04\0\x02\0\x12\x03)\x022\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03)\x0b\
    &\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03)'-\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03)01\n\x0b\n\x04\x04\0\x02\x01\x12\x03*\x02\x1d\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03*\x0b\x10\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03*\x11\x18\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03*\x1b\x1c\n\x0b\n\x04\x04\0\x02\x02\x12\x03+\x02\x1e\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03+\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03+\x12\x19\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03+\x1c\x1d\n\x0b\n\x04\x04\0\x02\x03\x12\x03,\
    \x02.\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\0\x02\
    \x03\x06\x12\x03,\x0b\x1c\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03,\x1d)\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03,,-\n\n\n\x02\x04\x01\x12\x04/\03\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03/\x08\x17\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x030\x02(\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\
    \x04\x01\x02\0\x06\x12\x030\x0b\x1d\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x030\x1e#\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x030&'\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x031\x02.\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x031\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x031\x0b#\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x031$)\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x031,-\n\x0b\n\x04\
    \x04\x01\x02\x02\x12\x032\x024\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x032\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x032\x0b%\n\x0c\n\x05\x04\x01\
    \x02\x02\x01\x12\x032&/\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03223\n\n\n\
    \x02\x04\x02\x12\x045\08\x01\n\n\n\x03\x04\x02\x01\x12\x035\x08\"\n\x0b\
    \n\x04\x04\x02\x02\0\x12\x036\x02\x1e\n\x0c\n\x05\x04\x02\x02\0\x04\x12\
    \x036\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x036\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x036\x12\x19\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x036\x1c\x1d\n\x0b\n\x04\x04\x02\x02\x01\x12\x037\x02\x1f\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x037\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x037\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x037\x12\x1a\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x037\x1d\x1e\n\n\n\x02\x04\x03\x12\x04:\0=\
    \x01\n\n\n\x03\x04\x03\x01\x12\x03:\x08\"\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03;\x02*\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03;\x02\n\n\x0c\n\x05\x04\
    \x03\x02\0\x06\x12\x03;\x0b\x1a\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03;\
    \x1b%\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03;()\n\x0b\n\x04\x04\x03\x02\
    \x01\x12\x03<\x02!\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03<\x02\n\n\x0c\
    \n\x05\x04\x03\x02\x01\x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03<\x12\x1c\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03<\x1f\x20\n\
    \n\n\x02\x04\x04\x12\x04?\0B\x01\n\n\n\x03\x04\x04\x01\x12\x03?\x08\x1c\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03@\x02\x1f\n\x0c\n\x05\x04\x04\x02\0\
    \x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03@\x0b\x0f\n\x0c\
    \n\x05\x04\x04\x02\0\x01\x12\x03@\x10\x1a\n\x0c\n\x05\x04\x04\x02\0\x03\
    \x12\x03@\x1d\x1e\n\x0b\n\x04\x04\x04\x02\x01\x12\x03A\x02\x1f\n\x0c\n\
    \x05\x04\x04\x02\x01\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\
    \x12\x03A\x0b\x10\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03A\x11\x1a\n\x0c\
    \n\x05\x04\x04\x02\x01\x03\x12\x03A\x1d\x1e\n\n\n\x02\x04\x05\x12\x04D\0\
    J\x01\n\n\n\x03\x04\x05\x01\x12\x03D\x08\x18\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03E\x021\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03E\x02\n\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x03E\x0b%\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03E&\
    ,\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03E/0\n\x0b\n\x04\x04\x05\x02\x01\
    \x12\x03F\x02\x1d\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03F\x02\n\n\x0c\n\
    \x05\x04\x05\x02\x01\x05\x12\x03F\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\
    \x01\x12\x03F\x12\x18\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03F\x1b\x1c\n\
    \x0b\n\x04\x04\x05\x02\x02\x12\x03G\x02\x1a\n\x0c\n\x05\x04\x05\x02\x02\
    \x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03G\x0b\x11\n\
    \x0c\n\x05\x04\x05\x02\x02\x01\x12\x03G\x12\x15\n\x0c\n\x05\x04\x05\x02\
    \x02\x03\x12\x03G\x18\x19\n\x0b\n\x04\x04\x05\x02\x03\x12\x03H\x023\n\
    \x0c\n\x05\x04\x05\x02\x03\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x03\x05\x12\x03H\x0b\x0f\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03H\x10\
    \x1d\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03H\x20!\n\x0c\n\x05\x04\x05\
    \x02\x03\x08\x12\x03H\"2\n\x0c\n\x05\x04\x05\x02\x03\x07\x12\x03H-1\n\
    \x0b\n\x04\x04\x05\x02\x04\x12\x03I\x024\n\x0c\n\x05\x04\x05\x02\x04\x04\
    \x12\x03I\x02\n\n\x0c\n\x05\x04\x05\x02\x04\x06\x12\x03I\x0b\x1f\n\x0c\n\
    \x05\x04\x05\x02\x04\x01\x12\x03I\x20/\n\x0c\n\x05\x04\x05\x02\x04\x03\
    \x12\x03I23\n\n\n\x02\x04\x06\x12\x04L\0O\x01\n\n\n\x03\x04\x06\x01\x12\
    \x03L\x08\x15\n\x0b\n\x04\x04\x06\x02\0\x12\x03M\x02&\n\x0c\n\x05\x04\
    \x06\x02\0\x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03M\x0b\
    \x1c\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03M\x1d!\n\x0c\n\x05\x04\x06\x02\
    \0\x03\x12\x03M$%\n\x0b\n\x04\x04\x06\x02\x01\x12\x03N\x02'\n\x0c\n\x05\
    \x04\x06\x02\x01\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\
    \x03N\x0b\x11\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03N\x12\"\n\x0c\n\x05\
    \x04\x06\x02\x01\x03\x12\x03N%&\n\x0b\n\x02\x04\x07\x12\x05Q\0\x81\x01\
    \x01\n\n\n\x03\x04\x07\x01\x12\x03Q\x08\x19\n\x0b\n\x04\x04\x07\x02\0\
    \x12\x03R\x021\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03R\x02\n\n\x0c\n\x05\
    \x04\x07\x02\0\x06\x12\x03R\x0b%\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03R&\
    ,\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03R/0\n\x0b\n\x04\x04\x07\x02\x01\
    \x12\x03S\x02)\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03S\x02\n\n\x0c\n\
    \x05\x04\x07\x02\x01\x06\x12\x03S\x0b\x1c\n\x0c\n\x05\x04\x07\x02\x01\
    \x01\x12\x03S\x1d$\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03S'(\n\x0b\n\
    \x04\x04\x07\x02\x02\x12\x03T\x02(\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\
    \x03T\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x06\x12\x03T\x0b\x1c\n\x0c\n\x05\
    \x04\x07\x02\x02\x01\x12\x03T\x1d#\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\
    \x03T&'\n\x0c\n\x04\x04\x07\x04\0\x12\x04U\x02g\x03\n\x0c\n\x05\x04\x07\
    \x04\0\x01\x12\x03U\x07\x1d\n\r\n\x06\x04\x07\x04\0\x02\0\x12\x03V\x04\
    \x1e\n\x0e\n\x07\x04\x07\x04\0\x02\0\x01\x12\x03V\x04\x19\n\x0e\n\x07\
    \x04\x07\x04\0\x02\0\x02\x12\x03V\x1c\x1d\nJ\n\x06\x04\x07\x04\0\x02\x01\
    \x12\x03X\x04'\x1a;\x20pipeline\x20set\x20up\x20for\x20failed\x20PIPELIN\
    E_SETUP_APPEND\x20recovery\n\n\x0e\n\x07\x04\x07\x04\0\x02\x01\x01\x12\
    \x03X\x04\"\n\x0e\n\x07\x04\x07\x04\0\x02\x01\x02\x12\x03X%&\n\x1f\n\x06\
    \x04\x07\x04\0\x02\x02\x12\x03Z\x04\x17\x1a\x10\x20data\x20streaming\n\n\
    \x0e\n\x07\x04\x07\x04\0\x02\x02\x01\x12\x03Z\x04\x12\n\x0e\n\x07\x04\
    \x07\x04\0\x02\x02\x02\x12\x03Z\x15\x16\nB\n\x06\x04\x07\x04\0\x02\x03\
    \x12\x03\\\x04*\x1a3\x20pipeline\x20setup\x20for\x20failed\x20data\x20st\
    reaming\x20recovery\n\n\x0e\n\x07\x04\x07\x04\0\x02\x03\x01\x12\x03\\\
    \x04%\n\x0e\n\x07\x04\x07\x04\0\x02\x03\x02\x12\x03\\()\n-\n\x06\x04\x07\
    \x04\0\x02\x04\x12\x03^\x04\x17\x1a\x1e\x20close\x20the\x20block\x20and\
    \x20pipeline\n\n\x0e\n\x07\x04\x07\x04\0\x02\x04\x01\x12\x03^\x04\x12\n\
    \x0e\n\x07\x04\x07\x04\0\x02\x04\x02\x12\x03^\x15\x16\n0\n\x06\x04\x07\
    \x04\0\x02\x05\x12\x03`\x04\x20\x1a!\x20Recover\x20a\x20failed\x20PIPELI\
    NE_CLOSE\n\n\x0e\n\x07\x04\x07\x04\0\x02\x05\x01\x12\x03`\x04\x1b\n\x0e\
    \n\x07\x04\x07\x04\0\x02\x05\x02\x12\x03`\x1e\x1f\n3\n\x06\x04\x07\x04\0\
    \x02\x06\x12\x03b\x04\x1e\x1a$\x20pipeline\x20set\x20up\x20for\x20block\
    \x20creation\n\n\x0e\n\x07\x04\x07\x04\0\x02\x06\x01\x12\x03b\x04\x19\n\
    \x0e\n\x07\x04\x07\x04\0\x02\x06\x02\x12\x03b\x1c\x1d\n2\n\x06\x04\x07\
    \x04\0\x02\x07\x12\x03d\x04\x15\x1a#\x20transfer\x20RBW\x20for\x20adding\
    \x20datanodes\n\n\x0e\n\x07\x04\x07\x04\0\x02\x07\x01\x12\x03d\x04\x10\n\
    \x0e\n\x07\x04\x07\x04\0\x02\x07\x02\x12\x03d\x13\x14\n8\n\x06\x04\x07\
    \x04\0\x02\x08\x12\x03f\x04\x1b\x1a)\x20transfer\x20Finalized\x20for\x20\
    adding\x20datanodes\n\n\x0e\n\x07\x04\x07\x04\0\x02\x08\x01\x12\x03f\x04\
    \x16\n\x0e\n\x07\x04\x07\x04\0\x02\x08\x02\x12\x03f\x19\x1a\n\x0b\n\x04\
    \x04\x07\x02\x03\x12\x03h\x02,\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x03h\
    \x02\n\n\x0c\n\x05\x04\x07\x02\x03\x06\x12\x03h\x0b!\n\x0c\n\x05\x04\x07\
    \x02\x03\x01\x12\x03h\"'\n\x0c\n\x05\x04\x07\x02\x03\x03\x12\x03h*+\n\
    \x0b\n\x04\x04\x07\x02\x04\x12\x03i\x02#\n\x0c\n\x05\x04\x07\x02\x04\x04\
    \x12\x03i\x02\n\n\x0c\n\x05\x04\x07\x02\x04\x05\x12\x03i\x0b\x11\n\x0c\n\
    \x05\x04\x07\x02\x04\x01\x12\x03i\x12\x1e\n\x0c\n\x05\x04\x07\x02\x04\
    \x03\x12\x03i!\"\n\x0b\n\x04\x04\x07\x02\x05\x12\x03j\x02#\n\x0c\n\x05\
    \x04\x07\x02\x05\x04\x12\x03j\x02\n\n\x0c\n\x05\x04\x07\x02\x05\x05\x12\
    \x03j\x0b\x11\n\x0c\n\x05\x04\x07\x02\x05\x01\x12\x03j\x12\x1e\n\x0c\n\
    \x05\x04\x07\x02\x05\x03\x12\x03j!\"\n\x0b\n\x04\x04\x07\x02\x06\x12\x03\
    k\x02#\n\x0c\n\x05\x04\x07\x02\x06\x04\x12\x03k\x02\n\n\x0c\n\x05\x04\
    \x07\x02\x06\x05\x12\x03k\x0b\x11\n\x0c\n\x05\x04\x07\x02\x06\x01\x12\
    \x03k\x12\x1e\n\x0c\n\x05\x04\x07\x02\x06\x03\x12\x03k!\"\n\x0b\n\x04\
    \x04\x07\x02\x07\x12\x03l\x02,\n\x0c\n\x05\x04\x07\x02\x07\x04\x12\x03l\
    \x02\n\n\x0c\n\x05\x04\x07\x02\x07\x05\x12\x03l\x0b\x11\n\x0c\n\x05\x04\
    \x07\x02\x07\x01\x12\x03l\x12'\n\x0c\n\x05\x04\x07\x02\x07\x03\x12\x03l*\
    +\nG\n\x04\x04\x07\x02\x08\x12\x03q\x02/\x1a:*\n\x20The\x20requested\x20\
    checksum\x20mechanism\x20for\x20this\x20block\x20write.\n\n\x0c\n\x05\
    \x04\x07\x02\x08\x04\x12\x03q\x02\n\n\x0c\n\x05\x04\x07\x02\x08\x06\x12\
    \x03q\x0b\x18\n\x0c\n\x05\x04\x07\x02\x08\x01\x12\x03q\x19*\n\x0c\n\x05\
    \x04\x07\x02\x08\x03\x12\x03q-.\n\x0b\n\x04\x04\x07\x02\t\x12\x03r\x025\
    \n\x0c\n\x05\x04\x07\x02\t\x04\x12\x03r\x02\n\n\x0c\n\x05\x04\x07\x02\t\
    \x06\x12\x03r\x0b\x1f\n\x0c\n\x05\x04\x07\x02\t\x01\x12\x03r\x20/\n\x0c\
    \n\x05\x04\x07\x02\t\x03\x12\x03r24\n\x0b\n\x04\x04\x07\x02\n\x12\x03s\
    \x02>\n\x0c\n\x05\x04\x07\x02\n\x04\x12\x03s\x02\n\n\x0c\n\x05\x04\x07\
    \x02\n\x06\x12\x03s\x0b\x1b\n\x0c\n\x05\x04\x07\x02\n\x01\x12\x03s\x1c'\
    \n\x0c\n\x05\x04\x07\x02\n\x03\x12\x03s*,\n\x0c\n\x05\x04\x07\x02\n\x08\
    \x12\x03s-=\n\x0c\n\x05\x04\x07\x02\n\x07\x12\x03s8<\n\x0b\n\x04\x04\x07\
    \x02\x0b\x12\x03t\x024\n\x0c\n\x05\x04\x07\x02\x0b\x04\x12\x03t\x02\n\n\
    \x0c\n\x05\x04\x07\x02\x0b\x06\x12\x03t\x0b\x1b\n\x0c\n\x05\x04\x07\x02\
    \x0b\x01\x12\x03t\x1c.\n\x0c\n\x05\x04\x07\x02\x0b\x03\x12\x03t13\n\xaf\
    \x01\n\x04\x04\x07\x02\x0c\x12\x03{\x028\x1a\xa1\x01*\n\x20Hint\x20to\
    \x20the\x20DataNode\x20that\x20the\x20block\x20can\x20be\x20allocated\
    \x20on\x20transient\n\x20storage\x20i.e.\x20memory\x20and\x20written\x20\
    to\x20disk\x20lazily.\x20The\x20DataNode\x20is\x20free\n\x20to\x20ignore\
    \x20this\x20hint.\n\n\x0c\n\x05\x04\x07\x02\x0c\x04\x12\x03{\x02\n\n\x0c\
    \n\x05\x04\x07\x02\x0c\x05\x12\x03{\x0b\x0f\n\x0c\n\x05\x04\x07\x02\x0c\
    \x01\x12\x03{\x10\x20\n\x0c\n\x05\x04\x07\x02\x0c\x03\x12\x03{#%\n\x0c\n\
    \x05\x04\x07\x02\x0c\x08\x12\x03{&7\n\x0c\n\x05\x04\x07\x02\x0c\x07\x12\
    \x03{16\nB\n\x04\x04\x07\x02\r\x12\x03}\x02/\x1a5whether\x20to\x20pin\
    \x20the\x20block,\x20so\x20Balancer\x20won't\x20move\x20it.\n\n\x0c\n\
    \x05\x04\x07\x02\r\x04\x12\x03}\x02\n\n\x0c\n\x05\x04\x07\x02\r\x05\x12\
    \x03}\x0b\x0f\n\x0c\n\x05\x04\x07\x02\r\x01\x12\x03}\x10\x17\n\x0c\n\x05\
    \x04\x07\x02\r\x03\x12\x03}\x1a\x1c\n\x0c\n\x05\x04\x07\x02\r\x08\x12\
    \x03}\x1d.\n\x0c\n\x05\x04\x07\x02\r\x07\x12\x03}(-\n\x0b\n\x04\x04\x07\
    \x02\x0e\x12\x03~\x02$\n\x0c\n\x05\x04\x07\x02\x0e\x04\x12\x03~\x02\n\n\
    \x0c\n\x05\x04\x07\x02\x0e\x05\x12\x03~\x0b\x0f\n\x0c\n\x05\x04\x07\x02\
    \x0e\x01\x12\x03~\x10\x1e\n\x0c\n\x05\x04\x07\x02\x0e\x03\x12\x03~!#\n\
    \x0b\n\x04\x04\x07\x02\x0f\x12\x03\x7f\x02!\n\x0c\n\x05\x04\x07\x02\x0f\
    \x04\x12\x03\x7f\x02\n\n\x0c\n\x05\x04\x07\x02\x0f\x05\x12\x03\x7f\x0b\
    \x11\n\x0c\n\x05\x04\x07\x02\x0f\x01\x12\x03\x7f\x12\x1b\n\x0c\n\x05\x04\
    \x07\x02\x0f\x03\x12\x03\x7f\x1e\x20\n\x0c\n\x04\x04\x07\x02\x10\x12\x04\
    \x80\x01\x02(\n\r\n\x05\x04\x07\x02\x10\x04\x12\x04\x80\x01\x02\n\n\r\n\
    \x05\x04\x07\x02\x10\x05\x12\x04\x80\x01\x0b\x11\n\r\n\x05\x04\x07\x02\
    \x10\x01\x12\x04\x80\x01\x12\"\n\r\n\x05\x04\x07\x02\x10\x03\x12\x04\x80\
    \x01%'\n\x0c\n\x02\x04\x08\x12\x06\x83\x01\0\x88\x01\x01\n\x0b\n\x03\x04\
    \x08\x01\x12\x04\x83\x01\x08\x1c\n\x0c\n\x04\x04\x08\x02\0\x12\x04\x84\
    \x01\x021\n\r\n\x05\x04\x08\x02\0\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\
    \x04\x08\x02\0\x06\x12\x04\x84\x01\x0b%\n\r\n\x05\x04\x08\x02\0\x01\x12\
    \x04\x84\x01&,\n\r\n\x05\x04\x08\x02\0\x03\x12\x04\x84\x01/0\n\x0c\n\x04\
    \x04\x08\x02\x01\x12\x04\x85\x01\x02)\n\r\n\x05\x04\x08\x02\x01\x04\x12\
    \x04\x85\x01\x02\n\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\x85\x01\x0b\x1c\
    \n\r\n\x05\x04\x08\x02\x01\x01\x12\x04\x85\x01\x1d$\n\r\n\x05\x04\x08\
    \x02\x01\x03\x12\x04\x85\x01'(\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\x86\
    \x01\x023\n\r\n\x05\x04\x08\x02\x02\x04\x12\x04\x86\x01\x02\n\n\r\n\x05\
    \x04\x08\x02\x02\x06\x12\x04\x86\x01\x0b\x1b\n\r\n\x05\x04\x08\x02\x02\
    \x01\x12\x04\x86\x01\x1c.\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\x86\x011\
    2\n\x0c\n\x04\x04\x08\x02\x03\x12\x04\x87\x01\x02'\n\r\n\x05\x04\x08\x02\
    \x03\x04\x12\x04\x87\x01\x02\n\n\r\n\x05\x04\x08\x02\x03\x05\x12\x04\x87\
    \x01\x0b\x11\n\r\n\x05\x04\x08\x02\x03\x01\x12\x04\x87\x01\x12\"\n\r\n\
    \x05\x04\x08\x02\x03\x03\x12\x04\x87\x01%&\n\x0c\n\x02\x04\t\x12\x06\x8a\
    \x01\0\x90\x01\x01\n\x0b\n\x03\x04\t\x01\x12\x04\x8a\x01\x08\x1b\n\x0c\n\
    \x04\x04\t\x02\0\x12\x04\x8b\x01\x02&\n\r\n\x05\x04\t\x02\0\x04\x12\x04\
    \x8b\x01\x02\n\n\r\n\x05\x04\t\x02\0\x06\x12\x04\x8b\x01\x0b\x1a\n\r\n\
    \x05\x04\t\x02\0\x01\x12\x04\x8b\x01\x1b!\n\r\n\x05\x04\t\x02\0\x03\x12\
    \x04\x8b\x01$%\n\x0c\n\x04\x04\t\x02\x01\x12\x04\x8c\x01\x02\x1e\n\r\n\
    \x05\x04\t\x02\x01\x04\x12\x04\x8c\x01\x02\n\n\r\n\x05\x04\t\x02\x01\x05\
    \x12\x04\x8c\x01\x0b\x11\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\x8c\x01\x12\
    \x19\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x8c\x01\x1c\x1d\n\x0c\n\x04\x04\
    \t\x02\x02\x12\x04\x8d\x01\x02(\n\r\n\x05\x04\t\x02\x02\x04\x12\x04\x8d\
    \x01\x02\n\n\r\n\x05\x04\t\x02\x02\x06\x12\x04\x8d\x01\x0b\x1c\n\r\n\x05\
    \x04\t\x02\x02\x01\x12\x04\x8d\x01\x1d#\n\r\n\x05\x04\t\x02\x02\x03\x12\
    \x04\x8d\x01&'\n\x0c\n\x04\x04\t\x02\x03\x12\x04\x8e\x01\x02=\n\r\n\x05\
    \x04\t\x02\x03\x04\x12\x04\x8e\x01\x02\n\n\r\n\x05\x04\t\x02\x03\x06\x12\
    \x04\x8e\x01\x0b\x1b\n\r\n\x05\x04\t\x02\x03\x01\x12\x04\x8e\x01\x1c'\n\
    \r\n\x05\x04\t\x02\x03\x03\x12\x04\x8e\x01*+\n\r\n\x05\x04\t\x02\x03\x08\
    \x12\x04\x8e\x01,<\n\r\n\x05\x04\t\x02\x03\x07\x12\x04\x8e\x017;\n\x0c\n\
    \x04\x04\t\x02\x04\x12\x04\x8f\x01\x02\x20\n\r\n\x05\x04\t\x02\x04\x04\
    \x12\x04\x8f\x01\x02\n\n\r\n\x05\x04\t\x02\x04\x05\x12\x04\x8f\x01\x0b\
    \x11\n\r\n\x05\x04\t\x02\x04\x01\x12\x04\x8f\x01\x12\x1b\n\r\n\x05\x04\t\
    \x02\x04\x03\x12\x04\x8f\x01\x1e\x1f\n\x0c\n\x02\x04\n\x12\x06\x92\x01\0\
    \x94\x01\x01\n\x0b\n\x03\x04\n\x01\x12\x04\x92\x01\x08\x18\n\x0c\n\x04\
    \x04\n\x02\0\x12\x04\x93\x01\x02&\n\r\n\x05\x04\n\x02\0\x04\x12\x04\x93\
    \x01\x02\n\n\r\n\x05\x04\n\x02\0\x06\x12\x04\x93\x01\x0b\x1a\n\r\n\x05\
    \x04\n\x02\0\x01\x12\x04\x93\x01\x1b!\n\r\n\x05\x04\n\x02\0\x03\x12\x04\
    \x93\x01$%\n\x0c\n\x02\x04\x0b\x12\x06\x96\x01\0\x98\x01\x01\n\x0b\n\x03\
    \x04\x0b\x01\x12\x04\x96\x01\x08\x1c\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\
    \x97\x01\x02&\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\x97\x01\x02\n\n\r\n\
    \x05\x04\x0b\x02\0\x06\x12\x04\x97\x01\x0b\x1a\n\r\n\x05\x04\x0b\x02\0\
    \x01\x12\x04\x97\x01\x1b!\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\x97\x01$%\
    \n\x0c\n\x02\x04\x0c\x12\x06\x9a\x01\0\xa2\x01\x01\n\x0b\n\x03\x04\x0c\
    \x01\x12\x04\x9a\x01\x08!\n\x0c\n\x04\x04\x0c\x02\0\x12\x04\x9b\x01\x02&\
    \n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\x9b\x01\x02\n\n\r\n\x05\x04\x0c\x02\
    \0\x06\x12\x04\x9b\x01\x0b\x1a\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\x9b\
    \x01\x1b!\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\x9b\x01$%\n\x0c\n\x04\x04\
    \x0c\x02\x01\x12\x04\x9c\x01\x02,\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04\
    \x9c\x01\x02\n\n\r\n\x05\x04\x0c\x02\x01\x06\x12\x04\x9c\x01\x0b\x1d\n\r\
    \n\x05\x04\x0c\x02\x01\x01\x12\x04\x9c\x01\x1e'\n\r\n\x05\x04\x0c\x02\
    \x01\x03\x12\x04\x9c\x01*+\n5\n\x04\x04\x0c\x02\x02\x12\x04\x9e\x01\x024\
    \x1a'\x20each\x20internal\x20block\x20has\x20a\x20block\x20token\n\n\r\n\
    \x05\x04\x0c\x02\x02\x04\x12\x04\x9e\x01\x02\n\n\r\n\x05\x04\x0c\x02\x02\
    \x06\x12\x04\x9e\x01\x0b#\n\r\n\x05\x04\x0c\x02\x02\x01\x12\x04\x9e\x01$\
    /\n\r\n\x05\x04\x0c\x02\x02\x03\x12\x04\x9e\x0123\n\x0c\n\x04\x04\x0c\
    \x02\x03\x12\x04\x9f\x01\x021\n\r\n\x05\x04\x0c\x02\x03\x04\x12\x04\x9f\
    \x01\x02\n\n\r\n\x05\x04\x0c\x02\x03\x06\x12\x04\x9f\x01\x0b#\n\r\n\x05\
    \x04\x0c\x02\x03\x01\x12\x04\x9f\x01$,\n\r\n\x05\x04\x0c\x02\x03\x03\x12\
    \x04\x9f\x01/0\n\x0c\n\x04\x04\x0c\x02\x04\x12\x04\xa0\x01\x02#\n\r\n\
    \x05\x04\x0c\x02\x04\x04\x12\x04\xa0\x01\x02\n\n\r\n\x05\x04\x0c\x02\x04\
    \x05\x12\x04\xa0\x01\x0b\x11\n\r\n\x05\x04\x0c\x02\x04\x01\x12\x04\xa0\
    \x01\x12\x1e\n\r\n\x05\x04\x0c\x02\x04\x03\x12\x04\xa0\x01!\"\n\x0c\n\
    \x04\x04\x0c\x02\x05\x12\x04\xa1\x01\x02(\n\r\n\x05\x04\x0c\x02\x05\x04\
    \x12\x04\xa1\x01\x02\n\n\r\n\x05\x04\x0c\x02\x05\x05\x12\x04\xa1\x01\x0b\
    \x11\n\r\n\x05\x04\x0c\x02\x05\x01\x12\x04\xa1\x01\x12#\n\r\n\x05\x04\
    \x0c\x02\x05\x03\x12\x04\xa1\x01&'\nE\n\x02\x04\r\x12\x06\xa7\x01\0\xaa\
    \x01\x01\x1a7*\n\x20An\x20ID\x20uniquely\x20identifying\x20a\x20shared\
    \x20memory\x20segment.\n\n\x0b\n\x03\x04\r\x01\x12\x04\xa7\x01\x08\x1e\n\
    \x0c\n\x04\x04\r\x02\0\x12\x04\xa8\x01\x02\x18\n\r\n\x05\x04\r\x02\0\x04\
    \x12\x04\xa8\x01\x02\n\n\r\n\x05\x04\r\x02\0\x05\x12\x04\xa8\x01\x0b\x10\
    \n\r\n\x05\x04\r\x02\0\x01\x12\x04\xa8\x01\x11\x13\n\r\n\x05\x04\r\x02\0\
    \x03\x12\x04\xa8\x01\x16\x17\n\x0c\n\x04\x04\r\x02\x01\x12\x04\xa9\x01\
    \x02\x18\n\r\n\x05\x04\r\x02\x01\x04\x12\x04\xa9\x01\x02\n\n\r\n\x05\x04\
    \r\x02\x01\x05\x12\x04\xa9\x01\x0b\x10\n\r\n\x05\x04\r\x02\x01\x01\x12\
    \x04\xa9\x01\x11\x13\n\r\n\x05\x04\r\x02\x01\x03\x12\x04\xa9\x01\x16\x17\
    \nS\n\x02\x04\x0e\x12\x06\xaf\x01\0\xb2\x01\x01\x1aE*\n\x20An\x20ID\x20u\
    niquely\x20identifying\x20a\x20slot\x20within\x20a\x20shared\x20memory\
    \x20segment.\n\n\x0b\n\x03\x04\x0e\x01\x12\x04\xaf\x01\x08\x20\n\x0c\n\
    \x04\x04\x0e\x02\0\x12\x04\xb0\x01\x02,\n\r\n\x05\x04\x0e\x02\0\x04\x12\
    \x04\xb0\x01\x02\n\n\r\n\x05\x04\x0e\x02\0\x06\x12\x04\xb0\x01\x0b!\n\r\
    \n\x05\x04\x0e\x02\0\x01\x12\x04\xb0\x01\"'\n\r\n\x05\x04\x0e\x02\0\x03\
    \x12\x04\xb0\x01*+\n\x0c\n\x04\x04\x0e\x02\x01\x12\x04\xb1\x01\x02\x1d\n\
    \r\n\x05\x04\x0e\x02\x01\x04\x12\x04\xb1\x01\x02\n\n\r\n\x05\x04\x0e\x02\
    \x01\x05\x12\x04\xb1\x01\x0b\x10\n\r\n\x05\x04\x0e\x02\x01\x01\x12\x04\
    \xb1\x01\x11\x18\n\r\n\x05\x04\x0e\x02\x01\x03\x12\x04\xb1\x01\x1b\x1c\n\
    \x0c\n\x02\x04\x0f\x12\x06\xb4\x01\0\xc8\x01\x01\n\x0b\n\x03\x04\x0f\x01\
    \x12\x04\xb4\x01\x08(\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\xb5\x01\x02&\n\r\
    \n\x05\x04\x0f\x02\0\x04\x12\x04\xb5\x01\x02\n\n\r\n\x05\x04\x0f\x02\0\
    \x06\x12\x04\xb5\x01\x0b\x1a\n\r\n\x05\x04\x0f\x02\0\x01\x12\x04\xb5\x01\
    \x1b!\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xb5\x01$%\n\x8b\x02\n\x04\x04\
    \x0f\x02\x01\x12\x04\xbc\x01\x02!\x1a\xfc\x01*\x20In\x20order\x20to\x20g\
    et\x20short-circuit\x20access\x20to\x20block\x20data,\x20clients\x20must\
    \x20set\x20this\n\x20to\x20the\x20highest\x20version\x20of\x20the\x20blo\
    ck\x20data\x20that\x20they\x20can\x20understand.\n\x20Currently\x201\x20\
    is\x20the\x20only\x20version,\x20but\x20more\x20versions\x20may\x20exist\
    \x20in\x20the\x20future\n\x20if\x20the\x20on-disk\x20format\x20changes.\
    \n\n\r\n\x05\x04\x0f\x02\x01\x04\x12\x04\xbc\x01\x02\n\n\r\n\x05\x04\x0f\
    \x02\x01\x05\x12\x04\xbc\x01\x0b\x11\n\r\n\x05\x04\x0f\x02\x01\x01\x12\
    \x04\xbc\x01\x12\x1c\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\xbc\x01\x1f\
    \x20\nE\n\x04\x04\x0f\x02\x02\x12\x04\xc1\x01\x02/\x1a7*\n\x20The\x20sha\
    red\x20memory\x20slot\x20to\x20use,\x20if\x20we\x20are\x20using\x20one.\
    \n\n\r\n\x05\x04\x0f\x02\x02\x04\x12\x04\xc1\x01\x02\n\n\r\n\x05\x04\x0f\
    \x02\x02\x06\x12\x04\xc1\x01\x0b#\n\r\n\x05\x04\x0f\x02\x02\x01\x12\x04\
    \xc1\x01$*\n\r\n\x05\x04\x0f\x02\x02\x03\x12\x04\xc1\x01-.\nm\n\x04\x04\
    \x0f\x02\x03\x12\x04\xc7\x01\x02B\x1a_*\n\x20True\x20if\x20the\x20client\
    \x20supports\x20verifying\x20that\x20the\x20file\x20descriptor\x20has\
    \x20been\n\x20sent\x20successfully.\n\n\r\n\x05\x04\x0f\x02\x03\x04\x12\
    \x04\xc7\x01\x02\n\n\r\n\x05\x04\x0f\x02\x03\x05\x12\x04\xc7\x01\x0b\x0f\
    \n\r\n\x05\x04\x0f\x02\x03\x01\x12\x04\xc7\x01\x10+\n\r\n\x05\x04\x0f\
    \x02\x03\x03\x12\x04\xc7\x01./\n\r\n\x05\x04\x0f\x02\x03\x08\x12\x04\xc7\
    \x010A\n\r\n\x05\x04\x0f\x02\x03\x07\x12\x04\xc7\x01;@\n\x0c\n\x02\x04\
    \x10\x12\x06\xca\x01\0\xcd\x01\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\xca\
    \x01\x08-\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xcb\x01\x02/\n\r\n\x05\x04\
    \x10\x02\0\x04\x12\x04\xcb\x01\x02\n\n\r\n\x05\x04\x10\x02\0\x06\x12\x04\
    \xcb\x01\x0b#\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xcb\x01$*\n\r\n\x05\
    \x04\x10\x02\0\x03\x12\x04\xcb\x01-.\n\x0c\n\x04\x04\x10\x02\x01\x12\x04\
    \xcc\x01\x024\n\r\n\x05\x04\x10\x02\x01\x04\x12\x04\xcc\x01\x02\n\n\r\n\
    \x05\x04\x10\x02\x01\x06\x12\x04\xcc\x01\x0b%\n\r\n\x05\x04\x10\x02\x01\
    \x01\x12\x04\xcc\x01&/\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\xcc\x0123\n\
    \x0c\n\x02\x04\x11\x12\x06\xcf\x01\0\xd2\x01\x01\n\x0b\n\x03\x04\x11\x01\
    \x12\x04\xcf\x01\x08.\n\x0c\n\x04\x04\x11\x02\0\x12\x04\xd0\x01\x02\x1d\
    \n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xd0\x01\x02\n\n\r\n\x05\x04\x11\x02\
    \0\x06\x12\x04\xd0\x01\x0b\x11\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xd0\
    \x01\x12\x18\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xd0\x01\x1b\x1c\n\x0c\n\
    \x04\x04\x11\x02\x01\x12\x04\xd1\x01\x02\x1c\n\r\n\x05\x04\x11\x02\x01\
    \x04\x12\x04\xd1\x01\x02\n\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\xd1\x01\
    \x0b\x11\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xd1\x01\x12\x17\n\r\n\x05\
    \x04\x11\x02\x01\x03\x12\x04\xd1\x01\x1a\x1b\n\x0c\n\x02\x04\x12\x12\x06\
    \xd4\x01\0\xd9\x01\x01\n\x0b\n\x03\x04\x12\x01\x12\x04\xd4\x01\x08#\n\
    \x7f\n\x04\x04\x12\x02\0\x12\x04\xd7\x01\x02!\x1aq\x20The\x20name\x20of\
    \x20the\x20client\x20requesting\x20the\x20shared\x20memory\x20segment.\
    \x20\x20This\x20is\n\x20purely\x20for\x20logging\x20/\x20debugging\x20pu\
    rposes.\n\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xd7\x01\x02\n\n\r\n\x05\
    \x04\x12\x02\0\x05\x12\x04\xd7\x01\x0b\x11\n\r\n\x05\x04\x12\x02\0\x01\
    \x12\x04\xd7\x01\x12\x1c\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xd7\x01\x1f\
    \x20\n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xd8\x01\x024\n\r\n\x05\x04\x12\
    \x02\x01\x04\x12\x04\xd8\x01\x02\n\n\r\n\x05\x04\x12\x02\x01\x06\x12\x04\
    \xd8\x01\x0b%\n\r\n\x05\x04\x12\x02\x01\x01\x12\x04\xd8\x01&/\n\r\n\x05\
    \x04\x12\x02\x01\x03\x12\x04\xd8\x0123\n\x0c\n\x02\x04\x13\x12\x06\xdb\
    \x01\0\xdf\x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xdb\x01\x08$\n\x0c\n\
    \x04\x04\x13\x02\0\x12\x04\xdc\x01\x02\x1d\n\r\n\x05\x04\x13\x02\0\x04\
    \x12\x04\xdc\x01\x02\n\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\xdc\x01\x0b\
    \x11\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xdc\x01\x12\x18\n\r\n\x05\x04\
    \x13\x02\0\x03\x12\x04\xdc\x01\x1b\x1c\n\x0c\n\x04\x04\x13\x02\x01\x12\
    \x04\xdd\x01\x02\x1c\n\r\n\x05\x04\x13\x02\x01\x04\x12\x04\xdd\x01\x02\n\
    \n\r\n\x05\x04\x13\x02\x01\x05\x12\x04\xdd\x01\x0b\x11\n\r\n\x05\x04\x13\
    \x02\x01\x01\x12\x04\xdd\x01\x12\x17\n\r\n\x05\x04\x13\x02\x01\x03\x12\
    \x04\xdd\x01\x1a\x1b\n\x0c\n\x04\x04\x13\x02\x02\x12\x04\xde\x01\x02)\n\
    \r\n\x05\x04\x13\x02\x02\x04\x12\x04\xde\x01\x02\n\n\r\n\x05\x04\x13\x02\
    \x02\x06\x12\x04\xde\x01\x0b!\n\r\n\x05\x04\x13\x02\x02\x01\x12\x04\xde\
    \x01\"$\n\r\n\x05\x04\x13\x02\x02\x03\x12\x04\xde\x01'(\n\x0c\n\x02\x04\
    \x14\x12\x06\xe1\x01\0\xe8\x01\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\xe1\
    \x01\x08\x19\n0\n\x04\x04\x14\x02\0\x12\x04\xe3\x01\x02&\x1a\"\x20All\
    \x20fields\x20must\x20be\x20fixed-length!\n\n\r\n\x05\x04\x14\x02\0\x04\
    \x12\x04\xe3\x01\x02\n\n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xe3\x01\x0b\
    \x13\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\xe3\x01\x14!\n\r\n\x05\x04\x14\
    \x02\0\x03\x12\x04\xe3\x01$%\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\xe4\x01\
    \x02\x1e\n\r\n\x05\x04\x14\x02\x01\x04\x12\x04\xe4\x01\x02\n\n\r\n\x05\
    \x04\x14\x02\x01\x05\x12\x04\xe4\x01\x0b\x13\n\r\n\x05\x04\x14\x02\x01\
    \x01\x12\x04\xe4\x01\x14\x19\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xe4\
    \x01\x1c\x1d\n\x0c\n\x04\x04\x14\x02\x02\x12\x04\xe5\x01\x02&\n\r\n\x05\
    \x04\x14\x02\x02\x04\x12\x04\xe5\x01\x02\n\n\r\n\x05\x04\x14\x02\x02\x05\
    \x12\x04\xe5\x01\x0b\x0f\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\xe5\x01\
    \x10!\n\r\n\x05\x04\x14\x02\x02\x03\x12\x04\xe5\x01$%\n\x0c\n\x04\x04\
    \x14\x02\x03\x12\x04\xe6\x01\x02\x20\n\r\n\x05\x04\x14\x02\x03\x04\x12\
    \x04\xe6\x01\x02\n\n\r\n\x05\x04\x14\x02\x03\x05\x12\x04\xe6\x01\x0b\x13\
    \n\r\n\x05\x04\x14\x02\x03\x01\x12\x04\xe6\x01\x14\x1b\n\r\n\x05\x04\x14\
    \x02\x03\x03\x12\x04\xe6\x01\x1e\x1f\n\x0c\n\x04\x04\x14\x02\x04\x12\x04\
    \xe7\x01\x020\n\r\n\x05\x04\x14\x02\x04\x04\x12\x04\xe7\x01\x02\n\n\r\n\
    \x05\x04\x14\x02\x04\x05\x12\x04\xe7\x01\x0b\x0f\n\r\n\x05\x04\x14\x02\
    \x04\x01\x12\x04\xe7\x01\x10\x19\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\
    \xe7\x01\x1c\x1d\n\r\n\x05\x04\x14\x02\x04\x08\x12\x04\xe7\x01\x1e/\n\r\
    \n\x05\x04\x14\x02\x04\x07\x12\x04\xe7\x01).\n&\n\x02\x05\0\x12\x06\xeb\
    \x01\0\xfa\x01\x01\x1a\x18\x20Status\x20is\x20a\x204-bit\x20enum\n\n\x0b\
    \n\x03\x05\0\x01\x12\x04\xeb\x01\x05\x0b\n\x0c\n\x04\x05\0\x02\0\x12\x04\
    \xec\x01\x02\x0e\n\r\n\x05\x05\0\x02\0\x01\x12\x04\xec\x01\x02\t\n\r\n\
    \x05\x05\0\x02\0\x02\x12\x04\xec\x01\x0c\r\n\x0c\n\x04\x05\0\x02\x01\x12\
    \x04\xed\x01\x02\x0c\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\xed\x01\x02\x07\
    \n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xed\x01\n\x0b\n\x0c\n\x04\x05\0\x02\
    \x02\x12\x04\xee\x01\x02\x15\n\r\n\x05\x05\0\x02\x02\x01\x12\x04\xee\x01\
    \x02\x10\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\xee\x01\x13\x14\n\x0c\n\x04\
    \x05\0\x02\x03\x12\x04\xef\x01\x02\x14\n\r\n\x05\x05\0\x02\x03\x01\x12\
    \x04\xef\x01\x02\x0f\n\r\n\x05\x05\0\x02\x03\x02\x12\x04\xef\x01\x12\x13\
    \n\x0c\n\x04\x05\0\x02\x04\x12\x04\xf0\x01\x02\x13\n\r\n\x05\x05\0\x02\
    \x04\x01\x12\x04\xf0\x01\x02\x0e\n\r\n\x05\x05\0\x02\x04\x02\x12\x04\xf0\
    \x01\x11\x12\n\x0c\n\x04\x05\0\x02\x05\x12\x04\xf1\x01\x02\x19\n\r\n\x05\
    \x05\0\x02\x05\x01\x12\x04\xf1\x01\x02\x14\n\r\n\x05\x05\0\x02\x05\x02\
    \x12\x04\xf1\x01\x17\x18\n\x0c\n\x04\x05\0\x02\x06\x12\x04\xf2\x01\x02\
    \x12\n\r\n\x05\x05\0\x02\x06\x01\x12\x04\xf2\x01\x02\r\n\r\n\x05\x05\0\
    \x02\x06\x02\x12\x04\xf2\x01\x10\x11\n\x0c\n\x04\x05\0\x02\x07\x12\x04\
    \xf3\x01\x02\x18\n\r\n\x05\x05\0\x02\x07\x01\x12\x04\xf3\x01\x02\x13\n\r\
    \n\x05\x05\0\x02\x07\x02\x12\x04\xf3\x01\x16\x17\n\x1d\n\x04\x05\0\x02\
    \x08\x12\x04\xf4\x01\x02\x12\"\x0f\x20Quick\x20restart\n\n\r\n\x05\x05\0\
    \x02\x08\x01\x12\x04\xf4\x01\x02\r\n\r\n\x05\x05\0\x02\x08\x02\x12\x04\
    \xf4\x01\x10\x11\n\x18\n\x04\x05\0\x02\t\x12\x04\xf5\x01\x02\x14\"\n\x20\
    Reserved\n\n\r\n\x05\x05\0\x02\t\x01\x12\x04\xf5\x01\x02\x0f\n\r\n\x05\
    \x05\0\x02\t\x02\x12\x04\xf5\x01\x12\x13\n\x18\n\x04\x05\0\x02\n\x12\x04\
    \xf6\x01\x02\x15\"\n\x20Reserved\n\n\r\n\x05\x05\0\x02\n\x01\x12\x04\xf6\
    \x01\x02\x0f\n\r\n\x05\x05\0\x02\n\x02\x12\x04\xf6\x01\x12\x14\n\x18\n\
    \x04\x05\0\x02\x0b\x12\x04\xf7\x01\x02\x15\"\n\x20Reserved\n\n\r\n\x05\
    \x05\0\x02\x0b\x01\x12\x04\xf7\x01\x02\x0f\n\r\n\x05\x05\0\x02\x0b\x02\
    \x12\x04\xf7\x01\x12\x14\n\x0c\n\x04\x05\0\x02\x0c\x12\x04\xf8\x01\x02\
    \x13\n\r\n\x05\x05\0\x02\x0c\x01\x12\x04\xf8\x01\x02\r\n\r\n\x05\x05\0\
    \x02\x0c\x02\x12\x04\xf8\x01\x10\x12\n\x0c\n\x04\x05\0\x02\r\x12\x04\xf9\
    \x01\x02\x1a\n\r\n\x05\x05\0\x02\r\x01\x12\x04\xf9\x01\x02\x14\n\r\n\x05\
    \x05\0\x02\r\x02\x12\x04\xf9\x01\x17\x19\n\x0c\n\x02\x05\x01\x12\x06\xfc\
    \x01\0\xff\x01\x01\n\x0b\n\x03\x05\x01\x01\x12\x04\xfc\x01\x05\x1b\n\x0c\
    \n\x04\x05\x01\x02\0\x12\x04\xfd\x01\x02&\n\r\n\x05\x05\x01\x02\0\x01\
    \x12\x04\xfd\x01\x02!\n\r\n\x05\x05\x01\x02\0\x02\x12\x04\xfd\x01$%\n\
    \x0c\n\x04\x05\x01\x02\x01\x12\x04\xfe\x01\x02\x1f\n\r\n\x05\x05\x01\x02\
    \x01\x01\x12\x04\xfe\x01\x02\x1a\n\r\n\x05\x05\x01\x02\x01\x02\x12\x04\
    \xfe\x01\x1d\x1e\n\x0c\n\x02\x04\x15\x12\x06\x81\x02\0\x86\x02\x01\n\x0b\
    \n\x03\x04\x15\x01\x12\x04\x81\x02\x08\x18\n\x0c\n\x04\x04\x15\x02\0\x12\
    \x04\x82\x02\x02\x1c\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\x82\x02\x02\n\n\
    \r\n\x05\x04\x15\x02\0\x05\x12\x04\x82\x02\x0b\x11\n\r\n\x05\x04\x15\x02\
    \0\x01\x12\x04\x82\x02\x12\x17\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\x82\
    \x02\x1a\x1b\n\x0c\n\x04\x04\x15\x02\x01\x12\x04\x83\x02\x02\x1c\n\r\n\
    \x05\x04\x15\x02\x01\x04\x12\x04\x83\x02\x02\n\n\r\n\x05\x04\x15\x02\x01\
    \x06\x12\x04\x83\x02\x0b\x11\n\r\n\x05\x04\x15\x02\x01\x01\x12\x04\x83\
    \x02\x12\x17\n\r\n\x05\x04\x15\x02\x01\x03\x12\x04\x83\x02\x1a\x1b\n\x0c\
    \n\x04\x04\x15\x02\x02\x12\x04\x84\x02\x02;\n\r\n\x05\x04\x15\x02\x02\
    \x04\x12\x04\x84\x02\x02\n\n\r\n\x05\x04\x15\x02\x02\x05\x12\x04\x84\x02\
    \x0b\x11\n\r\n\x05\x04\x15\x02\x02\x01\x12\x04\x84\x02\x12(\n\r\n\x05\
    \x04\x15\x02\x02\x03\x12\x04\x84\x02+,\n\r\n\x05\x04\x15\x02\x02\x08\x12\
    \x04\x84\x02-:\n\r\n\x05\x04\x15\x02\x02\x07\x12\x04\x84\x0289\n\x0c\n\
    \x04\x04\x15\x02\x03\x12\x04\x85\x02\x02)\n\r\n\x05\x04\x15\x02\x03\x04\
    \x12\x04\x85\x02\x02\n\n\r\n\x05\x04\x15\x02\x03\x05\x12\x04\x85\x02\x0b\
    \x11\n\r\n\x05\x04\x15\x02\x03\x01\x12\x04\x85\x02\x12\x16\n\r\n\x05\x04\
    \x15\x02\x03\x03\x12\x04\x85\x02\x19\x1a\n\r\n\x05\x04\x15\x02\x03\x08\
    \x12\x04\x85\x02\x1b(\n\x10\n\x08\x04\x15\x02\x03\x08\xe7\x07\0\x12\x04\
    \x85\x02\x1c'\n\x11\n\t\x04\x15\x02\x03\x08\xe7\x07\0\x02\x12\x04\x85\
    \x02\x1c\"\n\x12\n\n\x04\x15\x02\x03\x08\xe7\x07\0\x02\0\x12\x04\x85\x02\
    \x1c\"\n\x13\n\x0b\x04\x15\x02\x03\x08\xe7\x07\0\x02\0\x01\x12\x04\x85\
    \x02\x1c\"\n\x11\n\t\x04\x15\x02\x03\x08\xe7\x07\0\x03\x12\x04\x85\x02#'\
    \ne\n\x02\x04\x16\x12\x06\x8c\x02\0\x95\x02\x01\x1aW*\n\x20Sent\x20as\
    \x20part\x20of\x20the\x20BlockOpResponseProto\n\x20for\x20READ_BLOCK\x20\
    and\x20COPY_BLOCK\x20operations.\n\n\x0b\n\x03\x04\x16\x01\x12\x04\x8c\
    \x02\x08\x1f\n\x0c\n\x04\x04\x16\x02\0\x12\x04\x8d\x02\x02&\n\r\n\x05\
    \x04\x16\x02\0\x04\x12\x04\x8d\x02\x02\n\n\r\n\x05\x04\x16\x02\0\x06\x12\
    \x04\x8d\x02\x0b\x18\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\x8d\x02\x19!\n\
    \r\n\x05\x04\x16\x02\0\x03\x12\x04\x8d\x02$%\n\xa5\x01\n\x04\x04\x16\x02\
    \x01\x12\x04\x94\x02\x02\"\x1a\x96\x01*\n\x20The\x20offset\x20into\x20th\
    e\x20block\x20at\x20which\x20the\x20first\x20packet\n\x20will\x20start.\
    \x20This\x20is\x20necessary\x20since\x20reads\x20will\x20align\n\x20back\
    wards\x20to\x20a\x20checksum\x20chunk\x20boundary.\n\n\r\n\x05\x04\x16\
    \x02\x01\x04\x12\x04\x94\x02\x02\n\n\r\n\x05\x04\x16\x02\x01\x05\x12\x04\
    \x94\x02\x0b\x11\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\x94\x02\x12\x1d\n\
    \r\n\x05\x04\x16\x02\x01\x03\x12\x04\x94\x02\x20!\n\x0c\n\x02\x04\x17\
    \x12\x06\x97\x02\0\xaa\x02\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\x97\x02\
    \x08\x1c\n\x0c\n\x04\x04\x17\x02\0\x12\x04\x98\x02\x02\x1d\n\r\n\x05\x04\
    \x17\x02\0\x04\x12\x04\x98\x02\x02\n\n\r\n\x05\x04\x17\x02\0\x06\x12\x04\
    \x98\x02\x0b\x11\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\x98\x02\x12\x18\n\r\
    \n\x05\x04\x17\x02\0\x03\x12\x04\x98\x02\x1b\x1c\n\x0c\n\x04\x04\x17\x02\
    \x01\x12\x04\x9a\x02\x02#\n\r\n\x05\x04\x17\x02\x01\x04\x12\x04\x9a\x02\
    \x02\n\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\x9a\x02\x0b\x11\n\r\n\x05\
    \x04\x17\x02\x01\x01\x12\x04\x9a\x02\x12\x1e\n\r\n\x05\x04\x17\x02\x01\
    \x03\x12\x04\x9a\x02!\"\n\x0c\n\x04\x04\x17\x02\x02\x12\x04\x9b\x02\x02=\
    \n\r\n\x05\x04\x17\x02\x02\x04\x12\x04\x9b\x02\x02\n\n\r\n\x05\x04\x17\
    \x02\x02\x06\x12\x04\x9b\x02\x0b'\n\r\n\x05\x04\x17\x02\x02\x01\x12\x04\
    \x9b\x02(8\n\r\n\x05\x04\x17\x02\x02\x03\x12\x04\x9b\x02;<\n\x0c\n\x04\
    \x04\x17\x02\x03\x12\x04\x9c\x02\x02:\n\r\n\x05\x04\x17\x02\x03\x04\x12\
    \x04\x9c\x02\x02\n\n\r\n\x05\x04\x17\x02\x03\x06\x12\x04\x9c\x02\x0b\"\n\
    \r\n\x05\x04\x17\x02\x03\x01\x12\x04\x9c\x02#5\n\r\n\x05\x04\x17\x02\x03\
    \x03\x12\x04\x9c\x0289\nO\n\x04\x04\x17\x02\x04\x12\x04\x9f\x02\x02\x1e\
    \x1aA*\x20explanatory\x20text\x20which\x20may\x20be\x20useful\x20to\x20l\
    og\x20on\x20the\x20client\x20side\x20\n\r\n\x05\x04\x17\x02\x04\x04\x12\
    \x04\x9f\x02\x02\n\n\r\n\x05\x04\x17\x02\x04\x05\x12\x04\x9f\x02\x0b\x11\
    \n\r\n\x05\x04\x17\x02\x04\x01\x12\x04\x9f\x02\x12\x19\n\r\n\x05\x04\x17\
    \x02\x04\x03\x12\x04\x9f\x02\x1c\x1d\n\xc7\x02\n\x04\x04\x17\x02\x05\x12\
    \x04\xa9\x02\x020\x1a\xb8\x02*\x20If\x20the\x20server\x20chooses\x20to\
    \x20agree\x20to\x20the\x20request\x20of\x20a\x20client\x20for\n\x20short\
    -circuit\x20access,\x20it\x20will\x20send\x20a\x20response\x20message\
    \x20with\x20the\x20relevant\n\x20file\x20descriptors\x20attached.\n\n\
    \x20In\x20the\x20body\x20of\x20the\x20message,\x20this\x20version\x20num\
    ber\x20will\x20be\x20set\x20to\x20the\n\x20specific\x20version\x20number\
    \x20of\x20the\x20block\x20data\x20that\x20the\x20client\x20is\x20about\
    \x20to\n\x20read.\n\n\r\n\x05\x04\x17\x02\x05\x04\x12\x04\xa9\x02\x02\n\
    \n\r\n\x05\x04\x17\x02\x05\x05\x12\x04\xa9\x02\x0b\x11\n\r\n\x05\x04\x17\
    \x02\x05\x01\x12\x04\xa9\x02\x12+\n\r\n\x05\x04\x17\x02\x05\x03\x12\x04\
    \xa9\x02./\n`\n\x02\x04\x18\x12\x06\xb0\x02\0\xb2\x02\x01\x1aR*\n\x20Mes\
    sage\x20sent\x20from\x20the\x20client\x20to\x20the\x20DN\x20after\x20rea\
    ding\x20the\x20entire\n\x20read\x20request.\n\n\x0b\n\x03\x04\x18\x01\
    \x12\x04\xb0\x02\x08\x1d\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xb1\x02\x02\
    \x1d\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\xb1\x02\x02\n\n\r\n\x05\x04\x18\
    \x02\0\x06\x12\x04\xb1\x02\x0b\x11\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\
    \xb1\x02\x12\x18\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xb1\x02\x1b\x1c\n\
    \x0c\n\x02\x04\x19\x12\x06\xb4\x02\0\xb6\x02\x01\n\x0b\n\x03\x04\x19\x01\
    \x12\x04\xb4\x02\x08\x1a\n\x0c\n\x04\x04\x19\x02\0\x12\x04\xb5\x02\x02\
    \x1d\n\r\n\x05\x04\x19\x02\0\x04\x12\x04\xb5\x02\x02\n\n\r\n\x05\x04\x19\
    \x02\0\x06\x12\x04\xb5\x02\x0b\x11\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\
    \xb5\x02\x12\x18\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xb5\x02\x1b\x1c\n\
    \x0c\n\x02\x04\x1a\x12\x06\xb8\x02\0\xbd\x02\x01\n\x0b\n\x03\x04\x1a\x01\
    \x12\x04\xb8\x02\x08$\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\xb9\x02\x02\"\n\
    \r\n\x05\x04\x1a\x02\0\x04\x12\x04\xb9\x02\x02\n\n\r\n\x05\x04\x1a\x02\0\
    \x05\x12\x04\xb9\x02\x0b\x11\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xb9\x02\
    \x12\x1d\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xb9\x02\x20!\n\x0c\n\x04\
    \x04\x1a\x02\x01\x12\x04\xba\x02\x02\"\n\r\n\x05\x04\x1a\x02\x01\x04\x12\
    \x04\xba\x02\x02\n\n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\xba\x02\x0b\x11\
    \n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\xba\x02\x12\x1d\n\r\n\x05\x04\x1a\
    \x02\x01\x03\x12\x04\xba\x02\x20!\n\x0c\n\x04\x04\x1a\x02\x02\x12\x04\
    \xbb\x02\x02\x19\n\r\n\x05\x04\x1a\x02\x02\x04\x12\x04\xbb\x02\x02\n\n\r\
    \n\x05\x04\x1a\x02\x02\x05\x12\x04\xbb\x02\x0b\x10\n\r\n\x05\x04\x1a\x02\
    \x02\x01\x12\x04\xbb\x02\x11\x14\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\
    \xbb\x02\x17\x18\n\x0c\n\x04\x04\x1a\x02\x03\x12\x04\xbc\x02\x02)\n\r\n\
    \x05\x04\x1a\x02\x03\x04\x12\x04\xbc\x02\x02\n\n\r\n\x05\x04\x1a\x02\x03\
    \x06\x12\x04\xbc\x02\x0b\x1c\n\r\n\x05\x04\x1a\x02\x03\x01\x12\x04\xbc\
    \x02\x1d$\n\r\n\x05\x04\x1a\x02\x03\x03\x12\x04\xbc\x02'(\n\x0c\n\x02\
    \x04\x1b\x12\x06\xbf\x02\0\xc1\x02\x01\n\x0b\n\x03\x04\x1b\x01\x12\x04\
    \xbf\x02\x08\x15\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\xc0\x02\x02\x1f\n\r\n\
    \x05\x04\x1b\x02\0\x04\x12\x04\xc0\x02\x02\n\n\r\n\x05\x04\x1b\x02\0\x05\
    \x12\x04\xc0\x02\x0b\x11\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\xc0\x02\x12\
    \x1a\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xc0\x02\x1d\x1e\
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
