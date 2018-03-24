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
pub struct ExtendedBlockProto {
    // message fields
    poolId: ::protobuf::SingularField<::std::string::String>,
    blockId: ::std::option::Option<u64>,
    generationStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtendedBlockProto {}

impl ExtendedBlockProto {
    pub fn new() -> ExtendedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtendedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<ExtendedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtendedBlockProto,
        };
        unsafe {
            instance.get(ExtendedBlockProto::new)
        }
    }

    // required string poolId = 1;

    pub fn clear_poolId(&mut self) {
        self.poolId.clear();
    }

    pub fn has_poolId(&self) -> bool {
        self.poolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poolId(&mut self, v: ::std::string::String) {
        self.poolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_poolId(&mut self) -> &mut ::std::string::String {
        if self.poolId.is_none() {
            self.poolId.set_default();
        }
        self.poolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_poolId(&mut self) -> ::std::string::String {
        self.poolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_poolId(&self) -> &str {
        match self.poolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_poolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.poolId
    }

    fn mut_poolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.poolId
    }

    // required uint64 blockId = 2;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    fn get_blockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockId
    }

    fn mut_blockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockId
    }

    // required uint64 generationStamp = 3;

    pub fn clear_generationStamp(&mut self) {
        self.generationStamp = ::std::option::Option::None;
    }

    pub fn has_generationStamp(&self) -> bool {
        self.generationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generationStamp(&mut self, v: u64) {
        self.generationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_generationStamp(&self) -> u64 {
        self.generationStamp.unwrap_or(0)
    }

    fn get_generationStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.generationStamp
    }

    fn mut_generationStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.generationStamp
    }

    // optional uint64 numBytes = 4;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }

    fn get_numBytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numBytes
    }

    fn mut_numBytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numBytes
    }
}

impl ::protobuf::Message for ExtendedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.poolId.is_none() {
            return false;
        }
        if self.blockId.is_none() {
            return false;
        }
        if self.generationStamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.poolId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.generationStamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numBytes = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.poolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.blockId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.generationStamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numBytes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.poolId.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.blockId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.generationStamp {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.numBytes {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for ExtendedBlockProto {
    fn new() -> ExtendedBlockProto {
        ExtendedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtendedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "poolId",
                    ExtendedBlockProto::get_poolId_for_reflect,
                    ExtendedBlockProto::mut_poolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockId",
                    ExtendedBlockProto::get_blockId_for_reflect,
                    ExtendedBlockProto::mut_blockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "generationStamp",
                    ExtendedBlockProto::get_generationStamp_for_reflect,
                    ExtendedBlockProto::mut_generationStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numBytes",
                    ExtendedBlockProto::get_numBytes_for_reflect,
                    ExtendedBlockProto::mut_numBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtendedBlockProto>(
                    "ExtendedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtendedBlockProto {
    fn clear(&mut self) {
        self.clear_poolId();
        self.clear_blockId();
        self.clear_generationStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExtendedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExtendedBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeIDProto {
    // message fields
    ipAddr: ::protobuf::SingularField<::std::string::String>,
    hostName: ::protobuf::SingularField<::std::string::String>,
    datanodeUuid: ::protobuf::SingularField<::std::string::String>,
    xferPort: ::std::option::Option<u32>,
    infoPort: ::std::option::Option<u32>,
    ipcPort: ::std::option::Option<u32>,
    infoSecurePort: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeIDProto {}

impl DatanodeIDProto {
    pub fn new() -> DatanodeIDProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeIDProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeIDProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeIDProto,
        };
        unsafe {
            instance.get(DatanodeIDProto::new)
        }
    }

    // required string ipAddr = 1;

    pub fn clear_ipAddr(&mut self) {
        self.ipAddr.clear();
    }

    pub fn has_ipAddr(&self) -> bool {
        self.ipAddr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipAddr(&mut self, v: ::std::string::String) {
        self.ipAddr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipAddr(&mut self) -> &mut ::std::string::String {
        if self.ipAddr.is_none() {
            self.ipAddr.set_default();
        }
        self.ipAddr.as_mut().unwrap()
    }

    // Take field
    pub fn take_ipAddr(&mut self) -> ::std::string::String {
        self.ipAddr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ipAddr(&self) -> &str {
        match self.ipAddr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ipAddr_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ipAddr
    }

    fn mut_ipAddr_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ipAddr
    }

    // required string hostName = 2;

    pub fn clear_hostName(&mut self) {
        self.hostName.clear();
    }

    pub fn has_hostName(&self) -> bool {
        self.hostName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostName(&mut self, v: ::std::string::String) {
        self.hostName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostName(&mut self) -> &mut ::std::string::String {
        if self.hostName.is_none() {
            self.hostName.set_default();
        }
        self.hostName.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostName(&mut self) -> ::std::string::String {
        self.hostName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostName(&self) -> &str {
        match self.hostName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostName
    }

    fn mut_hostName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostName
    }

    // required string datanodeUuid = 3;

    pub fn clear_datanodeUuid(&mut self) {
        self.datanodeUuid.clear();
    }

    pub fn has_datanodeUuid(&self) -> bool {
        self.datanodeUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanodeUuid(&mut self, v: ::std::string::String) {
        self.datanodeUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanodeUuid(&mut self) -> &mut ::std::string::String {
        if self.datanodeUuid.is_none() {
            self.datanodeUuid.set_default();
        }
        self.datanodeUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanodeUuid(&mut self) -> ::std::string::String {
        self.datanodeUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_datanodeUuid(&self) -> &str {
        match self.datanodeUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_datanodeUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.datanodeUuid
    }

    fn mut_datanodeUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.datanodeUuid
    }

    // required uint32 xferPort = 4;

    pub fn clear_xferPort(&mut self) {
        self.xferPort = ::std::option::Option::None;
    }

    pub fn has_xferPort(&self) -> bool {
        self.xferPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xferPort(&mut self, v: u32) {
        self.xferPort = ::std::option::Option::Some(v);
    }

    pub fn get_xferPort(&self) -> u32 {
        self.xferPort.unwrap_or(0)
    }

    fn get_xferPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xferPort
    }

    fn mut_xferPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xferPort
    }

    // required uint32 infoPort = 5;

    pub fn clear_infoPort(&mut self) {
        self.infoPort = ::std::option::Option::None;
    }

    pub fn has_infoPort(&self) -> bool {
        self.infoPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoPort(&mut self, v: u32) {
        self.infoPort = ::std::option::Option::Some(v);
    }

    pub fn get_infoPort(&self) -> u32 {
        self.infoPort.unwrap_or(0)
    }

    fn get_infoPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.infoPort
    }

    fn mut_infoPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.infoPort
    }

    // required uint32 ipcPort = 6;

    pub fn clear_ipcPort(&mut self) {
        self.ipcPort = ::std::option::Option::None;
    }

    pub fn has_ipcPort(&self) -> bool {
        self.ipcPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipcPort(&mut self, v: u32) {
        self.ipcPort = ::std::option::Option::Some(v);
    }

    pub fn get_ipcPort(&self) -> u32 {
        self.ipcPort.unwrap_or(0)
    }

    fn get_ipcPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ipcPort
    }

    fn mut_ipcPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ipcPort
    }

    // optional uint32 infoSecurePort = 7;

    pub fn clear_infoSecurePort(&mut self) {
        self.infoSecurePort = ::std::option::Option::None;
    }

    pub fn has_infoSecurePort(&self) -> bool {
        self.infoSecurePort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoSecurePort(&mut self, v: u32) {
        self.infoSecurePort = ::std::option::Option::Some(v);
    }

    pub fn get_infoSecurePort(&self) -> u32 {
        self.infoSecurePort.unwrap_or(0u32)
    }

    fn get_infoSecurePort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.infoSecurePort
    }

    fn mut_infoSecurePort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.infoSecurePort
    }
}

impl ::protobuf::Message for DatanodeIDProto {
    fn is_initialized(&self) -> bool {
        if self.ipAddr.is_none() {
            return false;
        }
        if self.hostName.is_none() {
            return false;
        }
        if self.datanodeUuid.is_none() {
            return false;
        }
        if self.xferPort.is_none() {
            return false;
        }
        if self.infoPort.is_none() {
            return false;
        }
        if self.ipcPort.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ipAddr)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.datanodeUuid)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xferPort = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.infoPort = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ipcPort = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.infoSecurePort = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.ipAddr.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.hostName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.datanodeUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.xferPort {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.infoPort {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ipcPort {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.infoSecurePort {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ipAddr.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.hostName.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.datanodeUuid.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.xferPort {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.infoPort {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.ipcPort {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.infoSecurePort {
            os.write_uint32(7, v)?;
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

impl ::protobuf::MessageStatic for DatanodeIDProto {
    fn new() -> DatanodeIDProto {
        DatanodeIDProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeIDProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ipAddr",
                    DatanodeIDProto::get_ipAddr_for_reflect,
                    DatanodeIDProto::mut_ipAddr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostName",
                    DatanodeIDProto::get_hostName_for_reflect,
                    DatanodeIDProto::mut_hostName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "datanodeUuid",
                    DatanodeIDProto::get_datanodeUuid_for_reflect,
                    DatanodeIDProto::mut_datanodeUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xferPort",
                    DatanodeIDProto::get_xferPort_for_reflect,
                    DatanodeIDProto::mut_xferPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "infoPort",
                    DatanodeIDProto::get_infoPort_for_reflect,
                    DatanodeIDProto::mut_infoPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ipcPort",
                    DatanodeIDProto::get_ipcPort_for_reflect,
                    DatanodeIDProto::mut_ipcPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "infoSecurePort",
                    DatanodeIDProto::get_infoSecurePort_for_reflect,
                    DatanodeIDProto::mut_infoSecurePort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeIDProto>(
                    "DatanodeIDProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeIDProto {
    fn clear(&mut self) {
        self.clear_ipAddr();
        self.clear_hostName();
        self.clear_datanodeUuid();
        self.clear_xferPort();
        self.clear_infoPort();
        self.clear_ipcPort();
        self.clear_infoSecurePort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeIDProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeIDProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeLocalInfoProto {
    // message fields
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    configVersion: ::protobuf::SingularField<::std::string::String>,
    uptime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeLocalInfoProto {}

impl DatanodeLocalInfoProto {
    pub fn new() -> DatanodeLocalInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeLocalInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeLocalInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeLocalInfoProto,
        };
        unsafe {
            instance.get(DatanodeLocalInfoProto::new)
        }
    }

    // required string softwareVersion = 1;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion(&mut self) -> &mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        }
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion(&self) -> &str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_softwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.softwareVersion
    }

    fn mut_softwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.softwareVersion
    }

    // required string configVersion = 2;

    pub fn clear_configVersion(&mut self) {
        self.configVersion.clear();
    }

    pub fn has_configVersion(&self) -> bool {
        self.configVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_configVersion(&mut self, v: ::std::string::String) {
        self.configVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_configVersion(&mut self) -> &mut ::std::string::String {
        if self.configVersion.is_none() {
            self.configVersion.set_default();
        }
        self.configVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_configVersion(&mut self) -> ::std::string::String {
        self.configVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_configVersion(&self) -> &str {
        match self.configVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_configVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.configVersion
    }

    fn mut_configVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.configVersion
    }

    // required uint64 uptime = 3;

    pub fn clear_uptime(&mut self) {
        self.uptime = ::std::option::Option::None;
    }

    pub fn has_uptime(&self) -> bool {
        self.uptime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uptime(&mut self, v: u64) {
        self.uptime = ::std::option::Option::Some(v);
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime.unwrap_or(0)
    }

    fn get_uptime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.uptime
    }

    fn mut_uptime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.uptime
    }
}

impl ::protobuf::Message for DatanodeLocalInfoProto {
    fn is_initialized(&self) -> bool {
        if self.softwareVersion.is_none() {
            return false;
        }
        if self.configVersion.is_none() {
            return false;
        }
        if self.uptime.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.softwareVersion)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.configVersion)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.uptime = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.softwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.configVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.uptime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.softwareVersion.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.configVersion.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.uptime {
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

impl ::protobuf::MessageStatic for DatanodeLocalInfoProto {
    fn new() -> DatanodeLocalInfoProto {
        DatanodeLocalInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeLocalInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "softwareVersion",
                    DatanodeLocalInfoProto::get_softwareVersion_for_reflect,
                    DatanodeLocalInfoProto::mut_softwareVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "configVersion",
                    DatanodeLocalInfoProto::get_configVersion_for_reflect,
                    DatanodeLocalInfoProto::mut_configVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uptime",
                    DatanodeLocalInfoProto::get_uptime_for_reflect,
                    DatanodeLocalInfoProto::mut_uptime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeLocalInfoProto>(
                    "DatanodeLocalInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeLocalInfoProto {
    fn clear(&mut self) {
        self.clear_softwareVersion();
        self.clear_configVersion();
        self.clear_uptime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeLocalInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeLocalInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeVolumeInfoProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    storageType: ::std::option::Option<StorageTypeProto>,
    usedSpace: ::std::option::Option<u64>,
    freeSpace: ::std::option::Option<u64>,
    reservedSpace: ::std::option::Option<u64>,
    reservedSpaceForReplicas: ::std::option::Option<u64>,
    numBlocks: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeVolumeInfoProto {}

impl DatanodeVolumeInfoProto {
    pub fn new() -> DatanodeVolumeInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeVolumeInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeVolumeInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeVolumeInfoProto,
        };
        unsafe {
            instance.get(DatanodeVolumeInfoProto::new)
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .hadoop.hdfs.StorageTypeProto storageType = 2;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> StorageTypeProto {
        self.storageType.unwrap_or(StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<StorageTypeProto> {
        &mut self.storageType
    }

    // required uint64 usedSpace = 3;

    pub fn clear_usedSpace(&mut self) {
        self.usedSpace = ::std::option::Option::None;
    }

    pub fn has_usedSpace(&self) -> bool {
        self.usedSpace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_usedSpace(&mut self, v: u64) {
        self.usedSpace = ::std::option::Option::Some(v);
    }

    pub fn get_usedSpace(&self) -> u64 {
        self.usedSpace.unwrap_or(0)
    }

    fn get_usedSpace_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.usedSpace
    }

    fn mut_usedSpace_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.usedSpace
    }

    // required uint64 freeSpace = 4;

    pub fn clear_freeSpace(&mut self) {
        self.freeSpace = ::std::option::Option::None;
    }

    pub fn has_freeSpace(&self) -> bool {
        self.freeSpace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_freeSpace(&mut self, v: u64) {
        self.freeSpace = ::std::option::Option::Some(v);
    }

    pub fn get_freeSpace(&self) -> u64 {
        self.freeSpace.unwrap_or(0)
    }

    fn get_freeSpace_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.freeSpace
    }

    fn mut_freeSpace_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.freeSpace
    }

    // required uint64 reservedSpace = 5;

    pub fn clear_reservedSpace(&mut self) {
        self.reservedSpace = ::std::option::Option::None;
    }

    pub fn has_reservedSpace(&self) -> bool {
        self.reservedSpace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reservedSpace(&mut self, v: u64) {
        self.reservedSpace = ::std::option::Option::Some(v);
    }

    pub fn get_reservedSpace(&self) -> u64 {
        self.reservedSpace.unwrap_or(0)
    }

    fn get_reservedSpace_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.reservedSpace
    }

    fn mut_reservedSpace_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.reservedSpace
    }

    // required uint64 reservedSpaceForReplicas = 6;

    pub fn clear_reservedSpaceForReplicas(&mut self) {
        self.reservedSpaceForReplicas = ::std::option::Option::None;
    }

    pub fn has_reservedSpaceForReplicas(&self) -> bool {
        self.reservedSpaceForReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reservedSpaceForReplicas(&mut self, v: u64) {
        self.reservedSpaceForReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_reservedSpaceForReplicas(&self) -> u64 {
        self.reservedSpaceForReplicas.unwrap_or(0)
    }

    fn get_reservedSpaceForReplicas_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.reservedSpaceForReplicas
    }

    fn mut_reservedSpaceForReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.reservedSpaceForReplicas
    }

    // required uint64 numBlocks = 7;

    pub fn clear_numBlocks(&mut self) {
        self.numBlocks = ::std::option::Option::None;
    }

    pub fn has_numBlocks(&self) -> bool {
        self.numBlocks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBlocks(&mut self, v: u64) {
        self.numBlocks = ::std::option::Option::Some(v);
    }

    pub fn get_numBlocks(&self) -> u64 {
        self.numBlocks.unwrap_or(0)
    }

    fn get_numBlocks_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numBlocks
    }

    fn mut_numBlocks_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numBlocks
    }
}

impl ::protobuf::Message for DatanodeVolumeInfoProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        }
        if self.storageType.is_none() {
            return false;
        }
        if self.usedSpace.is_none() {
            return false;
        }
        if self.freeSpace.is_none() {
            return false;
        }
        if self.reservedSpace.is_none() {
            return false;
        }
        if self.reservedSpaceForReplicas.is_none() {
            return false;
        }
        if self.numBlocks.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.usedSpace = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.freeSpace = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.reservedSpace = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.reservedSpaceForReplicas = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numBlocks = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.usedSpace {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.freeSpace {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reservedSpace {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reservedSpaceForReplicas {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numBlocks {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.usedSpace {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.freeSpace {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.reservedSpace {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.reservedSpaceForReplicas {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.numBlocks {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for DatanodeVolumeInfoProto {
    fn new() -> DatanodeVolumeInfoProto {
        DatanodeVolumeInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeVolumeInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    DatanodeVolumeInfoProto::get_path_for_reflect,
                    DatanodeVolumeInfoProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageType",
                    DatanodeVolumeInfoProto::get_storageType_for_reflect,
                    DatanodeVolumeInfoProto::mut_storageType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "usedSpace",
                    DatanodeVolumeInfoProto::get_usedSpace_for_reflect,
                    DatanodeVolumeInfoProto::mut_usedSpace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "freeSpace",
                    DatanodeVolumeInfoProto::get_freeSpace_for_reflect,
                    DatanodeVolumeInfoProto::mut_freeSpace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "reservedSpace",
                    DatanodeVolumeInfoProto::get_reservedSpace_for_reflect,
                    DatanodeVolumeInfoProto::mut_reservedSpace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "reservedSpaceForReplicas",
                    DatanodeVolumeInfoProto::get_reservedSpaceForReplicas_for_reflect,
                    DatanodeVolumeInfoProto::mut_reservedSpaceForReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numBlocks",
                    DatanodeVolumeInfoProto::get_numBlocks_for_reflect,
                    DatanodeVolumeInfoProto::mut_numBlocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeVolumeInfoProto>(
                    "DatanodeVolumeInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeVolumeInfoProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_storageType();
        self.clear_usedSpace();
        self.clear_freeSpace();
        self.clear_reservedSpace();
        self.clear_reservedSpaceForReplicas();
        self.clear_numBlocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeVolumeInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeVolumeInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeInfosProto {
    // message fields
    datanodes: ::protobuf::RepeatedField<DatanodeInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeInfosProto {}

impl DatanodeInfosProto {
    pub fn new() -> DatanodeInfosProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfosProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfosProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfosProto,
        };
        unsafe {
            instance.get(DatanodeInfosProto::new)
        }
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto datanodes = 1;

    pub fn clear_datanodes(&mut self) {
        self.datanodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_datanodes(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.datanodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_datanodes(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.datanodes
    }

    // Take field
    pub fn take_datanodes(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.datanodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_datanodes(&self) -> &[DatanodeInfoProto] {
        &self.datanodes
    }

    fn get_datanodes_for_reflect(&self) -> &::protobuf::RepeatedField<DatanodeInfoProto> {
        &self.datanodes
    }

    fn mut_datanodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.datanodes
    }
}

impl ::protobuf::Message for DatanodeInfosProto {
    fn is_initialized(&self) -> bool {
        for v in &self.datanodes {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.datanodes)?;
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
        for value in &self.datanodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.datanodes {
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

impl ::protobuf::MessageStatic for DatanodeInfosProto {
    fn new() -> DatanodeInfosProto {
        DatanodeInfosProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfosProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeInfoProto>>(
                    "datanodes",
                    DatanodeInfosProto::get_datanodes_for_reflect,
                    DatanodeInfosProto::mut_datanodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfosProto>(
                    "DatanodeInfosProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfosProto {
    fn clear(&mut self) {
        self.clear_datanodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeInfosProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfosProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeInfoProto {
    // message fields
    id: ::protobuf::SingularPtrField<DatanodeIDProto>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    lastUpdate: ::std::option::Option<u64>,
    xceiverCount: ::std::option::Option<u32>,
    location: ::protobuf::SingularField<::std::string::String>,
    nonDfsUsed: ::std::option::Option<u64>,
    adminState: ::std::option::Option<DatanodeInfoProto_AdminState>,
    cacheCapacity: ::std::option::Option<u64>,
    cacheUsed: ::std::option::Option<u64>,
    lastUpdateMonotonic: ::std::option::Option<u64>,
    upgradeDomain: ::protobuf::SingularField<::std::string::String>,
    lastBlockReportTime: ::std::option::Option<u64>,
    lastBlockReportMonotonic: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeInfoProto {}

impl DatanodeInfoProto {
    pub fn new() -> DatanodeInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfoProto,
        };
        unsafe {
            instance.get(DatanodeInfoProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeIDProto id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: DatanodeIDProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut DatanodeIDProto {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> DatanodeIDProto {
        self.id.take().unwrap_or_else(|| DatanodeIDProto::new())
    }

    pub fn get_id(&self) -> &DatanodeIDProto {
        self.id.as_ref().unwrap_or_else(|| DatanodeIDProto::default_instance())
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeIDProto> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeIDProto> {
        &mut self.id
    }

    // optional uint64 capacity = 2;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    fn get_capacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.capacity
    }

    // optional uint64 dfsUsed = 3;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    fn get_dfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dfsUsed
    }

    fn mut_dfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dfsUsed
    }

    // optional uint64 remaining = 4;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    fn get_remaining_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.remaining
    }

    fn mut_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.remaining
    }

    // optional uint64 blockPoolUsed = 5;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    fn get_blockPoolUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockPoolUsed
    }

    fn mut_blockPoolUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockPoolUsed
    }

    // optional uint64 lastUpdate = 6;

    pub fn clear_lastUpdate(&mut self) {
        self.lastUpdate = ::std::option::Option::None;
    }

    pub fn has_lastUpdate(&self) -> bool {
        self.lastUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdate(&mut self, v: u64) {
        self.lastUpdate = ::std::option::Option::Some(v);
    }

    pub fn get_lastUpdate(&self) -> u64 {
        self.lastUpdate.unwrap_or(0u64)
    }

    fn get_lastUpdate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastUpdate
    }

    fn mut_lastUpdate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastUpdate
    }

    // optional uint32 xceiverCount = 7;

    pub fn clear_xceiverCount(&mut self) {
        self.xceiverCount = ::std::option::Option::None;
    }

    pub fn has_xceiverCount(&self) -> bool {
        self.xceiverCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xceiverCount(&mut self, v: u32) {
        self.xceiverCount = ::std::option::Option::Some(v);
    }

    pub fn get_xceiverCount(&self) -> u32 {
        self.xceiverCount.unwrap_or(0u32)
    }

    fn get_xceiverCount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xceiverCount
    }

    fn mut_xceiverCount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xceiverCount
    }

    // optional string location = 8;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.location
    }

    // optional uint64 nonDfsUsed = 9;

    pub fn clear_nonDfsUsed(&mut self) {
        self.nonDfsUsed = ::std::option::Option::None;
    }

    pub fn has_nonDfsUsed(&self) -> bool {
        self.nonDfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonDfsUsed(&mut self, v: u64) {
        self.nonDfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_nonDfsUsed(&self) -> u64 {
        self.nonDfsUsed.unwrap_or(0)
    }

    fn get_nonDfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nonDfsUsed
    }

    fn mut_nonDfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nonDfsUsed
    }

    // optional .hadoop.hdfs.DatanodeInfoProto.AdminState adminState = 10;

    pub fn clear_adminState(&mut self) {
        self.adminState = ::std::option::Option::None;
    }

    pub fn has_adminState(&self) -> bool {
        self.adminState.is_some()
    }

    // Param is passed by value, moved
    pub fn set_adminState(&mut self, v: DatanodeInfoProto_AdminState) {
        self.adminState = ::std::option::Option::Some(v);
    }

    pub fn get_adminState(&self) -> DatanodeInfoProto_AdminState {
        self.adminState.unwrap_or(DatanodeInfoProto_AdminState::NORMAL)
    }

    fn get_adminState_for_reflect(&self) -> &::std::option::Option<DatanodeInfoProto_AdminState> {
        &self.adminState
    }

    fn mut_adminState_for_reflect(&mut self) -> &mut ::std::option::Option<DatanodeInfoProto_AdminState> {
        &mut self.adminState
    }

    // optional uint64 cacheCapacity = 11;

    pub fn clear_cacheCapacity(&mut self) {
        self.cacheCapacity = ::std::option::Option::None;
    }

    pub fn has_cacheCapacity(&self) -> bool {
        self.cacheCapacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheCapacity(&mut self, v: u64) {
        self.cacheCapacity = ::std::option::Option::Some(v);
    }

    pub fn get_cacheCapacity(&self) -> u64 {
        self.cacheCapacity.unwrap_or(0u64)
    }

    fn get_cacheCapacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheCapacity
    }

    fn mut_cacheCapacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheCapacity
    }

    // optional uint64 cacheUsed = 12;

    pub fn clear_cacheUsed(&mut self) {
        self.cacheUsed = ::std::option::Option::None;
    }

    pub fn has_cacheUsed(&self) -> bool {
        self.cacheUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheUsed(&mut self, v: u64) {
        self.cacheUsed = ::std::option::Option::Some(v);
    }

    pub fn get_cacheUsed(&self) -> u64 {
        self.cacheUsed.unwrap_or(0u64)
    }

    fn get_cacheUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheUsed
    }

    fn mut_cacheUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheUsed
    }

    // optional uint64 lastUpdateMonotonic = 13;

    pub fn clear_lastUpdateMonotonic(&mut self) {
        self.lastUpdateMonotonic = ::std::option::Option::None;
    }

    pub fn has_lastUpdateMonotonic(&self) -> bool {
        self.lastUpdateMonotonic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdateMonotonic(&mut self, v: u64) {
        self.lastUpdateMonotonic = ::std::option::Option::Some(v);
    }

    pub fn get_lastUpdateMonotonic(&self) -> u64 {
        self.lastUpdateMonotonic.unwrap_or(0u64)
    }

    fn get_lastUpdateMonotonic_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastUpdateMonotonic
    }

    fn mut_lastUpdateMonotonic_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastUpdateMonotonic
    }

    // optional string upgradeDomain = 14;

    pub fn clear_upgradeDomain(&mut self) {
        self.upgradeDomain.clear();
    }

    pub fn has_upgradeDomain(&self) -> bool {
        self.upgradeDomain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgradeDomain(&mut self, v: ::std::string::String) {
        self.upgradeDomain = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgradeDomain(&mut self) -> &mut ::std::string::String {
        if self.upgradeDomain.is_none() {
            self.upgradeDomain.set_default();
        }
        self.upgradeDomain.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgradeDomain(&mut self) -> ::std::string::String {
        self.upgradeDomain.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_upgradeDomain(&self) -> &str {
        match self.upgradeDomain.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_upgradeDomain_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.upgradeDomain
    }

    fn mut_upgradeDomain_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.upgradeDomain
    }

    // optional uint64 lastBlockReportTime = 15;

    pub fn clear_lastBlockReportTime(&mut self) {
        self.lastBlockReportTime = ::std::option::Option::None;
    }

    pub fn has_lastBlockReportTime(&self) -> bool {
        self.lastBlockReportTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastBlockReportTime(&mut self, v: u64) {
        self.lastBlockReportTime = ::std::option::Option::Some(v);
    }

    pub fn get_lastBlockReportTime(&self) -> u64 {
        self.lastBlockReportTime.unwrap_or(0u64)
    }

    fn get_lastBlockReportTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastBlockReportTime
    }

    fn mut_lastBlockReportTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastBlockReportTime
    }

    // optional uint64 lastBlockReportMonotonic = 16;

    pub fn clear_lastBlockReportMonotonic(&mut self) {
        self.lastBlockReportMonotonic = ::std::option::Option::None;
    }

    pub fn has_lastBlockReportMonotonic(&self) -> bool {
        self.lastBlockReportMonotonic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastBlockReportMonotonic(&mut self, v: u64) {
        self.lastBlockReportMonotonic = ::std::option::Option::Some(v);
    }

    pub fn get_lastBlockReportMonotonic(&self) -> u64 {
        self.lastBlockReportMonotonic.unwrap_or(0u64)
    }

    fn get_lastBlockReportMonotonic_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastBlockReportMonotonic
    }

    fn mut_lastBlockReportMonotonic_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastBlockReportMonotonic
    }
}

impl ::protobuf::Message for DatanodeInfoProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastUpdate = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xceiverCount = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nonDfsUsed = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.adminState = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheCapacity = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheUsed = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastUpdateMonotonic = ::std::option::Option::Some(tmp);
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.upgradeDomain)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastBlockReportTime = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastBlockReportMonotonic = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.capacity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dfsUsed {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blockPoolUsed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastUpdate {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.xceiverCount {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.nonDfsUsed {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.adminState {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if let Some(v) = self.cacheCapacity {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cacheUsed {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastUpdateMonotonic {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.upgradeDomain.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        if let Some(v) = self.lastBlockReportTime {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastBlockReportMonotonic {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.capacity {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.dfsUsed {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.remaining {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.blockPoolUsed {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.lastUpdate {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.xceiverCount {
            os.write_uint32(7, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.nonDfsUsed {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.adminState {
            os.write_enum(10, v.value())?;
        }
        if let Some(v) = self.cacheCapacity {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.cacheUsed {
            os.write_uint64(12, v)?;
        }
        if let Some(v) = self.lastUpdateMonotonic {
            os.write_uint64(13, v)?;
        }
        if let Some(ref v) = self.upgradeDomain.as_ref() {
            os.write_string(14, &v)?;
        }
        if let Some(v) = self.lastBlockReportTime {
            os.write_uint64(15, v)?;
        }
        if let Some(v) = self.lastBlockReportMonotonic {
            os.write_uint64(16, v)?;
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

impl ::protobuf::MessageStatic for DatanodeInfoProto {
    fn new() -> DatanodeInfoProto {
        DatanodeInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeIDProto>>(
                    "id",
                    DatanodeInfoProto::get_id_for_reflect,
                    DatanodeInfoProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "capacity",
                    DatanodeInfoProto::get_capacity_for_reflect,
                    DatanodeInfoProto::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dfsUsed",
                    DatanodeInfoProto::get_dfsUsed_for_reflect,
                    DatanodeInfoProto::mut_dfsUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "remaining",
                    DatanodeInfoProto::get_remaining_for_reflect,
                    DatanodeInfoProto::mut_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockPoolUsed",
                    DatanodeInfoProto::get_blockPoolUsed_for_reflect,
                    DatanodeInfoProto::mut_blockPoolUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastUpdate",
                    DatanodeInfoProto::get_lastUpdate_for_reflect,
                    DatanodeInfoProto::mut_lastUpdate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xceiverCount",
                    DatanodeInfoProto::get_xceiverCount_for_reflect,
                    DatanodeInfoProto::mut_xceiverCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    DatanodeInfoProto::get_location_for_reflect,
                    DatanodeInfoProto::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nonDfsUsed",
                    DatanodeInfoProto::get_nonDfsUsed_for_reflect,
                    DatanodeInfoProto::mut_nonDfsUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DatanodeInfoProto_AdminState>>(
                    "adminState",
                    DatanodeInfoProto::get_adminState_for_reflect,
                    DatanodeInfoProto::mut_adminState_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheCapacity",
                    DatanodeInfoProto::get_cacheCapacity_for_reflect,
                    DatanodeInfoProto::mut_cacheCapacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheUsed",
                    DatanodeInfoProto::get_cacheUsed_for_reflect,
                    DatanodeInfoProto::mut_cacheUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastUpdateMonotonic",
                    DatanodeInfoProto::get_lastUpdateMonotonic_for_reflect,
                    DatanodeInfoProto::mut_lastUpdateMonotonic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "upgradeDomain",
                    DatanodeInfoProto::get_upgradeDomain_for_reflect,
                    DatanodeInfoProto::mut_upgradeDomain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastBlockReportTime",
                    DatanodeInfoProto::get_lastBlockReportTime_for_reflect,
                    DatanodeInfoProto::mut_lastBlockReportTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastBlockReportMonotonic",
                    DatanodeInfoProto::get_lastBlockReportMonotonic_for_reflect,
                    DatanodeInfoProto::mut_lastBlockReportMonotonic_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfoProto>(
                    "DatanodeInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfoProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_lastUpdate();
        self.clear_xceiverCount();
        self.clear_location();
        self.clear_nonDfsUsed();
        self.clear_adminState();
        self.clear_cacheCapacity();
        self.clear_cacheUsed();
        self.clear_lastUpdateMonotonic();
        self.clear_upgradeDomain();
        self.clear_lastBlockReportTime();
        self.clear_lastBlockReportMonotonic();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeInfoProto_AdminState {
    NORMAL = 0,
    DECOMMISSION_INPROGRESS = 1,
    DECOMMISSIONED = 2,
    ENTERING_MAINTENANCE = 3,
    IN_MAINTENANCE = 4,
}

impl ::protobuf::ProtobufEnum for DatanodeInfoProto_AdminState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeInfoProto_AdminState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSION_INPROGRESS),
            2 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSIONED),
            3 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::ENTERING_MAINTENANCE),
            4 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::IN_MAINTENANCE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DatanodeInfoProto_AdminState] = &[
            DatanodeInfoProto_AdminState::NORMAL,
            DatanodeInfoProto_AdminState::DECOMMISSION_INPROGRESS,
            DatanodeInfoProto_AdminState::DECOMMISSIONED,
            DatanodeInfoProto_AdminState::ENTERING_MAINTENANCE,
            DatanodeInfoProto_AdminState::IN_MAINTENANCE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DatanodeInfoProto_AdminState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeInfoProto_AdminState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeInfoProto_AdminState {
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfoProto_AdminState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeStorageProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<DatanodeStorageProto_StorageState>,
    storageType: ::std::option::Option<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeStorageProto {}

impl DatanodeStorageProto {
    pub fn new() -> DatanodeStorageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeStorageProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeStorageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeStorageProto,
        };
        unsafe {
            instance.get(DatanodeStorageProto::new)
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid(&mut self) -> &mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        }
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid(&self) -> &str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageUuid
    }

    fn mut_storageUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageUuid
    }

    // optional .hadoop.hdfs.DatanodeStorageProto.StorageState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: DatanodeStorageProto_StorageState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> DatanodeStorageProto_StorageState {
        self.state.unwrap_or(DatanodeStorageProto_StorageState::NORMAL)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<DatanodeStorageProto_StorageState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<DatanodeStorageProto_StorageState> {
        &mut self.state
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 3;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> StorageTypeProto {
        self.storageType.unwrap_or(StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<StorageTypeProto> {
        &mut self.storageType
    }
}

impl ::protobuf::Message for DatanodeStorageProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.storageUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for DatanodeStorageProto {
    fn new() -> DatanodeStorageProto {
        DatanodeStorageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeStorageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    DatanodeStorageProto::get_storageUuid_for_reflect,
                    DatanodeStorageProto::mut_storageUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DatanodeStorageProto_StorageState>>(
                    "state",
                    DatanodeStorageProto::get_state_for_reflect,
                    DatanodeStorageProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageType",
                    DatanodeStorageProto::get_storageType_for_reflect,
                    DatanodeStorageProto::mut_storageType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeStorageProto>(
                    "DatanodeStorageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeStorageProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_state();
        self.clear_storageType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeStorageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeStorageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeStorageProto_StorageState {
    NORMAL = 0,
    READ_ONLY_SHARED = 1,
}

impl ::protobuf::ProtobufEnum for DatanodeStorageProto_StorageState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeStorageProto_StorageState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::READ_ONLY_SHARED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DatanodeStorageProto_StorageState] = &[
            DatanodeStorageProto_StorageState::NORMAL,
            DatanodeStorageProto_StorageState::READ_ONLY_SHARED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DatanodeStorageProto_StorageState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeStorageProto_StorageState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeStorageProto_StorageState {
}

impl ::protobuf::reflect::ProtobufValue for DatanodeStorageProto_StorageState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageReportProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    failed: ::std::option::Option<bool>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    storage: ::protobuf::SingularPtrField<DatanodeStorageProto>,
    nonDfsUsed: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageReportProto {}

impl StorageReportProto {
    pub fn new() -> StorageReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageReportProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageReportProto,
        };
        unsafe {
            instance.get(StorageReportProto::new)
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid(&mut self) -> &mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        }
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid(&self) -> &str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageUuid
    }

    fn mut_storageUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageUuid
    }

    // optional bool failed = 2;

    pub fn clear_failed(&mut self) {
        self.failed = ::std::option::Option::None;
    }

    pub fn has_failed(&self) -> bool {
        self.failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failed(&mut self, v: bool) {
        self.failed = ::std::option::Option::Some(v);
    }

    pub fn get_failed(&self) -> bool {
        self.failed.unwrap_or(false)
    }

    fn get_failed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.failed
    }

    fn mut_failed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.failed
    }

    // optional uint64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    fn get_capacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.capacity
    }

    // optional uint64 dfsUsed = 4;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    fn get_dfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dfsUsed
    }

    fn mut_dfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dfsUsed
    }

    // optional uint64 remaining = 5;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    fn get_remaining_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.remaining
    }

    fn mut_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.remaining
    }

    // optional uint64 blockPoolUsed = 6;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    fn get_blockPoolUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockPoolUsed
    }

    fn mut_blockPoolUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockPoolUsed
    }

    // optional .hadoop.hdfs.DatanodeStorageProto storage = 7;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: DatanodeStorageProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage(&mut self) -> &mut DatanodeStorageProto {
        if self.storage.is_none() {
            self.storage.set_default();
        }
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> DatanodeStorageProto {
        self.storage.take().unwrap_or_else(|| DatanodeStorageProto::new())
    }

    pub fn get_storage(&self) -> &DatanodeStorageProto {
        self.storage.as_ref().unwrap_or_else(|| DatanodeStorageProto::default_instance())
    }

    fn get_storage_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeStorageProto> {
        &self.storage
    }

    fn mut_storage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeStorageProto> {
        &mut self.storage
    }

    // optional uint64 nonDfsUsed = 8;

    pub fn clear_nonDfsUsed(&mut self) {
        self.nonDfsUsed = ::std::option::Option::None;
    }

    pub fn has_nonDfsUsed(&self) -> bool {
        self.nonDfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonDfsUsed(&mut self, v: u64) {
        self.nonDfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_nonDfsUsed(&self) -> u64 {
        self.nonDfsUsed.unwrap_or(0)
    }

    fn get_nonDfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nonDfsUsed
    }

    fn mut_nonDfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nonDfsUsed
    }
}

impl ::protobuf::Message for StorageReportProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        }
        for v in &self.storage {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.failed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storage)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nonDfsUsed = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.storageUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.failed {
            my_size += 2;
        }
        if let Some(v) = self.capacity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dfsUsed {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blockPoolUsed {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.storage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.nonDfsUsed {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.failed {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.capacity {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.dfsUsed {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.remaining {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.blockPoolUsed {
            os.write_uint64(6, v)?;
        }
        if let Some(ref v) = self.storage.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.nonDfsUsed {
            os.write_uint64(8, v)?;
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

impl ::protobuf::MessageStatic for StorageReportProto {
    fn new() -> StorageReportProto {
        StorageReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    StorageReportProto::get_storageUuid_for_reflect,
                    StorageReportProto::mut_storageUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "failed",
                    StorageReportProto::get_failed_for_reflect,
                    StorageReportProto::mut_failed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "capacity",
                    StorageReportProto::get_capacity_for_reflect,
                    StorageReportProto::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dfsUsed",
                    StorageReportProto::get_dfsUsed_for_reflect,
                    StorageReportProto::mut_dfsUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "remaining",
                    StorageReportProto::get_remaining_for_reflect,
                    StorageReportProto::mut_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockPoolUsed",
                    StorageReportProto::get_blockPoolUsed_for_reflect,
                    StorageReportProto::mut_blockPoolUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeStorageProto>>(
                    "storage",
                    StorageReportProto::get_storage_for_reflect,
                    StorageReportProto::mut_storage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nonDfsUsed",
                    StorageReportProto::get_nonDfsUsed_for_reflect,
                    StorageReportProto::mut_nonDfsUsed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageReportProto>(
                    "StorageReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageReportProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_failed();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_storage();
        self.clear_nonDfsUsed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageReportProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContentSummaryProto {
    // message fields
    length: ::std::option::Option<u64>,
    fileCount: ::std::option::Option<u64>,
    directoryCount: ::std::option::Option<u64>,
    quota: ::std::option::Option<u64>,
    spaceConsumed: ::std::option::Option<u64>,
    spaceQuota: ::std::option::Option<u64>,
    typeQuotaInfos: ::protobuf::SingularPtrField<StorageTypeQuotaInfosProto>,
    snapshotLength: ::std::option::Option<u64>,
    snapshotFileCount: ::std::option::Option<u64>,
    snapshotDirectoryCount: ::std::option::Option<u64>,
    snapshotSpaceConsumed: ::std::option::Option<u64>,
    erasureCodingPolicy: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContentSummaryProto {}

impl ContentSummaryProto {
    pub fn new() -> ContentSummaryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContentSummaryProto {
        static mut instance: ::protobuf::lazy::Lazy<ContentSummaryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContentSummaryProto,
        };
        unsafe {
            instance.get(ContentSummaryProto::new)
        }
    }

    // required uint64 length = 1;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // required uint64 fileCount = 2;

    pub fn clear_fileCount(&mut self) {
        self.fileCount = ::std::option::Option::None;
    }

    pub fn has_fileCount(&self) -> bool {
        self.fileCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileCount(&mut self, v: u64) {
        self.fileCount = ::std::option::Option::Some(v);
    }

    pub fn get_fileCount(&self) -> u64 {
        self.fileCount.unwrap_or(0)
    }

    fn get_fileCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileCount
    }

    fn mut_fileCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileCount
    }

    // required uint64 directoryCount = 3;

    pub fn clear_directoryCount(&mut self) {
        self.directoryCount = ::std::option::Option::None;
    }

    pub fn has_directoryCount(&self) -> bool {
        self.directoryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_directoryCount(&mut self, v: u64) {
        self.directoryCount = ::std::option::Option::Some(v);
    }

    pub fn get_directoryCount(&self) -> u64 {
        self.directoryCount.unwrap_or(0)
    }

    fn get_directoryCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.directoryCount
    }

    fn mut_directoryCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.directoryCount
    }

    // required uint64 quota = 4;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    fn get_quota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.quota
    }

    // required uint64 spaceConsumed = 5;

    pub fn clear_spaceConsumed(&mut self) {
        self.spaceConsumed = ::std::option::Option::None;
    }

    pub fn has_spaceConsumed(&self) -> bool {
        self.spaceConsumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceConsumed(&mut self, v: u64) {
        self.spaceConsumed = ::std::option::Option::Some(v);
    }

    pub fn get_spaceConsumed(&self) -> u64 {
        self.spaceConsumed.unwrap_or(0)
    }

    fn get_spaceConsumed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceConsumed
    }

    fn mut_spaceConsumed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceConsumed
    }

    // required uint64 spaceQuota = 6;

    pub fn clear_spaceQuota(&mut self) {
        self.spaceQuota = ::std::option::Option::None;
    }

    pub fn has_spaceQuota(&self) -> bool {
        self.spaceQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceQuota(&mut self, v: u64) {
        self.spaceQuota = ::std::option::Option::Some(v);
    }

    pub fn get_spaceQuota(&self) -> u64 {
        self.spaceQuota.unwrap_or(0)
    }

    fn get_spaceQuota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceQuota
    }

    fn mut_spaceQuota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceQuota
    }

    // optional .hadoop.hdfs.StorageTypeQuotaInfosProto typeQuotaInfos = 7;

    pub fn clear_typeQuotaInfos(&mut self) {
        self.typeQuotaInfos.clear();
    }

    pub fn has_typeQuotaInfos(&self) -> bool {
        self.typeQuotaInfos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typeQuotaInfos(&mut self, v: StorageTypeQuotaInfosProto) {
        self.typeQuotaInfos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typeQuotaInfos(&mut self) -> &mut StorageTypeQuotaInfosProto {
        if self.typeQuotaInfos.is_none() {
            self.typeQuotaInfos.set_default();
        }
        self.typeQuotaInfos.as_mut().unwrap()
    }

    // Take field
    pub fn take_typeQuotaInfos(&mut self) -> StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.take().unwrap_or_else(|| StorageTypeQuotaInfosProto::new())
    }

    pub fn get_typeQuotaInfos(&self) -> &StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.as_ref().unwrap_or_else(|| StorageTypeQuotaInfosProto::default_instance())
    }

    fn get_typeQuotaInfos_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypeQuotaInfosProto> {
        &self.typeQuotaInfos
    }

    fn mut_typeQuotaInfos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypeQuotaInfosProto> {
        &mut self.typeQuotaInfos
    }

    // optional uint64 snapshotLength = 8;

    pub fn clear_snapshotLength(&mut self) {
        self.snapshotLength = ::std::option::Option::None;
    }

    pub fn has_snapshotLength(&self) -> bool {
        self.snapshotLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotLength(&mut self, v: u64) {
        self.snapshotLength = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotLength(&self) -> u64 {
        self.snapshotLength.unwrap_or(0)
    }

    fn get_snapshotLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.snapshotLength
    }

    fn mut_snapshotLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.snapshotLength
    }

    // optional uint64 snapshotFileCount = 9;

    pub fn clear_snapshotFileCount(&mut self) {
        self.snapshotFileCount = ::std::option::Option::None;
    }

    pub fn has_snapshotFileCount(&self) -> bool {
        self.snapshotFileCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotFileCount(&mut self, v: u64) {
        self.snapshotFileCount = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotFileCount(&self) -> u64 {
        self.snapshotFileCount.unwrap_or(0)
    }

    fn get_snapshotFileCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.snapshotFileCount
    }

    fn mut_snapshotFileCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.snapshotFileCount
    }

    // optional uint64 snapshotDirectoryCount = 10;

    pub fn clear_snapshotDirectoryCount(&mut self) {
        self.snapshotDirectoryCount = ::std::option::Option::None;
    }

    pub fn has_snapshotDirectoryCount(&self) -> bool {
        self.snapshotDirectoryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotDirectoryCount(&mut self, v: u64) {
        self.snapshotDirectoryCount = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotDirectoryCount(&self) -> u64 {
        self.snapshotDirectoryCount.unwrap_or(0)
    }

    fn get_snapshotDirectoryCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.snapshotDirectoryCount
    }

    fn mut_snapshotDirectoryCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.snapshotDirectoryCount
    }

    // optional uint64 snapshotSpaceConsumed = 11;

    pub fn clear_snapshotSpaceConsumed(&mut self) {
        self.snapshotSpaceConsumed = ::std::option::Option::None;
    }

    pub fn has_snapshotSpaceConsumed(&self) -> bool {
        self.snapshotSpaceConsumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotSpaceConsumed(&mut self, v: u64) {
        self.snapshotSpaceConsumed = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotSpaceConsumed(&self) -> u64 {
        self.snapshotSpaceConsumed.unwrap_or(0)
    }

    fn get_snapshotSpaceConsumed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.snapshotSpaceConsumed
    }

    fn mut_snapshotSpaceConsumed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.snapshotSpaceConsumed
    }

    // optional string erasureCodingPolicy = 12;

    pub fn clear_erasureCodingPolicy(&mut self) {
        self.erasureCodingPolicy.clear();
    }

    pub fn has_erasureCodingPolicy(&self) -> bool {
        self.erasureCodingPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_erasureCodingPolicy(&mut self, v: ::std::string::String) {
        self.erasureCodingPolicy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_erasureCodingPolicy(&mut self) -> &mut ::std::string::String {
        if self.erasureCodingPolicy.is_none() {
            self.erasureCodingPolicy.set_default();
        }
        self.erasureCodingPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_erasureCodingPolicy(&mut self) -> ::std::string::String {
        self.erasureCodingPolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_erasureCodingPolicy(&self) -> &str {
        match self.erasureCodingPolicy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_erasureCodingPolicy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.erasureCodingPolicy
    }

    fn mut_erasureCodingPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.erasureCodingPolicy
    }
}

impl ::protobuf::Message for ContentSummaryProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
            return false;
        }
        if self.fileCount.is_none() {
            return false;
        }
        if self.directoryCount.is_none() {
            return false;
        }
        if self.quota.is_none() {
            return false;
        }
        if self.spaceConsumed.is_none() {
            return false;
        }
        if self.spaceQuota.is_none() {
            return false;
        }
        for v in &self.typeQuotaInfos {
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
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileCount = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.directoryCount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceConsumed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceQuota = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.typeQuotaInfos)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.snapshotLength = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.snapshotFileCount = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.snapshotDirectoryCount = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.snapshotSpaceConsumed = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.erasureCodingPolicy)?;
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
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fileCount {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.directoryCount {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quota {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceConsumed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceQuota {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.typeQuotaInfos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.snapshotLength {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.snapshotFileCount {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.snapshotDirectoryCount {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.snapshotSpaceConsumed {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.erasureCodingPolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.fileCount {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.directoryCount {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.quota {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.spaceConsumed {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.spaceQuota {
            os.write_uint64(6, v)?;
        }
        if let Some(ref v) = self.typeQuotaInfos.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.snapshotLength {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.snapshotFileCount {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.snapshotDirectoryCount {
            os.write_uint64(10, v)?;
        }
        if let Some(v) = self.snapshotSpaceConsumed {
            os.write_uint64(11, v)?;
        }
        if let Some(ref v) = self.erasureCodingPolicy.as_ref() {
            os.write_string(12, &v)?;
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

impl ::protobuf::MessageStatic for ContentSummaryProto {
    fn new() -> ContentSummaryProto {
        ContentSummaryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContentSummaryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    ContentSummaryProto::get_length_for_reflect,
                    ContentSummaryProto::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileCount",
                    ContentSummaryProto::get_fileCount_for_reflect,
                    ContentSummaryProto::mut_fileCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "directoryCount",
                    ContentSummaryProto::get_directoryCount_for_reflect,
                    ContentSummaryProto::mut_directoryCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    ContentSummaryProto::get_quota_for_reflect,
                    ContentSummaryProto::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceConsumed",
                    ContentSummaryProto::get_spaceConsumed_for_reflect,
                    ContentSummaryProto::mut_spaceConsumed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceQuota",
                    ContentSummaryProto::get_spaceQuota_for_reflect,
                    ContentSummaryProto::mut_spaceQuota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypeQuotaInfosProto>>(
                    "typeQuotaInfos",
                    ContentSummaryProto::get_typeQuotaInfos_for_reflect,
                    ContentSummaryProto::mut_typeQuotaInfos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "snapshotLength",
                    ContentSummaryProto::get_snapshotLength_for_reflect,
                    ContentSummaryProto::mut_snapshotLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "snapshotFileCount",
                    ContentSummaryProto::get_snapshotFileCount_for_reflect,
                    ContentSummaryProto::mut_snapshotFileCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "snapshotDirectoryCount",
                    ContentSummaryProto::get_snapshotDirectoryCount_for_reflect,
                    ContentSummaryProto::mut_snapshotDirectoryCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "snapshotSpaceConsumed",
                    ContentSummaryProto::get_snapshotSpaceConsumed_for_reflect,
                    ContentSummaryProto::mut_snapshotSpaceConsumed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "erasureCodingPolicy",
                    ContentSummaryProto::get_erasureCodingPolicy_for_reflect,
                    ContentSummaryProto::mut_erasureCodingPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContentSummaryProto>(
                    "ContentSummaryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContentSummaryProto {
    fn clear(&mut self) {
        self.clear_length();
        self.clear_fileCount();
        self.clear_directoryCount();
        self.clear_quota();
        self.clear_spaceConsumed();
        self.clear_spaceQuota();
        self.clear_typeQuotaInfos();
        self.clear_snapshotLength();
        self.clear_snapshotFileCount();
        self.clear_snapshotDirectoryCount();
        self.clear_snapshotSpaceConsumed();
        self.clear_erasureCodingPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContentSummaryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContentSummaryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QuotaUsageProto {
    // message fields
    fileAndDirectoryCount: ::std::option::Option<u64>,
    quota: ::std::option::Option<u64>,
    spaceConsumed: ::std::option::Option<u64>,
    spaceQuota: ::std::option::Option<u64>,
    typeQuotaInfos: ::protobuf::SingularPtrField<StorageTypeQuotaInfosProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QuotaUsageProto {}

impl QuotaUsageProto {
    pub fn new() -> QuotaUsageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QuotaUsageProto {
        static mut instance: ::protobuf::lazy::Lazy<QuotaUsageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QuotaUsageProto,
        };
        unsafe {
            instance.get(QuotaUsageProto::new)
        }
    }

    // required uint64 fileAndDirectoryCount = 1;

    pub fn clear_fileAndDirectoryCount(&mut self) {
        self.fileAndDirectoryCount = ::std::option::Option::None;
    }

    pub fn has_fileAndDirectoryCount(&self) -> bool {
        self.fileAndDirectoryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileAndDirectoryCount(&mut self, v: u64) {
        self.fileAndDirectoryCount = ::std::option::Option::Some(v);
    }

    pub fn get_fileAndDirectoryCount(&self) -> u64 {
        self.fileAndDirectoryCount.unwrap_or(0)
    }

    fn get_fileAndDirectoryCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileAndDirectoryCount
    }

    fn mut_fileAndDirectoryCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileAndDirectoryCount
    }

    // required uint64 quota = 2;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    fn get_quota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.quota
    }

    // required uint64 spaceConsumed = 3;

    pub fn clear_spaceConsumed(&mut self) {
        self.spaceConsumed = ::std::option::Option::None;
    }

    pub fn has_spaceConsumed(&self) -> bool {
        self.spaceConsumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceConsumed(&mut self, v: u64) {
        self.spaceConsumed = ::std::option::Option::Some(v);
    }

    pub fn get_spaceConsumed(&self) -> u64 {
        self.spaceConsumed.unwrap_or(0)
    }

    fn get_spaceConsumed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceConsumed
    }

    fn mut_spaceConsumed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceConsumed
    }

    // required uint64 spaceQuota = 4;

    pub fn clear_spaceQuota(&mut self) {
        self.spaceQuota = ::std::option::Option::None;
    }

    pub fn has_spaceQuota(&self) -> bool {
        self.spaceQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceQuota(&mut self, v: u64) {
        self.spaceQuota = ::std::option::Option::Some(v);
    }

    pub fn get_spaceQuota(&self) -> u64 {
        self.spaceQuota.unwrap_or(0)
    }

    fn get_spaceQuota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceQuota
    }

    fn mut_spaceQuota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceQuota
    }

    // optional .hadoop.hdfs.StorageTypeQuotaInfosProto typeQuotaInfos = 5;

    pub fn clear_typeQuotaInfos(&mut self) {
        self.typeQuotaInfos.clear();
    }

    pub fn has_typeQuotaInfos(&self) -> bool {
        self.typeQuotaInfos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typeQuotaInfos(&mut self, v: StorageTypeQuotaInfosProto) {
        self.typeQuotaInfos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typeQuotaInfos(&mut self) -> &mut StorageTypeQuotaInfosProto {
        if self.typeQuotaInfos.is_none() {
            self.typeQuotaInfos.set_default();
        }
        self.typeQuotaInfos.as_mut().unwrap()
    }

    // Take field
    pub fn take_typeQuotaInfos(&mut self) -> StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.take().unwrap_or_else(|| StorageTypeQuotaInfosProto::new())
    }

    pub fn get_typeQuotaInfos(&self) -> &StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.as_ref().unwrap_or_else(|| StorageTypeQuotaInfosProto::default_instance())
    }

    fn get_typeQuotaInfos_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypeQuotaInfosProto> {
        &self.typeQuotaInfos
    }

    fn mut_typeQuotaInfos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypeQuotaInfosProto> {
        &mut self.typeQuotaInfos
    }
}

impl ::protobuf::Message for QuotaUsageProto {
    fn is_initialized(&self) -> bool {
        if self.fileAndDirectoryCount.is_none() {
            return false;
        }
        if self.quota.is_none() {
            return false;
        }
        if self.spaceConsumed.is_none() {
            return false;
        }
        if self.spaceQuota.is_none() {
            return false;
        }
        for v in &self.typeQuotaInfos {
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
                    let tmp = is.read_uint64()?;
                    self.fileAndDirectoryCount = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceConsumed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceQuota = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.typeQuotaInfos)?;
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
        if let Some(v) = self.fileAndDirectoryCount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quota {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceConsumed {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceQuota {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.typeQuotaInfos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileAndDirectoryCount {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.quota {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.spaceConsumed {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.spaceQuota {
            os.write_uint64(4, v)?;
        }
        if let Some(ref v) = self.typeQuotaInfos.as_ref() {
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

impl ::protobuf::MessageStatic for QuotaUsageProto {
    fn new() -> QuotaUsageProto {
        QuotaUsageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<QuotaUsageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileAndDirectoryCount",
                    QuotaUsageProto::get_fileAndDirectoryCount_for_reflect,
                    QuotaUsageProto::mut_fileAndDirectoryCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    QuotaUsageProto::get_quota_for_reflect,
                    QuotaUsageProto::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceConsumed",
                    QuotaUsageProto::get_spaceConsumed_for_reflect,
                    QuotaUsageProto::mut_spaceConsumed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceQuota",
                    QuotaUsageProto::get_spaceQuota_for_reflect,
                    QuotaUsageProto::mut_spaceQuota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypeQuotaInfosProto>>(
                    "typeQuotaInfos",
                    QuotaUsageProto::get_typeQuotaInfos_for_reflect,
                    QuotaUsageProto::mut_typeQuotaInfos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QuotaUsageProto>(
                    "QuotaUsageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QuotaUsageProto {
    fn clear(&mut self) {
        self.clear_fileAndDirectoryCount();
        self.clear_quota();
        self.clear_spaceConsumed();
        self.clear_spaceQuota();
        self.clear_typeQuotaInfos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QuotaUsageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuotaUsageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageTypeQuotaInfosProto {
    // message fields
    typeQuotaInfo: ::protobuf::RepeatedField<StorageTypeQuotaInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageTypeQuotaInfosProto {}

impl StorageTypeQuotaInfosProto {
    pub fn new() -> StorageTypeQuotaInfosProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypeQuotaInfosProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypeQuotaInfosProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypeQuotaInfosProto,
        };
        unsafe {
            instance.get(StorageTypeQuotaInfosProto::new)
        }
    }

    // repeated .hadoop.hdfs.StorageTypeQuotaInfoProto typeQuotaInfo = 1;

    pub fn clear_typeQuotaInfo(&mut self) {
        self.typeQuotaInfo.clear();
    }

    // Param is passed by value, moved
    pub fn set_typeQuotaInfo(&mut self, v: ::protobuf::RepeatedField<StorageTypeQuotaInfoProto>) {
        self.typeQuotaInfo = v;
    }

    // Mutable pointer to the field.
    pub fn mut_typeQuotaInfo(&mut self) -> &mut ::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        &mut self.typeQuotaInfo
    }

    // Take field
    pub fn take_typeQuotaInfo(&mut self) -> ::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        ::std::mem::replace(&mut self.typeQuotaInfo, ::protobuf::RepeatedField::new())
    }

    pub fn get_typeQuotaInfo(&self) -> &[StorageTypeQuotaInfoProto] {
        &self.typeQuotaInfo
    }

    fn get_typeQuotaInfo_for_reflect(&self) -> &::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        &self.typeQuotaInfo
    }

    fn mut_typeQuotaInfo_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        &mut self.typeQuotaInfo
    }
}

impl ::protobuf::Message for StorageTypeQuotaInfosProto {
    fn is_initialized(&self) -> bool {
        for v in &self.typeQuotaInfo {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.typeQuotaInfo)?;
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
        for value in &self.typeQuotaInfo {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.typeQuotaInfo {
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

impl ::protobuf::MessageStatic for StorageTypeQuotaInfosProto {
    fn new() -> StorageTypeQuotaInfosProto {
        StorageTypeQuotaInfosProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypeQuotaInfosProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypeQuotaInfoProto>>(
                    "typeQuotaInfo",
                    StorageTypeQuotaInfosProto::get_typeQuotaInfo_for_reflect,
                    StorageTypeQuotaInfosProto::mut_typeQuotaInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypeQuotaInfosProto>(
                    "StorageTypeQuotaInfosProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypeQuotaInfosProto {
    fn clear(&mut self) {
        self.clear_typeQuotaInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageTypeQuotaInfosProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageTypeQuotaInfosProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageTypeQuotaInfoProto {
    // message fields
    field_type: ::std::option::Option<StorageTypeProto>,
    quota: ::std::option::Option<u64>,
    consumed: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageTypeQuotaInfoProto {}

impl StorageTypeQuotaInfoProto {
    pub fn new() -> StorageTypeQuotaInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypeQuotaInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypeQuotaInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypeQuotaInfoProto,
        };
        unsafe {
            instance.get(StorageTypeQuotaInfoProto::new)
        }
    }

    // required .hadoop.hdfs.StorageTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: StorageTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> StorageTypeProto {
        self.field_type.unwrap_or(StorageTypeProto::DISK)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<StorageTypeProto> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<StorageTypeProto> {
        &mut self.field_type
    }

    // required uint64 quota = 2;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    fn get_quota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.quota
    }

    // required uint64 consumed = 3;

    pub fn clear_consumed(&mut self) {
        self.consumed = ::std::option::Option::None;
    }

    pub fn has_consumed(&self) -> bool {
        self.consumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumed(&mut self, v: u64) {
        self.consumed = ::std::option::Option::Some(v);
    }

    pub fn get_consumed(&self) -> u64 {
        self.consumed.unwrap_or(0)
    }

    fn get_consumed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.consumed
    }

    fn mut_consumed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.consumed
    }
}

impl ::protobuf::Message for StorageTypeQuotaInfoProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.quota.is_none() {
            return false;
        }
        if self.consumed.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.consumed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.quota {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.consumed {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.quota {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.consumed {
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

impl ::protobuf::MessageStatic for StorageTypeQuotaInfoProto {
    fn new() -> StorageTypeQuotaInfoProto {
        StorageTypeQuotaInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypeQuotaInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "type",
                    StorageTypeQuotaInfoProto::get_field_type_for_reflect,
                    StorageTypeQuotaInfoProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    StorageTypeQuotaInfoProto::get_quota_for_reflect,
                    StorageTypeQuotaInfoProto::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "consumed",
                    StorageTypeQuotaInfoProto::get_consumed_for_reflect,
                    StorageTypeQuotaInfoProto::mut_consumed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypeQuotaInfoProto>(
                    "StorageTypeQuotaInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypeQuotaInfoProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_quota();
        self.clear_consumed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageTypeQuotaInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageTypeQuotaInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CorruptFileBlocksProto {
    // message fields
    files: ::protobuf::RepeatedField<::std::string::String>,
    cookie: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CorruptFileBlocksProto {}

impl CorruptFileBlocksProto {
    pub fn new() -> CorruptFileBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CorruptFileBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<CorruptFileBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CorruptFileBlocksProto,
        };
        unsafe {
            instance.get(CorruptFileBlocksProto::new)
        }
    }

    // repeated string files = 1;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[::std::string::String] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.files
    }

    // required string cookie = 2;

    pub fn clear_cookie(&mut self) {
        self.cookie.clear();
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: ::std::string::String) {
        self.cookie = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cookie(&mut self) -> &mut ::std::string::String {
        if self.cookie.is_none() {
            self.cookie.set_default();
        }
        self.cookie.as_mut().unwrap()
    }

    // Take field
    pub fn take_cookie(&mut self) -> ::std::string::String {
        self.cookie.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cookie(&self) -> &str {
        match self.cookie.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cookie_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cookie
    }
}

impl ::protobuf::Message for CorruptFileBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.cookie.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.files)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cookie)?;
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
        for value in &self.files {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.cookie.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.files {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.cookie.as_ref() {
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

impl ::protobuf::MessageStatic for CorruptFileBlocksProto {
    fn new() -> CorruptFileBlocksProto {
        CorruptFileBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CorruptFileBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "files",
                    CorruptFileBlocksProto::get_files_for_reflect,
                    CorruptFileBlocksProto::mut_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cookie",
                    CorruptFileBlocksProto::get_cookie_for_reflect,
                    CorruptFileBlocksProto::mut_cookie_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CorruptFileBlocksProto>(
                    "CorruptFileBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CorruptFileBlocksProto {
    fn clear(&mut self) {
        self.clear_files();
        self.clear_cookie();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CorruptFileBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CorruptFileBlocksProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageTypesProto {
    // message fields
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageTypesProto {}

impl StorageTypesProto {
    pub fn new() -> StorageTypesProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypesProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypesProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypesProto,
        };
        unsafe {
            instance.get(StorageTypesProto::new)
        }
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 1;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }
}

impl ::protobuf::Message for StorageTypesProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
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
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.storageTypes {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for StorageTypesProto {
    fn new() -> StorageTypesProto {
        StorageTypesProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypesProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    StorageTypesProto::get_storageTypes_for_reflect,
                    StorageTypesProto::mut_storageTypes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypesProto>(
                    "StorageTypesProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypesProto {
    fn clear(&mut self) {
        self.clear_storageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageTypesProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageTypesProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockStoragePolicyProto {
    // message fields
    policyId: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    creationPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    creationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    replicationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockStoragePolicyProto {}

impl BlockStoragePolicyProto {
    pub fn new() -> BlockStoragePolicyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockStoragePolicyProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockStoragePolicyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockStoragePolicyProto,
        };
        unsafe {
            instance.get(BlockStoragePolicyProto::new)
        }
    }

    // required uint32 policyId = 1;

    pub fn clear_policyId(&mut self) {
        self.policyId = ::std::option::Option::None;
    }

    pub fn has_policyId(&self) -> bool {
        self.policyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policyId(&mut self, v: u32) {
        self.policyId = ::std::option::Option::Some(v);
    }

    pub fn get_policyId(&self) -> u32 {
        self.policyId.unwrap_or(0)
    }

    fn get_policyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.policyId
    }

    fn mut_policyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.policyId
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required .hadoop.hdfs.StorageTypesProto creationPolicy = 3;

    pub fn clear_creationPolicy(&mut self) {
        self.creationPolicy.clear();
    }

    pub fn has_creationPolicy(&self) -> bool {
        self.creationPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationPolicy(&mut self, v: StorageTypesProto) {
        self.creationPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationPolicy(&mut self) -> &mut StorageTypesProto {
        if self.creationPolicy.is_none() {
            self.creationPolicy.set_default();
        }
        self.creationPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationPolicy(&mut self) -> StorageTypesProto {
        self.creationPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationPolicy(&self) -> &StorageTypesProto {
        self.creationPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_creationPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.creationPolicy
    }

    fn mut_creationPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.creationPolicy
    }

    // optional .hadoop.hdfs.StorageTypesProto creationFallbackPolicy = 4;

    pub fn clear_creationFallbackPolicy(&mut self) {
        self.creationFallbackPolicy.clear();
    }

    pub fn has_creationFallbackPolicy(&self) -> bool {
        self.creationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.creationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationFallbackPolicy(&mut self) -> &mut StorageTypesProto {
        if self.creationFallbackPolicy.is_none() {
            self.creationFallbackPolicy.set_default();
        }
        self.creationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.creationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationFallbackPolicy(&self) -> &StorageTypesProto {
        self.creationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_creationFallbackPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.creationFallbackPolicy
    }

    fn mut_creationFallbackPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.creationFallbackPolicy
    }

    // optional .hadoop.hdfs.StorageTypesProto replicationFallbackPolicy = 5;

    pub fn clear_replicationFallbackPolicy(&mut self) {
        self.replicationFallbackPolicy.clear();
    }

    pub fn has_replicationFallbackPolicy(&self) -> bool {
        self.replicationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.replicationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replicationFallbackPolicy(&mut self) -> &mut StorageTypesProto {
        if self.replicationFallbackPolicy.is_none() {
            self.replicationFallbackPolicy.set_default();
        }
        self.replicationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_replicationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.replicationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_replicationFallbackPolicy(&self) -> &StorageTypesProto {
        self.replicationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_replicationFallbackPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.replicationFallbackPolicy
    }

    fn mut_replicationFallbackPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.replicationFallbackPolicy
    }
}

impl ::protobuf::Message for BlockStoragePolicyProto {
    fn is_initialized(&self) -> bool {
        if self.policyId.is_none() {
            return false;
        }
        if self.name.is_none() {
            return false;
        }
        if self.creationPolicy.is_none() {
            return false;
        }
        for v in &self.creationPolicy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.creationFallbackPolicy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.replicationFallbackPolicy {
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
                    let tmp = is.read_uint32()?;
                    self.policyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creationPolicy)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creationFallbackPolicy)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.replicationFallbackPolicy)?;
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
        if let Some(v) = self.policyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.creationPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.creationFallbackPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.replicationFallbackPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.policyId {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.creationPolicy.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.creationFallbackPolicy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.replicationFallbackPolicy.as_ref() {
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

impl ::protobuf::MessageStatic for BlockStoragePolicyProto {
    fn new() -> BlockStoragePolicyProto {
        BlockStoragePolicyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockStoragePolicyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "policyId",
                    BlockStoragePolicyProto::get_policyId_for_reflect,
                    BlockStoragePolicyProto::mut_policyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    BlockStoragePolicyProto::get_name_for_reflect,
                    BlockStoragePolicyProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "creationPolicy",
                    BlockStoragePolicyProto::get_creationPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_creationPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "creationFallbackPolicy",
                    BlockStoragePolicyProto::get_creationFallbackPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_creationFallbackPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "replicationFallbackPolicy",
                    BlockStoragePolicyProto::get_replicationFallbackPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_replicationFallbackPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockStoragePolicyProto>(
                    "BlockStoragePolicyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockStoragePolicyProto {
    fn clear(&mut self) {
        self.clear_policyId();
        self.clear_name();
        self.clear_creationPolicy();
        self.clear_creationFallbackPolicy();
        self.clear_replicationFallbackPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockStoragePolicyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockStoragePolicyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocatedBlockProto {
    // message fields
    b: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    offset: ::std::option::Option<u64>,
    locs: ::protobuf::RepeatedField<DatanodeInfoProto>,
    corrupt: ::std::option::Option<bool>,
    blockToken: ::protobuf::SingularPtrField<security::TokenProto>,
    isCached: ::std::vec::Vec<bool>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    storageIDs: ::protobuf::RepeatedField<::std::string::String>,
    blockIndices: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    blockTokens: ::protobuf::RepeatedField<security::TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocatedBlockProto {}

impl LocatedBlockProto {
    pub fn new() -> LocatedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlockProto,
        };
        unsafe {
            instance.get(LocatedBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto b = 1;

    pub fn clear_b(&mut self) {
        self.b.clear();
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ExtendedBlockProto) {
        self.b = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b(&mut self) -> &mut ExtendedBlockProto {
        if self.b.is_none() {
            self.b.set_default();
        }
        self.b.as_mut().unwrap()
    }

    // Take field
    pub fn take_b(&mut self) -> ExtendedBlockProto {
        self.b.take().unwrap_or_else(|| ExtendedBlockProto::new())
    }

    pub fn get_b(&self) -> &ExtendedBlockProto {
        self.b.as_ref().unwrap_or_else(|| ExtendedBlockProto::default_instance())
    }

    fn get_b_for_reflect(&self) -> &::protobuf::SingularPtrField<ExtendedBlockProto> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ExtendedBlockProto> {
        &mut self.b
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

    // repeated .hadoop.hdfs.DatanodeInfoProto locs = 3;

    pub fn clear_locs(&mut self) {
        self.locs.clear();
    }

    // Param is passed by value, moved
    pub fn set_locs(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.locs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locs(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.locs
    }

    // Take field
    pub fn take_locs(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.locs, ::protobuf::RepeatedField::new())
    }

    pub fn get_locs(&self) -> &[DatanodeInfoProto] {
        &self.locs
    }

    fn get_locs_for_reflect(&self) -> &::protobuf::RepeatedField<DatanodeInfoProto> {
        &self.locs
    }

    fn mut_locs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.locs
    }

    // required bool corrupt = 4;

    pub fn clear_corrupt(&mut self) {
        self.corrupt = ::std::option::Option::None;
    }

    pub fn has_corrupt(&self) -> bool {
        self.corrupt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_corrupt(&mut self, v: bool) {
        self.corrupt = ::std::option::Option::Some(v);
    }

    pub fn get_corrupt(&self) -> bool {
        self.corrupt.unwrap_or(false)
    }

    fn get_corrupt_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.corrupt
    }

    fn mut_corrupt_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.corrupt
    }

    // required .hadoop.common.TokenProto blockToken = 5;

    pub fn clear_blockToken(&mut self) {
        self.blockToken.clear();
    }

    pub fn has_blockToken(&self) -> bool {
        self.blockToken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockToken(&mut self, v: security::TokenProto) {
        self.blockToken = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockToken(&mut self) -> &mut security::TokenProto {
        if self.blockToken.is_none() {
            self.blockToken.set_default();
        }
        self.blockToken.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockToken(&mut self) -> security::TokenProto {
        self.blockToken.take().unwrap_or_else(|| security::TokenProto::new())
    }

    pub fn get_blockToken(&self) -> &security::TokenProto {
        self.blockToken.as_ref().unwrap_or_else(|| security::TokenProto::default_instance())
    }

    fn get_blockToken_for_reflect(&self) -> &::protobuf::SingularPtrField<security::TokenProto> {
        &self.blockToken
    }

    fn mut_blockToken_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<security::TokenProto> {
        &mut self.blockToken
    }

    // repeated bool isCached = 6;

    pub fn clear_isCached(&mut self) {
        self.isCached.clear();
    }

    // Param is passed by value, moved
    pub fn set_isCached(&mut self, v: ::std::vec::Vec<bool>) {
        self.isCached = v;
    }

    // Mutable pointer to the field.
    pub fn mut_isCached(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.isCached
    }

    // Take field
    pub fn take_isCached(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.isCached, ::std::vec::Vec::new())
    }

    pub fn get_isCached(&self) -> &[bool] {
        &self.isCached
    }

    fn get_isCached_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.isCached
    }

    fn mut_isCached_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.isCached
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 7;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // repeated string storageIDs = 8;

    pub fn clear_storageIDs(&mut self) {
        self.storageIDs.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageIDs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageIDs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageIDs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIDs
    }

    // Take field
    pub fn take_storageIDs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageIDs, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageIDs(&self) -> &[::std::string::String] {
        &self.storageIDs
    }

    fn get_storageIDs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageIDs
    }

    fn mut_storageIDs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIDs
    }

    // optional bytes blockIndices = 9;

    pub fn clear_blockIndices(&mut self) {
        self.blockIndices.clear();
    }

    pub fn has_blockIndices(&self) -> bool {
        self.blockIndices.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockIndices(&mut self, v: ::std::vec::Vec<u8>) {
        self.blockIndices = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockIndices(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.blockIndices.is_none() {
            self.blockIndices.set_default();
        }
        self.blockIndices.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockIndices(&mut self) -> ::std::vec::Vec<u8> {
        self.blockIndices.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_blockIndices(&self) -> &[u8] {
        match self.blockIndices.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_blockIndices_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.blockIndices
    }

    fn mut_blockIndices_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.blockIndices
    }

    // repeated .hadoop.common.TokenProto blockTokens = 10;

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
}

impl ::protobuf::Message for LocatedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        }
        if self.offset.is_none() {
            return false;
        }
        if self.corrupt.is_none() {
            return false;
        }
        if self.blockToken.is_none() {
            return false;
        }
        for v in &self.b {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blockToken {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blockTokens {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.b)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locs)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.corrupt = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blockToken)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.isCached)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageIDs)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.blockIndices)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blockTokens)?;
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
        if let Some(ref v) = self.b.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.locs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.corrupt {
            my_size += 2;
        }
        if let Some(ref v) = self.blockToken.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.isCached.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.isCached.len() as u32) + (self.isCached.len() * 1) as u32;
        }
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in &self.storageIDs {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        if let Some(ref v) = self.blockIndices.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        for value in &self.blockTokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.b.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.offset {
            os.write_uint64(2, v)?;
        }
        for v in &self.locs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.corrupt {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.blockToken.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.isCached.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.isCached.len() * 1) as u32)?;
            for v in &self.isCached {
                os.write_bool_no_tag(*v)?;
            };
        }
        for v in &self.storageTypes {
            os.write_enum(7, v.value())?;
        };
        for v in &self.storageIDs {
            os.write_string(8, &v)?;
        };
        if let Some(ref v) = self.blockIndices.as_ref() {
            os.write_bytes(9, &v)?;
        }
        for v in &self.blockTokens {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LocatedBlockProto {
    fn new() -> LocatedBlockProto {
        LocatedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExtendedBlockProto>>(
                    "b",
                    LocatedBlockProto::get_b_for_reflect,
                    LocatedBlockProto::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    LocatedBlockProto::get_offset_for_reflect,
                    LocatedBlockProto::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeInfoProto>>(
                    "locs",
                    LocatedBlockProto::get_locs_for_reflect,
                    LocatedBlockProto::mut_locs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "corrupt",
                    LocatedBlockProto::get_corrupt_for_reflect,
                    LocatedBlockProto::mut_corrupt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<security::TokenProto>>(
                    "blockToken",
                    LocatedBlockProto::get_blockToken_for_reflect,
                    LocatedBlockProto::mut_blockToken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isCached",
                    LocatedBlockProto::get_isCached_for_reflect,
                    LocatedBlockProto::mut_isCached_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    LocatedBlockProto::get_storageTypes_for_reflect,
                    LocatedBlockProto::mut_storageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageIDs",
                    LocatedBlockProto::get_storageIDs_for_reflect,
                    LocatedBlockProto::mut_storageIDs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "blockIndices",
                    LocatedBlockProto::get_blockIndices_for_reflect,
                    LocatedBlockProto::mut_blockIndices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<security::TokenProto>>(
                    "blockTokens",
                    LocatedBlockProto::get_blockTokens_for_reflect,
                    LocatedBlockProto::mut_blockTokens_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlockProto>(
                    "LocatedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlockProto {
    fn clear(&mut self) {
        self.clear_b();
        self.clear_offset();
        self.clear_locs();
        self.clear_corrupt();
        self.clear_blockToken();
        self.clear_isCached();
        self.clear_storageTypes();
        self.clear_storageIDs();
        self.clear_blockIndices();
        self.clear_blockTokens();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocatedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocatedBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataEncryptionKeyProto {
    // message fields
    keyId: ::std::option::Option<u32>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    encryptionKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    expiryDate: ::std::option::Option<u64>,
    encryptionAlgorithm: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataEncryptionKeyProto {}

impl DataEncryptionKeyProto {
    pub fn new() -> DataEncryptionKeyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataEncryptionKeyProto {
        static mut instance: ::protobuf::lazy::Lazy<DataEncryptionKeyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataEncryptionKeyProto,
        };
        unsafe {
            instance.get(DataEncryptionKeyProto::new)
        }
    }

    // required uint32 keyId = 1;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    fn get_keyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.keyId
    }

    fn mut_keyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.keyId
    }

    // required string blockPoolId = 2;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }

    // required bytes nonce = 3;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.nonce.is_none() {
            self.nonce.set_default();
        }
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_nonce(&self) -> &[u8] {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.nonce
    }

    // required bytes encryptionKey = 4;

    pub fn clear_encryptionKey(&mut self) {
        self.encryptionKey.clear();
    }

    pub fn has_encryptionKey(&self) -> bool {
        self.encryptionKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.encryptionKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encryptionKey.is_none() {
            self.encryptionKey.set_default();
        }
        self.encryptionKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionKey(&mut self) -> ::std::vec::Vec<u8> {
        self.encryptionKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encryptionKey(&self) -> &[u8] {
        match self.encryptionKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encryptionKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encryptionKey
    }

    fn mut_encryptionKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encryptionKey
    }

    // required uint64 expiryDate = 5;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    fn get_expiryDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expiryDate
    }

    fn mut_expiryDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expiryDate
    }

    // optional string encryptionAlgorithm = 6;

    pub fn clear_encryptionAlgorithm(&mut self) {
        self.encryptionAlgorithm.clear();
    }

    pub fn has_encryptionAlgorithm(&self) -> bool {
        self.encryptionAlgorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionAlgorithm(&mut self, v: ::std::string::String) {
        self.encryptionAlgorithm = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionAlgorithm(&mut self) -> &mut ::std::string::String {
        if self.encryptionAlgorithm.is_none() {
            self.encryptionAlgorithm.set_default();
        }
        self.encryptionAlgorithm.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionAlgorithm(&mut self) -> ::std::string::String {
        self.encryptionAlgorithm.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_encryptionAlgorithm(&self) -> &str {
        match self.encryptionAlgorithm.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_encryptionAlgorithm_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.encryptionAlgorithm
    }

    fn mut_encryptionAlgorithm_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.encryptionAlgorithm
    }
}

impl ::protobuf::Message for DataEncryptionKeyProto {
    fn is_initialized(&self) -> bool {
        if self.keyId.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
            return false;
        }
        if self.nonce.is_none() {
            return false;
        }
        if self.encryptionKey.is_none() {
            return false;
        }
        if self.expiryDate.is_none() {
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
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.nonce)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encryptionKey)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.encryptionAlgorithm)?;
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
        if let Some(v) = self.keyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.encryptionKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.encryptionAlgorithm.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyId {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.nonce.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.encryptionKey.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.expiryDate {
            os.write_uint64(5, v)?;
        }
        if let Some(ref v) = self.encryptionAlgorithm.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for DataEncryptionKeyProto {
    fn new() -> DataEncryptionKeyProto {
        DataEncryptionKeyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataEncryptionKeyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "keyId",
                    DataEncryptionKeyProto::get_keyId_for_reflect,
                    DataEncryptionKeyProto::mut_keyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    DataEncryptionKeyProto::get_blockPoolId_for_reflect,
                    DataEncryptionKeyProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nonce",
                    DataEncryptionKeyProto::get_nonce_for_reflect,
                    DataEncryptionKeyProto::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encryptionKey",
                    DataEncryptionKeyProto::get_encryptionKey_for_reflect,
                    DataEncryptionKeyProto::mut_encryptionKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    DataEncryptionKeyProto::get_expiryDate_for_reflect,
                    DataEncryptionKeyProto::mut_expiryDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "encryptionAlgorithm",
                    DataEncryptionKeyProto::get_encryptionAlgorithm_for_reflect,
                    DataEncryptionKeyProto::mut_encryptionAlgorithm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataEncryptionKeyProto>(
                    "DataEncryptionKeyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataEncryptionKeyProto {
    fn clear(&mut self) {
        self.clear_keyId();
        self.clear_blockPoolId();
        self.clear_nonce();
        self.clear_encryptionKey();
        self.clear_expiryDate();
        self.clear_encryptionAlgorithm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataEncryptionKeyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataEncryptionKeyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileEncryptionInfoProto {}

impl FileEncryptionInfoProto {
    pub fn new() -> FileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<FileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileEncryptionInfoProto,
        };
        unsafe {
            instance.get(FileEncryptionInfoProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
    }

    // required bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required bytes iv = 4;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        }
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv(&self) -> &[u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_iv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.iv
    }

    fn mut_iv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.iv
    }

    // required string keyName = 5;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName(&mut self) -> &mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        }
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName(&self) -> &str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_keyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.keyName
    }

    fn mut_keyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.keyName
    }

    // required string ezKeyVersionName = 6;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName(&mut self) -> &mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        }
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName(&self) -> &str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ezKeyVersionName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ezKeyVersionName
    }

    fn mut_ezKeyVersionName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ezKeyVersionName
    }
}

impl ::protobuf::Message for FileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        }
        if self.cryptoProtocolVersion.is_none() {
            return false;
        }
        if self.key.is_none() {
            return false;
        }
        if self.iv.is_none() {
            return false;
        }
        if self.keyName.is_none() {
            return false;
        }
        if self.ezKeyVersionName.is_none() {
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
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.iv)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.iv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.key.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.iv.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for FileEncryptionInfoProto {
    fn new() -> FileEncryptionInfoProto {
        FileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    FileEncryptionInfoProto::get_suite_for_reflect,
                    FileEncryptionInfoProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    FileEncryptionInfoProto::get_cryptoProtocolVersion_for_reflect,
                    FileEncryptionInfoProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    FileEncryptionInfoProto::get_key_for_reflect,
                    FileEncryptionInfoProto::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "iv",
                    FileEncryptionInfoProto::get_iv_for_reflect,
                    FileEncryptionInfoProto::mut_iv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    FileEncryptionInfoProto::get_keyName_for_reflect,
                    FileEncryptionInfoProto::mut_keyName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    FileEncryptionInfoProto::get_ezKeyVersionName_for_reflect,
                    FileEncryptionInfoProto::mut_ezKeyVersionName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileEncryptionInfoProto>(
                    "FileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_key();
        self.clear_iv();
        self.clear_keyName();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PerFileEncryptionInfoProto {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PerFileEncryptionInfoProto {}

impl PerFileEncryptionInfoProto {
    pub fn new() -> PerFileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerFileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<PerFileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerFileEncryptionInfoProto,
        };
        unsafe {
            instance.get(PerFileEncryptionInfoProto::new)
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required bytes iv = 2;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        }
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv(&self) -> &[u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_iv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.iv
    }

    fn mut_iv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.iv
    }

    // required string ezKeyVersionName = 3;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName(&mut self) -> &mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        }
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName(&self) -> &str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ezKeyVersionName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ezKeyVersionName
    }

    fn mut_ezKeyVersionName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ezKeyVersionName
    }
}

impl ::protobuf::Message for PerFileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        }
        if self.iv.is_none() {
            return false;
        }
        if self.ezKeyVersionName.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.iv)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.iv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.iv.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
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

impl ::protobuf::MessageStatic for PerFileEncryptionInfoProto {
    fn new() -> PerFileEncryptionInfoProto {
        PerFileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerFileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    PerFileEncryptionInfoProto::get_key_for_reflect,
                    PerFileEncryptionInfoProto::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "iv",
                    PerFileEncryptionInfoProto::get_iv_for_reflect,
                    PerFileEncryptionInfoProto::mut_iv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    PerFileEncryptionInfoProto::get_ezKeyVersionName_for_reflect,
                    PerFileEncryptionInfoProto::mut_ezKeyVersionName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerFileEncryptionInfoProto>(
                    "PerFileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerFileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_iv();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PerFileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PerFileEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZoneEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    reencryptionProto: ::protobuf::SingularPtrField<ReencryptionInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZoneEncryptionInfoProto {}

impl ZoneEncryptionInfoProto {
    pub fn new() -> ZoneEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZoneEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ZoneEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZoneEncryptionInfoProto,
        };
        unsafe {
            instance.get(ZoneEncryptionInfoProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
    }

    // required string keyName = 3;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName(&mut self) -> &mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        }
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName(&self) -> &str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_keyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.keyName
    }

    fn mut_keyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.keyName
    }

    // optional .hadoop.hdfs.ReencryptionInfoProto reencryptionProto = 4;

    pub fn clear_reencryptionProto(&mut self) {
        self.reencryptionProto.clear();
    }

    pub fn has_reencryptionProto(&self) -> bool {
        self.reencryptionProto.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reencryptionProto(&mut self, v: ReencryptionInfoProto) {
        self.reencryptionProto = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reencryptionProto(&mut self) -> &mut ReencryptionInfoProto {
        if self.reencryptionProto.is_none() {
            self.reencryptionProto.set_default();
        }
        self.reencryptionProto.as_mut().unwrap()
    }

    // Take field
    pub fn take_reencryptionProto(&mut self) -> ReencryptionInfoProto {
        self.reencryptionProto.take().unwrap_or_else(|| ReencryptionInfoProto::new())
    }

    pub fn get_reencryptionProto(&self) -> &ReencryptionInfoProto {
        self.reencryptionProto.as_ref().unwrap_or_else(|| ReencryptionInfoProto::default_instance())
    }

    fn get_reencryptionProto_for_reflect(&self) -> &::protobuf::SingularPtrField<ReencryptionInfoProto> {
        &self.reencryptionProto
    }

    fn mut_reencryptionProto_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReencryptionInfoProto> {
        &mut self.reencryptionProto
    }
}

impl ::protobuf::Message for ZoneEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        }
        if self.cryptoProtocolVersion.is_none() {
            return false;
        }
        if self.keyName.is_none() {
            return false;
        }
        for v in &self.reencryptionProto {
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
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reencryptionProto)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.reencryptionProto.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.reencryptionProto.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ZoneEncryptionInfoProto {
    fn new() -> ZoneEncryptionInfoProto {
        ZoneEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZoneEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    ZoneEncryptionInfoProto::get_suite_for_reflect,
                    ZoneEncryptionInfoProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    ZoneEncryptionInfoProto::get_cryptoProtocolVersion_for_reflect,
                    ZoneEncryptionInfoProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    ZoneEncryptionInfoProto::get_keyName_for_reflect,
                    ZoneEncryptionInfoProto::mut_keyName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReencryptionInfoProto>>(
                    "reencryptionProto",
                    ZoneEncryptionInfoProto::get_reencryptionProto_for_reflect,
                    ZoneEncryptionInfoProto::mut_reencryptionProto_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZoneEncryptionInfoProto>(
                    "ZoneEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZoneEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.clear_reencryptionProto();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZoneEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZoneEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReencryptionInfoProto {
    // message fields
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    submissionTime: ::std::option::Option<u64>,
    canceled: ::std::option::Option<bool>,
    numReencrypted: ::std::option::Option<i64>,
    numFailures: ::std::option::Option<i64>,
    completionTime: ::std::option::Option<u64>,
    lastFile: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReencryptionInfoProto {}

impl ReencryptionInfoProto {
    pub fn new() -> ReencryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReencryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ReencryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReencryptionInfoProto,
        };
        unsafe {
            instance.get(ReencryptionInfoProto::new)
        }
    }

    // required string ezKeyVersionName = 1;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName(&mut self) -> &mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        }
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName(&self) -> &str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ezKeyVersionName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ezKeyVersionName
    }

    fn mut_ezKeyVersionName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ezKeyVersionName
    }

    // required uint64 submissionTime = 2;

    pub fn clear_submissionTime(&mut self) {
        self.submissionTime = ::std::option::Option::None;
    }

    pub fn has_submissionTime(&self) -> bool {
        self.submissionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_submissionTime(&mut self, v: u64) {
        self.submissionTime = ::std::option::Option::Some(v);
    }

    pub fn get_submissionTime(&self) -> u64 {
        self.submissionTime.unwrap_or(0)
    }

    fn get_submissionTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.submissionTime
    }

    fn mut_submissionTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.submissionTime
    }

    // required bool canceled = 3;

    pub fn clear_canceled(&mut self) {
        self.canceled = ::std::option::Option::None;
    }

    pub fn has_canceled(&self) -> bool {
        self.canceled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_canceled(&mut self, v: bool) {
        self.canceled = ::std::option::Option::Some(v);
    }

    pub fn get_canceled(&self) -> bool {
        self.canceled.unwrap_or(false)
    }

    fn get_canceled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.canceled
    }

    fn mut_canceled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.canceled
    }

    // required int64 numReencrypted = 4;

    pub fn clear_numReencrypted(&mut self) {
        self.numReencrypted = ::std::option::Option::None;
    }

    pub fn has_numReencrypted(&self) -> bool {
        self.numReencrypted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numReencrypted(&mut self, v: i64) {
        self.numReencrypted = ::std::option::Option::Some(v);
    }

    pub fn get_numReencrypted(&self) -> i64 {
        self.numReencrypted.unwrap_or(0)
    }

    fn get_numReencrypted_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.numReencrypted
    }

    fn mut_numReencrypted_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.numReencrypted
    }

    // required int64 numFailures = 5;

    pub fn clear_numFailures(&mut self) {
        self.numFailures = ::std::option::Option::None;
    }

    pub fn has_numFailures(&self) -> bool {
        self.numFailures.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numFailures(&mut self, v: i64) {
        self.numFailures = ::std::option::Option::Some(v);
    }

    pub fn get_numFailures(&self) -> i64 {
        self.numFailures.unwrap_or(0)
    }

    fn get_numFailures_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.numFailures
    }

    fn mut_numFailures_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.numFailures
    }

    // optional uint64 completionTime = 6;

    pub fn clear_completionTime(&mut self) {
        self.completionTime = ::std::option::Option::None;
    }

    pub fn has_completionTime(&self) -> bool {
        self.completionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_completionTime(&mut self, v: u64) {
        self.completionTime = ::std::option::Option::Some(v);
    }

    pub fn get_completionTime(&self) -> u64 {
        self.completionTime.unwrap_or(0)
    }

    fn get_completionTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.completionTime
    }

    fn mut_completionTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.completionTime
    }

    // optional string lastFile = 7;

    pub fn clear_lastFile(&mut self) {
        self.lastFile.clear();
    }

    pub fn has_lastFile(&self) -> bool {
        self.lastFile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastFile(&mut self, v: ::std::string::String) {
        self.lastFile = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastFile(&mut self) -> &mut ::std::string::String {
        if self.lastFile.is_none() {
            self.lastFile.set_default();
        }
        self.lastFile.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastFile(&mut self) -> ::std::string::String {
        self.lastFile.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_lastFile(&self) -> &str {
        match self.lastFile.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_lastFile_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.lastFile
    }

    fn mut_lastFile_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.lastFile
    }
}

impl ::protobuf::Message for ReencryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.ezKeyVersionName.is_none() {
            return false;
        }
        if self.submissionTime.is_none() {
            return false;
        }
        if self.canceled.is_none() {
            return false;
        }
        if self.numReencrypted.is_none() {
            return false;
        }
        if self.numFailures.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.submissionTime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.canceled = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.numReencrypted = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.numFailures = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.completionTime = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.lastFile)?;
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
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.submissionTime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.canceled {
            my_size += 2;
        }
        if let Some(v) = self.numReencrypted {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numFailures {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.completionTime {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.lastFile.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.submissionTime {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.canceled {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.numReencrypted {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.numFailures {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.completionTime {
            os.write_uint64(6, v)?;
        }
        if let Some(ref v) = self.lastFile.as_ref() {
            os.write_string(7, &v)?;
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

impl ::protobuf::MessageStatic for ReencryptionInfoProto {
    fn new() -> ReencryptionInfoProto {
        ReencryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReencryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    ReencryptionInfoProto::get_ezKeyVersionName_for_reflect,
                    ReencryptionInfoProto::mut_ezKeyVersionName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "submissionTime",
                    ReencryptionInfoProto::get_submissionTime_for_reflect,
                    ReencryptionInfoProto::mut_submissionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canceled",
                    ReencryptionInfoProto::get_canceled_for_reflect,
                    ReencryptionInfoProto::mut_canceled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "numReencrypted",
                    ReencryptionInfoProto::get_numReencrypted_for_reflect,
                    ReencryptionInfoProto::mut_numReencrypted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "numFailures",
                    ReencryptionInfoProto::get_numFailures_for_reflect,
                    ReencryptionInfoProto::mut_numFailures_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "completionTime",
                    ReencryptionInfoProto::get_completionTime_for_reflect,
                    ReencryptionInfoProto::mut_completionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lastFile",
                    ReencryptionInfoProto::get_lastFile_for_reflect,
                    ReencryptionInfoProto::mut_lastFile_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReencryptionInfoProto>(
                    "ReencryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReencryptionInfoProto {
    fn clear(&mut self) {
        self.clear_ezKeyVersionName();
        self.clear_submissionTime();
        self.clear_canceled();
        self.clear_numReencrypted();
        self.clear_numFailures();
        self.clear_completionTime();
        self.clear_lastFile();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReencryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReencryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CipherOptionProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    inKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    inIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CipherOptionProto {}

impl CipherOptionProto {
    pub fn new() -> CipherOptionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CipherOptionProto {
        static mut instance: ::protobuf::lazy::Lazy<CipherOptionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CipherOptionProto,
        };
        unsafe {
            instance.get(CipherOptionProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // optional bytes inKey = 2;

    pub fn clear_inKey(&mut self) {
        self.inKey.clear();
    }

    pub fn has_inKey(&self) -> bool {
        self.inKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.inKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.inKey.is_none() {
            self.inKey.set_default();
        }
        self.inKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_inKey(&mut self) -> ::std::vec::Vec<u8> {
        self.inKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inKey(&self) -> &[u8] {
        match self.inKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_inKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.inKey
    }

    fn mut_inKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.inKey
    }

    // optional bytes inIv = 3;

    pub fn clear_inIv(&mut self) {
        self.inIv.clear();
    }

    pub fn has_inIv(&self) -> bool {
        self.inIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.inIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inIv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.inIv.is_none() {
            self.inIv.set_default();
        }
        self.inIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_inIv(&mut self) -> ::std::vec::Vec<u8> {
        self.inIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inIv(&self) -> &[u8] {
        match self.inIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_inIv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.inIv
    }

    fn mut_inIv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.inIv
    }

    // optional bytes outKey = 4;

    pub fn clear_outKey(&mut self) {
        self.outKey.clear();
    }

    pub fn has_outKey(&self) -> bool {
        self.outKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.outKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.outKey.is_none() {
            self.outKey.set_default();
        }
        self.outKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_outKey(&mut self) -> ::std::vec::Vec<u8> {
        self.outKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outKey(&self) -> &[u8] {
        match self.outKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_outKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.outKey
    }

    fn mut_outKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.outKey
    }

    // optional bytes outIv = 5;

    pub fn clear_outIv(&mut self) {
        self.outIv.clear();
    }

    pub fn has_outIv(&self) -> bool {
        self.outIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.outIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outIv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.outIv.is_none() {
            self.outIv.set_default();
        }
        self.outIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_outIv(&mut self) -> ::std::vec::Vec<u8> {
        self.outIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outIv(&self) -> &[u8] {
        match self.outIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_outIv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.outIv
    }

    fn mut_outIv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.outIv
    }
}

impl ::protobuf::Message for CipherOptionProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
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
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.inKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.inIv)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.outKey)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.outIv)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.inKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.inIv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.outKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.outIv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.inKey.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.inIv.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.outKey.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.outIv.as_ref() {
            os.write_bytes(5, &v)?;
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

impl ::protobuf::MessageStatic for CipherOptionProto {
    fn new() -> CipherOptionProto {
        CipherOptionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CipherOptionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    CipherOptionProto::get_suite_for_reflect,
                    CipherOptionProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "inKey",
                    CipherOptionProto::get_inKey_for_reflect,
                    CipherOptionProto::mut_inKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "inIv",
                    CipherOptionProto::get_inIv_for_reflect,
                    CipherOptionProto::mut_inIv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "outKey",
                    CipherOptionProto::get_outKey_for_reflect,
                    CipherOptionProto::mut_outKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "outIv",
                    CipherOptionProto::get_outIv_for_reflect,
                    CipherOptionProto::mut_outIv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CipherOptionProto>(
                    "CipherOptionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CipherOptionProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_inKey();
        self.clear_inIv();
        self.clear_outKey();
        self.clear_outIv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CipherOptionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CipherOptionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocatedBlocksProto {
    // message fields
    fileLength: ::std::option::Option<u64>,
    blocks: ::protobuf::RepeatedField<LocatedBlockProto>,
    underConstruction: ::std::option::Option<bool>,
    lastBlock: ::protobuf::SingularPtrField<LocatedBlockProto>,
    isLastBlockComplete: ::std::option::Option<bool>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    ecPolicy: ::protobuf::SingularPtrField<ErasureCodingPolicyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocatedBlocksProto {}

impl LocatedBlocksProto {
    pub fn new() -> LocatedBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlocksProto,
        };
        unsafe {
            instance.get(LocatedBlocksProto::new)
        }
    }

    // required uint64 fileLength = 1;

    pub fn clear_fileLength(&mut self) {
        self.fileLength = ::std::option::Option::None;
    }

    pub fn has_fileLength(&self) -> bool {
        self.fileLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileLength(&mut self, v: u64) {
        self.fileLength = ::std::option::Option::Some(v);
    }

    pub fn get_fileLength(&self) -> u64 {
        self.fileLength.unwrap_or(0)
    }

    fn get_fileLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileLength
    }

    fn mut_fileLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileLength
    }

    // repeated .hadoop.hdfs.LocatedBlockProto blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<LocatedBlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<LocatedBlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<LocatedBlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[LocatedBlockProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<LocatedBlockProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LocatedBlockProto> {
        &mut self.blocks
    }

    // required bool underConstruction = 3;

    pub fn clear_underConstruction(&mut self) {
        self.underConstruction = ::std::option::Option::None;
    }

    pub fn has_underConstruction(&self) -> bool {
        self.underConstruction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_underConstruction(&mut self, v: bool) {
        self.underConstruction = ::std::option::Option::Some(v);
    }

    pub fn get_underConstruction(&self) -> bool {
        self.underConstruction.unwrap_or(false)
    }

    fn get_underConstruction_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.underConstruction
    }

    fn mut_underConstruction_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.underConstruction
    }

    // optional .hadoop.hdfs.LocatedBlockProto lastBlock = 4;

    pub fn clear_lastBlock(&mut self) {
        self.lastBlock.clear();
    }

    pub fn has_lastBlock(&self) -> bool {
        self.lastBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastBlock(&mut self, v: LocatedBlockProto) {
        self.lastBlock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastBlock(&mut self) -> &mut LocatedBlockProto {
        if self.lastBlock.is_none() {
            self.lastBlock.set_default();
        }
        self.lastBlock.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastBlock(&mut self) -> LocatedBlockProto {
        self.lastBlock.take().unwrap_or_else(|| LocatedBlockProto::new())
    }

    pub fn get_lastBlock(&self) -> &LocatedBlockProto {
        self.lastBlock.as_ref().unwrap_or_else(|| LocatedBlockProto::default_instance())
    }

    fn get_lastBlock_for_reflect(&self) -> &::protobuf::SingularPtrField<LocatedBlockProto> {
        &self.lastBlock
    }

    fn mut_lastBlock_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LocatedBlockProto> {
        &mut self.lastBlock
    }

    // required bool isLastBlockComplete = 5;

    pub fn clear_isLastBlockComplete(&mut self) {
        self.isLastBlockComplete = ::std::option::Option::None;
    }

    pub fn has_isLastBlockComplete(&self) -> bool {
        self.isLastBlockComplete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isLastBlockComplete(&mut self, v: bool) {
        self.isLastBlockComplete = ::std::option::Option::Some(v);
    }

    pub fn get_isLastBlockComplete(&self) -> bool {
        self.isLastBlockComplete.unwrap_or(false)
    }

    fn get_isLastBlockComplete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isLastBlockComplete
    }

    fn mut_isLastBlockComplete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isLastBlockComplete
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 6;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo(&mut self) -> &mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        }
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo(&self) -> &FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }

    fn get_fileEncryptionInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &self.fileEncryptionInfo
    }

    fn mut_fileEncryptionInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &mut self.fileEncryptionInfo
    }

    // optional .hadoop.hdfs.ErasureCodingPolicyProto ecPolicy = 7;

    pub fn clear_ecPolicy(&mut self) {
        self.ecPolicy.clear();
    }

    pub fn has_ecPolicy(&self) -> bool {
        self.ecPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicy(&mut self, v: ErasureCodingPolicyProto) {
        self.ecPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicy(&mut self) -> &mut ErasureCodingPolicyProto {
        if self.ecPolicy.is_none() {
            self.ecPolicy.set_default();
        }
        self.ecPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicy(&mut self) -> ErasureCodingPolicyProto {
        self.ecPolicy.take().unwrap_or_else(|| ErasureCodingPolicyProto::new())
    }

    pub fn get_ecPolicy(&self) -> &ErasureCodingPolicyProto {
        self.ecPolicy.as_ref().unwrap_or_else(|| ErasureCodingPolicyProto::default_instance())
    }

    fn get_ecPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &self.ecPolicy
    }

    fn mut_ecPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &mut self.ecPolicy
    }
}

impl ::protobuf::Message for LocatedBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.fileLength.is_none() {
            return false;
        }
        if self.underConstruction.is_none() {
            return false;
        }
        if self.isLastBlockComplete.is_none() {
            return false;
        }
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lastBlock {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileEncryptionInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileLength = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.underConstruction = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastBlock)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isLastBlockComplete = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileEncryptionInfo)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ecPolicy)?;
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
        if let Some(v) = self.fileLength {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.underConstruction {
            my_size += 2;
        }
        if let Some(ref v) = self.lastBlock.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.isLastBlockComplete {
            my_size += 2;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ecPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileLength {
            os.write_uint64(1, v)?;
        }
        for v in &self.blocks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.underConstruction {
            os.write_bool(3, v)?;
        }
        if let Some(ref v) = self.lastBlock.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.isLastBlockComplete {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ecPolicy.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LocatedBlocksProto {
    fn new() -> LocatedBlocksProto {
        LocatedBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileLength",
                    LocatedBlocksProto::get_fileLength_for_reflect,
                    LocatedBlocksProto::mut_fileLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlockProto>>(
                    "blocks",
                    LocatedBlocksProto::get_blocks_for_reflect,
                    LocatedBlocksProto::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "underConstruction",
                    LocatedBlocksProto::get_underConstruction_for_reflect,
                    LocatedBlocksProto::mut_underConstruction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlockProto>>(
                    "lastBlock",
                    LocatedBlocksProto::get_lastBlock_for_reflect,
                    LocatedBlocksProto::mut_lastBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isLastBlockComplete",
                    LocatedBlocksProto::get_isLastBlockComplete_for_reflect,
                    LocatedBlocksProto::mut_isLastBlockComplete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileEncryptionInfoProto>>(
                    "fileEncryptionInfo",
                    LocatedBlocksProto::get_fileEncryptionInfo_for_reflect,
                    LocatedBlocksProto::mut_fileEncryptionInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ErasureCodingPolicyProto>>(
                    "ecPolicy",
                    LocatedBlocksProto::get_ecPolicy_for_reflect,
                    LocatedBlocksProto::mut_ecPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlocksProto>(
                    "LocatedBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlocksProto {
    fn clear(&mut self) {
        self.clear_fileLength();
        self.clear_blocks();
        self.clear_underConstruction();
        self.clear_lastBlock();
        self.clear_isLastBlockComplete();
        self.clear_fileEncryptionInfo();
        self.clear_ecPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocatedBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocatedBlocksProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ECSchemaOptionEntryProto {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ECSchemaOptionEntryProto {}

impl ECSchemaOptionEntryProto {
    pub fn new() -> ECSchemaOptionEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ECSchemaOptionEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<ECSchemaOptionEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ECSchemaOptionEntryProto,
        };
        unsafe {
            instance.get(ECSchemaOptionEntryProto::new)
        }
    }

    // required string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for ECSchemaOptionEntryProto {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        }
        if self.value.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for ECSchemaOptionEntryProto {
    fn new() -> ECSchemaOptionEntryProto {
        ECSchemaOptionEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ECSchemaOptionEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    ECSchemaOptionEntryProto::get_key_for_reflect,
                    ECSchemaOptionEntryProto::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    ECSchemaOptionEntryProto::get_value_for_reflect,
                    ECSchemaOptionEntryProto::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ECSchemaOptionEntryProto>(
                    "ECSchemaOptionEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ECSchemaOptionEntryProto {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ECSchemaOptionEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ECSchemaOptionEntryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ECSchemaProto {
    // message fields
    codecName: ::protobuf::SingularField<::std::string::String>,
    dataUnits: ::std::option::Option<u32>,
    parityUnits: ::std::option::Option<u32>,
    options: ::protobuf::RepeatedField<ECSchemaOptionEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ECSchemaProto {}

impl ECSchemaProto {
    pub fn new() -> ECSchemaProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ECSchemaProto {
        static mut instance: ::protobuf::lazy::Lazy<ECSchemaProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ECSchemaProto,
        };
        unsafe {
            instance.get(ECSchemaProto::new)
        }
    }

    // required string codecName = 1;

    pub fn clear_codecName(&mut self) {
        self.codecName.clear();
    }

    pub fn has_codecName(&self) -> bool {
        self.codecName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codecName(&mut self, v: ::std::string::String) {
        self.codecName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codecName(&mut self) -> &mut ::std::string::String {
        if self.codecName.is_none() {
            self.codecName.set_default();
        }
        self.codecName.as_mut().unwrap()
    }

    // Take field
    pub fn take_codecName(&mut self) -> ::std::string::String {
        self.codecName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codecName(&self) -> &str {
        match self.codecName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_codecName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.codecName
    }

    fn mut_codecName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.codecName
    }

    // required uint32 dataUnits = 2;

    pub fn clear_dataUnits(&mut self) {
        self.dataUnits = ::std::option::Option::None;
    }

    pub fn has_dataUnits(&self) -> bool {
        self.dataUnits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataUnits(&mut self, v: u32) {
        self.dataUnits = ::std::option::Option::Some(v);
    }

    pub fn get_dataUnits(&self) -> u32 {
        self.dataUnits.unwrap_or(0)
    }

    fn get_dataUnits_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dataUnits
    }

    fn mut_dataUnits_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dataUnits
    }

    // required uint32 parityUnits = 3;

    pub fn clear_parityUnits(&mut self) {
        self.parityUnits = ::std::option::Option::None;
    }

    pub fn has_parityUnits(&self) -> bool {
        self.parityUnits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parityUnits(&mut self, v: u32) {
        self.parityUnits = ::std::option::Option::Some(v);
    }

    pub fn get_parityUnits(&self) -> u32 {
        self.parityUnits.unwrap_or(0)
    }

    fn get_parityUnits_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.parityUnits
    }

    fn mut_parityUnits_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.parityUnits
    }

    // repeated .hadoop.hdfs.ECSchemaOptionEntryProto options = 4;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<ECSchemaOptionEntryProto>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<ECSchemaOptionEntryProto> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<ECSchemaOptionEntryProto> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[ECSchemaOptionEntryProto] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<ECSchemaOptionEntryProto> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ECSchemaOptionEntryProto> {
        &mut self.options
    }
}

impl ::protobuf::Message for ECSchemaProto {
    fn is_initialized(&self) -> bool {
        if self.codecName.is_none() {
            return false;
        }
        if self.dataUnits.is_none() {
            return false;
        }
        if self.parityUnits.is_none() {
            return false;
        }
        for v in &self.options {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codecName)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dataUnits = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.parityUnits = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
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
        if let Some(ref v) = self.codecName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.dataUnits {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.parityUnits {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.codecName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.dataUnits {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.parityUnits {
            os.write_uint32(3, v)?;
        }
        for v in &self.options {
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

impl ::protobuf::MessageStatic for ECSchemaProto {
    fn new() -> ECSchemaProto {
        ECSchemaProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ECSchemaProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "codecName",
                    ECSchemaProto::get_codecName_for_reflect,
                    ECSchemaProto::mut_codecName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dataUnits",
                    ECSchemaProto::get_dataUnits_for_reflect,
                    ECSchemaProto::mut_dataUnits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "parityUnits",
                    ECSchemaProto::get_parityUnits_for_reflect,
                    ECSchemaProto::mut_parityUnits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ECSchemaOptionEntryProto>>(
                    "options",
                    ECSchemaProto::get_options_for_reflect,
                    ECSchemaProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ECSchemaProto>(
                    "ECSchemaProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ECSchemaProto {
    fn clear(&mut self) {
        self.clear_codecName();
        self.clear_dataUnits();
        self.clear_parityUnits();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ECSchemaProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ECSchemaProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErasureCodingPolicyProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    schema: ::protobuf::SingularPtrField<ECSchemaProto>,
    cellSize: ::std::option::Option<u32>,
    id: ::std::option::Option<u32>,
    state: ::std::option::Option<ErasureCodingPolicyState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ErasureCodingPolicyProto {}

impl ErasureCodingPolicyProto {
    pub fn new() -> ErasureCodingPolicyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErasureCodingPolicyProto {
        static mut instance: ::protobuf::lazy::Lazy<ErasureCodingPolicyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErasureCodingPolicyProto,
        };
        unsafe {
            instance.get(ErasureCodingPolicyProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .hadoop.hdfs.ECSchemaProto schema = 2;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: ECSchemaProto) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut ECSchemaProto {
        if self.schema.is_none() {
            self.schema.set_default();
        }
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> ECSchemaProto {
        self.schema.take().unwrap_or_else(|| ECSchemaProto::new())
    }

    pub fn get_schema(&self) -> &ECSchemaProto {
        self.schema.as_ref().unwrap_or_else(|| ECSchemaProto::default_instance())
    }

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<ECSchemaProto> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ECSchemaProto> {
        &mut self.schema
    }

    // optional uint32 cellSize = 3;

    pub fn clear_cellSize(&mut self) {
        self.cellSize = ::std::option::Option::None;
    }

    pub fn has_cellSize(&self) -> bool {
        self.cellSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cellSize(&mut self, v: u32) {
        self.cellSize = ::std::option::Option::Some(v);
    }

    pub fn get_cellSize(&self) -> u32 {
        self.cellSize.unwrap_or(0)
    }

    fn get_cellSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cellSize
    }

    fn mut_cellSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cellSize
    }

    // required uint32 id = 4;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional .hadoop.hdfs.ErasureCodingPolicyState state = 5;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ErasureCodingPolicyState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> ErasureCodingPolicyState {
        self.state.unwrap_or(ErasureCodingPolicyState::ENABLED)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<ErasureCodingPolicyState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<ErasureCodingPolicyState> {
        &mut self.state
    }
}

impl ::protobuf::Message for ErasureCodingPolicyProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        for v in &self.schema {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cellSize = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.cellSize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.schema.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.cellSize {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(5, v.value())?;
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

impl ::protobuf::MessageStatic for ErasureCodingPolicyProto {
    fn new() -> ErasureCodingPolicyProto {
        ErasureCodingPolicyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErasureCodingPolicyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ErasureCodingPolicyProto::get_name_for_reflect,
                    ErasureCodingPolicyProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ECSchemaProto>>(
                    "schema",
                    ErasureCodingPolicyProto::get_schema_for_reflect,
                    ErasureCodingPolicyProto::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cellSize",
                    ErasureCodingPolicyProto::get_cellSize_for_reflect,
                    ErasureCodingPolicyProto::mut_cellSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    ErasureCodingPolicyProto::get_id_for_reflect,
                    ErasureCodingPolicyProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErasureCodingPolicyState>>(
                    "state",
                    ErasureCodingPolicyProto::get_state_for_reflect,
                    ErasureCodingPolicyProto::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErasureCodingPolicyProto>(
                    "ErasureCodingPolicyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErasureCodingPolicyProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_schema();
        self.clear_cellSize();
        self.clear_id();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErasureCodingPolicyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErasureCodingPolicyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddErasureCodingPolicyResponseProto {
    // message fields
    policy: ::protobuf::SingularPtrField<ErasureCodingPolicyProto>,
    succeed: ::std::option::Option<bool>,
    errorMsg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddErasureCodingPolicyResponseProto {}

impl AddErasureCodingPolicyResponseProto {
    pub fn new() -> AddErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<AddErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(AddErasureCodingPolicyResponseProto::new)
        }
    }

    // required .hadoop.hdfs.ErasureCodingPolicyProto policy = 1;

    pub fn clear_policy(&mut self) {
        self.policy.clear();
    }

    pub fn has_policy(&self) -> bool {
        self.policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policy(&mut self, v: ErasureCodingPolicyProto) {
        self.policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_policy(&mut self) -> &mut ErasureCodingPolicyProto {
        if self.policy.is_none() {
            self.policy.set_default();
        }
        self.policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_policy(&mut self) -> ErasureCodingPolicyProto {
        self.policy.take().unwrap_or_else(|| ErasureCodingPolicyProto::new())
    }

    pub fn get_policy(&self) -> &ErasureCodingPolicyProto {
        self.policy.as_ref().unwrap_or_else(|| ErasureCodingPolicyProto::default_instance())
    }

    fn get_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &self.policy
    }

    fn mut_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &mut self.policy
    }

    // required bool succeed = 2;

    pub fn clear_succeed(&mut self) {
        self.succeed = ::std::option::Option::None;
    }

    pub fn has_succeed(&self) -> bool {
        self.succeed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_succeed(&mut self, v: bool) {
        self.succeed = ::std::option::Option::Some(v);
    }

    pub fn get_succeed(&self) -> bool {
        self.succeed.unwrap_or(false)
    }

    fn get_succeed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.succeed
    }

    fn mut_succeed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.succeed
    }

    // optional string errorMsg = 3;

    pub fn clear_errorMsg(&mut self) {
        self.errorMsg.clear();
    }

    pub fn has_errorMsg(&self) -> bool {
        self.errorMsg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMsg(&mut self, v: ::std::string::String) {
        self.errorMsg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMsg(&mut self) -> &mut ::std::string::String {
        if self.errorMsg.is_none() {
            self.errorMsg.set_default();
        }
        self.errorMsg.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMsg(&mut self) -> ::std::string::String {
        self.errorMsg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMsg(&self) -> &str {
        match self.errorMsg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_errorMsg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.errorMsg
    }

    fn mut_errorMsg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.errorMsg
    }
}

impl ::protobuf::Message for AddErasureCodingPolicyResponseProto {
    fn is_initialized(&self) -> bool {
        if self.policy.is_none() {
            return false;
        }
        if self.succeed.is_none() {
            return false;
        }
        for v in &self.policy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.policy)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.succeed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.errorMsg)?;
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
        if let Some(ref v) = self.policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.succeed {
            my_size += 2;
        }
        if let Some(ref v) = self.errorMsg.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.policy.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.succeed {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.errorMsg.as_ref() {
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

impl ::protobuf::MessageStatic for AddErasureCodingPolicyResponseProto {
    fn new() -> AddErasureCodingPolicyResponseProto {
        AddErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ErasureCodingPolicyProto>>(
                    "policy",
                    AddErasureCodingPolicyResponseProto::get_policy_for_reflect,
                    AddErasureCodingPolicyResponseProto::mut_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "succeed",
                    AddErasureCodingPolicyResponseProto::get_succeed_for_reflect,
                    AddErasureCodingPolicyResponseProto::mut_succeed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "errorMsg",
                    AddErasureCodingPolicyResponseProto::get_errorMsg_for_reflect,
                    AddErasureCodingPolicyResponseProto::mut_errorMsg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddErasureCodingPolicyResponseProto>(
                    "AddErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.clear_policy();
        self.clear_succeed();
        self.clear_errorMsg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HdfsFileStatusProto {
    // message fields
    fileType: ::std::option::Option<HdfsFileStatusProto_FileType>,
    path: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<u64>,
    permission: ::protobuf::SingularPtrField<super::acl::FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    modification_time: ::std::option::Option<u64>,
    access_time: ::std::option::Option<u64>,
    symlink: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    block_replication: ::std::option::Option<u32>,
    blocksize: ::std::option::Option<u64>,
    locations: ::protobuf::SingularPtrField<LocatedBlocksProto>,
    fileId: ::std::option::Option<u64>,
    childrenNum: ::std::option::Option<i32>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    storagePolicy: ::std::option::Option<u32>,
    ecPolicy: ::protobuf::SingularPtrField<ErasureCodingPolicyProto>,
    flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HdfsFileStatusProto {}

impl HdfsFileStatusProto {
    pub fn new() -> HdfsFileStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HdfsFileStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<HdfsFileStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HdfsFileStatusProto,
        };
        unsafe {
            instance.get(HdfsFileStatusProto::new)
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto.FileType fileType = 1;

    pub fn clear_fileType(&mut self) {
        self.fileType = ::std::option::Option::None;
    }

    pub fn has_fileType(&self) -> bool {
        self.fileType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileType(&mut self, v: HdfsFileStatusProto_FileType) {
        self.fileType = ::std::option::Option::Some(v);
    }

    pub fn get_fileType(&self) -> HdfsFileStatusProto_FileType {
        self.fileType.unwrap_or(HdfsFileStatusProto_FileType::IS_DIR)
    }

    fn get_fileType_for_reflect(&self) -> &::std::option::Option<HdfsFileStatusProto_FileType> {
        &self.fileType
    }

    fn mut_fileType_for_reflect(&mut self) -> &mut ::std::option::Option<HdfsFileStatusProto_FileType> {
        &mut self.fileType
    }

    // required bytes path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::vec::Vec<u8>) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::vec::Vec<u8> {
        self.path.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_path(&self) -> &[u8] {
        match self.path.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.path
    }

    // required uint64 length = 3;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 4;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: super::acl::FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut super::acl::FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> super::acl::FsPermissionProto {
        self.permission.take().unwrap_or_else(|| super::acl::FsPermissionProto::new())
    }

    pub fn get_permission(&self) -> &super::acl::FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| super::acl::FsPermissionProto::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<super::acl::FsPermissionProto> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::acl::FsPermissionProto> {
        &mut self.permission
    }

    // required string owner = 5;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner(&mut self) -> &mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        }
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner(&self) -> &str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner
    }

    fn mut_owner_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner
    }

    // required string group = 6;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // required uint64 modification_time = 7;

    pub fn clear_modification_time(&mut self) {
        self.modification_time = ::std::option::Option::None;
    }

    pub fn has_modification_time(&self) -> bool {
        self.modification_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modification_time(&mut self, v: u64) {
        self.modification_time = ::std::option::Option::Some(v);
    }

    pub fn get_modification_time(&self) -> u64 {
        self.modification_time.unwrap_or(0)
    }

    fn get_modification_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modification_time
    }

    fn mut_modification_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modification_time
    }

    // required uint64 access_time = 8;

    pub fn clear_access_time(&mut self) {
        self.access_time = ::std::option::Option::None;
    }

    pub fn has_access_time(&self) -> bool {
        self.access_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_time(&mut self, v: u64) {
        self.access_time = ::std::option::Option::Some(v);
    }

    pub fn get_access_time(&self) -> u64 {
        self.access_time.unwrap_or(0)
    }

    fn get_access_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.access_time
    }

    fn mut_access_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.access_time
    }

    // optional bytes symlink = 9;

    pub fn clear_symlink(&mut self) {
        self.symlink.clear();
    }

    pub fn has_symlink(&self) -> bool {
        self.symlink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlink(&mut self, v: ::std::vec::Vec<u8>) {
        self.symlink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.symlink.is_none() {
            self.symlink.set_default();
        }
        self.symlink.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlink(&mut self) -> ::std::vec::Vec<u8> {
        self.symlink.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_symlink(&self) -> &[u8] {
        match self.symlink.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_symlink_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.symlink
    }

    fn mut_symlink_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.symlink
    }

    // optional uint32 block_replication = 10;

    pub fn clear_block_replication(&mut self) {
        self.block_replication = ::std::option::Option::None;
    }

    pub fn has_block_replication(&self) -> bool {
        self.block_replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block_replication(&mut self, v: u32) {
        self.block_replication = ::std::option::Option::Some(v);
    }

    pub fn get_block_replication(&self) -> u32 {
        self.block_replication.unwrap_or(0u32)
    }

    fn get_block_replication_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.block_replication
    }

    fn mut_block_replication_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.block_replication
    }

    // optional uint64 blocksize = 11;

    pub fn clear_blocksize(&mut self) {
        self.blocksize = ::std::option::Option::None;
    }

    pub fn has_blocksize(&self) -> bool {
        self.blocksize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocksize(&mut self, v: u64) {
        self.blocksize = ::std::option::Option::Some(v);
    }

    pub fn get_blocksize(&self) -> u64 {
        self.blocksize.unwrap_or(0u64)
    }

    fn get_blocksize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blocksize
    }

    fn mut_blocksize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blocksize
    }

    // optional .hadoop.hdfs.LocatedBlocksProto locations = 12;

    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }

    pub fn has_locations(&self) -> bool {
        self.locations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locations(&mut self, v: LocatedBlocksProto) {
        self.locations = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locations(&mut self) -> &mut LocatedBlocksProto {
        if self.locations.is_none() {
            self.locations.set_default();
        }
        self.locations.as_mut().unwrap()
    }

    // Take field
    pub fn take_locations(&mut self) -> LocatedBlocksProto {
        self.locations.take().unwrap_or_else(|| LocatedBlocksProto::new())
    }

    pub fn get_locations(&self) -> &LocatedBlocksProto {
        self.locations.as_ref().unwrap_or_else(|| LocatedBlocksProto::default_instance())
    }

    fn get_locations_for_reflect(&self) -> &::protobuf::SingularPtrField<LocatedBlocksProto> {
        &self.locations
    }

    fn mut_locations_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LocatedBlocksProto> {
        &mut self.locations
    }

    // optional uint64 fileId = 13;

    pub fn clear_fileId(&mut self) {
        self.fileId = ::std::option::Option::None;
    }

    pub fn has_fileId(&self) -> bool {
        self.fileId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileId(&mut self, v: u64) {
        self.fileId = ::std::option::Option::Some(v);
    }

    pub fn get_fileId(&self) -> u64 {
        self.fileId.unwrap_or(0u64)
    }

    fn get_fileId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileId
    }

    fn mut_fileId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileId
    }

    // optional int32 childrenNum = 14;

    pub fn clear_childrenNum(&mut self) {
        self.childrenNum = ::std::option::Option::None;
    }

    pub fn has_childrenNum(&self) -> bool {
        self.childrenNum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_childrenNum(&mut self, v: i32) {
        self.childrenNum = ::std::option::Option::Some(v);
    }

    pub fn get_childrenNum(&self) -> i32 {
        self.childrenNum.unwrap_or(-1i32)
    }

    fn get_childrenNum_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.childrenNum
    }

    fn mut_childrenNum_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.childrenNum
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 15;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo(&mut self) -> &mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        }
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo(&self) -> &FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }

    fn get_fileEncryptionInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &self.fileEncryptionInfo
    }

    fn mut_fileEncryptionInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &mut self.fileEncryptionInfo
    }

    // optional uint32 storagePolicy = 16;

    pub fn clear_storagePolicy(&mut self) {
        self.storagePolicy = ::std::option::Option::None;
    }

    pub fn has_storagePolicy(&self) -> bool {
        self.storagePolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storagePolicy(&mut self, v: u32) {
        self.storagePolicy = ::std::option::Option::Some(v);
    }

    pub fn get_storagePolicy(&self) -> u32 {
        self.storagePolicy.unwrap_or(0u32)
    }

    fn get_storagePolicy_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.storagePolicy
    }

    fn mut_storagePolicy_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.storagePolicy
    }

    // optional .hadoop.hdfs.ErasureCodingPolicyProto ecPolicy = 17;

    pub fn clear_ecPolicy(&mut self) {
        self.ecPolicy.clear();
    }

    pub fn has_ecPolicy(&self) -> bool {
        self.ecPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicy(&mut self, v: ErasureCodingPolicyProto) {
        self.ecPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicy(&mut self) -> &mut ErasureCodingPolicyProto {
        if self.ecPolicy.is_none() {
            self.ecPolicy.set_default();
        }
        self.ecPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicy(&mut self) -> ErasureCodingPolicyProto {
        self.ecPolicy.take().unwrap_or_else(|| ErasureCodingPolicyProto::new())
    }

    pub fn get_ecPolicy(&self) -> &ErasureCodingPolicyProto {
        self.ecPolicy.as_ref().unwrap_or_else(|| ErasureCodingPolicyProto::default_instance())
    }

    fn get_ecPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &self.ecPolicy
    }

    fn mut_ecPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ErasureCodingPolicyProto> {
        &mut self.ecPolicy
    }

    // optional uint32 flags = 18;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0u32)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }
}

impl ::protobuf::Message for HdfsFileStatusProto {
    fn is_initialized(&self) -> bool {
        if self.fileType.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
        if self.length.is_none() {
            return false;
        }
        if self.permission.is_none() {
            return false;
        }
        if self.owner.is_none() {
            return false;
        }
        if self.group.is_none() {
            return false;
        }
        if self.modification_time.is_none() {
            return false;
        }
        if self.access_time.is_none() {
            return false;
        }
        for v in &self.permission {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileEncryptionInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.fileType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modification_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.access_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.symlink)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.block_replication = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blocksize = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locations)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileId = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.childrenNum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileEncryptionInfo)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.storagePolicy = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ecPolicy)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.fileType {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.modification_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.access_time {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.symlink.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        if let Some(v) = self.block_replication {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blocksize {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.locations.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.fileId {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.childrenNum {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storagePolicy {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ecPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileType {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.modification_time {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.access_time {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.symlink.as_ref() {
            os.write_bytes(9, &v)?;
        }
        if let Some(v) = self.block_replication {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.blocksize {
            os.write_uint64(11, v)?;
        }
        if let Some(ref v) = self.locations.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.fileId {
            os.write_uint64(13, v)?;
        }
        if let Some(v) = self.childrenNum {
            os.write_int32(14, v)?;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storagePolicy {
            os.write_uint32(16, v)?;
        }
        if let Some(ref v) = self.ecPolicy.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(18, v)?;
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

impl ::protobuf::MessageStatic for HdfsFileStatusProto {
    fn new() -> HdfsFileStatusProto {
        HdfsFileStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HdfsFileStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HdfsFileStatusProto_FileType>>(
                    "fileType",
                    HdfsFileStatusProto::get_fileType_for_reflect,
                    HdfsFileStatusProto::mut_fileType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "path",
                    HdfsFileStatusProto::get_path_for_reflect,
                    HdfsFileStatusProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    HdfsFileStatusProto::get_length_for_reflect,
                    HdfsFileStatusProto::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::acl::FsPermissionProto>>(
                    "permission",
                    HdfsFileStatusProto::get_permission_for_reflect,
                    HdfsFileStatusProto::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    HdfsFileStatusProto::get_owner_for_reflect,
                    HdfsFileStatusProto::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    HdfsFileStatusProto::get_group_for_reflect,
                    HdfsFileStatusProto::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modification_time",
                    HdfsFileStatusProto::get_modification_time_for_reflect,
                    HdfsFileStatusProto::mut_modification_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "access_time",
                    HdfsFileStatusProto::get_access_time_for_reflect,
                    HdfsFileStatusProto::mut_access_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "symlink",
                    HdfsFileStatusProto::get_symlink_for_reflect,
                    HdfsFileStatusProto::mut_symlink_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "block_replication",
                    HdfsFileStatusProto::get_block_replication_for_reflect,
                    HdfsFileStatusProto::mut_block_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blocksize",
                    HdfsFileStatusProto::get_blocksize_for_reflect,
                    HdfsFileStatusProto::mut_blocksize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlocksProto>>(
                    "locations",
                    HdfsFileStatusProto::get_locations_for_reflect,
                    HdfsFileStatusProto::mut_locations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileId",
                    HdfsFileStatusProto::get_fileId_for_reflect,
                    HdfsFileStatusProto::mut_fileId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "childrenNum",
                    HdfsFileStatusProto::get_childrenNum_for_reflect,
                    HdfsFileStatusProto::mut_childrenNum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileEncryptionInfoProto>>(
                    "fileEncryptionInfo",
                    HdfsFileStatusProto::get_fileEncryptionInfo_for_reflect,
                    HdfsFileStatusProto::mut_fileEncryptionInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "storagePolicy",
                    HdfsFileStatusProto::get_storagePolicy_for_reflect,
                    HdfsFileStatusProto::mut_storagePolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ErasureCodingPolicyProto>>(
                    "ecPolicy",
                    HdfsFileStatusProto::get_ecPolicy_for_reflect,
                    HdfsFileStatusProto::mut_ecPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    HdfsFileStatusProto::get_flags_for_reflect,
                    HdfsFileStatusProto::mut_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HdfsFileStatusProto>(
                    "HdfsFileStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HdfsFileStatusProto {
    fn clear(&mut self) {
        self.clear_fileType();
        self.clear_path();
        self.clear_length();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_modification_time();
        self.clear_access_time();
        self.clear_symlink();
        self.clear_block_replication();
        self.clear_blocksize();
        self.clear_locations();
        self.clear_fileId();
        self.clear_childrenNum();
        self.clear_fileEncryptionInfo();
        self.clear_storagePolicy();
        self.clear_ecPolicy();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HdfsFileStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HdfsFileStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HdfsFileStatusProto_FileType {
    IS_DIR = 1,
    IS_FILE = 2,
    IS_SYMLINK = 3,
}

impl ::protobuf::ProtobufEnum for HdfsFileStatusProto_FileType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HdfsFileStatusProto_FileType> {
        match value {
            1 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_DIR),
            2 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_FILE),
            3 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HdfsFileStatusProto_FileType] = &[
            HdfsFileStatusProto_FileType::IS_DIR,
            HdfsFileStatusProto_FileType::IS_FILE,
            HdfsFileStatusProto_FileType::IS_SYMLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HdfsFileStatusProto_FileType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HdfsFileStatusProto_FileType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HdfsFileStatusProto_FileType {
}

impl ::protobuf::reflect::ProtobufValue for HdfsFileStatusProto_FileType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HdfsFileStatusProto_Flags {
    HAS_ACL = 1,
    HAS_CRYPT = 2,
    HAS_EC = 4,
}

impl ::protobuf::ProtobufEnum for HdfsFileStatusProto_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HdfsFileStatusProto_Flags> {
        match value {
            1 => ::std::option::Option::Some(HdfsFileStatusProto_Flags::HAS_ACL),
            2 => ::std::option::Option::Some(HdfsFileStatusProto_Flags::HAS_CRYPT),
            4 => ::std::option::Option::Some(HdfsFileStatusProto_Flags::HAS_EC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HdfsFileStatusProto_Flags] = &[
            HdfsFileStatusProto_Flags::HAS_ACL,
            HdfsFileStatusProto_Flags::HAS_CRYPT,
            HdfsFileStatusProto_Flags::HAS_EC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HdfsFileStatusProto_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HdfsFileStatusProto_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HdfsFileStatusProto_Flags {
}

impl ::protobuf::reflect::ProtobufValue for HdfsFileStatusProto_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FsServerDefaultsProto {
    // message fields
    blockSize: ::std::option::Option<u64>,
    bytesPerChecksum: ::std::option::Option<u32>,
    writePacketSize: ::std::option::Option<u32>,
    replication: ::std::option::Option<u32>,
    fileBufferSize: ::std::option::Option<u32>,
    encryptDataTransfer: ::std::option::Option<bool>,
    trashInterval: ::std::option::Option<u64>,
    checksumType: ::std::option::Option<ChecksumTypeProto>,
    keyProviderUri: ::protobuf::SingularField<::std::string::String>,
    policyId: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FsServerDefaultsProto {}

impl FsServerDefaultsProto {
    pub fn new() -> FsServerDefaultsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsServerDefaultsProto {
        static mut instance: ::protobuf::lazy::Lazy<FsServerDefaultsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsServerDefaultsProto,
        };
        unsafe {
            instance.get(FsServerDefaultsProto::new)
        }
    }

    // required uint64 blockSize = 1;

    pub fn clear_blockSize(&mut self) {
        self.blockSize = ::std::option::Option::None;
    }

    pub fn has_blockSize(&self) -> bool {
        self.blockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockSize(&mut self, v: u64) {
        self.blockSize = ::std::option::Option::Some(v);
    }

    pub fn get_blockSize(&self) -> u64 {
        self.blockSize.unwrap_or(0)
    }

    fn get_blockSize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockSize
    }

    fn mut_blockSize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockSize
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

    // required uint32 writePacketSize = 3;

    pub fn clear_writePacketSize(&mut self) {
        self.writePacketSize = ::std::option::Option::None;
    }

    pub fn has_writePacketSize(&self) -> bool {
        self.writePacketSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writePacketSize(&mut self, v: u32) {
        self.writePacketSize = ::std::option::Option::Some(v);
    }

    pub fn get_writePacketSize(&self) -> u32 {
        self.writePacketSize.unwrap_or(0)
    }

    fn get_writePacketSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.writePacketSize
    }

    fn mut_writePacketSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.writePacketSize
    }

    // required uint32 replication = 4;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: u32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication(&self) -> u32 {
        self.replication.unwrap_or(0)
    }

    fn get_replication_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.replication
    }

    fn mut_replication_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.replication
    }

    // required uint32 fileBufferSize = 5;

    pub fn clear_fileBufferSize(&mut self) {
        self.fileBufferSize = ::std::option::Option::None;
    }

    pub fn has_fileBufferSize(&self) -> bool {
        self.fileBufferSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileBufferSize(&mut self, v: u32) {
        self.fileBufferSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileBufferSize(&self) -> u32 {
        self.fileBufferSize.unwrap_or(0)
    }

    fn get_fileBufferSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fileBufferSize
    }

    fn mut_fileBufferSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fileBufferSize
    }

    // optional bool encryptDataTransfer = 6;

    pub fn clear_encryptDataTransfer(&mut self) {
        self.encryptDataTransfer = ::std::option::Option::None;
    }

    pub fn has_encryptDataTransfer(&self) -> bool {
        self.encryptDataTransfer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptDataTransfer(&mut self, v: bool) {
        self.encryptDataTransfer = ::std::option::Option::Some(v);
    }

    pub fn get_encryptDataTransfer(&self) -> bool {
        self.encryptDataTransfer.unwrap_or(false)
    }

    fn get_encryptDataTransfer_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.encryptDataTransfer
    }

    fn mut_encryptDataTransfer_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.encryptDataTransfer
    }

    // optional uint64 trashInterval = 7;

    pub fn clear_trashInterval(&mut self) {
        self.trashInterval = ::std::option::Option::None;
    }

    pub fn has_trashInterval(&self) -> bool {
        self.trashInterval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trashInterval(&mut self, v: u64) {
        self.trashInterval = ::std::option::Option::Some(v);
    }

    pub fn get_trashInterval(&self) -> u64 {
        self.trashInterval.unwrap_or(0u64)
    }

    fn get_trashInterval_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.trashInterval
    }

    fn mut_trashInterval_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.trashInterval
    }

    // optional .hadoop.hdfs.ChecksumTypeProto checksumType = 8;

    pub fn clear_checksumType(&mut self) {
        self.checksumType = ::std::option::Option::None;
    }

    pub fn has_checksumType(&self) -> bool {
        self.checksumType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumType(&mut self, v: ChecksumTypeProto) {
        self.checksumType = ::std::option::Option::Some(v);
    }

    pub fn get_checksumType(&self) -> ChecksumTypeProto {
        self.checksumType.unwrap_or(ChecksumTypeProto::CHECKSUM_CRC32)
    }

    fn get_checksumType_for_reflect(&self) -> &::std::option::Option<ChecksumTypeProto> {
        &self.checksumType
    }

    fn mut_checksumType_for_reflect(&mut self) -> &mut ::std::option::Option<ChecksumTypeProto> {
        &mut self.checksumType
    }

    // optional string keyProviderUri = 9;

    pub fn clear_keyProviderUri(&mut self) {
        self.keyProviderUri.clear();
    }

    pub fn has_keyProviderUri(&self) -> bool {
        self.keyProviderUri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyProviderUri(&mut self, v: ::std::string::String) {
        self.keyProviderUri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyProviderUri(&mut self) -> &mut ::std::string::String {
        if self.keyProviderUri.is_none() {
            self.keyProviderUri.set_default();
        }
        self.keyProviderUri.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyProviderUri(&mut self) -> ::std::string::String {
        self.keyProviderUri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyProviderUri(&self) -> &str {
        match self.keyProviderUri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_keyProviderUri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.keyProviderUri
    }

    fn mut_keyProviderUri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.keyProviderUri
    }

    // optional uint32 policyId = 10;

    pub fn clear_policyId(&mut self) {
        self.policyId = ::std::option::Option::None;
    }

    pub fn has_policyId(&self) -> bool {
        self.policyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policyId(&mut self, v: u32) {
        self.policyId = ::std::option::Option::Some(v);
    }

    pub fn get_policyId(&self) -> u32 {
        self.policyId.unwrap_or(0u32)
    }

    fn get_policyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.policyId
    }

    fn mut_policyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.policyId
    }
}

impl ::protobuf::Message for FsServerDefaultsProto {
    fn is_initialized(&self) -> bool {
        if self.blockSize.is_none() {
            return false;
        }
        if self.bytesPerChecksum.is_none() {
            return false;
        }
        if self.writePacketSize.is_none() {
            return false;
        }
        if self.replication.is_none() {
            return false;
        }
        if self.fileBufferSize.is_none() {
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
                    self.blockSize = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.writePacketSize = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.replication = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fileBufferSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.encryptDataTransfer = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.trashInterval = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.checksumType = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyProviderUri)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.policyId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.blockSize {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytesPerChecksum {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.writePacketSize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replication {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fileBufferSize {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.encryptDataTransfer {
            my_size += 2;
        }
        if let Some(v) = self.trashInterval {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checksumType {
            my_size += ::protobuf::rt::enum_size(8, v);
        }
        if let Some(ref v) = self.keyProviderUri.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(v) = self.policyId {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockSize {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.bytesPerChecksum {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.writePacketSize {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.replication {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.fileBufferSize {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.encryptDataTransfer {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.trashInterval {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.checksumType {
            os.write_enum(8, v.value())?;
        }
        if let Some(ref v) = self.keyProviderUri.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(v) = self.policyId {
            os.write_uint32(10, v)?;
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

impl ::protobuf::MessageStatic for FsServerDefaultsProto {
    fn new() -> FsServerDefaultsProto {
        FsServerDefaultsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsServerDefaultsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockSize",
                    FsServerDefaultsProto::get_blockSize_for_reflect,
                    FsServerDefaultsProto::mut_blockSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerChecksum",
                    FsServerDefaultsProto::get_bytesPerChecksum_for_reflect,
                    FsServerDefaultsProto::mut_bytesPerChecksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "writePacketSize",
                    FsServerDefaultsProto::get_writePacketSize_for_reflect,
                    FsServerDefaultsProto::mut_writePacketSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "replication",
                    FsServerDefaultsProto::get_replication_for_reflect,
                    FsServerDefaultsProto::mut_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fileBufferSize",
                    FsServerDefaultsProto::get_fileBufferSize_for_reflect,
                    FsServerDefaultsProto::mut_fileBufferSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "encryptDataTransfer",
                    FsServerDefaultsProto::get_encryptDataTransfer_for_reflect,
                    FsServerDefaultsProto::mut_encryptDataTransfer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "trashInterval",
                    FsServerDefaultsProto::get_trashInterval_for_reflect,
                    FsServerDefaultsProto::mut_trashInterval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumTypeProto>>(
                    "checksumType",
                    FsServerDefaultsProto::get_checksumType_for_reflect,
                    FsServerDefaultsProto::mut_checksumType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyProviderUri",
                    FsServerDefaultsProto::get_keyProviderUri_for_reflect,
                    FsServerDefaultsProto::mut_keyProviderUri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "policyId",
                    FsServerDefaultsProto::get_policyId_for_reflect,
                    FsServerDefaultsProto::mut_policyId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsServerDefaultsProto>(
                    "FsServerDefaultsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsServerDefaultsProto {
    fn clear(&mut self) {
        self.clear_blockSize();
        self.clear_bytesPerChecksum();
        self.clear_writePacketSize();
        self.clear_replication();
        self.clear_fileBufferSize();
        self.clear_encryptDataTransfer();
        self.clear_trashInterval();
        self.clear_checksumType();
        self.clear_keyProviderUri();
        self.clear_policyId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FsServerDefaultsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FsServerDefaultsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DirectoryListingProto {
    // message fields
    partialListing: ::protobuf::RepeatedField<HdfsFileStatusProto>,
    remainingEntries: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirectoryListingProto {}

impl DirectoryListingProto {
    pub fn new() -> DirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<DirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirectoryListingProto,
        };
        unsafe {
            instance.get(DirectoryListingProto::new)
        }
    }

    // repeated .hadoop.hdfs.HdfsFileStatusProto partialListing = 1;

    pub fn clear_partialListing(&mut self) {
        self.partialListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_partialListing(&mut self, v: ::protobuf::RepeatedField<HdfsFileStatusProto>) {
        self.partialListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partialListing(&mut self) -> &mut ::protobuf::RepeatedField<HdfsFileStatusProto> {
        &mut self.partialListing
    }

    // Take field
    pub fn take_partialListing(&mut self) -> ::protobuf::RepeatedField<HdfsFileStatusProto> {
        ::std::mem::replace(&mut self.partialListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_partialListing(&self) -> &[HdfsFileStatusProto] {
        &self.partialListing
    }

    fn get_partialListing_for_reflect(&self) -> &::protobuf::RepeatedField<HdfsFileStatusProto> {
        &self.partialListing
    }

    fn mut_partialListing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HdfsFileStatusProto> {
        &mut self.partialListing
    }

    // required uint32 remainingEntries = 2;

    pub fn clear_remainingEntries(&mut self) {
        self.remainingEntries = ::std::option::Option::None;
    }

    pub fn has_remainingEntries(&self) -> bool {
        self.remainingEntries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remainingEntries(&mut self, v: u32) {
        self.remainingEntries = ::std::option::Option::Some(v);
    }

    pub fn get_remainingEntries(&self) -> u32 {
        self.remainingEntries.unwrap_or(0)
    }

    fn get_remainingEntries_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.remainingEntries
    }

    fn mut_remainingEntries_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.remainingEntries
    }
}

impl ::protobuf::Message for DirectoryListingProto {
    fn is_initialized(&self) -> bool {
        if self.remainingEntries.is_none() {
            return false;
        }
        for v in &self.partialListing {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partialListing)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.remainingEntries = ::std::option::Option::Some(tmp);
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
        for value in &self.partialListing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.remainingEntries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.partialListing {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.remainingEntries {
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

impl ::protobuf::MessageStatic for DirectoryListingProto {
    fn new() -> DirectoryListingProto {
        DirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HdfsFileStatusProto>>(
                    "partialListing",
                    DirectoryListingProto::get_partialListing_for_reflect,
                    DirectoryListingProto::mut_partialListing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "remainingEntries",
                    DirectoryListingProto::get_remainingEntries_for_reflect,
                    DirectoryListingProto::mut_remainingEntries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirectoryListingProto>(
                    "DirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirectoryListingProto {
    fn clear(&mut self) {
        self.clear_partialListing();
        self.clear_remainingEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirectoryListingProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshottableDirectoryStatusProto {
    // message fields
    dirStatus: ::protobuf::SingularPtrField<HdfsFileStatusProto>,
    snapshot_quota: ::std::option::Option<u32>,
    snapshot_number: ::std::option::Option<u32>,
    parent_fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshottableDirectoryStatusProto {}

impl SnapshottableDirectoryStatusProto {
    pub fn new() -> SnapshottableDirectoryStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryStatusProto,
        };
        unsafe {
            instance.get(SnapshottableDirectoryStatusProto::new)
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto dirStatus = 1;

    pub fn clear_dirStatus(&mut self) {
        self.dirStatus.clear();
    }

    pub fn has_dirStatus(&self) -> bool {
        self.dirStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dirStatus(&mut self, v: HdfsFileStatusProto) {
        self.dirStatus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dirStatus(&mut self) -> &mut HdfsFileStatusProto {
        if self.dirStatus.is_none() {
            self.dirStatus.set_default();
        }
        self.dirStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_dirStatus(&mut self) -> HdfsFileStatusProto {
        self.dirStatus.take().unwrap_or_else(|| HdfsFileStatusProto::new())
    }

    pub fn get_dirStatus(&self) -> &HdfsFileStatusProto {
        self.dirStatus.as_ref().unwrap_or_else(|| HdfsFileStatusProto::default_instance())
    }

    fn get_dirStatus_for_reflect(&self) -> &::protobuf::SingularPtrField<HdfsFileStatusProto> {
        &self.dirStatus
    }

    fn mut_dirStatus_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HdfsFileStatusProto> {
        &mut self.dirStatus
    }

    // required uint32 snapshot_quota = 2;

    pub fn clear_snapshot_quota(&mut self) {
        self.snapshot_quota = ::std::option::Option::None;
    }

    pub fn has_snapshot_quota(&self) -> bool {
        self.snapshot_quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_quota(&mut self, v: u32) {
        self.snapshot_quota = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_quota(&self) -> u32 {
        self.snapshot_quota.unwrap_or(0)
    }

    fn get_snapshot_quota_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshot_quota
    }

    fn mut_snapshot_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshot_quota
    }

    // required uint32 snapshot_number = 3;

    pub fn clear_snapshot_number(&mut self) {
        self.snapshot_number = ::std::option::Option::None;
    }

    pub fn has_snapshot_number(&self) -> bool {
        self.snapshot_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_number(&mut self, v: u32) {
        self.snapshot_number = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_number(&self) -> u32 {
        self.snapshot_number.unwrap_or(0)
    }

    fn get_snapshot_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshot_number
    }

    fn mut_snapshot_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshot_number
    }

    // required bytes parent_fullpath = 4;

    pub fn clear_parent_fullpath(&mut self) {
        self.parent_fullpath.clear();
    }

    pub fn has_parent_fullpath(&self) -> bool {
        self.parent_fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_fullpath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.parent_fullpath.is_none() {
            self.parent_fullpath.set_default();
        }
        self.parent_fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.parent_fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_parent_fullpath(&self) -> &[u8] {
        match self.parent_fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_parent_fullpath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.parent_fullpath
    }

    fn mut_parent_fullpath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.parent_fullpath
    }
}

impl ::protobuf::Message for SnapshottableDirectoryStatusProto {
    fn is_initialized(&self) -> bool {
        if self.dirStatus.is_none() {
            return false;
        }
        if self.snapshot_quota.is_none() {
            return false;
        }
        if self.snapshot_number.is_none() {
            return false;
        }
        if self.parent_fullpath.is_none() {
            return false;
        }
        for v in &self.dirStatus {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dirStatus)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.snapshot_quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.snapshot_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.parent_fullpath)?;
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
        if let Some(ref v) = self.dirStatus.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.snapshot_quota {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.snapshot_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.parent_fullpath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.dirStatus.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.snapshot_quota {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.snapshot_number {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.parent_fullpath.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for SnapshottableDirectoryStatusProto {
    fn new() -> SnapshottableDirectoryStatusProto {
        SnapshottableDirectoryStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HdfsFileStatusProto>>(
                    "dirStatus",
                    SnapshottableDirectoryStatusProto::get_dirStatus_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_dirStatus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshot_quota",
                    SnapshottableDirectoryStatusProto::get_snapshot_quota_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_snapshot_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshot_number",
                    SnapshottableDirectoryStatusProto::get_snapshot_number_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_snapshot_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "parent_fullpath",
                    SnapshottableDirectoryStatusProto::get_parent_fullpath_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_parent_fullpath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryStatusProto>(
                    "SnapshottableDirectoryStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryStatusProto {
    fn clear(&mut self) {
        self.clear_dirStatus();
        self.clear_snapshot_quota();
        self.clear_snapshot_number();
        self.clear_parent_fullpath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshottableDirectoryStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshottableDirectoryListingProto {
    // message fields
    snapshottableDirListing: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshottableDirectoryListingProto {}

impl SnapshottableDirectoryListingProto {
    pub fn new() -> SnapshottableDirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryListingProto,
        };
        unsafe {
            instance.get(SnapshottableDirectoryListingProto::new)
        }
    }

    // repeated .hadoop.hdfs.SnapshottableDirectoryStatusProto snapshottableDirListing = 1;

    pub fn clear_snapshottableDirListing(&mut self) {
        self.snapshottableDirListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_snapshottableDirListing(&mut self, v: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>) {
        self.snapshottableDirListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_snapshottableDirListing(&mut self) -> &mut ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &mut self.snapshottableDirListing
    }

    // Take field
    pub fn take_snapshottableDirListing(&mut self) -> ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        ::std::mem::replace(&mut self.snapshottableDirListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_snapshottableDirListing(&self) -> &[SnapshottableDirectoryStatusProto] {
        &self.snapshottableDirListing
    }

    fn get_snapshottableDirListing_for_reflect(&self) -> &::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &self.snapshottableDirListing
    }

    fn mut_snapshottableDirListing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &mut self.snapshottableDirListing
    }
}

impl ::protobuf::Message for SnapshottableDirectoryListingProto {
    fn is_initialized(&self) -> bool {
        for v in &self.snapshottableDirListing {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.snapshottableDirListing)?;
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
        for value in &self.snapshottableDirListing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.snapshottableDirListing {
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

impl ::protobuf::MessageStatic for SnapshottableDirectoryListingProto {
    fn new() -> SnapshottableDirectoryListingProto {
        SnapshottableDirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshottableDirectoryStatusProto>>(
                    "snapshottableDirListing",
                    SnapshottableDirectoryListingProto::get_snapshottableDirListing_for_reflect,
                    SnapshottableDirectoryListingProto::mut_snapshottableDirListing_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryListingProto>(
                    "SnapshottableDirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryListingProto {
    fn clear(&mut self) {
        self.clear_snapshottableDirListing();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshottableDirectoryListingProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffReportEntryProto {
    // message fields
    fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    modificationLabel: ::protobuf::SingularField<::std::string::String>,
    targetPath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffReportEntryProto {}

impl SnapshotDiffReportEntryProto {
    pub fn new() -> SnapshotDiffReportEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportEntryProto,
        };
        unsafe {
            instance.get(SnapshotDiffReportEntryProto::new)
        }
    }

    // required bytes fullpath = 1;

    pub fn clear_fullpath(&mut self) {
        self.fullpath.clear();
    }

    pub fn has_fullpath(&self) -> bool {
        self.fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fullpath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.fullpath.is_none() {
            self.fullpath.set_default();
        }
        self.fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_fullpath(&self) -> &[u8] {
        match self.fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_fullpath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.fullpath
    }

    fn mut_fullpath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.fullpath
    }

    // required string modificationLabel = 2;

    pub fn clear_modificationLabel(&mut self) {
        self.modificationLabel.clear();
    }

    pub fn has_modificationLabel(&self) -> bool {
        self.modificationLabel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationLabel(&mut self, v: ::std::string::String) {
        self.modificationLabel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modificationLabel(&mut self) -> &mut ::std::string::String {
        if self.modificationLabel.is_none() {
            self.modificationLabel.set_default();
        }
        self.modificationLabel.as_mut().unwrap()
    }

    // Take field
    pub fn take_modificationLabel(&mut self) -> ::std::string::String {
        self.modificationLabel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_modificationLabel(&self) -> &str {
        match self.modificationLabel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_modificationLabel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.modificationLabel
    }

    fn mut_modificationLabel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.modificationLabel
    }

    // optional bytes targetPath = 3;

    pub fn clear_targetPath(&mut self) {
        self.targetPath.clear();
    }

    pub fn has_targetPath(&self) -> bool {
        self.targetPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetPath(&mut self, v: ::std::vec::Vec<u8>) {
        self.targetPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetPath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.targetPath.is_none() {
            self.targetPath.set_default();
        }
        self.targetPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetPath(&mut self) -> ::std::vec::Vec<u8> {
        self.targetPath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_targetPath(&self) -> &[u8] {
        match self.targetPath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_targetPath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.targetPath
    }

    fn mut_targetPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.targetPath
    }
}

impl ::protobuf::Message for SnapshotDiffReportEntryProto {
    fn is_initialized(&self) -> bool {
        if self.fullpath.is_none() {
            return false;
        }
        if self.modificationLabel.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.fullpath)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.modificationLabel)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.targetPath)?;
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
        if let Some(ref v) = self.fullpath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.modificationLabel.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.targetPath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.fullpath.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.modificationLabel.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.targetPath.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for SnapshotDiffReportEntryProto {
    fn new() -> SnapshotDiffReportEntryProto {
        SnapshotDiffReportEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "fullpath",
                    SnapshotDiffReportEntryProto::get_fullpath_for_reflect,
                    SnapshotDiffReportEntryProto::mut_fullpath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "modificationLabel",
                    SnapshotDiffReportEntryProto::get_modificationLabel_for_reflect,
                    SnapshotDiffReportEntryProto::mut_modificationLabel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "targetPath",
                    SnapshotDiffReportEntryProto::get_targetPath_for_reflect,
                    SnapshotDiffReportEntryProto::mut_targetPath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportEntryProto>(
                    "SnapshotDiffReportEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportEntryProto {
    fn clear(&mut self) {
        self.clear_fullpath();
        self.clear_modificationLabel();
        self.clear_targetPath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffReportEntryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffReportProto {
    // message fields
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    fromSnapshot: ::protobuf::SingularField<::std::string::String>,
    toSnapshot: ::protobuf::SingularField<::std::string::String>,
    diffReportEntries: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffReportProto {}

impl SnapshotDiffReportProto {
    pub fn new() -> SnapshotDiffReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportProto,
        };
        unsafe {
            instance.get(SnapshotDiffReportProto::new)
        }
    }

    // required string snapshotRoot = 1;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot(&mut self) -> &mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        }
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot(&self) -> &str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotRoot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotRoot
    }

    fn mut_snapshotRoot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotRoot
    }

    // required string fromSnapshot = 2;

    pub fn clear_fromSnapshot(&mut self) {
        self.fromSnapshot.clear();
    }

    pub fn has_fromSnapshot(&self) -> bool {
        self.fromSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromSnapshot(&mut self, v: ::std::string::String) {
        self.fromSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromSnapshot(&mut self) -> &mut ::std::string::String {
        if self.fromSnapshot.is_none() {
            self.fromSnapshot.set_default();
        }
        self.fromSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromSnapshot(&mut self) -> ::std::string::String {
        self.fromSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromSnapshot(&self) -> &str {
        match self.fromSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fromSnapshot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fromSnapshot
    }

    fn mut_fromSnapshot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fromSnapshot
    }

    // required string toSnapshot = 3;

    pub fn clear_toSnapshot(&mut self) {
        self.toSnapshot.clear();
    }

    pub fn has_toSnapshot(&self) -> bool {
        self.toSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_toSnapshot(&mut self, v: ::std::string::String) {
        self.toSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_toSnapshot(&mut self) -> &mut ::std::string::String {
        if self.toSnapshot.is_none() {
            self.toSnapshot.set_default();
        }
        self.toSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_toSnapshot(&mut self) -> ::std::string::String {
        self.toSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_toSnapshot(&self) -> &str {
        match self.toSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_toSnapshot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.toSnapshot
    }

    fn mut_toSnapshot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.toSnapshot
    }

    // repeated .hadoop.hdfs.SnapshotDiffReportEntryProto diffReportEntries = 4;

    pub fn clear_diffReportEntries(&mut self) {
        self.diffReportEntries.clear();
    }

    // Param is passed by value, moved
    pub fn set_diffReportEntries(&mut self, v: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>) {
        self.diffReportEntries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_diffReportEntries(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &mut self.diffReportEntries
    }

    // Take field
    pub fn take_diffReportEntries(&mut self) -> ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        ::std::mem::replace(&mut self.diffReportEntries, ::protobuf::RepeatedField::new())
    }

    pub fn get_diffReportEntries(&self) -> &[SnapshotDiffReportEntryProto] {
        &self.diffReportEntries
    }

    fn get_diffReportEntries_for_reflect(&self) -> &::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &self.diffReportEntries
    }

    fn mut_diffReportEntries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &mut self.diffReportEntries
    }
}

impl ::protobuf::Message for SnapshotDiffReportProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotRoot.is_none() {
            return false;
        }
        if self.fromSnapshot.is_none() {
            return false;
        }
        if self.toSnapshot.is_none() {
            return false;
        }
        for v in &self.diffReportEntries {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotRoot)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fromSnapshot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.toSnapshot)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.diffReportEntries)?;
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
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.fromSnapshot.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.toSnapshot.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.diffReportEntries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.fromSnapshot.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.toSnapshot.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.diffReportEntries {
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

impl ::protobuf::MessageStatic for SnapshotDiffReportProto {
    fn new() -> SnapshotDiffReportProto {
        SnapshotDiffReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotRoot",
                    SnapshotDiffReportProto::get_snapshotRoot_for_reflect,
                    SnapshotDiffReportProto::mut_snapshotRoot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fromSnapshot",
                    SnapshotDiffReportProto::get_fromSnapshot_for_reflect,
                    SnapshotDiffReportProto::mut_fromSnapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "toSnapshot",
                    SnapshotDiffReportProto::get_toSnapshot_for_reflect,
                    SnapshotDiffReportProto::mut_toSnapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshotDiffReportEntryProto>>(
                    "diffReportEntries",
                    SnapshotDiffReportProto::get_diffReportEntries_for_reflect,
                    SnapshotDiffReportProto::mut_diffReportEntries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportProto>(
                    "SnapshotDiffReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportProto {
    fn clear(&mut self) {
        self.clear_snapshotRoot();
        self.clear_fromSnapshot();
        self.clear_toSnapshot();
        self.clear_diffReportEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffReportProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockProto {
    // message fields
    blockId: ::std::option::Option<u64>,
    genStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockProto {}

impl BlockProto {
    pub fn new() -> BlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockProto,
        };
        unsafe {
            instance.get(BlockProto::new)
        }
    }

    // required uint64 blockId = 1;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    fn get_blockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockId
    }

    fn mut_blockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockId
    }

    // required uint64 genStamp = 2;

    pub fn clear_genStamp(&mut self) {
        self.genStamp = ::std::option::Option::None;
    }

    pub fn has_genStamp(&self) -> bool {
        self.genStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genStamp(&mut self, v: u64) {
        self.genStamp = ::std::option::Option::Some(v);
    }

    pub fn get_genStamp(&self) -> u64 {
        self.genStamp.unwrap_or(0)
    }

    fn get_genStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.genStamp
    }

    fn mut_genStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.genStamp
    }

    // optional uint64 numBytes = 3;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }

    fn get_numBytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numBytes
    }

    fn mut_numBytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numBytes
    }
}

impl ::protobuf::Message for BlockProto {
    fn is_initialized(&self) -> bool {
        if self.blockId.is_none() {
            return false;
        }
        if self.genStamp.is_none() {
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
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genStamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numBytes = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.blockId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.genStamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numBytes {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.genStamp {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.numBytes {
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

impl ::protobuf::MessageStatic for BlockProto {
    fn new() -> BlockProto {
        BlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockId",
                    BlockProto::get_blockId_for_reflect,
                    BlockProto::mut_blockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genStamp",
                    BlockProto::get_genStamp_for_reflect,
                    BlockProto::mut_genStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numBytes",
                    BlockProto::get_numBytes_for_reflect,
                    BlockProto::mut_numBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockProto>(
                    "BlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockProto {
    fn clear(&mut self) {
        self.clear_blockId();
        self.clear_genStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotInfoProto {
    // message fields
    snapshotName: ::protobuf::SingularField<::std::string::String>,
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    permission: ::protobuf::SingularPtrField<super::acl::FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    createTime: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotInfoProto {}

impl SnapshotInfoProto {
    pub fn new() -> SnapshotInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotInfoProto,
        };
        unsafe {
            instance.get(SnapshotInfoProto::new)
        }
    }

    // required string snapshotName = 1;

    pub fn clear_snapshotName(&mut self) {
        self.snapshotName.clear();
    }

    pub fn has_snapshotName(&self) -> bool {
        self.snapshotName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotName(&mut self, v: ::std::string::String) {
        self.snapshotName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotName(&mut self) -> &mut ::std::string::String {
        if self.snapshotName.is_none() {
            self.snapshotName.set_default();
        }
        self.snapshotName.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotName(&mut self) -> ::std::string::String {
        self.snapshotName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotName(&self) -> &str {
        match self.snapshotName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotName
    }

    fn mut_snapshotName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotName
    }

    // required string snapshotRoot = 2;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot(&mut self) -> &mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        }
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot(&self) -> &str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotRoot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotRoot
    }

    fn mut_snapshotRoot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotRoot
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 3;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: super::acl::FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut super::acl::FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> super::acl::FsPermissionProto {
        self.permission.take().unwrap_or_else(|| super::acl::FsPermissionProto::new())
    }

    pub fn get_permission(&self) -> &super::acl::FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| super::acl::FsPermissionProto::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<super::acl::FsPermissionProto> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::acl::FsPermissionProto> {
        &mut self.permission
    }

    // required string owner = 4;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner(&mut self) -> &mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        }
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner(&self) -> &str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner
    }

    fn mut_owner_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner
    }

    // required string group = 5;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // required string createTime = 6;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    pub fn has_createTime(&self) -> bool {
        self.createTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        if self.createTime.is_none() {
            self.createTime.set_default();
        }
        self.createTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        self.createTime.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        match self.createTime.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_createTime_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.createTime
    }
}

impl ::protobuf::Message for SnapshotInfoProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotName.is_none() {
            return false;
        }
        if self.snapshotRoot.is_none() {
            return false;
        }
        if self.permission.is_none() {
            return false;
        }
        if self.owner.is_none() {
            return false;
        }
        if self.group.is_none() {
            return false;
        }
        if self.createTime.is_none() {
            return false;
        }
        for v in &self.permission {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotRoot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.createTime)?;
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
        if let Some(ref v) = self.snapshotName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.createTime.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.snapshotName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.createTime.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for SnapshotInfoProto {
    fn new() -> SnapshotInfoProto {
        SnapshotInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotName",
                    SnapshotInfoProto::get_snapshotName_for_reflect,
                    SnapshotInfoProto::mut_snapshotName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotRoot",
                    SnapshotInfoProto::get_snapshotRoot_for_reflect,
                    SnapshotInfoProto::mut_snapshotRoot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::acl::FsPermissionProto>>(
                    "permission",
                    SnapshotInfoProto::get_permission_for_reflect,
                    SnapshotInfoProto::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    SnapshotInfoProto::get_owner_for_reflect,
                    SnapshotInfoProto::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    SnapshotInfoProto::get_group_for_reflect,
                    SnapshotInfoProto::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    SnapshotInfoProto::get_createTime_for_reflect,
                    SnapshotInfoProto::mut_createTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotInfoProto>(
                    "SnapshotInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotInfoProto {
    fn clear(&mut self) {
        self.clear_snapshotName();
        self.clear_snapshotRoot();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_createTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollingUpgradeStatusProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    finalized: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollingUpgradeStatusProto {}

impl RollingUpgradeStatusProto {
    pub fn new() -> RollingUpgradeStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpgradeStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpgradeStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpgradeStatusProto,
        };
        unsafe {
            instance.get(RollingUpgradeStatusProto::new)
        }
    }

    // required string blockPoolId = 1;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }

    // optional bool finalized = 2;

    pub fn clear_finalized(&mut self) {
        self.finalized = ::std::option::Option::None;
    }

    pub fn has_finalized(&self) -> bool {
        self.finalized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_finalized(&mut self, v: bool) {
        self.finalized = ::std::option::Option::Some(v);
    }

    pub fn get_finalized(&self) -> bool {
        self.finalized.unwrap_or(false)
    }

    fn get_finalized_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.finalized
    }

    fn mut_finalized_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.finalized
    }
}

impl ::protobuf::Message for RollingUpgradeStatusProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.finalized = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.finalized {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.finalized {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for RollingUpgradeStatusProto {
    fn new() -> RollingUpgradeStatusProto {
        RollingUpgradeStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpgradeStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    RollingUpgradeStatusProto::get_blockPoolId_for_reflect,
                    RollingUpgradeStatusProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "finalized",
                    RollingUpgradeStatusProto::get_finalized_for_reflect,
                    RollingUpgradeStatusProto::mut_finalized_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpgradeStatusProto>(
                    "RollingUpgradeStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpgradeStatusProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.clear_finalized();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollingUpgradeStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollingUpgradeStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageUuidsProto {
    // message fields
    storageUuids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageUuidsProto {}

impl StorageUuidsProto {
    pub fn new() -> StorageUuidsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageUuidsProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageUuidsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageUuidsProto,
        };
        unsafe {
            instance.get(StorageUuidsProto::new)
        }
    }

    // repeated string storageUuids = 1;

    pub fn clear_storageUuids(&mut self) {
        self.storageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageUuids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // Take field
    pub fn take_storageUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageUuids(&self) -> &[::std::string::String] {
        &self.storageUuids
    }

    fn get_storageUuids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageUuids
    }

    fn mut_storageUuids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }
}

impl ::protobuf::Message for StorageUuidsProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageUuids)?;
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
        for value in &self.storageUuids {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.storageUuids {
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

impl ::protobuf::MessageStatic for StorageUuidsProto {
    fn new() -> StorageUuidsProto {
        StorageUuidsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageUuidsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuids",
                    StorageUuidsProto::get_storageUuids_for_reflect,
                    StorageUuidsProto::mut_storageUuids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageUuidsProto>(
                    "StorageUuidsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageUuidsProto {
    fn clear(&mut self) {
        self.clear_storageUuids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageUuidsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageUuidsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockTokenSecretProto {
    // message fields
    expiryDate: ::std::option::Option<u64>,
    keyId: ::std::option::Option<u32>,
    userId: ::protobuf::SingularField<::std::string::String>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blockId: ::std::option::Option<u64>,
    modes: ::std::vec::Vec<AccessModeProto>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    storageIds: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockTokenSecretProto {}

impl BlockTokenSecretProto {
    pub fn new() -> BlockTokenSecretProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockTokenSecretProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockTokenSecretProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockTokenSecretProto,
        };
        unsafe {
            instance.get(BlockTokenSecretProto::new)
        }
    }

    // optional uint64 expiryDate = 1;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    fn get_expiryDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expiryDate
    }

    fn mut_expiryDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expiryDate
    }

    // optional uint32 keyId = 2;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    fn get_keyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.keyId
    }

    fn mut_keyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.keyId
    }

    // optional string userId = 3;

    pub fn clear_userId(&mut self) {
        self.userId.clear();
    }

    pub fn has_userId(&self) -> bool {
        self.userId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userId(&mut self, v: ::std::string::String) {
        self.userId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userId(&mut self) -> &mut ::std::string::String {
        if self.userId.is_none() {
            self.userId.set_default();
        }
        self.userId.as_mut().unwrap()
    }

    // Take field
    pub fn take_userId(&mut self) -> ::std::string::String {
        self.userId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_userId(&self) -> &str {
        match self.userId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_userId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.userId
    }

    fn mut_userId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.userId
    }

    // optional string blockPoolId = 4;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }

    // optional uint64 blockId = 5;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    fn get_blockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockId
    }

    fn mut_blockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockId
    }

    // repeated .hadoop.hdfs.AccessModeProto modes = 6;

    pub fn clear_modes(&mut self) {
        self.modes.clear();
    }

    // Param is passed by value, moved
    pub fn set_modes(&mut self, v: ::std::vec::Vec<AccessModeProto>) {
        self.modes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_modes(&mut self) -> &mut ::std::vec::Vec<AccessModeProto> {
        &mut self.modes
    }

    // Take field
    pub fn take_modes(&mut self) -> ::std::vec::Vec<AccessModeProto> {
        ::std::mem::replace(&mut self.modes, ::std::vec::Vec::new())
    }

    pub fn get_modes(&self) -> &[AccessModeProto] {
        &self.modes
    }

    fn get_modes_for_reflect(&self) -> &::std::vec::Vec<AccessModeProto> {
        &self.modes
    }

    fn mut_modes_for_reflect(&mut self) -> &mut ::std::vec::Vec<AccessModeProto> {
        &mut self.modes
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 7;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // repeated string storageIds = 8;

    pub fn clear_storageIds(&mut self) {
        self.storageIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageIds(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageIds(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIds
    }

    // Take field
    pub fn take_storageIds(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageIds(&self) -> &[::std::string::String] {
        &self.storageIds
    }

    fn get_storageIds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageIds
    }

    fn mut_storageIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIds
    }
}

impl ::protobuf::Message for BlockTokenSecretProto {
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
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.userId)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.modes)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageIds)?;
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
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.keyId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.userId.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.blockId {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.modes {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in &self.storageIds {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.expiryDate {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.keyId {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.userId.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.blockId {
            os.write_uint64(5, v)?;
        }
        for v in &self.modes {
            os.write_enum(6, v.value())?;
        };
        for v in &self.storageTypes {
            os.write_enum(7, v.value())?;
        };
        for v in &self.storageIds {
            os.write_string(8, &v)?;
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

impl ::protobuf::MessageStatic for BlockTokenSecretProto {
    fn new() -> BlockTokenSecretProto {
        BlockTokenSecretProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockTokenSecretProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    BlockTokenSecretProto::get_expiryDate_for_reflect,
                    BlockTokenSecretProto::mut_expiryDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "keyId",
                    BlockTokenSecretProto::get_keyId_for_reflect,
                    BlockTokenSecretProto::mut_keyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userId",
                    BlockTokenSecretProto::get_userId_for_reflect,
                    BlockTokenSecretProto::mut_userId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    BlockTokenSecretProto::get_blockPoolId_for_reflect,
                    BlockTokenSecretProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockId",
                    BlockTokenSecretProto::get_blockId_for_reflect,
                    BlockTokenSecretProto::mut_blockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AccessModeProto>>(
                    "modes",
                    BlockTokenSecretProto::get_modes_for_reflect,
                    BlockTokenSecretProto::mut_modes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    BlockTokenSecretProto::get_storageTypes_for_reflect,
                    BlockTokenSecretProto::mut_storageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageIds",
                    BlockTokenSecretProto::get_storageIds_for_reflect,
                    BlockTokenSecretProto::mut_storageIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockTokenSecretProto>(
                    "BlockTokenSecretProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockTokenSecretProto {
    fn clear(&mut self) {
        self.clear_expiryDate();
        self.clear_keyId();
        self.clear_userId();
        self.clear_blockPoolId();
        self.clear_blockId();
        self.clear_modes();
        self.clear_storageTypes();
        self.clear_storageIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockTokenSecretProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockTokenSecretProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StorageTypeProto {
    DISK = 1,
    SSD = 2,
    ARCHIVE = 3,
    RAM_DISK = 4,
}

impl ::protobuf::ProtobufEnum for StorageTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StorageTypeProto> {
        match value {
            1 => ::std::option::Option::Some(StorageTypeProto::DISK),
            2 => ::std::option::Option::Some(StorageTypeProto::SSD),
            3 => ::std::option::Option::Some(StorageTypeProto::ARCHIVE),
            4 => ::std::option::Option::Some(StorageTypeProto::RAM_DISK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StorageTypeProto] = &[
            StorageTypeProto::DISK,
            StorageTypeProto::SSD,
            StorageTypeProto::ARCHIVE,
            StorageTypeProto::RAM_DISK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<StorageTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StorageTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StorageTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for StorageTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BlockTypeProto {
    CONTIGUOUS = 0,
    STRIPED = 1,
}

impl ::protobuf::ProtobufEnum for BlockTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BlockTypeProto> {
        match value {
            0 => ::std::option::Option::Some(BlockTypeProto::CONTIGUOUS),
            1 => ::std::option::Option::Some(BlockTypeProto::STRIPED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BlockTypeProto] = &[
            BlockTypeProto::CONTIGUOUS,
            BlockTypeProto::STRIPED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BlockTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BlockTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BlockTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for BlockTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CipherSuiteProto {
    UNKNOWN = 1,
    AES_CTR_NOPADDING = 2,
}

impl ::protobuf::ProtobufEnum for CipherSuiteProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CipherSuiteProto> {
        match value {
            1 => ::std::option::Option::Some(CipherSuiteProto::UNKNOWN),
            2 => ::std::option::Option::Some(CipherSuiteProto::AES_CTR_NOPADDING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CipherSuiteProto] = &[
            CipherSuiteProto::UNKNOWN,
            CipherSuiteProto::AES_CTR_NOPADDING,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CipherSuiteProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CipherSuiteProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CipherSuiteProto {
}

impl ::protobuf::reflect::ProtobufValue for CipherSuiteProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CryptoProtocolVersionProto {
    UNKNOWN_PROTOCOL_VERSION = 1,
    ENCRYPTION_ZONES = 2,
}

impl ::protobuf::ProtobufEnum for CryptoProtocolVersionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CryptoProtocolVersionProto> {
        match value {
            1 => ::std::option::Option::Some(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION),
            2 => ::std::option::Option::Some(CryptoProtocolVersionProto::ENCRYPTION_ZONES),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CryptoProtocolVersionProto] = &[
            CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION,
            CryptoProtocolVersionProto::ENCRYPTION_ZONES,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CryptoProtocolVersionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CryptoProtocolVersionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CryptoProtocolVersionProto {
}

impl ::protobuf::reflect::ProtobufValue for CryptoProtocolVersionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErasureCodingPolicyState {
    DISABLED = 1,
    ENABLED = 2,
    REMOVED = 3,
}

impl ::protobuf::ProtobufEnum for ErasureCodingPolicyState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErasureCodingPolicyState> {
        match value {
            1 => ::std::option::Option::Some(ErasureCodingPolicyState::DISABLED),
            2 => ::std::option::Option::Some(ErasureCodingPolicyState::ENABLED),
            3 => ::std::option::Option::Some(ErasureCodingPolicyState::REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErasureCodingPolicyState] = &[
            ErasureCodingPolicyState::DISABLED,
            ErasureCodingPolicyState::ENABLED,
            ErasureCodingPolicyState::REMOVED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ErasureCodingPolicyState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErasureCodingPolicyState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErasureCodingPolicyState {
}

impl ::protobuf::reflect::ProtobufValue for ErasureCodingPolicyState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumTypeProto {
    CHECKSUM_NULL = 0,
    CHECKSUM_CRC32 = 1,
    CHECKSUM_CRC32C = 2,
}

impl ::protobuf::ProtobufEnum for ChecksumTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumTypeProto> {
        match value {
            0 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_NULL),
            1 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32),
            2 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32C),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumTypeProto] = &[
            ChecksumTypeProto::CHECKSUM_NULL,
            ChecksumTypeProto::CHECKSUM_CRC32,
            ChecksumTypeProto::CHECKSUM_CRC32C,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ChecksumTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for ChecksumTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AccessModeProto {
    READ = 1,
    WRITE = 2,
    COPY = 3,
    REPLACE = 4,
}

impl ::protobuf::ProtobufEnum for AccessModeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AccessModeProto> {
        match value {
            1 => ::std::option::Option::Some(AccessModeProto::READ),
            2 => ::std::option::Option::Some(AccessModeProto::WRITE),
            3 => ::std::option::Option::Some(AccessModeProto::COPY),
            4 => ::std::option::Option::Some(AccessModeProto::REPLACE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AccessModeProto] = &[
            AccessModeProto::READ,
            AccessModeProto::WRITE,
            AccessModeProto::COPY,
            AccessModeProto::REPLACE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AccessModeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AccessModeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AccessModeProto {
}

impl ::protobuf::reflect::ProtobufValue for AccessModeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nhdfs.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.proto\x1a\tacl.proto\"\
    \x8f\x01\n\x12ExtendedBlockProto\x12\x16\n\x06poolId\x18\x01\x20\x02(\tR\
    \x06poolId\x12\x18\n\x07blockId\x18\x02\x20\x02(\x04R\x07blockId\x12(\n\
    \x0fgenerationStamp\x18\x03\x20\x02(\x04R\x0fgenerationStamp\x12\x1d\n\
    \x08numBytes\x18\x04\x20\x01(\x04:\x010R\x08numBytes\"\xe6\x01\n\x0fData\
    nodeIDProto\x12\x16\n\x06ipAddr\x18\x01\x20\x02(\tR\x06ipAddr\x12\x1a\n\
    \x08hostName\x18\x02\x20\x02(\tR\x08hostName\x12\"\n\x0cdatanodeUuid\x18\
    \x03\x20\x02(\tR\x0cdatanodeUuid\x12\x1a\n\x08xferPort\x18\x04\x20\x02(\
    \rR\x08xferPort\x12\x1a\n\x08infoPort\x18\x05\x20\x02(\rR\x08infoPort\
    \x12\x18\n\x07ipcPort\x18\x06\x20\x02(\rR\x07ipcPort\x12)\n\x0einfoSecur\
    ePort\x18\x07\x20\x01(\r:\x010R\x0einfoSecurePort\"\x80\x01\n\x16Datanod\
    eLocalInfoProto\x12(\n\x0fsoftwareVersion\x18\x01\x20\x02(\tR\x0fsoftwar\
    eVersion\x12$\n\rconfigVersion\x18\x02\x20\x02(\tR\rconfigVersion\x12\
    \x16\n\x06uptime\x18\x03\x20\x02(\x04R\x06uptime\"\xaa\x02\n\x17Datanode\
    VolumeInfoProto\x12\x12\n\x04path\x18\x01\x20\x02(\tR\x04path\x12?\n\x0b\
    storageType\x18\x02\x20\x02(\x0e2\x1d.hadoop.hdfs.StorageTypeProtoR\x0bs\
    torageType\x12\x1c\n\tusedSpace\x18\x03\x20\x02(\x04R\tusedSpace\x12\x1c\
    \n\tfreeSpace\x18\x04\x20\x02(\x04R\tfreeSpace\x12$\n\rreservedSpace\x18\
    \x05\x20\x02(\x04R\rreservedSpace\x12:\n\x18reservedSpaceForReplicas\x18\
    \x06\x20\x02(\x04R\x18reservedSpaceForReplicas\x12\x1c\n\tnumBlocks\x18\
    \x07\x20\x02(\x04R\tnumBlocks\"R\n\x12DatanodeInfosProto\x12<\n\tdatanod\
    es\x18\x01\x20\x03(\x0b2\x1e.hadoop.hdfs.DatanodeInfoProtoR\tdatanodes\"\
    \xb2\x06\n\x11DatanodeInfoProto\x12,\n\x02id\x18\x01\x20\x02(\x0b2\x1c.h\
    adoop.hdfs.DatanodeIDProtoR\x02id\x12\x1d\n\x08capacity\x18\x02\x20\x01(\
    \x04:\x010R\x08capacity\x12\x1b\n\x07dfsUsed\x18\x03\x20\x01(\x04:\x010R\
    \x07dfsUsed\x12\x1f\n\tremaining\x18\x04\x20\x01(\x04:\x010R\tremaining\
    \x12'\n\rblockPoolUsed\x18\x05\x20\x01(\x04:\x010R\rblockPoolUsed\x12!\n\
    \nlastUpdate\x18\x06\x20\x01(\x04:\x010R\nlastUpdate\x12%\n\x0cxceiverCo\
    unt\x18\x07\x20\x01(\r:\x010R\x0cxceiverCount\x12\x1a\n\x08location\x18\
    \x08\x20\x01(\tR\x08location\x12\x1e\n\nnonDfsUsed\x18\t\x20\x01(\x04R\n\
    nonDfsUsed\x12Q\n\nadminState\x18\n\x20\x01(\x0e2).hadoop.hdfs.DatanodeI\
    nfoProto.AdminState:\x06NORMALR\nadminState\x12'\n\rcacheCapacity\x18\
    \x0b\x20\x01(\x04:\x010R\rcacheCapacity\x12\x1f\n\tcacheUsed\x18\x0c\x20\
    \x01(\x04:\x010R\tcacheUsed\x123\n\x13lastUpdateMonotonic\x18\r\x20\x01(\
    \x04:\x010R\x13lastUpdateMonotonic\x12$\n\rupgradeDomain\x18\x0e\x20\x01\
    (\tR\rupgradeDomain\x123\n\x13lastBlockReportTime\x18\x0f\x20\x01(\x04:\
    \x010R\x13lastBlockReportTime\x12=\n\x18lastBlockReportMonotonic\x18\x10\
    \x20\x01(\x04:\x010R\x18lastBlockReportMonotonic\"w\n\nAdminState\x12\n\
    \n\x06NORMAL\x10\0\x12\x1b\n\x17DECOMMISSION_INPROGRESS\x10\x01\x12\x12\
    \n\x0eDECOMMISSIONED\x10\x02\x12\x18\n\x14ENTERING_MAINTENANCE\x10\x03\
    \x12\x12\n\x0eIN_MAINTENANCE\x10\x04\"\xff\x01\n\x14DatanodeStorageProto\
    \x12\x20\n\x0bstorageUuid\x18\x01\x20\x02(\tR\x0bstorageUuid\x12L\n\x05s\
    tate\x18\x02\x20\x01(\x0e2..hadoop.hdfs.DatanodeStorageProto.StorageStat\
    e:\x06NORMALR\x05state\x12E\n\x0bstorageType\x18\x03\x20\x01(\x0e2\x1d.h\
    adoop.hdfs.StorageTypeProto:\x04DISKR\x0bstorageType\"0\n\x0cStorageStat\
    e\x12\n\n\x06NORMAL\x10\0\x12\x14\n\x10READ_ONLY_SHARED\x10\x01\"\xbc\
    \x02\n\x12StorageReportProto\x12$\n\x0bstorageUuid\x18\x01\x20\x02(\tR\
    \x0bstorageUuidB\x02\x18\x01\x12\x1d\n\x06failed\x18\x02\x20\x01(\x08:\
    \x05falseR\x06failed\x12\x1d\n\x08capacity\x18\x03\x20\x01(\x04:\x010R\
    \x08capacity\x12\x1b\n\x07dfsUsed\x18\x04\x20\x01(\x04:\x010R\x07dfsUsed\
    \x12\x1f\n\tremaining\x18\x05\x20\x01(\x04:\x010R\tremaining\x12'\n\rblo\
    ckPoolUsed\x18\x06\x20\x01(\x04:\x010R\rblockPoolUsed\x12;\n\x07storage\
    \x18\x07\x20\x01(\x0b2!.hadoop.hdfs.DatanodeStorageProtoR\x07storage\x12\
    \x1e\n\nnonDfsUsed\x18\x08\x20\x01(\x04R\nnonDfsUsed\"\x96\x04\n\x13Cont\
    entSummaryProto\x12\x16\n\x06length\x18\x01\x20\x02(\x04R\x06length\x12\
    \x1c\n\tfileCount\x18\x02\x20\x02(\x04R\tfileCount\x12&\n\x0edirectoryCo\
    unt\x18\x03\x20\x02(\x04R\x0edirectoryCount\x12\x14\n\x05quota\x18\x04\
    \x20\x02(\x04R\x05quota\x12$\n\rspaceConsumed\x18\x05\x20\x02(\x04R\rspa\
    ceConsumed\x12\x1e\n\nspaceQuota\x18\x06\x20\x02(\x04R\nspaceQuota\x12O\
    \n\x0etypeQuotaInfos\x18\x07\x20\x01(\x0b2'.hadoop.hdfs.StorageTypeQuota\
    InfosProtoR\x0etypeQuotaInfos\x12&\n\x0esnapshotLength\x18\x08\x20\x01(\
    \x04R\x0esnapshotLength\x12,\n\x11snapshotFileCount\x18\t\x20\x01(\x04R\
    \x11snapshotFileCount\x126\n\x16snapshotDirectoryCount\x18\n\x20\x01(\
    \x04R\x16snapshotDirectoryCount\x124\n\x15snapshotSpaceConsumed\x18\x0b\
    \x20\x01(\x04R\x15snapshotSpaceConsumed\x120\n\x13erasureCodingPolicy\
    \x18\x0c\x20\x01(\tR\x13erasureCodingPolicy\"\xf4\x01\n\x0fQuotaUsagePro\
    to\x124\n\x15fileAndDirectoryCount\x18\x01\x20\x02(\x04R\x15fileAndDirec\
    toryCount\x12\x14\n\x05quota\x18\x02\x20\x02(\x04R\x05quota\x12$\n\rspac\
    eConsumed\x18\x03\x20\x02(\x04R\rspaceConsumed\x12\x1e\n\nspaceQuota\x18\
    \x04\x20\x02(\x04R\nspaceQuota\x12O\n\x0etypeQuotaInfos\x18\x05\x20\x01(\
    \x0b2'.hadoop.hdfs.StorageTypeQuotaInfosProtoR\x0etypeQuotaInfos\"j\n\
    \x1aStorageTypeQuotaInfosProto\x12L\n\rtypeQuotaInfo\x18\x01\x20\x03(\
    \x0b2&.hadoop.hdfs.StorageTypeQuotaInfoProtoR\rtypeQuotaInfo\"\x80\x01\n\
    \x19StorageTypeQuotaInfoProto\x121\n\x04type\x18\x01\x20\x02(\x0e2\x1d.h\
    adoop.hdfs.StorageTypeProtoR\x04type\x12\x14\n\x05quota\x18\x02\x20\x02(\
    \x04R\x05quota\x12\x1a\n\x08consumed\x18\x03\x20\x02(\x04R\x08consumed\"\
    F\n\x16CorruptFileBlocksProto\x12\x14\n\x05files\x18\x01\x20\x03(\tR\x05\
    files\x12\x16\n\x06cookie\x18\x02\x20\x02(\tR\x06cookie\"V\n\x11StorageT\
    ypesProto\x12A\n\x0cstorageTypes\x18\x01\x20\x03(\x0e2\x1d.hadoop.hdfs.S\
    torageTypeProtoR\x0cstorageTypes\"\xc7\x02\n\x17BlockStoragePolicyProto\
    \x12\x1a\n\x08policyId\x18\x01\x20\x02(\rR\x08policyId\x12\x12\n\x04name\
    \x18\x02\x20\x02(\tR\x04name\x12F\n\x0ecreationPolicy\x18\x03\x20\x02(\
    \x0b2\x1e.hadoop.hdfs.StorageTypesProtoR\x0ecreationPolicy\x12V\n\x16cre\
    ationFallbackPolicy\x18\x04\x20\x01(\x0b2\x1e.hadoop.hdfs.StorageTypesPr\
    otoR\x16creationFallbackPolicy\x12\\\n\x19replicationFallbackPolicy\x18\
    \x05\x20\x01(\x0b2\x1e.hadoop.hdfs.StorageTypesProtoR\x19replicationFall\
    backPolicy\"\xc7\x03\n\x11LocatedBlockProto\x12-\n\x01b\x18\x01\x20\x02(\
    \x0b2\x1f.hadoop.hdfs.ExtendedBlockProtoR\x01b\x12\x16\n\x06offset\x18\
    \x02\x20\x02(\x04R\x06offset\x122\n\x04locs\x18\x03\x20\x03(\x0b2\x1e.ha\
    doop.hdfs.DatanodeInfoProtoR\x04locs\x12\x18\n\x07corrupt\x18\x04\x20\
    \x02(\x08R\x07corrupt\x129\n\nblockToken\x18\x05\x20\x02(\x0b2\x19.hadoo\
    p.common.TokenProtoR\nblockToken\x12\x1e\n\x08isCached\x18\x06\x20\x03(\
    \x08R\x08isCachedB\x02\x10\x01\x12A\n\x0cstorageTypes\x18\x07\x20\x03(\
    \x0e2\x1d.hadoop.hdfs.StorageTypeProtoR\x0cstorageTypes\x12\x1e\n\nstora\
    geIDs\x18\x08\x20\x03(\tR\nstorageIDs\x12\"\n\x0cblockIndices\x18\t\x20\
    \x01(\x0cR\x0cblockIndices\x12;\n\x0bblockTokens\x18\n\x20\x03(\x0b2\x19\
    .hadoop.common.TokenProtoR\x0bblockTokens\"\xde\x01\n\x16DataEncryptionK\
    eyProto\x12\x14\n\x05keyId\x18\x01\x20\x02(\rR\x05keyId\x12\x20\n\x0bblo\
    ckPoolId\x18\x02\x20\x02(\tR\x0bblockPoolId\x12\x14\n\x05nonce\x18\x03\
    \x20\x02(\x0cR\x05nonce\x12$\n\rencryptionKey\x18\x04\x20\x02(\x0cR\renc\
    ryptionKey\x12\x1e\n\nexpiryDate\x18\x05\x20\x02(\x04R\nexpiryDate\x120\
    \n\x13encryptionAlgorithm\x18\x06\x20\x01(\tR\x13encryptionAlgorithm\"\
    \x95\x02\n\x17FileEncryptionInfoProto\x123\n\x05suite\x18\x01\x20\x02(\
    \x0e2\x1d.hadoop.hdfs.CipherSuiteProtoR\x05suite\x12]\n\x15cryptoProtoco\
    lVersion\x18\x02\x20\x02(\x0e2'.hadoop.hdfs.CryptoProtocolVersionProtoR\
    \x15cryptoProtocolVersion\x12\x10\n\x03key\x18\x03\x20\x02(\x0cR\x03key\
    \x12\x0e\n\x02iv\x18\x04\x20\x02(\x0cR\x02iv\x12\x18\n\x07keyName\x18\
    \x05\x20\x02(\tR\x07keyName\x12*\n\x10ezKeyVersionName\x18\x06\x20\x02(\
    \tR\x10ezKeyVersionName\"j\n\x1aPerFileEncryptionInfoProto\x12\x10\n\x03\
    key\x18\x01\x20\x02(\x0cR\x03key\x12\x0e\n\x02iv\x18\x02\x20\x02(\x0cR\
    \x02iv\x12*\n\x10ezKeyVersionName\x18\x03\x20\x02(\tR\x10ezKeyVersionNam\
    e\"\x99\x02\n\x17ZoneEncryptionInfoProto\x123\n\x05suite\x18\x01\x20\x02\
    (\x0e2\x1d.hadoop.hdfs.CipherSuiteProtoR\x05suite\x12]\n\x15cryptoProtoc\
    olVersion\x18\x02\x20\x02(\x0e2'.hadoop.hdfs.CryptoProtocolVersionProtoR\
    \x15cryptoProtocolVersion\x12\x18\n\x07keyName\x18\x03\x20\x02(\tR\x07ke\
    yName\x12P\n\x11reencryptionProto\x18\x04\x20\x01(\x0b2\".hadoop.hdfs.Re\
    encryptionInfoProtoR\x11reencryptionProto\"\x95\x02\n\x15ReencryptionInf\
    oProto\x12*\n\x10ezKeyVersionName\x18\x01\x20\x02(\tR\x10ezKeyVersionNam\
    e\x12&\n\x0esubmissionTime\x18\x02\x20\x02(\x04R\x0esubmissionTime\x12\
    \x1a\n\x08canceled\x18\x03\x20\x02(\x08R\x08canceled\x12&\n\x0enumReencr\
    ypted\x18\x04\x20\x02(\x03R\x0enumReencrypted\x12\x20\n\x0bnumFailures\
    \x18\x05\x20\x02(\x03R\x0bnumFailures\x12&\n\x0ecompletionTime\x18\x06\
    \x20\x01(\x04R\x0ecompletionTime\x12\x1a\n\x08lastFile\x18\x07\x20\x01(\
    \tR\x08lastFile\"\xa0\x01\n\x11CipherOptionProto\x123\n\x05suite\x18\x01\
    \x20\x02(\x0e2\x1d.hadoop.hdfs.CipherSuiteProtoR\x05suite\x12\x14\n\x05i\
    nKey\x18\x02\x20\x01(\x0cR\x05inKey\x12\x12\n\x04inIv\x18\x03\x20\x01(\
    \x0cR\x04inIv\x12\x16\n\x06outKey\x18\x04\x20\x01(\x0cR\x06outKey\x12\
    \x14\n\x05outIv\x18\x05\x20\x01(\x0cR\x05outIv\"\xa3\x03\n\x12LocatedBlo\
    cksProto\x12\x1e\n\nfileLength\x18\x01\x20\x02(\x04R\nfileLength\x126\n\
    \x06blocks\x18\x02\x20\x03(\x0b2\x1e.hadoop.hdfs.LocatedBlockProtoR\x06b\
    locks\x12,\n\x11underConstruction\x18\x03\x20\x02(\x08R\x11underConstruc\
    tion\x12<\n\tlastBlock\x18\x04\x20\x01(\x0b2\x1e.hadoop.hdfs.LocatedBloc\
    kProtoR\tlastBlock\x120\n\x13isLastBlockComplete\x18\x05\x20\x02(\x08R\
    \x13isLastBlockComplete\x12T\n\x12fileEncryptionInfo\x18\x06\x20\x01(\
    \x0b2$.hadoop.hdfs.FileEncryptionInfoProtoR\x12fileEncryptionInfo\x12A\n\
    \x08ecPolicy\x18\x07\x20\x01(\x0b2%.hadoop.hdfs.ErasureCodingPolicyProto\
    R\x08ecPolicy\"B\n\x18ECSchemaOptionEntryProto\x12\x10\n\x03key\x18\x01\
    \x20\x02(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x02(\tR\x05value\"\
    \xae\x01\n\rECSchemaProto\x12\x1c\n\tcodecName\x18\x01\x20\x02(\tR\tcode\
    cName\x12\x1c\n\tdataUnits\x18\x02\x20\x02(\rR\tdataUnits\x12\x20\n\x0bp\
    arityUnits\x18\x03\x20\x02(\rR\x0bparityUnits\x12?\n\x07options\x18\x04\
    \x20\x03(\x0b2%.hadoop.hdfs.ECSchemaOptionEntryProtoR\x07options\"\xd4\
    \x01\n\x18ErasureCodingPolicyProto\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x122\n\x06schema\x18\x02\x20\x01(\x0b2\x1a.hadoop.hdfs.ECSchema\
    ProtoR\x06schema\x12\x1a\n\x08cellSize\x18\x03\x20\x01(\rR\x08cellSize\
    \x12\x0e\n\x02id\x18\x04\x20\x02(\rR\x02id\x12D\n\x05state\x18\x05\x20\
    \x01(\x0e2%.hadoop.hdfs.ErasureCodingPolicyState:\x07ENABLEDR\x05state\"\
    \x9a\x01\n#AddErasureCodingPolicyResponseProto\x12=\n\x06policy\x18\x01\
    \x20\x02(\x0b2%.hadoop.hdfs.ErasureCodingPolicyProtoR\x06policy\x12\x18\
    \n\x07succeed\x18\x02\x20\x02(\x08R\x07succeed\x12\x1a\n\x08errorMsg\x18\
    \x03\x20\x01(\tR\x08errorMsg\"\xee\x06\n\x13HdfsFileStatusProto\x12E\n\
    \x08fileType\x18\x01\x20\x02(\x0e2).hadoop.hdfs.HdfsFileStatusProto.File\
    TypeR\x08fileType\x12\x12\n\x04path\x18\x02\x20\x02(\x0cR\x04path\x12\
    \x16\n\x06length\x18\x03\x20\x02(\x04R\x06length\x12>\n\npermission\x18\
    \x04\x20\x02(\x0b2\x1e.hadoop.hdfs.FsPermissionProtoR\npermission\x12\
    \x14\n\x05owner\x18\x05\x20\x02(\tR\x05owner\x12\x14\n\x05group\x18\x06\
    \x20\x02(\tR\x05group\x12+\n\x11modification_time\x18\x07\x20\x02(\x04R\
    \x10modificationTime\x12\x1f\n\x0baccess_time\x18\x08\x20\x02(\x04R\nacc\
    essTime\x12\x18\n\x07symlink\x18\t\x20\x01(\x0cR\x07symlink\x12.\n\x11bl\
    ock_replication\x18\n\x20\x01(\r:\x010R\x10blockReplication\x12\x1f\n\tb\
    locksize\x18\x0b\x20\x01(\x04:\x010R\tblocksize\x12=\n\tlocations\x18\
    \x0c\x20\x01(\x0b2\x1f.hadoop.hdfs.LocatedBlocksProtoR\tlocations\x12\
    \x19\n\x06fileId\x18\r\x20\x01(\x04:\x010R\x06fileId\x12$\n\x0bchildrenN\
    um\x18\x0e\x20\x01(\x05:\x02-1R\x0bchildrenNum\x12T\n\x12fileEncryptionI\
    nfo\x18\x0f\x20\x01(\x0b2$.hadoop.hdfs.FileEncryptionInfoProtoR\x12fileE\
    ncryptionInfo\x12'\n\rstoragePolicy\x18\x10\x20\x01(\r:\x010R\rstoragePo\
    licy\x12A\n\x08ecPolicy\x18\x11\x20\x01(\x0b2%.hadoop.hdfs.ErasureCoding\
    PolicyProtoR\x08ecPolicy\x12\x17\n\x05flags\x18\x12\x20\x01(\r:\x010R\
    \x05flags\"3\n\x08FileType\x12\n\n\x06IS_DIR\x10\x01\x12\x0b\n\x07IS_FIL\
    E\x10\x02\x12\x0e\n\nIS_SYMLINK\x10\x03\"/\n\x05Flags\x12\x0b\n\x07HAS_A\
    CL\x10\x01\x12\r\n\tHAS_CRYPT\x10\x02\x12\n\n\x06HAS_EC\x10\x04\"\xd2\
    \x03\n\x15FsServerDefaultsProto\x12\x1c\n\tblockSize\x18\x01\x20\x02(\
    \x04R\tblockSize\x12*\n\x10bytesPerChecksum\x18\x02\x20\x02(\rR\x10bytes\
    PerChecksum\x12(\n\x0fwritePacketSize\x18\x03\x20\x02(\rR\x0fwritePacket\
    Size\x12\x20\n\x0breplication\x18\x04\x20\x02(\rR\x0breplication\x12&\n\
    \x0efileBufferSize\x18\x05\x20\x02(\rR\x0efileBufferSize\x127\n\x13encry\
    ptDataTransfer\x18\x06\x20\x01(\x08:\x05falseR\x13encryptDataTransfer\
    \x12'\n\rtrashInterval\x18\x07\x20\x01(\x04:\x010R\rtrashInterval\x12R\n\
    \x0cchecksumType\x18\x08\x20\x01(\x0e2\x1e.hadoop.hdfs.ChecksumTypeProto\
    :\x0eCHECKSUM_CRC32R\x0cchecksumType\x12&\n\x0ekeyProviderUri\x18\t\x20\
    \x01(\tR\x0ekeyProviderUri\x12\x1d\n\x08policyId\x18\n\x20\x01(\r:\x010R\
    \x08policyId\"\x8d\x01\n\x15DirectoryListingProto\x12H\n\x0epartialListi\
    ng\x18\x01\x20\x03(\x0b2\x20.hadoop.hdfs.HdfsFileStatusProtoR\x0epartial\
    Listing\x12*\n\x10remainingEntries\x18\x02\x20\x02(\rR\x10remainingEntri\
    es\"\xdc\x01\n!SnapshottableDirectoryStatusProto\x12>\n\tdirStatus\x18\
    \x01\x20\x02(\x0b2\x20.hadoop.hdfs.HdfsFileStatusProtoR\tdirStatus\x12%\
    \n\x0esnapshot_quota\x18\x02\x20\x02(\rR\rsnapshotQuota\x12'\n\x0fsnapsh\
    ot_number\x18\x03\x20\x02(\rR\x0esnapshotNumber\x12'\n\x0fparent_fullpat\
    h\x18\x04\x20\x02(\x0cR\x0eparentFullpath\"\x8e\x01\n\"SnapshottableDire\
    ctoryListingProto\x12h\n\x17snapshottableDirListing\x18\x01\x20\x03(\x0b\
    2..hadoop.hdfs.SnapshottableDirectoryStatusProtoR\x17snapshottableDirLis\
    ting\"\x88\x01\n\x1cSnapshotDiffReportEntryProto\x12\x1a\n\x08fullpath\
    \x18\x01\x20\x02(\x0cR\x08fullpath\x12,\n\x11modificationLabel\x18\x02\
    \x20\x02(\tR\x11modificationLabel\x12\x1e\n\ntargetPath\x18\x03\x20\x01(\
    \x0cR\ntargetPath\"\xda\x01\n\x17SnapshotDiffReportProto\x12\"\n\x0csnap\
    shotRoot\x18\x01\x20\x02(\tR\x0csnapshotRoot\x12\"\n\x0cfromSnapshot\x18\
    \x02\x20\x02(\tR\x0cfromSnapshot\x12\x1e\n\ntoSnapshot\x18\x03\x20\x02(\
    \tR\ntoSnapshot\x12W\n\x11diffReportEntries\x18\x04\x20\x03(\x0b2).hadoo\
    p.hdfs.SnapshotDiffReportEntryProtoR\x11diffReportEntries\"a\n\nBlockPro\
    to\x12\x18\n\x07blockId\x18\x01\x20\x02(\x04R\x07blockId\x12\x1a\n\x08ge\
    nStamp\x18\x02\x20\x02(\x04R\x08genStamp\x12\x1d\n\x08numBytes\x18\x03\
    \x20\x01(\x04:\x010R\x08numBytes\"\xe7\x01\n\x11SnapshotInfoProto\x12\"\
    \n\x0csnapshotName\x18\x01\x20\x02(\tR\x0csnapshotName\x12\"\n\x0csnapsh\
    otRoot\x18\x02\x20\x02(\tR\x0csnapshotRoot\x12>\n\npermission\x18\x03\
    \x20\x02(\x0b2\x1e.hadoop.hdfs.FsPermissionProtoR\npermission\x12\x14\n\
    \x05owner\x18\x04\x20\x02(\tR\x05owner\x12\x14\n\x05group\x18\x05\x20\
    \x02(\tR\x05group\x12\x1e\n\ncreateTime\x18\x06\x20\x02(\tR\ncreateTime\
    \"b\n\x19RollingUpgradeStatusProto\x12\x20\n\x0bblockPoolId\x18\x01\x20\
    \x02(\tR\x0bblockPoolId\x12#\n\tfinalized\x18\x02\x20\x01(\x08:\x05false\
    R\tfinalized\"7\n\x11StorageUuidsProto\x12\"\n\x0cstorageUuids\x18\x01\
    \x20\x03(\tR\x0cstorageUuids\"\xb8\x02\n\x15BlockTokenSecretProto\x12\
    \x1e\n\nexpiryDate\x18\x01\x20\x01(\x04R\nexpiryDate\x12\x14\n\x05keyId\
    \x18\x02\x20\x01(\rR\x05keyId\x12\x16\n\x06userId\x18\x03\x20\x01(\tR\
    \x06userId\x12\x20\n\x0bblockPoolId\x18\x04\x20\x01(\tR\x0bblockPoolId\
    \x12\x18\n\x07blockId\x18\x05\x20\x01(\x04R\x07blockId\x122\n\x05modes\
    \x18\x06\x20\x03(\x0e2\x1c.hadoop.hdfs.AccessModeProtoR\x05modes\x12A\n\
    \x0cstorageTypes\x18\x07\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageTypeProtoR\
    \x0cstorageTypes\x12\x1e\n\nstorageIds\x18\x08\x20\x03(\tR\nstorageIds*@\
    \n\x10StorageTypeProto\x12\x08\n\x04DISK\x10\x01\x12\x07\n\x03SSD\x10\
    \x02\x12\x0b\n\x07ARCHIVE\x10\x03\x12\x0c\n\x08RAM_DISK\x10\x04*-\n\x0eB\
    lockTypeProto\x12\x0e\n\nCONTIGUOUS\x10\0\x12\x0b\n\x07STRIPED\x10\x01*6\
    \n\x10CipherSuiteProto\x12\x0b\n\x07UNKNOWN\x10\x01\x12\x15\n\x11AES_CTR\
    _NOPADDING\x10\x02*P\n\x1aCryptoProtocolVersionProto\x12\x1c\n\x18UNKNOW\
    N_PROTOCOL_VERSION\x10\x01\x12\x14\n\x10ENCRYPTION_ZONES\x10\x02*B\n\x18\
    ErasureCodingPolicyState\x12\x0c\n\x08DISABLED\x10\x01\x12\x0b\n\x07ENAB\
    LED\x10\x02\x12\x0b\n\x07REMOVED\x10\x03*O\n\x11ChecksumTypeProto\x12\
    \x11\n\rCHECKSUM_NULL\x10\0\x12\x12\n\x0eCHECKSUM_CRC32\x10\x01\x12\x13\
    \n\x0fCHECKSUM_CRC32C\x10\x02*=\n\x0fAccessModeProto\x12\x08\n\x04READ\
    \x10\x01\x12\t\n\x05WRITE\x10\x02\x12\x08\n\x04COPY\x10\x03\x12\x0b\n\
    \x07REPLACE\x10\x04B6\n%org.apache.hadoop.hdfs.protocol.protoB\nHdfsProt\
    os\xa0\x01\x01J\xce\xc1\x01\n\x07\x12\x05\x1c\0\xd8\x04\x01\n\x08\n\x01\
    \x08\x12\x03\x1c\0>\n\xc1\x08\n\x04\x08\xe7\x07\0\x12\x03\x1c\0>2\x83\
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
    \x20*stable*\x20.proto\x20interface.\n2\x80\x01\x20This\x20file\x20conta\
    ins\x20protocol\x20buffers\x20that\x20are\x20used\x20throughout\x20HDFS\
    \x20--\x20i.e.\n\x20by\x20the\x20client,\x20server,\x20and\x20data\x20tr\
    ansfer\x20protocols.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x1c\x07\x13\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x1c\x07\x13\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x1c\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\
    \x03\x1c\x16=\n\x08\n\x01\x08\x12\x03\x1d\0+\n\x0b\n\x04\x08\xe7\x07\x01\
    \x12\x03\x1d\0+\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x1d\x07\x1b\n\r\
    \n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x1d\x07\x1b\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x1d\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\
    \x03\x1d\x1e*\n\x08\n\x01\x08\x12\x03\x1e\0,\n\x0b\n\x04\x08\xe7\x07\x02\
    \x12\x03\x1e\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1e\x07$\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\x1e\x07$\n\x0e\n\x07\x08\xe7\x07\x02\
    \x02\0\x01\x12\x03\x1e\x07$\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1e'\
    +\n\x08\n\x01\x02\x12\x03\x1f\x08\x13\n\t\n\x02\x03\0\x12\x03!\x07\x17\n\
    \t\n\x02\x03\x01\x12\x03\"\x07\x12\n/\n\x02\x04\0\x12\x04'\0-\x01\x1a#*\
    \n\x20Extended\x20block\x20idenfies\x20a\x20block\n\n\n\n\x03\x04\0\x01\
    \x12\x03'\x08\x1a\n>\n\x04\x04\0\x02\0\x12\x03(\x02\x1d\"1\x20Block\x20p\
    ool\x20id\x20-\x20gloablly\x20unique\x20across\x20clusters\n\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03(\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03(\x0b\
    \x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03(\x12\x18\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03(\x1b\x1c\n)\n\x04\x04\0\x02\x01\x12\x03)\x02\x1e\"\x1c\
    \x20the\x20local\x20id\x20within\x20a\x20pool\n\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03)\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03)\x12\x19\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03)\x1c\x1d\n\x0b\n\x04\x04\0\x02\x02\x12\x03*\x02&\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03*\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03*\x12!\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03*$%\n+\n\x04\x04\0\x02\x03\x12\x03+\x02-\"\x1e\
    \x20len\x20does\x20not\x20belong\x20in\x20ebid\x20\n\n\x0c\n\x05\x04\0\
    \x02\x03\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03+\x0b\
    \x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03+\x12\x1a\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03+\x1d\x1e\n\x0c\n\x05\x04\0\x02\x03\x08\x12\x03+\x1f\
    ,\n\x0c\n\x05\x04\0\x02\x03\x07\x12\x03+*+\n%\n\x02\x04\x01\x12\x042\0=\
    \x01\x1a\x19*\n\x20Identifies\x20a\x20Datanode\n\n\n\n\x03\x04\x01\x01\
    \x12\x032\x08\x17\n\x19\n\x04\x04\x01\x02\0\x12\x033\x02\x1d\"\x0c\x20IP\
    \x20address\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x033\x02\n\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x033\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x033\x12\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x033\x1b\x1c\n\x17\n\x04\
    \x04\x01\x02\x01\x12\x034\x02\x1f\"\n\x20hostname\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x034\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x034\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x034\x12\x1a\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x034\x1d\x1e\n1\n\x04\x04\x01\x02\x02\x12\x035\x02#\
    \"$\x20UUID\x20assigned\x20to\x20the\x20Datanode.\x20For\n\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x035\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\
    \x035\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x035\x12\x1e\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x035!\"\nu\n\x04\x04\x01\x02\x03\x12\x039\
    \x02\x1f\x1aQ\x20upgraded\x20clusters\x20this\x20is\x20the\x20same\n\x20\
    as\x20the\x20original\x20StorageID\x20of\x20the\n\x20Datanode.\n\"\x15\
    \x20data\x20streaming\x20port\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x039\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x039\x0b\x11\n\x0c\n\x05\x04\
    \x01\x02\x03\x01\x12\x039\x12\x1a\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\
    \x039\x1d\x1e\n!\n\x04\x04\x01\x02\x04\x12\x03:\x02\x1f\"\x14\x20datanod\
    e\x20http\x20port\n\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03:\x02\n\n\x0c\
    \n\x05\x04\x01\x02\x04\x05\x12\x03:\x0b\x11\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03:\x12\x1a\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03:\x1d\x1e\n\
    \x1e\n\x04\x04\x01\x02\x05\x12\x03;\x02\x1e\"\x11\x20ipc\x20server\x20po\
    rt\n\n\x0c\n\x05\x04\x01\x02\x05\x04\x12\x03;\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x05\x05\x12\x03;\x0b\x11\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03;\
    \x12\x19\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03;\x1c\x1d\n\"\n\x04\x04\
    \x01\x02\x06\x12\x03<\x023\"\x15\x20datanode\x20https\x20port\n\n\x0c\n\
    \x05\x04\x01\x02\x06\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\x01\x02\x06\x05\
    \x12\x03<\x0b\x11\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03<\x12\x20\n\x0c\
    \n\x05\x04\x01\x02\x06\x03\x12\x03<#$\n\x0c\n\x05\x04\x01\x02\x06\x08\
    \x12\x03<%2\n\x0c\n\x05\x04\x01\x02\x06\x07\x12\x03<01\n*\n\x02\x04\x02\
    \x12\x04B\0F\x01\x1a\x1e*\n\x20Datanode\x20local\x20information\n\n\n\n\
    \x03\x04\x02\x01\x12\x03B\x08\x1e\n\x0b\n\x04\x04\x02\x02\0\x12\x03C\x02\
    &\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03C\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03C\x12!\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03C$%\n\x0b\n\x04\x04\x02\x02\x01\x12\x03D\
    \x02$\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03D\x02\n\n\x0c\n\x05\x04\x02\
    \x02\x01\x05\x12\x03D\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03D\
    \x12\x1f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03D\"#\n\x0b\n\x04\x04\x02\
    \x02\x02\x12\x03E\x02\x1d\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03E\x02\n\
    \n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03E\x0b\x11\n\x0c\n\x05\x04\x02\
    \x02\x02\x01\x12\x03E\x12\x18\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03E\
    \x1b\x1c\n+\n\x02\x04\x03\x12\x04K\0S\x01\x1a\x1f*\n\x20Datanode\x20volu\
    me\x20information\n\n\n\n\x03\x04\x03\x01\x12\x03K\x08\x1f\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03L\x02\x1b\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03L\
    \x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03L\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03L\x12\x16\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03L\
    \x19\x1a\n\x0b\n\x04\x04\x03\x02\x01\x12\x03M\x02,\n\x0c\n\x05\x04\x03\
    \x02\x01\x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03M\x0b\
    \x1b\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03M\x1c'\n\x0c\n\x05\x04\x03\
    \x02\x01\x03\x12\x03M*+\n\x0b\n\x04\x04\x03\x02\x02\x12\x03N\x02\x20\n\
    \x0c\n\x05\x04\x03\x02\x02\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x02\x05\x12\x03N\x0b\x11\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03N\x12\
    \x1b\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03N\x1e\x1f\n\x0b\n\x04\x04\
    \x03\x02\x03\x12\x03O\x02\x20\n\x0c\n\x05\x04\x03\x02\x03\x04\x12\x03O\
    \x02\n\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03O\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\x03\x01\x12\x03O\x12\x1b\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\
    \x03O\x1e\x1f\n\x0b\n\x04\x04\x03\x02\x04\x12\x03P\x02$\n\x0c\n\x05\x04\
    \x03\x02\x04\x04\x12\x03P\x02\n\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x03P\
    \x0b\x11\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03P\x12\x1f\n\x0c\n\x05\
    \x04\x03\x02\x04\x03\x12\x03P\"#\n\x0b\n\x04\x04\x03\x02\x05\x12\x03Q\
    \x02/\n\x0c\n\x05\x04\x03\x02\x05\x04\x12\x03Q\x02\n\n\x0c\n\x05\x04\x03\
    \x02\x05\x05\x12\x03Q\x0b\x11\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x03Q\
    \x12*\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03Q-.\n\x0b\n\x04\x04\x03\x02\
    \x06\x12\x03R\x02\x20\n\x0c\n\x05\x04\x03\x02\x06\x04\x12\x03R\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x06\x05\x12\x03R\x0b\x11\n\x0c\n\x05\x04\x03\x02\
    \x06\x01\x12\x03R\x12\x1b\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\x03R\x1e\
    \x1f\n\"\n\x02\x04\x04\x12\x04X\0Z\x01\x1a\x16*\n\x20DatanodeInfo\x20arr\
    ay\n\n\n\n\x03\x04\x04\x01\x12\x03X\x08\x1a\n\x0b\n\x04\x04\x04\x02\0\
    \x12\x03Y\x02+\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03Y\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x06\x12\x03Y\x0b\x1c\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03Y\x1d&\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03Y)*\n(\n\x02\x04\x05\x12\
    \x04_\0x\x01\x1a\x1c*\n\x20The\x20status\x20of\x20a\x20Datanode\n\n\n\n\
    \x03\x04\x05\x01\x12\x03_\x08\x19\n\x0b\n\x04\x04\x05\x02\0\x12\x03`\x02\
    \"\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x05\x02\
    \0\x06\x12\x03`\x0b\x1a\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03`\x1b\x1d\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03`\x20!\n\x0b\n\x04\x04\x05\x02\x01\
    \x12\x03a\x02-\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03a\x02\n\n\x0c\n\
    \x05\x04\x05\x02\x01\x05\x12\x03a\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\
    \x01\x12\x03a\x12\x1a\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03a\x1d\x1e\n\
    \x0c\n\x05\x04\x05\x02\x01\x08\x12\x03a\x1f,\n\x0c\n\x05\x04\x05\x02\x01\
    \x07\x12\x03a*+\n\x0b\n\x04\x04\x05\x02\x02\x12\x03b\x02,\n\x0c\n\x05\
    \x04\x05\x02\x02\x04\x12\x03b\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\
    \x03b\x0b\x11\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03b\x12\x19\n\x0c\n\
    \x05\x04\x05\x02\x02\x03\x12\x03b\x1c\x1d\n\x0c\n\x05\x04\x05\x02\x02\
    \x08\x12\x03b\x1e+\n\x0c\n\x05\x04\x05\x02\x02\x07\x12\x03b)*\n\x0b\n\
    \x04\x04\x05\x02\x03\x12\x03c\x02.\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\
    \x03c\x02\n\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x03c\x0b\x11\n\x0c\n\x05\
    \x04\x05\x02\x03\x01\x12\x03c\x12\x1b\n\x0c\n\x05\x04\x05\x02\x03\x03\
    \x12\x03c\x1e\x1f\n\x0c\n\x05\x04\x05\x02\x03\x08\x12\x03c\x20-\n\x0c\n\
    \x05\x04\x05\x02\x03\x07\x12\x03c+,\n\x0b\n\x04\x04\x05\x02\x04\x12\x03d\
    \x022\n\x0c\n\x05\x04\x05\x02\x04\x04\x12\x03d\x02\n\n\x0c\n\x05\x04\x05\
    \x02\x04\x05\x12\x03d\x0b\x11\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\x03d\
    \x12\x1f\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x03d\"#\n\x0c\n\x05\x04\x05\
    \x02\x04\x08\x12\x03d$1\n\x0c\n\x05\x04\x05\x02\x04\x07\x12\x03d/0\n\x0b\
    \n\x04\x04\x05\x02\x05\x12\x03e\x02/\n\x0c\n\x05\x04\x05\x02\x05\x04\x12\
    \x03e\x02\n\n\x0c\n\x05\x04\x05\x02\x05\x05\x12\x03e\x0b\x11\n\x0c\n\x05\
    \x04\x05\x02\x05\x01\x12\x03e\x12\x1c\n\x0c\n\x05\x04\x05\x02\x05\x03\
    \x12\x03e\x1f\x20\n\x0c\n\x05\x04\x05\x02\x05\x08\x12\x03e!.\n\x0c\n\x05\
    \x04\x05\x02\x05\x07\x12\x03e,-\n\x0b\n\x04\x04\x05\x02\x06\x12\x03f\x02\
    1\n\x0c\n\x05\x04\x05\x02\x06\x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x06\x05\x12\x03f\x0b\x11\n\x0c\n\x05\x04\x05\x02\x06\x01\x12\x03f\x12\
    \x1e\n\x0c\n\x05\x04\x05\x02\x06\x03\x12\x03f!\"\n\x0c\n\x05\x04\x05\x02\
    \x06\x08\x12\x03f#0\n\x0c\n\x05\x04\x05\x02\x06\x07\x12\x03f./\n\x0b\n\
    \x04\x04\x05\x02\x07\x12\x03g\x02\x1f\n\x0c\n\x05\x04\x05\x02\x07\x04\
    \x12\x03g\x02\n\n\x0c\n\x05\x04\x05\x02\x07\x05\x12\x03g\x0b\x11\n\x0c\n\
    \x05\x04\x05\x02\x07\x01\x12\x03g\x12\x1a\n\x0c\n\x05\x04\x05\x02\x07\
    \x03\x12\x03g\x1d\x1e\n\x0b\n\x04\x04\x05\x02\x08\x12\x03h\x02!\n\x0c\n\
    \x05\x04\x05\x02\x08\x04\x12\x03h\x02\n\n\x0c\n\x05\x04\x05\x02\x08\x05\
    \x12\x03h\x0b\x11\n\x0c\n\x05\x04\x05\x02\x08\x01\x12\x03h\x12\x1c\n\x0c\
    \n\x05\x04\x05\x02\x08\x03\x12\x03h\x1f\x20\n\x0c\n\x04\x04\x05\x04\0\
    \x12\x04i\x02o\x03\n\x0c\n\x05\x04\x05\x04\0\x01\x12\x03i\x07\x11\n\r\n\
    \x06\x04\x05\x04\0\x02\0\x12\x03j\x04\x0f\n\x0e\n\x07\x04\x05\x04\0\x02\
    \0\x01\x12\x03j\x04\n\n\x0e\n\x07\x04\x05\x04\0\x02\0\x02\x12\x03j\r\x0e\
    \n\r\n\x06\x04\x05\x04\0\x02\x01\x12\x03k\x04\x20\n\x0e\n\x07\x04\x05\
    \x04\0\x02\x01\x01\x12\x03k\x04\x1b\n\x0e\n\x07\x04\x05\x04\0\x02\x01\
    \x02\x12\x03k\x1e\x1f\n\r\n\x06\x04\x05\x04\0\x02\x02\x12\x03l\x04\x17\n\
    \x0e\n\x07\x04\x05\x04\0\x02\x02\x01\x12\x03l\x04\x12\n\x0e\n\x07\x04\
    \x05\x04\0\x02\x02\x02\x12\x03l\x15\x16\n\r\n\x06\x04\x05\x04\0\x02\x03\
    \x12\x03m\x04\x1d\n\x0e\n\x07\x04\x05\x04\0\x02\x03\x01\x12\x03m\x04\x18\
    \n\x0e\n\x07\x04\x05\x04\0\x02\x03\x02\x12\x03m\x1b\x1c\n\r\n\x06\x04\
    \x05\x04\0\x02\x04\x12\x03n\x04\x17\n\x0e\n\x07\x04\x05\x04\0\x02\x04\
    \x01\x12\x03n\x04\x12\n\x0e\n\x07\x04\x05\x04\0\x02\x04\x02\x12\x03n\x15\
    \x16\n\x0b\n\x04\x04\x05\x02\t\x12\x03q\x029\n\x0c\n\x05\x04\x05\x02\t\
    \x04\x12\x03q\x02\n\n\x0c\n\x05\x04\x05\x02\t\x06\x12\x03q\x0b\x15\n\x0c\
    \n\x05\x04\x05\x02\t\x01\x12\x03q\x16\x20\n\x0c\n\x05\x04\x05\x02\t\x03\
    \x12\x03q#%\n\x0c\n\x05\x04\x05\x02\t\x08\x12\x03q&8\n\x0c\n\x05\x04\x05\
    \x02\t\x07\x12\x03q17\n\x0b\n\x04\x04\x05\x02\n\x12\x03r\x023\n\x0c\n\
    \x05\x04\x05\x02\n\x04\x12\x03r\x02\n\n\x0c\n\x05\x04\x05\x02\n\x05\x12\
    \x03r\x0b\x11\n\x0c\n\x05\x04\x05\x02\n\x01\x12\x03r\x12\x1f\n\x0c\n\x05\
    \x04\x05\x02\n\x03\x12\x03r\"$\n\x0c\n\x05\x04\x05\x02\n\x08\x12\x03r%2\
    \n\x0c\n\x05\x04\x05\x02\n\x07\x12\x03r01\n\x0b\n\x04\x04\x05\x02\x0b\
    \x12\x03s\x02/\n\x0c\n\x05\x04\x05\x02\x0b\x04\x12\x03s\x02\n\n\x0c\n\
    \x05\x04\x05\x02\x0b\x05\x12\x03s\x0b\x11\n\x0c\n\x05\x04\x05\x02\x0b\
    \x01\x12\x03s\x12\x1b\n\x0c\n\x05\x04\x05\x02\x0b\x03\x12\x03s\x1e\x20\n\
    \x0c\n\x05\x04\x05\x02\x0b\x08\x12\x03s!.\n\x0c\n\x05\x04\x05\x02\x0b\
    \x07\x12\x03s,-\n\x0b\n\x04\x04\x05\x02\x0c\x12\x03t\x029\n\x0c\n\x05\
    \x04\x05\x02\x0c\x04\x12\x03t\x02\n\n\x0c\n\x05\x04\x05\x02\x0c\x05\x12\
    \x03t\x0b\x11\n\x0c\n\x05\x04\x05\x02\x0c\x01\x12\x03t\x12%\n\x0c\n\x05\
    \x04\x05\x02\x0c\x03\x12\x03t(*\n\x0c\n\x05\x04\x05\x02\x0c\x08\x12\x03t\
    +8\n\x0c\n\x05\x04\x05\x02\x0c\x07\x12\x03t67\n\x0b\n\x04\x04\x05\x02\r\
    \x12\x03u\x02%\n\x0c\n\x05\x04\x05\x02\r\x04\x12\x03u\x02\n\n\x0c\n\x05\
    \x04\x05\x02\r\x05\x12\x03u\x0b\x11\n\x0c\n\x05\x04\x05\x02\r\x01\x12\
    \x03u\x12\x1f\n\x0c\n\x05\x04\x05\x02\r\x03\x12\x03u\"$\n\x0b\n\x04\x04\
    \x05\x02\x0e\x12\x03v\x029\n\x0c\n\x05\x04\x05\x02\x0e\x04\x12\x03v\x02\
    \n\n\x0c\n\x05\x04\x05\x02\x0e\x05\x12\x03v\x0b\x11\n\x0c\n\x05\x04\x05\
    \x02\x0e\x01\x12\x03v\x12%\n\x0c\n\x05\x04\x05\x02\x0e\x03\x12\x03v(*\n\
    \x0c\n\x05\x04\x05\x02\x0e\x08\x12\x03v+8\n\x0c\n\x05\x04\x05\x02\x0e\
    \x07\x12\x03v67\n\x0b\n\x04\x04\x05\x02\x0f\x12\x03w\x02>\n\x0c\n\x05\
    \x04\x05\x02\x0f\x04\x12\x03w\x02\n\n\x0c\n\x05\x04\x05\x02\x0f\x05\x12\
    \x03w\x0b\x11\n\x0c\n\x05\x04\x05\x02\x0f\x01\x12\x03w\x12*\n\x0c\n\x05\
    \x04\x05\x02\x0f\x03\x12\x03w-/\n\x0c\n\x05\x04\x05\x02\x0f\x08\x12\x03w\
    0=\n\x0c\n\x05\x04\x05\x02\x0f\x07\x12\x03w;<\n?\n\x02\x04\x06\x12\x05}\
    \0\x86\x01\x01\x1a2*\n\x20Represents\x20a\x20storage\x20available\x20on\
    \x20the\x20datanode\n\n\n\n\x03\x04\x06\x01\x12\x03}\x08\x1c\n\r\n\x04\
    \x04\x06\x04\0\x12\x05~\x02\x81\x01\x03\n\x0c\n\x05\x04\x06\x04\0\x01\
    \x12\x03~\x07\x13\n\r\n\x06\x04\x06\x04\0\x02\0\x12\x03\x7f\x04\x0f\n\
    \x0e\n\x07\x04\x06\x04\0\x02\0\x01\x12\x03\x7f\x04\n\n\x0e\n\x07\x04\x06\
    \x04\0\x02\0\x02\x12\x03\x7f\r\x0e\n\x0e\n\x06\x04\x06\x04\0\x02\x01\x12\
    \x04\x80\x01\x04\x19\n\x0f\n\x07\x04\x06\x04\0\x02\x01\x01\x12\x04\x80\
    \x01\x04\x14\n\x0f\n\x07\x04\x06\x04\0\x02\x01\x02\x12\x04\x80\x01\x17\
    \x18\n\x0c\n\x04\x04\x06\x02\0\x12\x04\x83\x01\x02\"\n\r\n\x05\x04\x06\
    \x02\0\x04\x12\x04\x83\x01\x02\n\n\r\n\x05\x04\x06\x02\0\x05\x12\x04\x83\
    \x01\x0b\x11\n\r\n\x05\x04\x06\x02\0\x01\x12\x04\x83\x01\x12\x1d\n\r\n\
    \x05\x04\x06\x02\0\x03\x12\x04\x83\x01\x20!\n\x0c\n\x04\x04\x06\x02\x01\
    \x12\x04\x84\x01\x025\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04\x84\x01\x02\
    \n\n\r\n\x05\x04\x06\x02\x01\x06\x12\x04\x84\x01\x0b\x17\n\r\n\x05\x04\
    \x06\x02\x01\x01\x12\x04\x84\x01\x18\x1d\n\r\n\x05\x04\x06\x02\x01\x03\
    \x12\x04\x84\x01\x20!\n\r\n\x05\x04\x06\x02\x01\x08\x12\x04\x84\x01\"4\n\
    \r\n\x05\x04\x06\x02\x01\x07\x12\x04\x84\x01-3\n\x0c\n\x04\x04\x06\x02\
    \x02\x12\x04\x85\x01\x02=\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04\x85\x01\
    \x02\n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\x85\x01\x0b\x1b\n\r\n\x05\
    \x04\x06\x02\x02\x01\x12\x04\x85\x01\x1c'\n\r\n\x05\x04\x06\x02\x02\x03\
    \x12\x04\x85\x01*+\n\r\n\x05\x04\x06\x02\x02\x08\x12\x04\x85\x01,<\n\r\n\
    \x05\x04\x06\x02\x02\x07\x12\x04\x85\x017;\n\x0c\n\x02\x04\x07\x12\x06\
    \x88\x01\0\x91\x01\x01\n\x0b\n\x03\x04\x07\x01\x12\x04\x88\x01\x08\x1a\n\
    \x0c\n\x04\x04\x07\x02\0\x12\x04\x89\x01\x028\n\r\n\x05\x04\x07\x02\0\
    \x04\x12\x04\x89\x01\x02\n\n\r\n\x05\x04\x07\x02\0\x05\x12\x04\x89\x01\
    \x0b\x11\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\x89\x01\x12\x1d\n\r\n\x05\
    \x04\x07\x02\0\x03\x12\x04\x89\x01\x20!\n\r\n\x05\x04\x07\x02\0\x08\x12\
    \x04\x89\x01\"7\n\x10\n\x08\x04\x07\x02\0\x08\xe7\x07\0\x12\x04\x89\x01$\
    5\n\x11\n\t\x04\x07\x02\0\x08\xe7\x07\0\x02\x12\x04\x89\x01$.\n\x12\n\n\
    \x04\x07\x02\0\x08\xe7\x07\0\x02\0\x12\x04\x89\x01$.\n\x13\n\x0b\x04\x07\
    \x02\0\x08\xe7\x07\0\x02\0\x01\x12\x04\x89\x01$.\n\x11\n\t\x04\x07\x02\0\
    \x08\xe7\x07\0\x03\x12\x04\x89\x0115\n\x0c\n\x04\x04\x07\x02\x01\x12\x04\
    \x8a\x01\x02/\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\x8a\x01\x02\n\n\r\n\
    \x05\x04\x07\x02\x01\x05\x12\x04\x8a\x01\x0b\x0f\n\r\n\x05\x04\x07\x02\
    \x01\x01\x12\x04\x8a\x01\x10\x16\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\
    \x8a\x01\x19\x1a\n\r\n\x05\x04\x07\x02\x01\x08\x12\x04\x8a\x01\x1b.\n\r\
    \n\x05\x04\x07\x02\x01\x07\x12\x04\x8a\x01',\n\x0c\n\x04\x04\x07\x02\x02\
    \x12\x04\x8b\x01\x02/\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04\x8b\x01\x02\
    \n\n\r\n\x05\x04\x07\x02\x02\x05\x12\x04\x8b\x01\x0b\x11\n\r\n\x05\x04\
    \x07\x02\x02\x01\x12\x04\x8b\x01\x12\x1a\n\r\n\x05\x04\x07\x02\x02\x03\
    \x12\x04\x8b\x01\x1d\x1e\n\r\n\x05\x04\x07\x02\x02\x08\x12\x04\x8b\x01\
    \x1f.\n\r\n\x05\x04\x07\x02\x02\x07\x12\x04\x8b\x01+,\n\x0c\n\x04\x04\
    \x07\x02\x03\x12\x04\x8c\x01\x02.\n\r\n\x05\x04\x07\x02\x03\x04\x12\x04\
    \x8c\x01\x02\n\n\r\n\x05\x04\x07\x02\x03\x05\x12\x04\x8c\x01\x0b\x11\n\r\
    \n\x05\x04\x07\x02\x03\x01\x12\x04\x8c\x01\x12\x19\n\r\n\x05\x04\x07\x02\
    \x03\x03\x12\x04\x8c\x01\x1c\x1d\n\r\n\x05\x04\x07\x02\x03\x08\x12\x04\
    \x8c\x01\x1e-\n\r\n\x05\x04\x07\x02\x03\x07\x12\x04\x8c\x01*+\n\x0c\n\
    \x04\x04\x07\x02\x04\x12\x04\x8d\x01\x020\n\r\n\x05\x04\x07\x02\x04\x04\
    \x12\x04\x8d\x01\x02\n\n\r\n\x05\x04\x07\x02\x04\x05\x12\x04\x8d\x01\x0b\
    \x11\n\r\n\x05\x04\x07\x02\x04\x01\x12\x04\x8d\x01\x12\x1b\n\r\n\x05\x04\
    \x07\x02\x04\x03\x12\x04\x8d\x01\x1e\x1f\n\r\n\x05\x04\x07\x02\x04\x08\
    \x12\x04\x8d\x01\x20/\n\r\n\x05\x04\x07\x02\x04\x07\x12\x04\x8d\x01,-\n\
    \x0c\n\x04\x04\x07\x02\x05\x12\x04\x8e\x01\x024\n\r\n\x05\x04\x07\x02\
    \x05\x04\x12\x04\x8e\x01\x02\n\n\r\n\x05\x04\x07\x02\x05\x05\x12\x04\x8e\
    \x01\x0b\x11\n\r\n\x05\x04\x07\x02\x05\x01\x12\x04\x8e\x01\x12\x1f\n\r\n\
    \x05\x04\x07\x02\x05\x03\x12\x04\x8e\x01\"#\n\r\n\x05\x04\x07\x02\x05\
    \x08\x12\x04\x8e\x01$3\n\r\n\x05\x04\x07\x02\x05\x07\x12\x04\x8e\x0101\n\
    &\n\x04\x04\x07\x02\x06\x12\x04\x8f\x01\x02,\"\x18\x20supersedes\x20Stor\
    ageUuid\n\n\r\n\x05\x04\x07\x02\x06\x04\x12\x04\x8f\x01\x02\n\n\r\n\x05\
    \x04\x07\x02\x06\x06\x12\x04\x8f\x01\x0b\x1f\n\r\n\x05\x04\x07\x02\x06\
    \x01\x12\x04\x8f\x01\x20'\n\r\n\x05\x04\x07\x02\x06\x03\x12\x04\x8f\x01*\
    +\n\x0c\n\x04\x04\x07\x02\x07\x12\x04\x90\x01\x02!\n\r\n\x05\x04\x07\x02\
    \x07\x04\x12\x04\x90\x01\x02\n\n\r\n\x05\x04\x07\x02\x07\x05\x12\x04\x90\
    \x01\x0b\x11\n\r\n\x05\x04\x07\x02\x07\x01\x12\x04\x90\x01\x12\x1c\n\r\n\
    \x05\x04\x07\x02\x07\x03\x12\x04\x90\x01\x1f\x20\n0\n\x02\x04\x08\x12\
    \x06\x96\x01\0\xa3\x01\x01\x1a\"*\n\x20Summary\x20of\x20a\x20file\x20or\
    \x20directory\n\n\x0b\n\x03\x04\x08\x01\x12\x04\x96\x01\x08\x1b\n\x0c\n\
    \x04\x04\x08\x02\0\x12\x04\x97\x01\x02\x1d\n\r\n\x05\x04\x08\x02\0\x04\
    \x12\x04\x97\x01\x02\n\n\r\n\x05\x04\x08\x02\0\x05\x12\x04\x97\x01\x0b\
    \x11\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x97\x01\x12\x18\n\r\n\x05\x04\
    \x08\x02\0\x03\x12\x04\x97\x01\x1b\x1c\n\x0c\n\x04\x04\x08\x02\x01\x12\
    \x04\x98\x01\x02\x20\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04\x98\x01\x02\n\
    \n\r\n\x05\x04\x08\x02\x01\x05\x12\x04\x98\x01\x0b\x11\n\r\n\x05\x04\x08\
    \x02\x01\x01\x12\x04\x98\x01\x12\x1b\n\r\n\x05\x04\x08\x02\x01\x03\x12\
    \x04\x98\x01\x1e\x1f\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\x99\x01\x02%\n\
    \r\n\x05\x04\x08\x02\x02\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\x04\x08\x02\
    \x02\x05\x12\x04\x99\x01\x0b\x11\n\r\n\x05\x04\x08\x02\x02\x01\x12\x04\
    \x99\x01\x12\x20\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\x99\x01#$\n\x0c\n\
    \x04\x04\x08\x02\x03\x12\x04\x9a\x01\x02\x1c\n\r\n\x05\x04\x08\x02\x03\
    \x04\x12\x04\x9a\x01\x02\n\n\r\n\x05\x04\x08\x02\x03\x05\x12\x04\x9a\x01\
    \x0b\x11\n\r\n\x05\x04\x08\x02\x03\x01\x12\x04\x9a\x01\x12\x17\n\r\n\x05\
    \x04\x08\x02\x03\x03\x12\x04\x9a\x01\x1a\x1b\n\x0c\n\x04\x04\x08\x02\x04\
    \x12\x04\x9b\x01\x02$\n\r\n\x05\x04\x08\x02\x04\x04\x12\x04\x9b\x01\x02\
    \n\n\r\n\x05\x04\x08\x02\x04\x05\x12\x04\x9b\x01\x0b\x11\n\r\n\x05\x04\
    \x08\x02\x04\x01\x12\x04\x9b\x01\x12\x1f\n\r\n\x05\x04\x08\x02\x04\x03\
    \x12\x04\x9b\x01\"#\n\x0c\n\x04\x04\x08\x02\x05\x12\x04\x9c\x01\x02!\n\r\
    \n\x05\x04\x08\x02\x05\x04\x12\x04\x9c\x01\x02\n\n\r\n\x05\x04\x08\x02\
    \x05\x05\x12\x04\x9c\x01\x0b\x11\n\r\n\x05\x04\x08\x02\x05\x01\x12\x04\
    \x9c\x01\x12\x1c\n\r\n\x05\x04\x08\x02\x05\x03\x12\x04\x9c\x01\x1f\x20\n\
    \x0c\n\x04\x04\x08\x02\x06\x12\x04\x9d\x01\x029\n\r\n\x05\x04\x08\x02\
    \x06\x04\x12\x04\x9d\x01\x02\n\n\r\n\x05\x04\x08\x02\x06\x06\x12\x04\x9d\
    \x01\x0b%\n\r\n\x05\x04\x08\x02\x06\x01\x12\x04\x9d\x01&4\n\r\n\x05\x04\
    \x08\x02\x06\x03\x12\x04\x9d\x0178\n\x0c\n\x04\x04\x08\x02\x07\x12\x04\
    \x9e\x01\x02%\n\r\n\x05\x04\x08\x02\x07\x04\x12\x04\x9e\x01\x02\n\n\r\n\
    \x05\x04\x08\x02\x07\x05\x12\x04\x9e\x01\x0b\x11\n\r\n\x05\x04\x08\x02\
    \x07\x01\x12\x04\x9e\x01\x12\x20\n\r\n\x05\x04\x08\x02\x07\x03\x12\x04\
    \x9e\x01#$\n\x0c\n\x04\x04\x08\x02\x08\x12\x04\x9f\x01\x02(\n\r\n\x05\
    \x04\x08\x02\x08\x04\x12\x04\x9f\x01\x02\n\n\r\n\x05\x04\x08\x02\x08\x05\
    \x12\x04\x9f\x01\x0b\x11\n\r\n\x05\x04\x08\x02\x08\x01\x12\x04\x9f\x01\
    \x12#\n\r\n\x05\x04\x08\x02\x08\x03\x12\x04\x9f\x01&'\n\x0c\n\x04\x04\
    \x08\x02\t\x12\x04\xa0\x01\x02.\n\r\n\x05\x04\x08\x02\t\x04\x12\x04\xa0\
    \x01\x02\n\n\r\n\x05\x04\x08\x02\t\x05\x12\x04\xa0\x01\x0b\x11\n\r\n\x05\
    \x04\x08\x02\t\x01\x12\x04\xa0\x01\x12(\n\r\n\x05\x04\x08\x02\t\x03\x12\
    \x04\xa0\x01+-\n\x0c\n\x04\x04\x08\x02\n\x12\x04\xa1\x01\x02-\n\r\n\x05\
    \x04\x08\x02\n\x04\x12\x04\xa1\x01\x02\n\n\r\n\x05\x04\x08\x02\n\x05\x12\
    \x04\xa1\x01\x0b\x11\n\r\n\x05\x04\x08\x02\n\x01\x12\x04\xa1\x01\x12'\n\
    \r\n\x05\x04\x08\x02\n\x03\x12\x04\xa1\x01*,\n\x0c\n\x04\x04\x08\x02\x0b\
    \x12\x04\xa2\x01\x02+\n\r\n\x05\x04\x08\x02\x0b\x04\x12\x04\xa2\x01\x02\
    \n\n\r\n\x05\x04\x08\x02\x0b\x05\x12\x04\xa2\x01\x0b\x11\n\r\n\x05\x04\
    \x08\x02\x0b\x01\x12\x04\xa2\x01\x12%\n\r\n\x05\x04\x08\x02\x0b\x03\x12\
    \x04\xa2\x01(*\n7\n\x02\x04\t\x12\x06\xa8\x01\0\xae\x01\x01\x1a)*\n\x20S\
    ummary\x20of\x20quota\x20usage\x20of\x20a\x20directory\n\n\x0b\n\x03\x04\
    \t\x01\x12\x04\xa8\x01\x08\x17\n\x0c\n\x04\x04\t\x02\0\x12\x04\xa9\x01\
    \x02,\n\r\n\x05\x04\t\x02\0\x04\x12\x04\xa9\x01\x02\n\n\r\n\x05\x04\t\
    \x02\0\x05\x12\x04\xa9\x01\x0b\x11\n\r\n\x05\x04\t\x02\0\x01\x12\x04\xa9\
    \x01\x12'\n\r\n\x05\x04\t\x02\0\x03\x12\x04\xa9\x01*+\n\x0c\n\x04\x04\t\
    \x02\x01\x12\x04\xaa\x01\x02\x1c\n\r\n\x05\x04\t\x02\x01\x04\x12\x04\xaa\
    \x01\x02\n\n\r\n\x05\x04\t\x02\x01\x05\x12\x04\xaa\x01\x0b\x11\n\r\n\x05\
    \x04\t\x02\x01\x01\x12\x04\xaa\x01\x12\x17\n\r\n\x05\x04\t\x02\x01\x03\
    \x12\x04\xaa\x01\x1a\x1b\n\x0c\n\x04\x04\t\x02\x02\x12\x04\xab\x01\x02$\
    \n\r\n\x05\x04\t\x02\x02\x04\x12\x04\xab\x01\x02\n\n\r\n\x05\x04\t\x02\
    \x02\x05\x12\x04\xab\x01\x0b\x11\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\xab\
    \x01\x12\x1f\n\r\n\x05\x04\t\x02\x02\x03\x12\x04\xab\x01\"#\n\x0c\n\x04\
    \x04\t\x02\x03\x12\x04\xac\x01\x02!\n\r\n\x05\x04\t\x02\x03\x04\x12\x04\
    \xac\x01\x02\n\n\r\n\x05\x04\t\x02\x03\x05\x12\x04\xac\x01\x0b\x11\n\r\n\
    \x05\x04\t\x02\x03\x01\x12\x04\xac\x01\x12\x1c\n\r\n\x05\x04\t\x02\x03\
    \x03\x12\x04\xac\x01\x1f\x20\n\x0c\n\x04\x04\t\x02\x04\x12\x04\xad\x01\
    \x029\n\r\n\x05\x04\t\x02\x04\x04\x12\x04\xad\x01\x02\n\n\r\n\x05\x04\t\
    \x02\x04\x06\x12\x04\xad\x01\x0b%\n\r\n\x05\x04\t\x02\x04\x01\x12\x04\
    \xad\x01&4\n\r\n\x05\x04\t\x02\x04\x03\x12\x04\xad\x0178\nQ\n\x02\x04\n\
    \x12\x06\xb3\x01\0\xb5\x01\x01\x1aC*\n\x20Storage\x20type\x20quota\x20an\
    d\x20usage\x20information\x20of\x20a\x20file\x20or\x20directory\n\n\x0b\
    \n\x03\x04\n\x01\x12\x04\xb3\x01\x08\"\n\x0c\n\x04\x04\n\x02\0\x12\x04\
    \xb4\x01\x027\n\r\n\x05\x04\n\x02\0\x04\x12\x04\xb4\x01\x02\n\n\r\n\x05\
    \x04\n\x02\0\x06\x12\x04\xb4\x01\x0b$\n\r\n\x05\x04\n\x02\0\x01\x12\x04\
    \xb4\x01%2\n\r\n\x05\x04\n\x02\0\x03\x12\x04\xb4\x0156\n\x0c\n\x02\x04\
    \x0b\x12\x06\xb7\x01\0\xbb\x01\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\xb7\
    \x01\x08!\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\xb8\x01\x02%\n\r\n\x05\x04\
    \x0b\x02\0\x04\x12\x04\xb8\x01\x02\n\n\r\n\x05\x04\x0b\x02\0\x06\x12\x04\
    \xb8\x01\x0b\x1b\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\xb8\x01\x1c\x20\n\r\
    \n\x05\x04\x0b\x02\0\x03\x12\x04\xb8\x01#$\n\x0c\n\x04\x04\x0b\x02\x01\
    \x12\x04\xb9\x01\x02\x1c\n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04\xb9\x01\
    \x02\n\n\r\n\x05\x04\x0b\x02\x01\x05\x12\x04\xb9\x01\x0b\x11\n\r\n\x05\
    \x04\x0b\x02\x01\x01\x12\x04\xb9\x01\x12\x17\n\r\n\x05\x04\x0b\x02\x01\
    \x03\x12\x04\xb9\x01\x1a\x1b\n\x0c\n\x04\x04\x0b\x02\x02\x12\x04\xba\x01\
    \x02\x1f\n\r\n\x05\x04\x0b\x02\x02\x04\x12\x04\xba\x01\x02\n\n\r\n\x05\
    \x04\x0b\x02\x02\x05\x12\x04\xba\x01\x0b\x11\n\r\n\x05\x04\x0b\x02\x02\
    \x01\x12\x04\xba\x01\x12\x1a\n\r\n\x05\x04\x0b\x02\x02\x03\x12\x04\xba\
    \x01\x1d\x1e\n\x95\x01\n\x02\x04\x0c\x12\x06\xc2\x01\0\xc5\x01\x01\x1a\
    \x86\x01*\n\x20Contains\x20a\x20list\x20of\x20paths\x20corresponding\x20\
    to\x20corrupt\x20files\x20and\x20a\x20cookie\n\x20used\x20for\x20iterati\
    ve\x20calls\x20to\x20NameNode.listCorruptFileBlocks.\n\n\n\x0b\n\x03\x04\
    \x0c\x01\x12\x04\xc2\x01\x08\x1e\n\x0c\n\x04\x04\x0c\x02\0\x12\x04\xc3\
    \x01\x01\x1b\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\xc3\x01\x01\t\n\r\n\x05\
    \x04\x0c\x02\0\x05\x12\x04\xc3\x01\n\x10\n\r\n\x05\x04\x0c\x02\0\x01\x12\
    \x04\xc3\x01\x11\x16\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\xc3\x01\x19\x1a\
    \n\x0c\n\x04\x04\x0c\x02\x01\x12\x04\xc4\x01\x01\x1e\n\r\n\x05\x04\x0c\
    \x02\x01\x04\x12\x04\xc4\x01\x01\t\n\r\n\x05\x04\x0c\x02\x01\x05\x12\x04\
    \xc4\x01\n\x10\n\r\n\x05\x04\x0c\x02\x01\x01\x12\x04\xc4\x01\x13\x19\n\r\
    \n\x05\x04\x0c\x02\x01\x03\x12\x04\xc4\x01\x1c\x1d\n4\n\x02\x05\0\x12\
    \x06\xca\x01\0\xcf\x01\x01\x1a&*\n\x20Types\x20of\x20recognized\x20stora\
    ge\x20media.\n\n\x0b\n\x03\x05\0\x01\x12\x04\xca\x01\x05\x15\n\x0c\n\x04\
    \x05\0\x02\0\x12\x04\xcb\x01\x02\x0b\n\r\n\x05\x05\0\x02\0\x01\x12\x04\
    \xcb\x01\x02\x06\n\r\n\x05\x05\0\x02\0\x02\x12\x04\xcb\x01\t\n\n\x0c\n\
    \x04\x05\0\x02\x01\x12\x04\xcc\x01\x02\n\n\r\n\x05\x05\0\x02\x01\x01\x12\
    \x04\xcc\x01\x02\x05\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xcc\x01\x08\t\n\
    \x0c\n\x04\x05\0\x02\x02\x12\x04\xcd\x01\x02\x0e\n\r\n\x05\x05\0\x02\x02\
    \x01\x12\x04\xcd\x01\x02\t\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\xcd\x01\
    \x0c\r\n\x0c\n\x04\x05\0\x02\x03\x12\x04\xce\x01\x02\x0f\n\r\n\x05\x05\0\
    \x02\x03\x01\x12\x04\xce\x01\x02\n\n\r\n\x05\x05\0\x02\x03\x02\x12\x04\
    \xce\x01\r\x0e\n-\n\x02\x05\x01\x12\x06\xd4\x01\0\xd7\x01\x01\x1a\x1f*\n\
    \x20Types\x20of\x20recognized\x20blocks.\n\n\x0b\n\x03\x05\x01\x01\x12\
    \x04\xd4\x01\x05\x13\n\x0c\n\x04\x05\x01\x02\0\x12\x04\xd5\x01\x02\x11\n\
    \r\n\x05\x05\x01\x02\0\x01\x12\x04\xd5\x01\x02\x0c\n\r\n\x05\x05\x01\x02\
    \0\x02\x12\x04\xd5\x01\x0f\x10\n\x0c\n\x04\x05\x01\x02\x01\x12\x04\xd6\
    \x01\x02\x0e\n\r\n\x05\x05\x01\x02\x01\x01\x12\x04\xd6\x01\x02\t\n\r\n\
    \x05\x05\x01\x02\x01\x02\x12\x04\xd6\x01\x0c\r\n+\n\x02\x04\r\x12\x06\
    \xdc\x01\0\xde\x01\x01\x1a\x1d*\n\x20A\x20list\x20of\x20storage\x20types\
    .\x20\n\n\x0b\n\x03\x04\r\x01\x12\x04\xdc\x01\x08\x19\n\x0c\n\x04\x04\r\
    \x02\0\x12\x04\xdd\x01\x02-\n\r\n\x05\x04\r\x02\0\x04\x12\x04\xdd\x01\
    \x02\n\n\r\n\x05\x04\r\x02\0\x06\x12\x04\xdd\x01\x0b\x1b\n\r\n\x05\x04\r\
    \x02\0\x01\x12\x04\xdd\x01\x1c(\n\r\n\x05\x04\r\x02\0\x03\x12\x04\xdd\
    \x01+,\n/\n\x02\x04\x0e\x12\x06\xe3\x01\0\xec\x01\x01\x1a!*\n\x20Block\
    \x20replica\x20storage\x20policy.\n\n\x0b\n\x03\x04\x0e\x01\x12\x04\xe3\
    \x01\x08\x1f\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\xe4\x01\x02\x1f\n\r\n\x05\
    \x04\x0e\x02\0\x04\x12\x04\xe4\x01\x02\n\n\r\n\x05\x04\x0e\x02\0\x05\x12\
    \x04\xe4\x01\x0b\x11\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\xe4\x01\x12\x1a\
    \n\r\n\x05\x04\x0e\x02\0\x03\x12\x04\xe4\x01\x1d\x1e\n\x0c\n\x04\x04\x0e\
    \x02\x01\x12\x04\xe5\x01\x02\x1b\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04\
    \xe5\x01\x02\n\n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\xe5\x01\x0b\x11\n\r\
    \n\x05\x04\x0e\x02\x01\x01\x12\x04\xe5\x01\x12\x16\n\r\n\x05\x04\x0e\x02\
    \x01\x03\x12\x04\xe5\x01\x19\x1a\n^\n\x04\x04\x0e\x02\x02\x12\x04\xe8\
    \x01\x020\x1aP\x20a\x20list\x20of\x20storage\x20types\x20for\x20storing\
    \x20the\x20block\x20replicas\x20when\x20creating\x20a\n\x20block.\n\n\r\
    \n\x05\x04\x0e\x02\x02\x04\x12\x04\xe8\x01\x02\n\n\r\n\x05\x04\x0e\x02\
    \x02\x06\x12\x04\xe8\x01\x0b\x1c\n\r\n\x05\x04\x0e\x02\x02\x01\x12\x04\
    \xe8\x01\x1d+\n\r\n\x05\x04\x0e\x02\x02\x03\x12\x04\xe8\x01./\nF\n\x04\
    \x04\x0e\x02\x03\x12\x04\xea\x01\x028\x1a8\x20A\x20list\x20of\x20storage\
    \x20types\x20for\x20creation\x20fallback\x20storage.\n\n\r\n\x05\x04\x0e\
    \x02\x03\x04\x12\x04\xea\x01\x02\n\n\r\n\x05\x04\x0e\x02\x03\x06\x12\x04\
    \xea\x01\x0b\x1c\n\r\n\x05\x04\x0e\x02\x03\x01\x12\x04\xea\x01\x1d3\n\r\
    \n\x05\x04\x0e\x02\x03\x03\x12\x04\xea\x0167\n\x0c\n\x04\x04\x0e\x02\x04\
    \x12\x04\xeb\x01\x02;\n\r\n\x05\x04\x0e\x02\x04\x04\x12\x04\xeb\x01\x02\
    \n\n\r\n\x05\x04\x0e\x02\x04\x06\x12\x04\xeb\x01\x0b\x1c\n\r\n\x05\x04\
    \x0e\x02\x04\x01\x12\x04\xeb\x01\x1d6\n\r\n\x05\x04\x0e\x02\x04\x03\x12\
    \x04\xeb\x019:\nR\n\x02\x04\x0f\x12\x06\xf2\x01\0\x82\x02\x01\x1aD*\n\
    \x20A\x20LocatedBlock\x20gives\x20information\x20about\x20a\x20block\x20\
    and\x20its\x20location.\n\n\x0b\n\x03\x04\x0f\x01\x12\x04\xf2\x01\x08\
    \x19\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\xf3\x01\x02%\n\r\n\x05\x04\x0f\
    \x02\0\x04\x12\x04\xf3\x01\x02\n\n\r\n\x05\x04\x0f\x02\0\x06\x12\x04\xf3\
    \x01\x0b\x1d\n\r\n\x05\x04\x0f\x02\0\x01\x12\x04\xf3\x01\x1e\x1f\n\r\n\
    \x05\x04\x0f\x02\0\x03\x12\x04\xf3\x01#$\n9\n\x04\x04\x0f\x02\x01\x12\
    \x04\xf4\x01\x02\x1d\"+\x20offset\x20of\x20first\x20byte\x20of\x20block\
    \x20in\x20the\x20file\n\n\r\n\x05\x04\x0f\x02\x01\x04\x12\x04\xf4\x01\
    \x02\n\n\r\n\x05\x04\x0f\x02\x01\x05\x12\x04\xf4\x01\x0b\x11\n\r\n\x05\
    \x04\x0f\x02\x01\x01\x12\x04\xf4\x01\x12\x18\n\r\n\x05\x04\x0f\x02\x01\
    \x03\x12\x04\xf4\x01\x1b\x1c\n;\n\x04\x04\x0f\x02\x02\x12\x04\xf5\x01\
    \x02&\"-\x20Locations\x20ordered\x20by\x20proximity\x20to\x20client\x20i\
    p\n\n\r\n\x05\x04\x0f\x02\x02\x04\x12\x04\xf5\x01\x02\n\n\r\n\x05\x04\
    \x0f\x02\x02\x06\x12\x04\xf5\x01\x0b\x1c\n\r\n\x05\x04\x0f\x02\x02\x01\
    \x12\x04\xf5\x01\x1d!\n\r\n\x05\x04\x0f\x02\x02\x03\x12\x04\xf5\x01$%\nG\
    \n\x04\x04\x0f\x02\x03\x12\x04\xf6\x01\x02\x1c\"9\x20true\x20if\x20all\
    \x20replicas\x20of\x20a\x20block\x20are\x20corrupt,\x20else\x20false\n\n\
    \r\n\x05\x04\x0f\x02\x03\x04\x12\x04\xf6\x01\x02\n\n\r\n\x05\x04\x0f\x02\
    \x03\x05\x12\x04\xf6\x01\x0b\x0f\n\r\n\x05\x04\x0f\x02\x03\x01\x12\x04\
    \xf6\x01\x10\x17\n\r\n\x05\x04\x0f\x02\x03\x03\x12\x04\xf6\x01\x1a\x1b\n\
    v\n\x04\x04\x0f\x02\x04\x12\x04\xfa\x01\x0232h\x20If\x20block\x20has\x20\
    few\x20corrupt\x20replicas,\x20they\x20are\x20filtered\x20and\x20\n\x20t\
    heir\x20locations\x20are\x20not\x20part\x20of\x20this\x20object\n\n\r\n\
    \x05\x04\x0f\x02\x04\x04\x12\x04\xfa\x01\x02\n\n\r\n\x05\x04\x0f\x02\x04\
    \x06\x12\x04\xfa\x01\x0b#\n\r\n\x05\x04\x0f\x02\x04\x01\x12\x04\xfa\x01$\
    .\n\r\n\x05\x04\x0f\x02\x04\x03\x12\x04\xfa\x0112\n/\n\x04\x04\x0f\x02\
    \x05\x12\x04\xfb\x01\x02+\"!\x20if\x20a\x20location\x20in\x20locs\x20is\
    \x20cached\n\n\r\n\x05\x04\x0f\x02\x05\x04\x12\x04\xfb\x01\x02\n\n\r\n\
    \x05\x04\x0f\x02\x05\x05\x12\x04\xfb\x01\x0b\x0f\n\r\n\x05\x04\x0f\x02\
    \x05\x01\x12\x04\xfb\x01\x10\x18\n\r\n\x05\x04\x0f\x02\x05\x03\x12\x04\
    \xfb\x01\x1b\x1c\n\r\n\x05\x04\x0f\x02\x05\x08\x12\x04\xfb\x01\x1d*\n\
    \x10\n\x08\x04\x0f\x02\x05\x08\xe7\x07\0\x12\x04\xfb\x01\x1e)\n\x11\n\t\
    \x04\x0f\x02\x05\x08\xe7\x07\0\x02\x12\x04\xfb\x01\x1e$\n\x12\n\n\x04\
    \x0f\x02\x05\x08\xe7\x07\0\x02\0\x12\x04\xfb\x01\x1e$\n\x13\n\x0b\x04\
    \x0f\x02\x05\x08\xe7\x07\0\x02\0\x01\x12\x04\xfb\x01\x1e$\n\x11\n\t\x04\
    \x0f\x02\x05\x08\xe7\x07\0\x03\x12\x04\xfb\x01%)\n\x0c\n\x04\x04\x0f\x02\
    \x06\x12\x04\xfc\x01\x02-\n\r\n\x05\x04\x0f\x02\x06\x04\x12\x04\xfc\x01\
    \x02\n\n\r\n\x05\x04\x0f\x02\x06\x06\x12\x04\xfc\x01\x0b\x1b\n\r\n\x05\
    \x04\x0f\x02\x06\x01\x12\x04\xfc\x01\x1c(\n\r\n\x05\x04\x0f\x02\x06\x03\
    \x12\x04\xfc\x01+,\n\x0c\n\x04\x04\x0f\x02\x07\x12\x04\xfd\x01\x02!\n\r\
    \n\x05\x04\x0f\x02\x07\x04\x12\x04\xfd\x01\x02\n\n\r\n\x05\x04\x0f\x02\
    \x07\x05\x12\x04\xfd\x01\x0b\x11\n\r\n\x05\x04\x0f\x02\x07\x01\x12\x04\
    \xfd\x01\x12\x1c\n\r\n\x05\x04\x0f\x02\x07\x03\x12\x04\xfd\x01\x1f\x20\n\
    o\n\x04\x04\x0f\x02\x08\x12\x04\x80\x02\x02\"\x1a\x1e\x20striped\x20bloc\
    k\x20related\x20fields\n\"A\x20used\x20for\x20striped\x20block\x20to\x20\
    indicate\x20block\x20index\x20for\x20each\x20storage\n\n\r\n\x05\x04\x0f\
    \x02\x08\x04\x12\x04\x80\x02\x02\n\n\r\n\x05\x04\x0f\x02\x08\x05\x12\x04\
    \x80\x02\x0b\x10\n\r\n\x05\x04\x0f\x02\x08\x01\x12\x04\x80\x02\x11\x1d\n\
    \r\n\x05\x04\x0f\x02\x08\x03\x12\x04\x80\x02\x20!\n5\n\x04\x04\x0f\x02\t\
    \x12\x04\x81\x02\x025\"'\x20each\x20internal\x20block\x20has\x20a\x20blo\
    ck\x20token\n\n\r\n\x05\x04\x0f\x02\t\x04\x12\x04\x81\x02\x02\n\n\r\n\
    \x05\x04\x0f\x02\t\x06\x12\x04\x81\x02\x0b#\n\r\n\x05\x04\x0f\x02\t\x01\
    \x12\x04\x81\x02$/\n\r\n\x05\x04\x0f\x02\t\x03\x12\x04\x81\x0224\n\x0c\n\
    \x02\x04\x10\x12\x06\x84\x02\0\x8b\x02\x01\n\x0b\n\x03\x04\x10\x01\x12\
    \x04\x84\x02\x08\x1e\n\x0c\n\x04\x04\x10\x02\0\x12\x04\x85\x02\x02\x1c\n\
    \r\n\x05\x04\x10\x02\0\x04\x12\x04\x85\x02\x02\n\n\r\n\x05\x04\x10\x02\0\
    \x05\x12\x04\x85\x02\x0b\x11\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\x85\x02\
    \x12\x17\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\x85\x02\x1a\x1b\n\x0c\n\x04\
    \x04\x10\x02\x01\x12\x04\x86\x02\x02\"\n\r\n\x05\x04\x10\x02\x01\x04\x12\
    \x04\x86\x02\x02\n\n\r\n\x05\x04\x10\x02\x01\x05\x12\x04\x86\x02\x0b\x11\
    \n\r\n\x05\x04\x10\x02\x01\x01\x12\x04\x86\x02\x12\x1d\n\r\n\x05\x04\x10\
    \x02\x01\x03\x12\x04\x86\x02\x20!\n\x0c\n\x04\x04\x10\x02\x02\x12\x04\
    \x87\x02\x02\x1b\n\r\n\x05\x04\x10\x02\x02\x04\x12\x04\x87\x02\x02\n\n\r\
    \n\x05\x04\x10\x02\x02\x05\x12\x04\x87\x02\x0b\x10\n\r\n\x05\x04\x10\x02\
    \x02\x01\x12\x04\x87\x02\x11\x16\n\r\n\x05\x04\x10\x02\x02\x03\x12\x04\
    \x87\x02\x19\x1a\n\x0c\n\x04\x04\x10\x02\x03\x12\x04\x88\x02\x02#\n\r\n\
    \x05\x04\x10\x02\x03\x04\x12\x04\x88\x02\x02\n\n\r\n\x05\x04\x10\x02\x03\
    \x05\x12\x04\x88\x02\x0b\x10\n\r\n\x05\x04\x10\x02\x03\x01\x12\x04\x88\
    \x02\x11\x1e\n\r\n\x05\x04\x10\x02\x03\x03\x12\x04\x88\x02!\"\n\x0c\n\
    \x04\x04\x10\x02\x04\x12\x04\x89\x02\x02!\n\r\n\x05\x04\x10\x02\x04\x04\
    \x12\x04\x89\x02\x02\n\n\r\n\x05\x04\x10\x02\x04\x05\x12\x04\x89\x02\x0b\
    \x11\n\r\n\x05\x04\x10\x02\x04\x01\x12\x04\x89\x02\x12\x1c\n\r\n\x05\x04\
    \x10\x02\x04\x03\x12\x04\x89\x02\x1f\x20\n\x0c\n\x04\x04\x10\x02\x05\x12\
    \x04\x8a\x02\x02*\n\r\n\x05\x04\x10\x02\x05\x04\x12\x04\x8a\x02\x02\n\n\
    \r\n\x05\x04\x10\x02\x05\x05\x12\x04\x8a\x02\x0b\x11\n\r\n\x05\x04\x10\
    \x02\x05\x01\x12\x04\x8a\x02\x12%\n\r\n\x05\x04\x10\x02\x05\x03\x12\x04\
    \x8a\x02()\n\x1f\n\x02\x05\x02\x12\x06\x90\x02\0\x93\x02\x01\x1a\x11*\n\
    \x20Cipher\x20suite.\n\n\x0b\n\x03\x05\x02\x01\x12\x04\x90\x02\x05\x15\n\
    \x0c\n\x04\x05\x02\x02\0\x12\x04\x91\x02\x04\x10\n\r\n\x05\x05\x02\x02\0\
    \x01\x12\x04\x91\x02\x04\x0b\n\r\n\x05\x05\x02\x02\0\x02\x12\x04\x91\x02\
    \x0e\x0f\n\x0c\n\x04\x05\x02\x02\x01\x12\x04\x92\x02\x04\x1a\n\r\n\x05\
    \x05\x02\x02\x01\x01\x12\x04\x92\x02\x04\x15\n\r\n\x05\x05\x02\x02\x01\
    \x02\x12\x04\x92\x02\x18\x19\nI\n\x02\x05\x03\x12\x06\x98\x02\0\x9b\x02\
    \x01\x1a;*\n\x20Crypto\x20protocol\x20version\x20used\x20to\x20access\
    \x20encrypted\x20files.\n\n\x0b\n\x03\x05\x03\x01\x12\x04\x98\x02\x05\
    \x1f\n\x0c\n\x04\x05\x03\x02\0\x12\x04\x99\x02\x04!\n\r\n\x05\x05\x03\
    \x02\0\x01\x12\x04\x99\x02\x04\x1c\n\r\n\x05\x05\x03\x02\0\x02\x12\x04\
    \x99\x02\x1f\x20\n\x0c\n\x04\x05\x03\x02\x01\x12\x04\x9a\x02\x04\x19\n\r\
    \n\x05\x05\x03\x02\x01\x01\x12\x04\x9a\x02\x04\x14\n\r\n\x05\x05\x03\x02\
    \x01\x02\x12\x04\x9a\x02\x17\x18\n4\n\x02\x04\x11\x12\x06\xa0\x02\0\xa7\
    \x02\x01\x1a&*\n\x20Encryption\x20information\x20for\x20a\x20file.\n\n\
    \x0b\n\x03\x04\x11\x01\x12\x04\xa0\x02\x08\x1f\n\x0c\n\x04\x04\x11\x02\0\
    \x12\x04\xa1\x02\x02&\n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xa1\x02\x02\n\
    \n\r\n\x05\x04\x11\x02\0\x06\x12\x04\xa1\x02\x0b\x1b\n\r\n\x05\x04\x11\
    \x02\0\x01\x12\x04\xa1\x02\x1c!\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xa1\
    \x02$%\n\x0c\n\x04\x04\x11\x02\x01\x12\x04\xa2\x02\x02@\n\r\n\x05\x04\
    \x11\x02\x01\x04\x12\x04\xa2\x02\x02\n\n\r\n\x05\x04\x11\x02\x01\x06\x12\
    \x04\xa2\x02\x0b%\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xa2\x02&;\n\r\n\
    \x05\x04\x11\x02\x01\x03\x12\x04\xa2\x02>?\n\x0c\n\x04\x04\x11\x02\x02\
    \x12\x04\xa3\x02\x02\x19\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\xa3\x02\
    \x02\n\n\r\n\x05\x04\x11\x02\x02\x05\x12\x04\xa3\x02\x0b\x10\n\r\n\x05\
    \x04\x11\x02\x02\x01\x12\x04\xa3\x02\x11\x14\n\r\n\x05\x04\x11\x02\x02\
    \x03\x12\x04\xa3\x02\x17\x18\n\x0c\n\x04\x04\x11\x02\x03\x12\x04\xa4\x02\
    \x02\x18\n\r\n\x05\x04\x11\x02\x03\x04\x12\x04\xa4\x02\x02\n\n\r\n\x05\
    \x04\x11\x02\x03\x05\x12\x04\xa4\x02\x0b\x10\n\r\n\x05\x04\x11\x02\x03\
    \x01\x12\x04\xa4\x02\x11\x13\n\r\n\x05\x04\x11\x02\x03\x03\x12\x04\xa4\
    \x02\x16\x17\n\x0c\n\x04\x04\x11\x02\x04\x12\x04\xa5\x02\x02\x1e\n\r\n\
    \x05\x04\x11\x02\x04\x04\x12\x04\xa5\x02\x02\n\n\r\n\x05\x04\x11\x02\x04\
    \x05\x12\x04\xa5\x02\x0b\x11\n\r\n\x05\x04\x11\x02\x04\x01\x12\x04\xa5\
    \x02\x12\x19\n\r\n\x05\x04\x11\x02\x04\x03\x12\x04\xa5\x02\x1c\x1d\n\x0c\
    \n\x04\x04\x11\x02\x05\x12\x04\xa6\x02\x02'\n\r\n\x05\x04\x11\x02\x05\
    \x04\x12\x04\xa6\x02\x02\n\n\r\n\x05\x04\x11\x02\x05\x05\x12\x04\xa6\x02\
    \x0b\x11\n\r\n\x05\x04\x11\x02\x05\x01\x12\x04\xa6\x02\x12\"\n\r\n\x05\
    \x04\x11\x02\x05\x03\x12\x04\xa6\x02%&\nZ\n\x02\x04\x12\x12\x06\xad\x02\
    \0\xb1\x02\x01\x1aL*\n\x20Encryption\x20information\x20for\x20an\x20indi\
    vidual\n\x20file\x20within\x20an\x20encryption\x20zone\n\n\x0b\n\x03\x04\
    \x12\x01\x12\x04\xad\x02\x08\"\n\x0c\n\x04\x04\x12\x02\0\x12\x04\xae\x02\
    \x02\x19\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xae\x02\x02\n\n\r\n\x05\x04\
    \x12\x02\0\x05\x12\x04\xae\x02\x0b\x10\n\r\n\x05\x04\x12\x02\0\x01\x12\
    \x04\xae\x02\x11\x14\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xae\x02\x17\x18\
    \n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xaf\x02\x02\x18\n\r\n\x05\x04\x12\
    \x02\x01\x04\x12\x04\xaf\x02\x02\n\n\r\n\x05\x04\x12\x02\x01\x05\x12\x04\
    \xaf\x02\x0b\x10\n\r\n\x05\x04\x12\x02\x01\x01\x12\x04\xaf\x02\x11\x13\n\
    \r\n\x05\x04\x12\x02\x01\x03\x12\x04\xaf\x02\x16\x17\n\x0c\n\x04\x04\x12\
    \x02\x02\x12\x04\xb0\x02\x02'\n\r\n\x05\x04\x12\x02\x02\x04\x12\x04\xb0\
    \x02\x02\n\n\r\n\x05\x04\x12\x02\x02\x05\x12\x04\xb0\x02\x0b\x11\n\r\n\
    \x05\x04\x12\x02\x02\x01\x12\x04\xb0\x02\x12\"\n\r\n\x05\x04\x12\x02\x02\
    \x03\x12\x04\xb0\x02%&\n@\n\x02\x04\x13\x12\x06\xb7\x02\0\xbc\x02\x01\
    \x1a2*\n\x20Encryption\x20information\x20for\x20an\x20encryption\n\x20zo\
    ne\n\n\x0b\n\x03\x04\x13\x01\x12\x04\xb7\x02\x08\x1f\n\x0c\n\x04\x04\x13\
    \x02\0\x12\x04\xb8\x02\x02&\n\r\n\x05\x04\x13\x02\0\x04\x12\x04\xb8\x02\
    \x02\n\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\xb8\x02\x0b\x1b\n\r\n\x05\x04\
    \x13\x02\0\x01\x12\x04\xb8\x02\x1c!\n\r\n\x05\x04\x13\x02\0\x03\x12\x04\
    \xb8\x02$%\n\x0c\n\x04\x04\x13\x02\x01\x12\x04\xb9\x02\x02@\n\r\n\x05\
    \x04\x13\x02\x01\x04\x12\x04\xb9\x02\x02\n\n\r\n\x05\x04\x13\x02\x01\x06\
    \x12\x04\xb9\x02\x0b%\n\r\n\x05\x04\x13\x02\x01\x01\x12\x04\xb9\x02&;\n\
    \r\n\x05\x04\x13\x02\x01\x03\x12\x04\xb9\x02>?\n\x0c\n\x04\x04\x13\x02\
    \x02\x12\x04\xba\x02\x02\x1e\n\r\n\x05\x04\x13\x02\x02\x04\x12\x04\xba\
    \x02\x02\n\n\r\n\x05\x04\x13\x02\x02\x05\x12\x04\xba\x02\x0b\x11\n\r\n\
    \x05\x04\x13\x02\x02\x01\x12\x04\xba\x02\x12\x19\n\r\n\x05\x04\x13\x02\
    \x02\x03\x12\x04\xba\x02\x1c\x1d\n\x0c\n\x04\x04\x13\x02\x03\x12\x04\xbb\
    \x02\x027\n\r\n\x05\x04\x13\x02\x03\x04\x12\x04\xbb\x02\x02\n\n\r\n\x05\
    \x04\x13\x02\x03\x06\x12\x04\xbb\x02\x0b\x20\n\r\n\x05\x04\x13\x02\x03\
    \x01\x12\x04\xbb\x02!2\n\r\n\x05\x04\x13\x02\x03\x03\x12\x04\xbb\x0256\n\
    B\n\x02\x04\x14\x12\x06\xc1\x02\0\xc9\x02\x01\x1a4*\n\x20Re-encryption\
    \x20information\x20for\x20an\x20encryption\x20zone\n\n\x0b\n\x03\x04\x14\
    \x01\x12\x04\xc1\x02\x08\x1d\n\x0c\n\x04\x04\x14\x02\0\x12\x04\xc2\x02\
    \x02'\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\xc2\x02\x02\n\n\r\n\x05\x04\
    \x14\x02\0\x05\x12\x04\xc2\x02\x0b\x11\n\r\n\x05\x04\x14\x02\0\x01\x12\
    \x04\xc2\x02\x12\"\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\xc2\x02%&\n\x0c\n\
    \x04\x04\x14\x02\x01\x12\x04\xc3\x02\x02%\n\r\n\x05\x04\x14\x02\x01\x04\
    \x12\x04\xc3\x02\x02\n\n\r\n\x05\x04\x14\x02\x01\x05\x12\x04\xc3\x02\x0b\
    \x11\n\r\n\x05\x04\x14\x02\x01\x01\x12\x04\xc3\x02\x12\x20\n\r\n\x05\x04\
    \x14\x02\x01\x03\x12\x04\xc3\x02#$\n\x0c\n\x04\x04\x14\x02\x02\x12\x04\
    \xc4\x02\x02\x1d\n\r\n\x05\x04\x14\x02\x02\x04\x12\x04\xc4\x02\x02\n\n\r\
    \n\x05\x04\x14\x02\x02\x05\x12\x04\xc4\x02\x0b\x0f\n\r\n\x05\x04\x14\x02\
    \x02\x01\x12\x04\xc4\x02\x10\x18\n\r\n\x05\x04\x14\x02\x02\x03\x12\x04\
    \xc4\x02\x1b\x1c\n\x0c\n\x04\x04\x14\x02\x03\x12\x04\xc5\x02\x02$\n\r\n\
    \x05\x04\x14\x02\x03\x04\x12\x04\xc5\x02\x02\n\n\r\n\x05\x04\x14\x02\x03\
    \x05\x12\x04\xc5\x02\x0b\x10\n\r\n\x05\x04\x14\x02\x03\x01\x12\x04\xc5\
    \x02\x11\x1f\n\r\n\x05\x04\x14\x02\x03\x03\x12\x04\xc5\x02\"#\n\x0c\n\
    \x04\x04\x14\x02\x04\x12\x04\xc6\x02\x02!\n\r\n\x05\x04\x14\x02\x04\x04\
    \x12\x04\xc6\x02\x02\n\n\r\n\x05\x04\x14\x02\x04\x05\x12\x04\xc6\x02\x0b\
    \x10\n\r\n\x05\x04\x14\x02\x04\x01\x12\x04\xc6\x02\x11\x1c\n\r\n\x05\x04\
    \x14\x02\x04\x03\x12\x04\xc6\x02\x1f\x20\n\x0c\n\x04\x04\x14\x02\x05\x12\
    \x04\xc7\x02\x02%\n\r\n\x05\x04\x14\x02\x05\x04\x12\x04\xc7\x02\x02\n\n\
    \r\n\x05\x04\x14\x02\x05\x05\x12\x04\xc7\x02\x0b\x11\n\r\n\x05\x04\x14\
    \x02\x05\x01\x12\x04\xc7\x02\x12\x20\n\r\n\x05\x04\x14\x02\x05\x03\x12\
    \x04\xc7\x02#$\n\x0c\n\x04\x04\x14\x02\x06\x12\x04\xc8\x02\x02\x1f\n\r\n\
    \x05\x04\x14\x02\x06\x04\x12\x04\xc8\x02\x02\n\n\r\n\x05\x04\x14\x02\x06\
    \x05\x12\x04\xc8\x02\x0b\x11\n\r\n\x05\x04\x14\x02\x06\x01\x12\x04\xc8\
    \x02\x12\x1a\n\r\n\x05\x04\x14\x02\x06\x03\x12\x04\xc8\x02\x1d\x1e\n\x1f\
    \n\x02\x04\x15\x12\x06\xce\x02\0\xd4\x02\x01\x1a\x11*\n\x20Cipher\x20opt\
    ion\n\n\x0b\n\x03\x04\x15\x01\x12\x04\xce\x02\x08\x19\n\x0c\n\x04\x04\
    \x15\x02\0\x12\x04\xcf\x02\x02&\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\xcf\
    \x02\x02\n\n\r\n\x05\x04\x15\x02\0\x06\x12\x04\xcf\x02\x0b\x1b\n\r\n\x05\
    \x04\x15\x02\0\x01\x12\x04\xcf\x02\x1c!\n\r\n\x05\x04\x15\x02\0\x03\x12\
    \x04\xcf\x02$%\n\x0c\n\x04\x04\x15\x02\x01\x12\x04\xd0\x02\x02\x1b\n\r\n\
    \x05\x04\x15\x02\x01\x04\x12\x04\xd0\x02\x02\n\n\r\n\x05\x04\x15\x02\x01\
    \x05\x12\x04\xd0\x02\x0b\x10\n\r\n\x05\x04\x15\x02\x01\x01\x12\x04\xd0\
    \x02\x11\x16\n\r\n\x05\x04\x15\x02\x01\x03\x12\x04\xd0\x02\x19\x1a\n\x0c\
    \n\x04\x04\x15\x02\x02\x12\x04\xd1\x02\x02\x1a\n\r\n\x05\x04\x15\x02\x02\
    \x04\x12\x04\xd1\x02\x02\n\n\r\n\x05\x04\x15\x02\x02\x05\x12\x04\xd1\x02\
    \x0b\x10\n\r\n\x05\x04\x15\x02\x02\x01\x12\x04\xd1\x02\x11\x15\n\r\n\x05\
    \x04\x15\x02\x02\x03\x12\x04\xd1\x02\x18\x19\n\x0c\n\x04\x04\x15\x02\x03\
    \x12\x04\xd2\x02\x02\x1c\n\r\n\x05\x04\x15\x02\x03\x04\x12\x04\xd2\x02\
    \x02\n\n\r\n\x05\x04\x15\x02\x03\x05\x12\x04\xd2\x02\x0b\x10\n\r\n\x05\
    \x04\x15\x02\x03\x01\x12\x04\xd2\x02\x11\x17\n\r\n\x05\x04\x15\x02\x03\
    \x03\x12\x04\xd2\x02\x1a\x1b\n\x0c\n\x04\x04\x15\x02\x04\x12\x04\xd3\x02\
    \x02\x1b\n\r\n\x05\x04\x15\x02\x04\x04\x12\x04\xd3\x02\x02\n\n\r\n\x05\
    \x04\x15\x02\x04\x05\x12\x04\xd3\x02\x0b\x10\n\r\n\x05\x04\x15\x02\x04\
    \x01\x12\x04\xd3\x02\x11\x16\n\r\n\x05\x04\x15\x02\x04\x03\x12\x04\xd3\
    \x02\x19\x1a\n;\n\x02\x04\x16\x12\x06\xd9\x02\0\xe3\x02\x01\x1a-*\n\x20A\
    \x20set\x20of\x20file\x20blocks\x20and\x20their\x20locations.\n\n\x0b\n\
    \x03\x04\x16\x01\x12\x04\xd9\x02\x08\x1a\n\x0c\n\x04\x04\x16\x02\0\x12\
    \x04\xda\x02\x02!\n\r\n\x05\x04\x16\x02\0\x04\x12\x04\xda\x02\x02\n\n\r\
    \n\x05\x04\x16\x02\0\x05\x12\x04\xda\x02\x0b\x11\n\r\n\x05\x04\x16\x02\0\
    \x01\x12\x04\xda\x02\x12\x1c\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\xda\x02\
    \x1f\x20\n\x0c\n\x04\x04\x16\x02\x01\x12\x04\xdb\x02\x02(\n\r\n\x05\x04\
    \x16\x02\x01\x04\x12\x04\xdb\x02\x02\n\n\r\n\x05\x04\x16\x02\x01\x06\x12\
    \x04\xdb\x02\x0b\x1c\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\xdb\x02\x1d#\
    \n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\xdb\x02&'\n\x0c\n\x04\x04\x16\x02\
    \x02\x12\x04\xdc\x02\x02&\n\r\n\x05\x04\x16\x02\x02\x04\x12\x04\xdc\x02\
    \x02\n\n\r\n\x05\x04\x16\x02\x02\x05\x12\x04\xdc\x02\x0b\x0f\n\r\n\x05\
    \x04\x16\x02\x02\x01\x12\x04\xdc\x02\x10!\n\r\n\x05\x04\x16\x02\x02\x03\
    \x12\x04\xdc\x02$%\n\x0c\n\x04\x04\x16\x02\x03\x12\x04\xdd\x02\x02+\n\r\
    \n\x05\x04\x16\x02\x03\x04\x12\x04\xdd\x02\x02\n\n\r\n\x05\x04\x16\x02\
    \x03\x06\x12\x04\xdd\x02\x0b\x1c\n\r\n\x05\x04\x16\x02\x03\x01\x12\x04\
    \xdd\x02\x1d&\n\r\n\x05\x04\x16\x02\x03\x03\x12\x04\xdd\x02)*\n\x0c\n\
    \x04\x04\x16\x02\x04\x12\x04\xde\x02\x02(\n\r\n\x05\x04\x16\x02\x04\x04\
    \x12\x04\xde\x02\x02\n\n\r\n\x05\x04\x16\x02\x04\x05\x12\x04\xde\x02\x0b\
    \x0f\n\r\n\x05\x04\x16\x02\x04\x01\x12\x04\xde\x02\x10#\n\r\n\x05\x04\
    \x16\x02\x04\x03\x12\x04\xde\x02&'\n\x0c\n\x04\x04\x16\x02\x05\x12\x04\
    \xdf\x02\x02:\n\r\n\x05\x04\x16\x02\x05\x04\x12\x04\xdf\x02\x02\n\n\r\n\
    \x05\x04\x16\x02\x05\x06\x12\x04\xdf\x02\x0b\"\n\r\n\x05\x04\x16\x02\x05\
    \x01\x12\x04\xdf\x02#5\n\r\n\x05\x04\x16\x02\x05\x03\x12\x04\xdf\x0289\n\
    1\n\x04\x04\x16\x02\x06\x12\x04\xe2\x02\x021\x1a#\x20Optional\x20field\
    \x20for\x20erasure\x20coding\n\n\r\n\x05\x04\x16\x02\x06\x04\x12\x04\xe2\
    \x02\x02\n\n\r\n\x05\x04\x16\x02\x06\x06\x12\x04\xe2\x02\x0b#\n\r\n\x05\
    \x04\x16\x02\x06\x01\x12\x04\xe2\x02$,\n\r\n\x05\x04\x16\x02\x06\x03\x12\
    \x04\xe2\x02/0\n(\n\x02\x04\x17\x12\x06\xe8\x02\0\xeb\x02\x01\x1a\x1a*\n\
    \x20ECSchema\x20options\x20entry\n\n\x0b\n\x03\x04\x17\x01\x12\x04\xe8\
    \x02\x08\x20\n\x0c\n\x04\x04\x17\x02\0\x12\x04\xe9\x02\x02\x1a\n\r\n\x05\
    \x04\x17\x02\0\x04\x12\x04\xe9\x02\x02\n\n\r\n\x05\x04\x17\x02\0\x05\x12\
    \x04\xe9\x02\x0b\x11\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xe9\x02\x12\x15\
    \n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xe9\x02\x18\x19\n\x0c\n\x04\x04\x17\
    \x02\x01\x12\x04\xea\x02\x02\x1c\n\r\n\x05\x04\x17\x02\x01\x04\x12\x04\
    \xea\x02\x02\n\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\xea\x02\x0b\x11\n\r\
    \n\x05\x04\x17\x02\x01\x01\x12\x04\xea\x02\x12\x17\n\r\n\x05\x04\x17\x02\
    \x01\x03\x12\x04\xea\x02\x1a\x1b\n,\n\x02\x04\x18\x12\x06\xf0\x02\0\xf5\
    \x02\x01\x1a\x1e*\n\x20ECSchema\x20for\x20erasurecoding\n\n\x0b\n\x03\
    \x04\x18\x01\x12\x04\xf0\x02\x08\x15\n\x0c\n\x04\x04\x18\x02\0\x12\x04\
    \xf1\x02\x02\x20\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\xf1\x02\x02\n\n\r\n\
    \x05\x04\x18\x02\0\x05\x12\x04\xf1\x02\x0b\x11\n\r\n\x05\x04\x18\x02\0\
    \x01\x12\x04\xf1\x02\x12\x1b\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xf1\x02\
    \x1e\x1f\n\x0c\n\x04\x04\x18\x02\x01\x12\x04\xf2\x02\x02\x20\n\r\n\x05\
    \x04\x18\x02\x01\x04\x12\x04\xf2\x02\x02\n\n\r\n\x05\x04\x18\x02\x01\x05\
    \x12\x04\xf2\x02\x0b\x11\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\xf2\x02\
    \x12\x1b\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xf2\x02\x1e\x1f\n\x0c\n\
    \x04\x04\x18\x02\x02\x12\x04\xf3\x02\x02\"\n\r\n\x05\x04\x18\x02\x02\x04\
    \x12\x04\xf3\x02\x02\n\n\r\n\x05\x04\x18\x02\x02\x05\x12\x04\xf3\x02\x0b\
    \x11\n\r\n\x05\x04\x18\x02\x02\x01\x12\x04\xf3\x02\x12\x1d\n\r\n\x05\x04\
    \x18\x02\x02\x03\x12\x04\xf3\x02\x20!\n\x0c\n\x04\x04\x18\x02\x03\x12\
    \x04\xf4\x02\x020\n\r\n\x05\x04\x18\x02\x03\x04\x12\x04\xf4\x02\x02\n\n\
    \r\n\x05\x04\x18\x02\x03\x06\x12\x04\xf4\x02\x0b#\n\r\n\x05\x04\x18\x02\
    \x03\x01\x12\x04\xf4\x02$+\n\r\n\x05\x04\x18\x02\x03\x03\x12\x04\xf4\x02\
    ./\n\"\n\x02\x05\x04\x12\x06\xfa\x02\0\xfe\x02\x01\x1a\x14*\n\x20EC\x20p\
    olicy\x20state.\n\n\x0b\n\x03\x05\x04\x01\x12\x04\xfa\x02\x05\x1d\n\x0c\
    \n\x04\x05\x04\x02\0\x12\x04\xfb\x02\x02\x0f\n\r\n\x05\x05\x04\x02\0\x01\
    \x12\x04\xfb\x02\x02\n\n\r\n\x05\x05\x04\x02\0\x02\x12\x04\xfb\x02\r\x0e\
    \n\x0c\n\x04\x05\x04\x02\x01\x12\x04\xfc\x02\x02\x0e\n\r\n\x05\x05\x04\
    \x02\x01\x01\x12\x04\xfc\x02\x02\t\n\r\n\x05\x05\x04\x02\x01\x02\x12\x04\
    \xfc\x02\x0c\r\n\x0c\n\x04\x05\x04\x02\x02\x12\x04\xfd\x02\x02\x0e\n\r\n\
    \x05\x05\x04\x02\x02\x01\x12\x04\xfd\x02\x02\t\n\r\n\x05\x05\x04\x02\x02\
    \x02\x12\x04\xfd\x02\x0c\r\n\x0c\n\x02\x04\x19\x12\x06\x80\x03\0\x86\x03\
    \x01\n\x0b\n\x03\x04\x19\x01\x12\x04\x80\x03\x08\x20\n\x0c\n\x04\x04\x19\
    \x02\0\x12\x04\x81\x03\x02\x1b\n\r\n\x05\x04\x19\x02\0\x04\x12\x04\x81\
    \x03\x02\n\n\r\n\x05\x04\x19\x02\0\x05\x12\x04\x81\x03\x0b\x11\n\r\n\x05\
    \x04\x19\x02\0\x01\x12\x04\x81\x03\x12\x16\n\r\n\x05\x04\x19\x02\0\x03\
    \x12\x04\x81\x03\x19\x1a\n\x0c\n\x04\x04\x19\x02\x01\x12\x04\x82\x03\x02\
    $\n\r\n\x05\x04\x19\x02\x01\x04\x12\x04\x82\x03\x02\n\n\r\n\x05\x04\x19\
    \x02\x01\x06\x12\x04\x82\x03\x0b\x18\n\r\n\x05\x04\x19\x02\x01\x01\x12\
    \x04\x82\x03\x19\x1f\n\r\n\x05\x04\x19\x02\x01\x03\x12\x04\x82\x03\"#\n\
    \x0c\n\x04\x04\x19\x02\x02\x12\x04\x83\x03\x02\x1f\n\r\n\x05\x04\x19\x02\
    \x02\x04\x12\x04\x83\x03\x02\n\n\r\n\x05\x04\x19\x02\x02\x05\x12\x04\x83\
    \x03\x0b\x11\n\r\n\x05\x04\x19\x02\x02\x01\x12\x04\x83\x03\x12\x1a\n\r\n\
    \x05\x04\x19\x02\x02\x03\x12\x04\x83\x03\x1d\x1e\n2\n\x04\x04\x19\x02\
    \x03\x12\x04\x84\x03\x02\x19\"$\x20Actually\x20a\x20byte\x20-\x20only\
    \x208\x20bits\x20used\n\n\r\n\x05\x04\x19\x02\x03\x04\x12\x04\x84\x03\
    \x02\n\n\r\n\x05\x04\x19\x02\x03\x05\x12\x04\x84\x03\x0b\x11\n\r\n\x05\
    \x04\x19\x02\x03\x01\x12\x04\x84\x03\x12\x14\n\r\n\x05\x04\x19\x02\x03\
    \x03\x12\x04\x84\x03\x17\x18\n\x0c\n\x04\x04\x19\x02\x04\x12\x04\x85\x03\
    \x02B\n\r\n\x05\x04\x19\x02\x04\x04\x12\x04\x85\x03\x02\n\n\r\n\x05\x04\
    \x19\x02\x04\x06\x12\x04\x85\x03\x0b#\n\r\n\x05\x04\x19\x02\x04\x01\x12\
    \x04\x85\x03$)\n\r\n\x05\x04\x19\x02\x04\x03\x12\x04\x85\x03,-\n\r\n\x05\
    \x04\x19\x02\x04\x08\x12\x04\x85\x03.A\n\r\n\x05\x04\x19\x02\x04\x07\x12\
    \x04\x85\x039@\n\x0c\n\x02\x04\x1a\x12\x06\x88\x03\0\x8c\x03\x01\n\x0b\n\
    \x03\x04\x1a\x01\x12\x04\x88\x03\x08+\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\
    \x89\x03\x02/\n\r\n\x05\x04\x1a\x02\0\x04\x12\x04\x89\x03\x02\n\n\r\n\
    \x05\x04\x1a\x02\0\x06\x12\x04\x89\x03\x0b#\n\r\n\x05\x04\x1a\x02\0\x01\
    \x12\x04\x89\x03$*\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\x89\x03-.\n\x0c\n\
    \x04\x04\x1a\x02\x01\x12\x04\x8a\x03\x02\x1c\n\r\n\x05\x04\x1a\x02\x01\
    \x04\x12\x04\x8a\x03\x02\n\n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\x8a\x03\
    \x0b\x0f\n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\x8a\x03\x10\x17\n\r\n\x05\
    \x04\x1a\x02\x01\x03\x12\x04\x8a\x03\x1a\x1b\n\x0c\n\x04\x04\x1a\x02\x02\
    \x12\x04\x8b\x03\x02\x1f\n\r\n\x05\x04\x1a\x02\x02\x04\x12\x04\x8b\x03\
    \x02\n\n\r\n\x05\x04\x1a\x02\x02\x05\x12\x04\x8b\x03\x0b\x11\n\r\n\x05\
    \x04\x1a\x02\x02\x01\x12\x04\x8b\x03\x12\x1a\n\r\n\x05\x04\x1a\x02\x02\
    \x03\x12\x04\x8b\x03\x1d\x1e\n\x8f\x01\n\x02\x04\x1b\x12\x06\x92\x03\0\
    \xbb\x03\x01\x1a\x80\x01*\n\x20Status\x20of\x20a\x20file,\x20directory\
    \x20or\x20symlink\n\x20Optionally\x20includes\x20a\x20file's\x20block\
    \x20locations\x20if\x20requested\x20by\x20client\x20on\x20the\x20rpc\x20\
    call.\n\n\x0b\n\x03\x04\x1b\x01\x12\x04\x92\x03\x08\x1b\n\x0e\n\x04\x04\
    \x1b\x04\0\x12\x06\x93\x03\x02\x97\x03\x03\n\r\n\x05\x04\x1b\x04\0\x01\
    \x12\x04\x93\x03\x07\x0f\n\x0e\n\x06\x04\x1b\x04\0\x02\0\x12\x04\x94\x03\
    \x04\x0f\n\x0f\n\x07\x04\x1b\x04\0\x02\0\x01\x12\x04\x94\x03\x04\n\n\x0f\
    \n\x07\x04\x1b\x04\0\x02\0\x02\x12\x04\x94\x03\r\x0e\n\x0e\n\x06\x04\x1b\
    \x04\0\x02\x01\x12\x04\x95\x03\x04\x10\n\x0f\n\x07\x04\x1b\x04\0\x02\x01\
    \x01\x12\x04\x95\x03\x04\x0b\n\x0f\n\x07\x04\x1b\x04\0\x02\x01\x02\x12\
    \x04\x95\x03\x0e\x0f\n\x0e\n\x06\x04\x1b\x04\0\x02\x02\x12\x04\x96\x03\
    \x04\x13\n\x0f\n\x07\x04\x1b\x04\0\x02\x02\x01\x12\x04\x96\x03\x04\x0e\n\
    \x0f\n\x07\x04\x1b\x04\0\x02\x02\x02\x12\x04\x96\x03\x11\x12\n\x0e\n\x04\
    \x04\x1b\x04\x01\x12\x06\x98\x03\x02\x9c\x03\x03\n\r\n\x05\x04\x1b\x04\
    \x01\x01\x12\x04\x98\x03\x07\x0c\n\x1a\n\x06\x04\x1b\x04\x01\x02\0\x12\
    \x04\x99\x03\x04\x15\"\n\x20has\x20ACLs\n\n\x0f\n\x07\x04\x1b\x04\x01\
    \x02\0\x01\x12\x04\x99\x03\x04\x0b\n\x0f\n\x07\x04\x1b\x04\x01\x02\0\x02\
    \x12\x04\x99\x03\x10\x14\n\x1b\n\x06\x04\x1b\x04\x01\x02\x01\x12\x04\x9a\
    \x03\x04\x15\"\x0b\x20encrypted\n\n\x0f\n\x07\x04\x1b\x04\x01\x02\x01\
    \x01\x12\x04\x9a\x03\x04\r\n\x0f\n\x07\x04\x1b\x04\x01\x02\x01\x02\x12\
    \x04\x9a\x03\x10\x14\n\x1f\n\x06\x04\x1b\x04\x01\x02\x02\x12\x04\x9b\x03\
    \x04\x15\"\x0f\x20erasure\x20coded\n\n\x0f\n\x07\x04\x1b\x04\x01\x02\x02\
    \x01\x12\x04\x9b\x03\x04\n\n\x0f\n\x07\x04\x1b\x04\x01\x02\x02\x02\x12\
    \x04\x9b\x03\x10\x14\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\x9d\x03\x02!\n\r\
    \n\x05\x04\x1b\x02\0\x04\x12\x04\x9d\x03\x02\n\n\r\n\x05\x04\x1b\x02\0\
    \x06\x12\x04\x9d\x03\x0b\x13\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\x9d\x03\
    \x14\x1c\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\x9d\x03\x1f\x20\n5\n\x04\
    \x04\x1b\x02\x01\x12\x04\x9e\x03\x02\x1a\"'\x20local\x20name\x20of\x20in\
    ode\x20encoded\x20java\x20UTF8\n\n\r\n\x05\x04\x1b\x02\x01\x04\x12\x04\
    \x9e\x03\x02\n\n\r\n\x05\x04\x1b\x02\x01\x05\x12\x04\x9e\x03\x0b\x10\n\r\
    \n\x05\x04\x1b\x02\x01\x01\x12\x04\x9e\x03\x11\x15\n\r\n\x05\x04\x1b\x02\
    \x01\x03\x12\x04\x9e\x03\x18\x19\n\x0c\n\x04\x04\x1b\x02\x02\x12\x04\x9f\
    \x03\x02\x1d\n\r\n\x05\x04\x1b\x02\x02\x04\x12\x04\x9f\x03\x02\n\n\r\n\
    \x05\x04\x1b\x02\x02\x05\x12\x04\x9f\x03\x0b\x11\n\r\n\x05\x04\x1b\x02\
    \x02\x01\x12\x04\x9f\x03\x12\x18\n\r\n\x05\x04\x1b\x02\x02\x03\x12\x04\
    \x9f\x03\x1b\x1c\n\x0c\n\x04\x04\x1b\x02\x03\x12\x04\xa0\x03\x02,\n\r\n\
    \x05\x04\x1b\x02\x03\x04\x12\x04\xa0\x03\x02\n\n\r\n\x05\x04\x1b\x02\x03\
    \x06\x12\x04\xa0\x03\x0b\x1c\n\r\n\x05\x04\x1b\x02\x03\x01\x12\x04\xa0\
    \x03\x1d'\n\r\n\x05\x04\x1b\x02\x03\x03\x12\x04\xa0\x03*+\n\x0c\n\x04\
    \x04\x1b\x02\x04\x12\x04\xa1\x03\x02\x1c\n\r\n\x05\x04\x1b\x02\x04\x04\
    \x12\x04\xa1\x03\x02\n\n\r\n\x05\x04\x1b\x02\x04\x05\x12\x04\xa1\x03\x0b\
    \x11\n\r\n\x05\x04\x1b\x02\x04\x01\x12\x04\xa1\x03\x12\x17\n\r\n\x05\x04\
    \x1b\x02\x04\x03\x12\x04\xa1\x03\x1a\x1b\n\x0c\n\x04\x04\x1b\x02\x05\x12\
    \x04\xa2\x03\x02\x1c\n\r\n\x05\x04\x1b\x02\x05\x04\x12\x04\xa2\x03\x02\n\
    \n\r\n\x05\x04\x1b\x02\x05\x05\x12\x04\xa2\x03\x0b\x11\n\r\n\x05\x04\x1b\
    \x02\x05\x01\x12\x04\xa2\x03\x12\x17\n\r\n\x05\x04\x1b\x02\x05\x03\x12\
    \x04\xa2\x03\x1a\x1b\n\x0c\n\x04\x04\x1b\x02\x06\x12\x04\xa3\x03\x02(\n\
    \r\n\x05\x04\x1b\x02\x06\x04\x12\x04\xa3\x03\x02\n\n\r\n\x05\x04\x1b\x02\
    \x06\x05\x12\x04\xa3\x03\x0b\x11\n\r\n\x05\x04\x1b\x02\x06\x01\x12\x04\
    \xa3\x03\x12#\n\r\n\x05\x04\x1b\x02\x06\x03\x12\x04\xa3\x03&'\n\x0c\n\
    \x04\x04\x1b\x02\x07\x12\x04\xa4\x03\x02\"\n\r\n\x05\x04\x1b\x02\x07\x04\
    \x12\x04\xa4\x03\x02\n\n\r\n\x05\x04\x1b\x02\x07\x05\x12\x04\xa4\x03\x0b\
    \x11\n\r\n\x05\x04\x1b\x02\x07\x01\x12\x04\xa4\x03\x12\x1d\n\r\n\x05\x04\
    \x1b\x02\x07\x03\x12\x04\xa4\x03\x20!\nT\n\x04\x04\x1b\x02\x08\x12\x04\
    \xa7\x03\x02\x1d\x1a\x1d\x20Optional\x20fields\x20for\x20symlink\n\"'\
    \x20if\x20symlink,\x20target\x20encoded\x20java\x20UTF8\x20\n\n\r\n\x05\
    \x04\x1b\x02\x08\x04\x12\x04\xa7\x03\x02\n\n\r\n\x05\x04\x1b\x02\x08\x05\
    \x12\x04\xa7\x03\x0b\x10\n\r\n\x05\x04\x1b\x02\x08\x01\x12\x04\xa7\x03\
    \x11\x18\n\r\n\x05\x04\x1b\x02\x08\x03\x12\x04\xa7\x03\x1b\x1c\n<\n\x04\
    \x04\x1b\x02\t\x12\x04\xaa\x03\x027\x1a\x1a\x20Optional\x20fields\x20for\
    \x20file\n\"\x12\x20only\x2016bits\x20used\n\n\r\n\x05\x04\x1b\x02\t\x04\
    \x12\x04\xaa\x03\x02\n\n\r\n\x05\x04\x1b\x02\t\x05\x12\x04\xaa\x03\x0b\
    \x11\n\r\n\x05\x04\x1b\x02\t\x01\x12\x04\xaa\x03\x12#\n\r\n\x05\x04\x1b\
    \x02\t\x03\x12\x04\xaa\x03&(\n\r\n\x05\x04\x1b\x02\t\x08\x12\x04\xaa\x03\
    )6\n\r\n\x05\x04\x1b\x02\t\x07\x12\x04\xaa\x0345\n\x0c\n\x04\x04\x1b\x02\
    \n\x12\x04\xab\x03\x02/\n\r\n\x05\x04\x1b\x02\n\x04\x12\x04\xab\x03\x02\
    \n\n\r\n\x05\x04\x1b\x02\n\x05\x12\x04\xab\x03\x0b\x11\n\r\n\x05\x04\x1b\
    \x02\n\x01\x12\x04\xab\x03\x12\x1b\n\r\n\x05\x04\x1b\x02\n\x03\x12\x04\
    \xab\x03\x1e\x20\n\r\n\x05\x04\x1b\x02\n\x08\x12\x04\xab\x03!.\n\r\n\x05\
    \x04\x1b\x02\n\x07\x12\x04\xab\x03,-\n/\n\x04\x04\x1b\x02\x0b\x12\x04\
    \xac\x03\x02-\"!\x20suppled\x20only\x20if\x20asked\x20by\x20client\n\n\r\
    \n\x05\x04\x1b\x02\x0b\x04\x12\x04\xac\x03\x02\n\n\r\n\x05\x04\x1b\x02\
    \x0b\x06\x12\x04\xac\x03\x0b\x1d\n\r\n\x05\x04\x1b\x02\x0b\x01\x12\x04\
    \xac\x03\x1e'\n\r\n\x05\x04\x1b\x02\x0b\x03\x12\x04\xac\x03*,\nE\n\x04\
    \x04\x1b\x02\x0c\x12\x04\xaf\x03\x02,\x1a\x1b\x20Optional\x20field\x20fo\
    r\x20fileId\n\"\x1a\x20default\x20as\x20an\x20invalid\x20id\n\n\r\n\x05\
    \x04\x1b\x02\x0c\x04\x12\x04\xaf\x03\x02\n\n\r\n\x05\x04\x1b\x02\x0c\x05\
    \x12\x04\xaf\x03\x0b\x11\n\r\n\x05\x04\x1b\x02\x0c\x01\x12\x04\xaf\x03\
    \x12\x18\n\r\n\x05\x04\x1b\x02\x0c\x03\x12\x04\xaf\x03\x1b\x1d\n\r\n\x05\
    \x04\x1b\x02\x0c\x08\x12\x04\xaf\x03\x1e+\n\r\n\x05\x04\x1b\x02\x0c\x07\
    \x12\x04\xaf\x03)*\n\x0c\n\x04\x04\x1b\x02\r\x12\x04\xb0\x03\x021\n\r\n\
    \x05\x04\x1b\x02\r\x04\x12\x04\xb0\x03\x02\n\n\r\n\x05\x04\x1b\x02\r\x05\
    \x12\x04\xb0\x03\x0b\x10\n\r\n\x05\x04\x1b\x02\r\x01\x12\x04\xb0\x03\x11\
    \x1c\n\r\n\x05\x04\x1b\x02\r\x03\x12\x04\xb0\x03\x1f!\n\r\n\x05\x04\x1b\
    \x02\r\x08\x12\x04\xb0\x03\"0\n\r\n\x05\x04\x1b\x02\r\x07\x12\x04\xb0\
    \x03-/\n2\n\x04\x04\x1b\x02\x0e\x12\x04\xb2\x03\x02;\x1a$\x20Optional\
    \x20field\x20for\x20file\x20encryption\n\n\r\n\x05\x04\x1b\x02\x0e\x04\
    \x12\x04\xb2\x03\x02\n\n\r\n\x05\x04\x1b\x02\x0e\x06\x12\x04\xb2\x03\x0b\
    \"\n\r\n\x05\x04\x1b\x02\x0e\x01\x12\x04\xb2\x03#5\n\r\n\x05\x04\x1b\x02\
    \x0e\x03\x12\x04\xb2\x038:\n'\n\x04\x04\x1b\x02\x0f\x12\x04\xb4\x03\x023\
    \"\x19\x20block\x20storage\x20policy\x20id\n\n\r\n\x05\x04\x1b\x02\x0f\
    \x04\x12\x04\xb4\x03\x02\n\n\r\n\x05\x04\x1b\x02\x0f\x05\x12\x04\xb4\x03\
    \x0b\x11\n\r\n\x05\x04\x1b\x02\x0f\x01\x12\x04\xb4\x03\x12\x1f\n\r\n\x05\
    \x04\x1b\x02\x0f\x03\x12\x04\xb4\x03\"$\n\r\n\x05\x04\x1b\x02\x0f\x08\
    \x12\x04\xb4\x03%2\n\r\n\x05\x04\x1b\x02\x0f\x07\x12\x04\xb4\x0301\n1\n\
    \x04\x04\x1b\x02\x10\x12\x04\xb7\x03\x022\x1a#\x20Optional\x20field\x20f\
    or\x20erasure\x20coding\n\n\r\n\x05\x04\x1b\x02\x10\x04\x12\x04\xb7\x03\
    \x02\n\n\r\n\x05\x04\x1b\x02\x10\x06\x12\x04\xb7\x03\x0b#\n\r\n\x05\x04\
    \x1b\x02\x10\x01\x12\x04\xb7\x03$,\n\r\n\x05\x04\x1b\x02\x10\x03\x12\x04\
    \xb7\x03/1\n\x1c\n\x04\x04\x1b\x02\x11\x12\x04\xba\x03\x02+\x1a\x0e\x20S\
    et\x20of\x20flags\n\n\r\n\x05\x04\x1b\x02\x11\x04\x12\x04\xba\x03\x02\n\
    \n\r\n\x05\x04\x1b\x02\x11\x05\x12\x04\xba\x03\x0b\x11\n\r\n\x05\x04\x1b\
    \x02\x11\x01\x12\x04\xba\x03\x12\x17\n\r\n\x05\x04\x1b\x02\x11\x03\x12\
    \x04\xba\x03\x1a\x1c\n\r\n\x05\x04\x1b\x02\x11\x08\x12\x04\xba\x03\x1d*\
    \n\r\n\x05\x04\x1b\x02\x11\x07\x12\x04\xba\x03()\n\xb5\x01\n\x02\x05\x05\
    \x12\x06\xc2\x03\0\xc6\x03\x01\x1a\xa6\x01*\n\x20Checksum\x20algorithms/\
    types\x20used\x20in\x20HDFS\n\x20Make\x20sure\x20this\x20enum's\x20integ\
    er\x20values\x20match\x20enum\x20values'\x20id\x20properties\x20defined\
    \n\x20in\x20org.apache.hadoop.util.DataChecksum.Type\n\n\x0b\n\x03\x05\
    \x05\x01\x12\x04\xc2\x03\x05\x16\n\x0c\n\x04\x05\x05\x02\0\x12\x04\xc3\
    \x03\x02\x14\n\r\n\x05\x05\x05\x02\0\x01\x12\x04\xc3\x03\x02\x0f\n\r\n\
    \x05\x05\x05\x02\0\x02\x12\x04\xc3\x03\x12\x13\n\x0c\n\x04\x05\x05\x02\
    \x01\x12\x04\xc4\x03\x02\x15\n\r\n\x05\x05\x05\x02\x01\x01\x12\x04\xc4\
    \x03\x02\x10\n\r\n\x05\x05\x05\x02\x01\x02\x12\x04\xc4\x03\x13\x14\n\x0c\
    \n\x04\x05\x05\x02\x02\x12\x04\xc5\x03\x02\x16\n\r\n\x05\x05\x05\x02\x02\
    \x01\x12\x04\xc5\x03\x02\x11\n\r\n\x05\x05\x05\x02\x02\x02\x12\x04\xc5\
    \x03\x14\x15\n&\n\x02\x04\x1c\x12\x06\xcb\x03\0\xd6\x03\x01\x1a\x18*\n\
    \x20HDFS\x20Server\x20Defaults\n\n\x0b\n\x03\x04\x1c\x01\x12\x04\xcb\x03\
    \x08\x1d\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\xcc\x03\x02\x20\n\r\n\x05\x04\
    \x1c\x02\0\x04\x12\x04\xcc\x03\x02\n\n\r\n\x05\x04\x1c\x02\0\x05\x12\x04\
    \xcc\x03\x0b\x11\n\r\n\x05\x04\x1c\x02\0\x01\x12\x04\xcc\x03\x12\x1b\n\r\
    \n\x05\x04\x1c\x02\0\x03\x12\x04\xcc\x03\x1e\x1f\n\x0c\n\x04\x04\x1c\x02\
    \x01\x12\x04\xcd\x03\x02'\n\r\n\x05\x04\x1c\x02\x01\x04\x12\x04\xcd\x03\
    \x02\n\n\r\n\x05\x04\x1c\x02\x01\x05\x12\x04\xcd\x03\x0b\x11\n\r\n\x05\
    \x04\x1c\x02\x01\x01\x12\x04\xcd\x03\x12\"\n\r\n\x05\x04\x1c\x02\x01\x03\
    \x12\x04\xcd\x03%&\n\x0c\n\x04\x04\x1c\x02\x02\x12\x04\xce\x03\x02&\n\r\
    \n\x05\x04\x1c\x02\x02\x04\x12\x04\xce\x03\x02\n\n\r\n\x05\x04\x1c\x02\
    \x02\x05\x12\x04\xce\x03\x0b\x11\n\r\n\x05\x04\x1c\x02\x02\x01\x12\x04\
    \xce\x03\x12!\n\r\n\x05\x04\x1c\x02\x02\x03\x12\x04\xce\x03$%\n4\n\x04\
    \x04\x1c\x02\x03\x12\x04\xcf\x03\x02\"\"&\x20Actually\x20a\x20short\x20-\
    \x20only\x2016\x20bits\x20used\n\n\r\n\x05\x04\x1c\x02\x03\x04\x12\x04\
    \xcf\x03\x02\n\n\r\n\x05\x04\x1c\x02\x03\x05\x12\x04\xcf\x03\x0b\x11\n\r\
    \n\x05\x04\x1c\x02\x03\x01\x12\x04\xcf\x03\x12\x1d\n\r\n\x05\x04\x1c\x02\
    \x03\x03\x12\x04\xcf\x03\x20!\n\x0c\n\x04\x04\x1c\x02\x04\x12\x04\xd0\
    \x03\x02%\n\r\n\x05\x04\x1c\x02\x04\x04\x12\x04\xd0\x03\x02\n\n\r\n\x05\
    \x04\x1c\x02\x04\x05\x12\x04\xd0\x03\x0b\x11\n\r\n\x05\x04\x1c\x02\x04\
    \x01\x12\x04\xd0\x03\x12\x20\n\r\n\x05\x04\x1c\x02\x04\x03\x12\x04\xd0\
    \x03#$\n\x0c\n\x04\x04\x1c\x02\x05\x12\x04\xd1\x03\x02:\n\r\n\x05\x04\
    \x1c\x02\x05\x04\x12\x04\xd1\x03\x02\n\n\r\n\x05\x04\x1c\x02\x05\x05\x12\
    \x04\xd1\x03\x0b\x0f\n\r\n\x05\x04\x1c\x02\x05\x01\x12\x04\xd1\x03\x10#\
    \n\r\n\x05\x04\x1c\x02\x05\x03\x12\x04\xd1\x03&'\n\r\n\x05\x04\x1c\x02\
    \x05\x08\x12\x04\xd1\x03(9\n\r\n\x05\x04\x1c\x02\x05\x07\x12\x04\xd1\x03\
    38\n\x0c\n\x04\x04\x1c\x02\x06\x12\x04\xd2\x03\x022\n\r\n\x05\x04\x1c\
    \x02\x06\x04\x12\x04\xd2\x03\x02\n\n\r\n\x05\x04\x1c\x02\x06\x05\x12\x04\
    \xd2\x03\x0b\x11\n\r\n\x05\x04\x1c\x02\x06\x01\x12\x04\xd2\x03\x12\x1f\n\
    \r\n\x05\x04\x1c\x02\x06\x03\x12\x04\xd2\x03\"#\n\r\n\x05\x04\x1c\x02\
    \x06\x08\x12\x04\xd2\x03$1\n\r\n\x05\x04\x1c\x02\x06\x07\x12\x04\xd2\x03\
    /0\n\x0c\n\x04\x04\x1c\x02\x07\x12\x04\xd3\x03\x02I\n\r\n\x05\x04\x1c\
    \x02\x07\x04\x12\x04\xd3\x03\x02\n\n\r\n\x05\x04\x1c\x02\x07\x06\x12\x04\
    \xd3\x03\x0b\x1c\n\r\n\x05\x04\x1c\x02\x07\x01\x12\x04\xd3\x03\x1d)\n\r\
    \n\x05\x04\x1c\x02\x07\x03\x12\x04\xd3\x03,-\n\r\n\x05\x04\x1c\x02\x07\
    \x08\x12\x04\xd3\x03.H\n\r\n\x05\x04\x1c\x02\x07\x07\x12\x04\xd3\x039G\n\
    \x0c\n\x04\x04\x1c\x02\x08\x12\x04\xd4\x03\x02%\n\r\n\x05\x04\x1c\x02\
    \x08\x04\x12\x04\xd4\x03\x02\n\n\r\n\x05\x04\x1c\x02\x08\x05\x12\x04\xd4\
    \x03\x0b\x11\n\r\n\x05\x04\x1c\x02\x08\x01\x12\x04\xd4\x03\x12\x20\n\r\n\
    \x05\x04\x1c\x02\x08\x03\x12\x04\xd4\x03#$\n\x0c\n\x04\x04\x1c\x02\t\x12\
    \x04\xd5\x03\x02.\n\r\n\x05\x04\x1c\x02\t\x04\x12\x04\xd5\x03\x02\n\n\r\
    \n\x05\x04\x1c\x02\t\x05\x12\x04\xd5\x03\x0b\x11\n\r\n\x05\x04\x1c\x02\t\
    \x01\x12\x04\xd5\x03\x12\x1a\n\r\n\x05\x04\x1c\x02\t\x03\x12\x04\xd5\x03\
    \x1d\x1f\n\r\n\x05\x04\x1c\x02\t\x08\x12\x04\xd5\x03\x20-\n\r\n\x05\x04\
    \x1c\x02\t\x07\x12\x04\xd5\x03+,\n#\n\x02\x04\x1d\x12\x06\xdc\x03\0\xdf\
    \x03\x01\x1a\x15*\n\x20Directory\x20listing\n\n\x0b\n\x03\x04\x1d\x01\
    \x12\x04\xdc\x03\x08\x1d\n\x0c\n\x04\x04\x1d\x02\0\x12\x04\xdd\x03\x022\
    \n\r\n\x05\x04\x1d\x02\0\x04\x12\x04\xdd\x03\x02\n\n\r\n\x05\x04\x1d\x02\
    \0\x06\x12\x04\xdd\x03\x0b\x1e\n\r\n\x05\x04\x1d\x02\0\x01\x12\x04\xdd\
    \x03\x1f-\n\r\n\x05\x04\x1d\x02\0\x03\x12\x04\xdd\x0301\n\x0c\n\x04\x04\
    \x1d\x02\x01\x12\x04\xde\x03\x02(\n\r\n\x05\x04\x1d\x02\x01\x04\x12\x04\
    \xde\x03\x02\n\n\r\n\x05\x04\x1d\x02\x01\x05\x12\x04\xde\x03\x0b\x11\n\r\
    \n\x05\x04\x1d\x02\x01\x01\x12\x04\xde\x03\x12\"\n\r\n\x05\x04\x1d\x02\
    \x01\x03\x12\x04\xde\x03&'\n\xcf\x01\n\x02\x04\x1e\x12\x06\xe6\x03\0\xed\
    \x03\x01\x1a\xc0\x01*\n\x20Status\x20of\x20a\x20snapshottable\x20directo\
    ry:\x20besides\x20the\x20normal\x20information\x20for\x20\n\x20a\x20dire\
    ctory\x20status,\x20also\x20include\x20snapshot\x20quota,\x20number\x20o\
    f\x20snapshots,\x20and\n\x20the\x20full\x20path\x20of\x20the\x20parent\
    \x20directory.\x20\n\n\x0b\n\x03\x04\x1e\x01\x12\x04\xe6\x03\x08)\n\x0c\
    \n\x04\x04\x1e\x02\0\x12\x04\xe7\x03\x02-\n\r\n\x05\x04\x1e\x02\0\x04\
    \x12\x04\xe7\x03\x02\n\n\r\n\x05\x04\x1e\x02\0\x06\x12\x04\xe7\x03\x0b\
    \x1e\n\r\n\x05\x04\x1e\x02\0\x01\x12\x04\xe7\x03\x1f(\n\r\n\x05\x04\x1e\
    \x02\0\x03\x12\x04\xe7\x03+,\n;\n\x04\x04\x1e\x02\x01\x12\x04\xea\x03\
    \x02%\x1a-\x20Fields\x20specific\x20for\x20snapshottable\x20directory\n\
    \n\r\n\x05\x04\x1e\x02\x01\x04\x12\x04\xea\x03\x02\n\n\r\n\x05\x04\x1e\
    \x02\x01\x05\x12\x04\xea\x03\x0b\x11\n\r\n\x05\x04\x1e\x02\x01\x01\x12\
    \x04\xea\x03\x12\x20\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\xea\x03#$\n\
    \x0c\n\x04\x04\x1e\x02\x02\x12\x04\xeb\x03\x02&\n\r\n\x05\x04\x1e\x02\
    \x02\x04\x12\x04\xeb\x03\x02\n\n\r\n\x05\x04\x1e\x02\x02\x05\x12\x04\xeb\
    \x03\x0b\x11\n\r\n\x05\x04\x1e\x02\x02\x01\x12\x04\xeb\x03\x12!\n\r\n\
    \x05\x04\x1e\x02\x02\x03\x12\x04\xeb\x03$%\n\x0c\n\x04\x04\x1e\x02\x03\
    \x12\x04\xec\x03\x02%\n\r\n\x05\x04\x1e\x02\x03\x04\x12\x04\xec\x03\x02\
    \n\n\r\n\x05\x04\x1e\x02\x03\x05\x12\x04\xec\x03\x0b\x10\n\r\n\x05\x04\
    \x1e\x02\x03\x01\x12\x04\xec\x03\x11\x20\n\r\n\x05\x04\x1e\x02\x03\x03\
    \x12\x04\xec\x03#$\n1\n\x02\x04\x1f\x12\x06\xf2\x03\0\xf4\x03\x01\x1a#*\
    \n\x20Snapshottable\x20directory\x20listing\n\n\x0b\n\x03\x04\x1f\x01\
    \x12\x04\xf2\x03\x08*\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xf3\x03\x02I\n\r\
    \n\x05\x04\x1f\x02\0\x04\x12\x04\xf3\x03\x02\n\n\r\n\x05\x04\x1f\x02\0\
    \x06\x12\x04\xf3\x03\x0b,\n\r\n\x05\x04\x1f\x02\0\x01\x12\x04\xf3\x03-D\
    \n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xf3\x03GH\n,\n\x02\x04\x20\x12\x06\
    \xf9\x03\0\xfd\x03\x01\x1a\x1e*\n\x20Snapshot\x20diff\x20report\x20entry\
    \n\n\x0b\n\x03\x04\x20\x01\x12\x04\xf9\x03\x08$\n\x0c\n\x04\x04\x20\x02\
    \0\x12\x04\xfa\x03\x02\x1e\n\r\n\x05\x04\x20\x02\0\x04\x12\x04\xfa\x03\
    \x02\n\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\xfa\x03\x0b\x10\n\r\n\x05\x04\
    \x20\x02\0\x01\x12\x04\xfa\x03\x11\x19\n\r\n\x05\x04\x20\x02\0\x03\x12\
    \x04\xfa\x03\x1c\x1d\n\x0c\n\x04\x04\x20\x02\x01\x12\x04\xfb\x03\x02(\n\
    \r\n\x05\x04\x20\x02\x01\x04\x12\x04\xfb\x03\x02\n\n\r\n\x05\x04\x20\x02\
    \x01\x05\x12\x04\xfb\x03\x0b\x11\n\r\n\x05\x04\x20\x02\x01\x01\x12\x04\
    \xfb\x03\x12#\n\r\n\x05\x04\x20\x02\x01\x03\x12\x04\xfb\x03&'\n\x0c\n\
    \x04\x04\x20\x02\x02\x12\x04\xfc\x03\x02\x20\n\r\n\x05\x04\x20\x02\x02\
    \x04\x12\x04\xfc\x03\x02\n\n\r\n\x05\x04\x20\x02\x02\x05\x12\x04\xfc\x03\
    \x0b\x10\n\r\n\x05\x04\x20\x02\x02\x01\x12\x04\xfc\x03\x11\x1b\n\r\n\x05\
    \x04\x20\x02\x02\x03\x12\x04\xfc\x03\x1e\x1f\n&\n\x02\x04!\x12\x06\x82\
    \x04\0\x88\x04\x01\x1a\x18*\n\x20Snapshot\x20diff\x20report\n\n\x0b\n\
    \x03\x04!\x01\x12\x04\x82\x04\x08\x1f\nE\n\x04\x04!\x02\0\x12\x04\x84\
    \x04\x02#\x1a7\x20full\x20path\x20of\x20the\x20directory\x20where\x20sna\
    pshots\x20were\x20taken\n\n\r\n\x05\x04!\x02\0\x04\x12\x04\x84\x04\x02\n\
    \n\r\n\x05\x04!\x02\0\x05\x12\x04\x84\x04\x0b\x11\n\r\n\x05\x04!\x02\0\
    \x01\x12\x04\x84\x04\x12\x1e\n\r\n\x05\x04!\x02\0\x03\x12\x04\x84\x04!\"\
    \n\x0c\n\x04\x04!\x02\x01\x12\x04\x85\x04\x02#\n\r\n\x05\x04!\x02\x01\
    \x04\x12\x04\x85\x04\x02\n\n\r\n\x05\x04!\x02\x01\x05\x12\x04\x85\x04\
    \x0b\x11\n\r\n\x05\x04!\x02\x01\x01\x12\x04\x85\x04\x12\x1e\n\r\n\x05\
    \x04!\x02\x01\x03\x12\x04\x85\x04!\"\n\x0c\n\x04\x04!\x02\x02\x12\x04\
    \x86\x04\x02!\n\r\n\x05\x04!\x02\x02\x04\x12\x04\x86\x04\x02\n\n\r\n\x05\
    \x04!\x02\x02\x05\x12\x04\x86\x04\x0b\x11\n\r\n\x05\x04!\x02\x02\x01\x12\
    \x04\x86\x04\x12\x1c\n\r\n\x05\x04!\x02\x02\x03\x12\x04\x86\x04\x1f\x20\
    \n\x0c\n\x04\x04!\x02\x03\x12\x04\x87\x04\x02>\n\r\n\x05\x04!\x02\x03\
    \x04\x12\x04\x87\x04\x02\n\n\r\n\x05\x04!\x02\x03\x06\x12\x04\x87\x04\
    \x0b'\n\r\n\x05\x04!\x02\x03\x01\x12\x04\x87\x04(9\n\r\n\x05\x04!\x02\
    \x03\x03\x12\x04\x87\x04<=\n\xa1\x02\n\x02\x04\"\x12\x06\x93\x04\0\x97\
    \x04\x01\x1a\x92\x02*\n\x20Block\x20information\n\n\x20Please\x20be\x20w\
    ary\x20of\x20adding\x20additional\x20fields\x20here,\x20since\x20INodeFi\
    les\n\x20need\x20to\x20fit\x20in\x20PB's\x20default\x20max\x20message\
    \x20size\x20of\x2064MB.\n\x20We\x20restrict\x20the\x20max\x20#\x20of\x20\
    blocks\x20per\x20file\n\x20(dfs.namenode.fs-limits.max-blocks-per-file),\
    \x20but\x20it's\x20better\n\x20to\x20avoid\x20changing\x20this.\n\n\x0b\
    \n\x03\x04\"\x01\x12\x04\x93\x04\x08\x12\n\x0c\n\x04\x04\"\x02\0\x12\x04\
    \x94\x04\x02\x1e\n\r\n\x05\x04\"\x02\0\x04\x12\x04\x94\x04\x02\n\n\r\n\
    \x05\x04\"\x02\0\x05\x12\x04\x94\x04\x0b\x11\n\r\n\x05\x04\"\x02\0\x01\
    \x12\x04\x94\x04\x12\x19\n\r\n\x05\x04\"\x02\0\x03\x12\x04\x94\x04\x1c\
    \x1d\n\x0c\n\x04\x04\"\x02\x01\x12\x04\x95\x04\x02\x1f\n\r\n\x05\x04\"\
    \x02\x01\x04\x12\x04\x95\x04\x02\n\n\r\n\x05\x04\"\x02\x01\x05\x12\x04\
    \x95\x04\x0b\x11\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\x95\x04\x12\x1a\n\r\
    \n\x05\x04\"\x02\x01\x03\x12\x04\x95\x04\x1d\x1e\n\x0c\n\x04\x04\"\x02\
    \x02\x12\x04\x96\x04\x02-\n\r\n\x05\x04\"\x02\x02\x04\x12\x04\x96\x04\
    \x02\n\n\r\n\x05\x04\"\x02\x02\x05\x12\x04\x96\x04\x0b\x11\n\r\n\x05\x04\
    \"\x02\x02\x01\x12\x04\x96\x04\x12\x1a\n\r\n\x05\x04\"\x02\x02\x03\x12\
    \x04\x96\x04\x1d\x1e\n\r\n\x05\x04\"\x02\x02\x08\x12\x04\x96\x04\x1f,\n\
    \r\n\x05\x04\"\x02\x02\x07\x12\x04\x96\x04*+\nO\n\x02\x04#\x12\x06\x9d\
    \x04\0\xa5\x04\x01\x1aA*\n\x20Information\x20related\x20to\x20a\x20snaps\
    hot\n\x20TODO:\x20add\x20more\x20information\n\n\x0b\n\x03\x04#\x01\x12\
    \x04\x9d\x04\x08\x19\n\x0c\n\x04\x04#\x02\0\x12\x04\x9e\x04\x02#\n\r\n\
    \x05\x04#\x02\0\x04\x12\x04\x9e\x04\x02\n\n\r\n\x05\x04#\x02\0\x05\x12\
    \x04\x9e\x04\x0b\x11\n\r\n\x05\x04#\x02\0\x01\x12\x04\x9e\x04\x12\x1e\n\
    \r\n\x05\x04#\x02\0\x03\x12\x04\x9e\x04!\"\n\x0c\n\x04\x04#\x02\x01\x12\
    \x04\x9f\x04\x02#\n\r\n\x05\x04#\x02\x01\x04\x12\x04\x9f\x04\x02\n\n\r\n\
    \x05\x04#\x02\x01\x05\x12\x04\x9f\x04\x0b\x11\n\r\n\x05\x04#\x02\x01\x01\
    \x12\x04\x9f\x04\x12\x1e\n\r\n\x05\x04#\x02\x01\x03\x12\x04\x9f\x04!\"\n\
    \x0c\n\x04\x04#\x02\x02\x12\x04\xa0\x04\x02,\n\r\n\x05\x04#\x02\x02\x04\
    \x12\x04\xa0\x04\x02\n\n\r\n\x05\x04#\x02\x02\x06\x12\x04\xa0\x04\x0b\
    \x1c\n\r\n\x05\x04#\x02\x02\x01\x12\x04\xa0\x04\x1d'\n\r\n\x05\x04#\x02\
    \x02\x03\x12\x04\xa0\x04*+\n\x0c\n\x04\x04#\x02\x03\x12\x04\xa1\x04\x02\
    \x1c\n\r\n\x05\x04#\x02\x03\x04\x12\x04\xa1\x04\x02\n\n\r\n\x05\x04#\x02\
    \x03\x05\x12\x04\xa1\x04\x0b\x11\n\r\n\x05\x04#\x02\x03\x01\x12\x04\xa1\
    \x04\x12\x17\n\r\n\x05\x04#\x02\x03\x03\x12\x04\xa1\x04\x1a\x1b\n\x0c\n\
    \x04\x04#\x02\x04\x12\x04\xa2\x04\x02\x1c\n\r\n\x05\x04#\x02\x04\x04\x12\
    \x04\xa2\x04\x02\n\n\r\n\x05\x04#\x02\x04\x05\x12\x04\xa2\x04\x0b\x11\n\
    \r\n\x05\x04#\x02\x04\x01\x12\x04\xa2\x04\x12\x17\n\r\n\x05\x04#\x02\x04\
    \x03\x12\x04\xa2\x04\x1a\x1b\n-\n\x04\x04#\x02\x05\x12\x04\xa3\x04\x02!\
    \"\x1f\x20TODO:\x20do\x20we\x20need\x20access\x20time?\n\n\r\n\x05\x04#\
    \x02\x05\x04\x12\x04\xa3\x04\x02\n\n\r\n\x05\x04#\x02\x05\x05\x12\x04\
    \xa3\x04\x0b\x11\n\r\n\x05\x04#\x02\x05\x01\x12\x04\xa3\x04\x12\x1c\n\r\
    \n\x05\x04#\x02\x05\x03\x12\x04\xa3\x04\x1f\x20\n(\n\x02\x04$\x12\x06\
    \xaa\x04\0\xad\x04\x01\x1a\x1a*\n\x20Rolling\x20upgrade\x20status\n\n\
    \x0b\n\x03\x04$\x01\x12\x04\xaa\x04\x08!\n\x0c\n\x04\x04$\x02\0\x12\x04\
    \xab\x04\x02\"\n\r\n\x05\x04$\x02\0\x04\x12\x04\xab\x04\x02\n\n\r\n\x05\
    \x04$\x02\0\x05\x12\x04\xab\x04\x0b\x11\n\r\n\x05\x04$\x02\0\x01\x12\x04\
    \xab\x04\x12\x1d\n\r\n\x05\x04$\x02\0\x03\x12\x04\xab\x04\x20!\n\x0c\n\
    \x04\x04$\x02\x01\x12\x04\xac\x04\x020\n\r\n\x05\x04$\x02\x01\x04\x12\
    \x04\xac\x04\x02\n\n\r\n\x05\x04$\x02\x01\x05\x12\x04\xac\x04\x0b\x0f\n\
    \r\n\x05\x04$\x02\x01\x01\x12\x04\xac\x04\x10\x19\n\r\n\x05\x04$\x02\x01\
    \x03\x12\x04\xac\x04\x1c\x1d\n\r\n\x05\x04$\x02\x01\x08\x12\x04\xac\x04\
    \x1e/\n\r\n\x05\x04$\x02\x01\x07\x12\x04\xac\x04).\n(\n\x02\x04%\x12\x06\
    \xb3\x04\0\xb5\x04\x01\x1a\x1a*\n\x20A\x20list\x20of\x20storage\x20IDs.\
    \n\n\x0b\n\x03\x04%\x01\x12\x04\xb3\x04\x08\x19\n\x0c\n\x04\x04%\x02\0\
    \x12\x04\xb4\x04\x02#\n\r\n\x05\x04%\x02\0\x04\x12\x04\xb4\x04\x02\n\n\r\
    \n\x05\x04%\x02\0\x05\x12\x04\xb4\x04\x0b\x11\n\r\n\x05\x04%\x02\0\x01\
    \x12\x04\xb4\x04\x12\x1e\n\r\n\x05\x04%\x02\0\x03\x12\x04\xb4\x04!\"\n/\
    \n\x02\x05\x06\x12\x06\xba\x04\0\xbf\x04\x01\x1a!*\n\x20File\x20access\
    \x20permissions\x20mode.\n\n\x0b\n\x03\x05\x06\x01\x12\x04\xba\x04\x05\
    \x14\n\x0c\n\x04\x05\x06\x02\0\x12\x04\xbb\x04\x04\r\n\r\n\x05\x05\x06\
    \x02\0\x01\x12\x04\xbb\x04\x04\x08\n\r\n\x05\x05\x06\x02\0\x02\x12\x04\
    \xbb\x04\x0b\x0c\n\x0c\n\x04\x05\x06\x02\x01\x12\x04\xbc\x04\x04\x0e\n\r\
    \n\x05\x05\x06\x02\x01\x01\x12\x04\xbc\x04\x04\t\n\r\n\x05\x05\x06\x02\
    \x01\x02\x12\x04\xbc\x04\x0c\r\n\x0c\n\x04\x05\x06\x02\x02\x12\x04\xbd\
    \x04\x04\r\n\r\n\x05\x05\x06\x02\x02\x01\x12\x04\xbd\x04\x04\x08\n\r\n\
    \x05\x05\x06\x02\x02\x02\x12\x04\xbd\x04\x0b\x0c\n\x0c\n\x04\x05\x06\x02\
    \x03\x12\x04\xbe\x04\x04\x10\n\r\n\x05\x05\x06\x02\x03\x01\x12\x04\xbe\
    \x04\x04\x0b\n\r\n\x05\x05\x06\x02\x03\x02\x12\x04\xbe\x04\x0e\x0f\n\xda\
    \x05\n\x02\x04&\x12\x06\xcf\x04\0\xd8\x04\x01\x1a\xcb\x05*\n\x20Secret\
    \x20information\x20for\x20the\x20BlockKeyProto.\x20This\x20is\x20not\x20\
    sent\x20on\x20the\x20wire\x20as\n\x20such\x20but\x20is\x20used\x20to\x20\
    pack\x20a\x20byte\x20array\x20and\x20encrypted\x20and\x20put\x20in\n\x20\
    BlockKeyProto.bytes\n\x20When\x20adding\x20further\x20fields,\x20make\
    \x20sure\x20they\x20are\x20optional\x20as\x20they\x20would\n\x20otherwis\
    e\x20not\x20be\x20backwards\x20compatible.\n\n\x20Note:\x20As\x20part\
    \x20of\x20the\x20migration\x20from\x20WritableUtils\x20based\x20tokens\
    \x20(aka\x20\"legacy\")\n\x20to\x20Protocol\x20Buffers,\x20we\x20use\x20\
    the\x20first\x20byte\x20to\x20determine\x20the\x20type.\x20If\x20the\n\
    \x20first\x20byte\x20is\x20<=0\x20then\x20it\x20is\x20a\x20legacy\x20tok\
    en.\x20This\x20means\x20that\x20when\x20using\n\x20protobuf\x20tokens,\
    \x20the\x20the\x20first\x20field\x20sent\x20must\x20have\x20a\x20`field_\
    number`\x20less\n\x20than\x2016\x20to\x20make\x20sure\x20that\x20the\x20\
    first\x20byte\x20is\x20positive.\x20Otherwise\x20it\x20could\x20be\n\x20\
    parsed\x20as\x20a\x20legacy\x20token.\x20See\x20HDFS-11026\x20for\x20mor\
    e\x20discussion.\n\n\x0b\n\x03\x04&\x01\x12\x04\xcf\x04\x08\x1d\n\x0c\n\
    \x04\x04&\x02\0\x12\x04\xd0\x04\x02!\n\r\n\x05\x04&\x02\0\x04\x12\x04\
    \xd0\x04\x02\n\n\r\n\x05\x04&\x02\0\x05\x12\x04\xd0\x04\x0b\x11\n\r\n\
    \x05\x04&\x02\0\x01\x12\x04\xd0\x04\x12\x1c\n\r\n\x05\x04&\x02\0\x03\x12\
    \x04\xd0\x04\x1f\x20\n\x0c\n\x04\x04&\x02\x01\x12\x04\xd1\x04\x02\x1c\n\
    \r\n\x05\x04&\x02\x01\x04\x12\x04\xd1\x04\x02\n\n\r\n\x05\x04&\x02\x01\
    \x05\x12\x04\xd1\x04\x0b\x11\n\r\n\x05\x04&\x02\x01\x01\x12\x04\xd1\x04\
    \x12\x17\n\r\n\x05\x04&\x02\x01\x03\x12\x04\xd1\x04\x1a\x1b\n\x0c\n\x04\
    \x04&\x02\x02\x12\x04\xd2\x04\x02\x1d\n\r\n\x05\x04&\x02\x02\x04\x12\x04\
    \xd2\x04\x02\n\n\r\n\x05\x04&\x02\x02\x05\x12\x04\xd2\x04\x0b\x11\n\r\n\
    \x05\x04&\x02\x02\x01\x12\x04\xd2\x04\x12\x18\n\r\n\x05\x04&\x02\x02\x03\
    \x12\x04\xd2\x04\x1b\x1c\n\x0c\n\x04\x04&\x02\x03\x12\x04\xd3\x04\x02\"\
    \n\r\n\x05\x04&\x02\x03\x04\x12\x04\xd3\x04\x02\n\n\r\n\x05\x04&\x02\x03\
    \x05\x12\x04\xd3\x04\x0b\x11\n\r\n\x05\x04&\x02\x03\x01\x12\x04\xd3\x04\
    \x12\x1d\n\r\n\x05\x04&\x02\x03\x03\x12\x04\xd3\x04\x20!\n\x0c\n\x04\x04\
    &\x02\x04\x12\x04\xd4\x04\x02\x1e\n\r\n\x05\x04&\x02\x04\x04\x12\x04\xd4\
    \x04\x02\n\n\r\n\x05\x04&\x02\x04\x05\x12\x04\xd4\x04\x0b\x11\n\r\n\x05\
    \x04&\x02\x04\x01\x12\x04\xd4\x04\x12\x19\n\r\n\x05\x04&\x02\x04\x03\x12\
    \x04\xd4\x04\x1c\x1d\n\x0c\n\x04\x04&\x02\x05\x12\x04\xd5\x04\x02%\n\r\n\
    \x05\x04&\x02\x05\x04\x12\x04\xd5\x04\x02\n\n\r\n\x05\x04&\x02\x05\x06\
    \x12\x04\xd5\x04\x0b\x1a\n\r\n\x05\x04&\x02\x05\x01\x12\x04\xd5\x04\x1b\
    \x20\n\r\n\x05\x04&\x02\x05\x03\x12\x04\xd5\x04#$\n\x0c\n\x04\x04&\x02\
    \x06\x12\x04\xd6\x04\x02-\n\r\n\x05\x04&\x02\x06\x04\x12\x04\xd6\x04\x02\
    \n\n\r\n\x05\x04&\x02\x06\x06\x12\x04\xd6\x04\x0b\x1b\n\r\n\x05\x04&\x02\
    \x06\x01\x12\x04\xd6\x04\x1c(\n\r\n\x05\x04&\x02\x06\x03\x12\x04\xd6\x04\
    +,\n\x0c\n\x04\x04&\x02\x07\x12\x04\xd7\x04\x02!\n\r\n\x05\x04&\x02\x07\
    \x04\x12\x04\xd7\x04\x02\n\n\r\n\x05\x04&\x02\x07\x05\x12\x04\xd7\x04\
    \x0b\x11\n\r\n\x05\x04&\x02\x07\x01\x12\x04\xd7\x04\x12\x1c\n\r\n\x05\
    \x04&\x02\x07\x03\x12\x04\xd7\x04\x1f\x20\
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
