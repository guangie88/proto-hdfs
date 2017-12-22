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
pub struct GetReplicaVisibleLengthRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReplicaVisibleLengthRequestProto {}

impl GetReplicaVisibleLengthRequestProto {
    pub fn new() -> GetReplicaVisibleLengthRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthRequestProto,
        };
        unsafe {
            instance.get(GetReplicaVisibleLengthRequestProto::new)
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
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

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthRequestProto {
    fn new() -> GetReplicaVisibleLengthRequestProto {
        GetReplicaVisibleLengthRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetReplicaVisibleLengthRequestProto::get_block_for_reflect,
                    GetReplicaVisibleLengthRequestProto::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthRequestProto>(
                    "GetReplicaVisibleLengthRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplicaVisibleLengthRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReplicaVisibleLengthResponseProto {
    // message fields
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReplicaVisibleLengthResponseProto {}

impl GetReplicaVisibleLengthResponseProto {
    pub fn new() -> GetReplicaVisibleLengthResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthResponseProto,
        };
        unsafe {
            instance.get(GetReplicaVisibleLengthResponseProto::new)
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthResponseProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
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
                    self.length = ::std::option::Option::Some(tmp);
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthResponseProto {
    fn new() -> GetReplicaVisibleLengthResponseProto {
        GetReplicaVisibleLengthResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    GetReplicaVisibleLengthResponseProto::get_length_for_reflect,
                    GetReplicaVisibleLengthResponseProto::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthResponseProto>(
                    "GetReplicaVisibleLengthResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthResponseProto {
    fn clear(&mut self) {
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplicaVisibleLengthResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshNamenodesRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshNamenodesRequestProto {}

impl RefreshNamenodesRequestProto {
    pub fn new() -> RefreshNamenodesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesRequestProto,
        };
        unsafe {
            instance.get(RefreshNamenodesRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesRequestProto {
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

impl ::protobuf::MessageStatic for RefreshNamenodesRequestProto {
    fn new() -> RefreshNamenodesRequestProto {
        RefreshNamenodesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesRequestProto>(
                    "RefreshNamenodesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshNamenodesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshNamenodesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshNamenodesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshNamenodesResponseProto {}

impl RefreshNamenodesResponseProto {
    pub fn new() -> RefreshNamenodesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesResponseProto,
        };
        unsafe {
            instance.get(RefreshNamenodesResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesResponseProto {
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

impl ::protobuf::MessageStatic for RefreshNamenodesResponseProto {
    fn new() -> RefreshNamenodesResponseProto {
        RefreshNamenodesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesResponseProto>(
                    "RefreshNamenodesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshNamenodesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshNamenodesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteBlockPoolRequestProto {
    // message fields
    blockPool: ::protobuf::SingularField<::std::string::String>,
    force: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteBlockPoolRequestProto {}

impl DeleteBlockPoolRequestProto {
    pub fn new() -> DeleteBlockPoolRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolRequestProto,
        };
        unsafe {
            instance.get(DeleteBlockPoolRequestProto::new)
        }
    }

    // required string blockPool = 1;

    pub fn clear_blockPool(&mut self) {
        self.blockPool.clear();
    }

    pub fn has_blockPool(&self) -> bool {
        self.blockPool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPool(&mut self, v: ::std::string::String) {
        self.blockPool = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPool(&mut self) -> &mut ::std::string::String {
        if self.blockPool.is_none() {
            self.blockPool.set_default();
        }
        self.blockPool.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPool(&mut self) -> ::std::string::String {
        self.blockPool.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPool(&self) -> &str {
        match self.blockPool.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPool_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPool
    }

    fn mut_blockPool_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPool
    }

    // required bool force = 2;

    pub fn clear_force(&mut self) {
        self.force = ::std::option::Option::None;
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: bool) {
        self.force = ::std::option::Option::Some(v);
    }

    pub fn get_force(&self) -> bool {
        self.force.unwrap_or(false)
    }

    fn get_force_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.force
    }

    fn mut_force_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.force
    }
}

impl ::protobuf::Message for DeleteBlockPoolRequestProto {
    fn is_initialized(&self) -> bool {
        if self.blockPool.is_none() {
            return false;
        }
        if self.force.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPool)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.force = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.blockPool.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.force {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPool.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.force {
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

impl ::protobuf::MessageStatic for DeleteBlockPoolRequestProto {
    fn new() -> DeleteBlockPoolRequestProto {
        DeleteBlockPoolRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPool",
                    DeleteBlockPoolRequestProto::get_blockPool_for_reflect,
                    DeleteBlockPoolRequestProto::mut_blockPool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "force",
                    DeleteBlockPoolRequestProto::get_force_for_reflect,
                    DeleteBlockPoolRequestProto::mut_force_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolRequestProto>(
                    "DeleteBlockPoolRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolRequestProto {
    fn clear(&mut self) {
        self.clear_blockPool();
        self.clear_force();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteBlockPoolRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteBlockPoolResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteBlockPoolResponseProto {}

impl DeleteBlockPoolResponseProto {
    pub fn new() -> DeleteBlockPoolResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolResponseProto,
        };
        unsafe {
            instance.get(DeleteBlockPoolResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DeleteBlockPoolResponseProto {
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

impl ::protobuf::MessageStatic for DeleteBlockPoolResponseProto {
    fn new() -> DeleteBlockPoolResponseProto {
        DeleteBlockPoolResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolResponseProto>(
                    "DeleteBlockPoolResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteBlockPoolResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockLocalPathInfoRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<security::TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockLocalPathInfoRequestProto {}

impl GetBlockLocalPathInfoRequestProto {
    pub fn new() -> GetBlockLocalPathInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoRequestProto,
        };
        unsafe {
            instance.get(GetBlockLocalPathInfoRequestProto::new)
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

    // required .hadoop.common.TokenProto token = 2;

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
}

impl ::protobuf::Message for GetBlockLocalPathInfoRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.token.is_none() {
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

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoRequestProto {
    fn new() -> GetBlockLocalPathInfoRequestProto {
        GetBlockLocalPathInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetBlockLocalPathInfoRequestProto::get_block_for_reflect,
                    GetBlockLocalPathInfoRequestProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<security::TokenProto>>(
                    "token",
                    GetBlockLocalPathInfoRequestProto::get_token_for_reflect,
                    GetBlockLocalPathInfoRequestProto::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoRequestProto>(
                    "GetBlockLocalPathInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockLocalPathInfoRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockLocalPathInfoResponseProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    localPath: ::protobuf::SingularField<::std::string::String>,
    localMetaPath: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockLocalPathInfoResponseProto {}

impl GetBlockLocalPathInfoResponseProto {
    pub fn new() -> GetBlockLocalPathInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoResponseProto,
        };
        unsafe {
            instance.get(GetBlockLocalPathInfoResponseProto::new)
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

    // required string localPath = 2;

    pub fn clear_localPath(&mut self) {
        self.localPath.clear();
    }

    pub fn has_localPath(&self) -> bool {
        self.localPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localPath(&mut self, v: ::std::string::String) {
        self.localPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localPath(&mut self) -> &mut ::std::string::String {
        if self.localPath.is_none() {
            self.localPath.set_default();
        }
        self.localPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localPath(&mut self) -> ::std::string::String {
        self.localPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localPath(&self) -> &str {
        match self.localPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_localPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.localPath
    }

    fn mut_localPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.localPath
    }

    // required string localMetaPath = 3;

    pub fn clear_localMetaPath(&mut self) {
        self.localMetaPath.clear();
    }

    pub fn has_localMetaPath(&self) -> bool {
        self.localMetaPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localMetaPath(&mut self, v: ::std::string::String) {
        self.localMetaPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localMetaPath(&mut self) -> &mut ::std::string::String {
        if self.localMetaPath.is_none() {
            self.localMetaPath.set_default();
        }
        self.localMetaPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localMetaPath(&mut self) -> ::std::string::String {
        self.localMetaPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localMetaPath(&self) -> &str {
        match self.localMetaPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_localMetaPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.localMetaPath
    }

    fn mut_localMetaPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.localMetaPath
    }
}

impl ::protobuf::Message for GetBlockLocalPathInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.localPath.is_none() {
            return false;
        }
        if self.localMetaPath.is_none() {
            return false;
        }
        for v in &self.block {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.localPath)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.localMetaPath)?;
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
        if let Some(ref v) = self.localPath.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.localMetaPath.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
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
        if let Some(ref v) = self.localPath.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.localMetaPath.as_ref() {
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

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoResponseProto {
    fn new() -> GetBlockLocalPathInfoResponseProto {
        GetBlockLocalPathInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetBlockLocalPathInfoResponseProto::get_block_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "localPath",
                    GetBlockLocalPathInfoResponseProto::get_localPath_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_localPath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "localMetaPath",
                    GetBlockLocalPathInfoResponseProto::get_localMetaPath_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_localMetaPath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoResponseProto>(
                    "GetBlockLocalPathInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoResponseProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_localPath();
        self.clear_localMetaPath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockLocalPathInfoResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownDatanodeRequestProto {
    // message fields
    forUpgrade: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownDatanodeRequestProto {}

impl ShutdownDatanodeRequestProto {
    pub fn new() -> ShutdownDatanodeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeRequestProto,
        };
        unsafe {
            instance.get(ShutdownDatanodeRequestProto::new)
        }
    }

    // required bool forUpgrade = 1;

    pub fn clear_forUpgrade(&mut self) {
        self.forUpgrade = ::std::option::Option::None;
    }

    pub fn has_forUpgrade(&self) -> bool {
        self.forUpgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forUpgrade(&mut self, v: bool) {
        self.forUpgrade = ::std::option::Option::Some(v);
    }

    pub fn get_forUpgrade(&self) -> bool {
        self.forUpgrade.unwrap_or(false)
    }

    fn get_forUpgrade_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.forUpgrade
    }

    fn mut_forUpgrade_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.forUpgrade
    }
}

impl ::protobuf::Message for ShutdownDatanodeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.forUpgrade.is_none() {
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
                    let tmp = is.read_bool()?;
                    self.forUpgrade = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.forUpgrade {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.forUpgrade {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for ShutdownDatanodeRequestProto {
    fn new() -> ShutdownDatanodeRequestProto {
        ShutdownDatanodeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "forUpgrade",
                    ShutdownDatanodeRequestProto::get_forUpgrade_for_reflect,
                    ShutdownDatanodeRequestProto::mut_forUpgrade_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeRequestProto>(
                    "ShutdownDatanodeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeRequestProto {
    fn clear(&mut self) {
        self.clear_forUpgrade();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownDatanodeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownDatanodeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownDatanodeResponseProto {}

impl ShutdownDatanodeResponseProto {
    pub fn new() -> ShutdownDatanodeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeResponseProto,
        };
        unsafe {
            instance.get(ShutdownDatanodeResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ShutdownDatanodeResponseProto {
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

impl ::protobuf::MessageStatic for ShutdownDatanodeResponseProto {
    fn new() -> ShutdownDatanodeResponseProto {
        ShutdownDatanodeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeResponseProto>(
                    "ShutdownDatanodeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownDatanodeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EvictWritersRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EvictWritersRequestProto {}

impl EvictWritersRequestProto {
    pub fn new() -> EvictWritersRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EvictWritersRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<EvictWritersRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvictWritersRequestProto,
        };
        unsafe {
            instance.get(EvictWritersRequestProto::new)
        }
    }
}

impl ::protobuf::Message for EvictWritersRequestProto {
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

impl ::protobuf::MessageStatic for EvictWritersRequestProto {
    fn new() -> EvictWritersRequestProto {
        EvictWritersRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EvictWritersRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EvictWritersRequestProto>(
                    "EvictWritersRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EvictWritersRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EvictWritersRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvictWritersRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EvictWritersResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EvictWritersResponseProto {}

impl EvictWritersResponseProto {
    pub fn new() -> EvictWritersResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EvictWritersResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<EvictWritersResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvictWritersResponseProto,
        };
        unsafe {
            instance.get(EvictWritersResponseProto::new)
        }
    }
}

impl ::protobuf::Message for EvictWritersResponseProto {
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

impl ::protobuf::MessageStatic for EvictWritersResponseProto {
    fn new() -> EvictWritersResponseProto {
        EvictWritersResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EvictWritersResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EvictWritersResponseProto>(
                    "EvictWritersResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EvictWritersResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EvictWritersResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvictWritersResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDatanodeInfoRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDatanodeInfoRequestProto {}

impl GetDatanodeInfoRequestProto {
    pub fn new() -> GetDatanodeInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoRequestProto,
        };
        unsafe {
            instance.get(GetDatanodeInfoRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetDatanodeInfoRequestProto {
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

impl ::protobuf::MessageStatic for GetDatanodeInfoRequestProto {
    fn new() -> GetDatanodeInfoRequestProto {
        GetDatanodeInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoRequestProto>(
                    "GetDatanodeInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDatanodeInfoRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDatanodeInfoResponseProto {
    // message fields
    localInfo: ::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDatanodeInfoResponseProto {}

impl GetDatanodeInfoResponseProto {
    pub fn new() -> GetDatanodeInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoResponseProto,
        };
        unsafe {
            instance.get(GetDatanodeInfoResponseProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeLocalInfoProto localInfo = 1;

    pub fn clear_localInfo(&mut self) {
        self.localInfo.clear();
    }

    pub fn has_localInfo(&self) -> bool {
        self.localInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localInfo(&mut self, v: super::hdfs::DatanodeLocalInfoProto) {
        self.localInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localInfo(&mut self) -> &mut super::hdfs::DatanodeLocalInfoProto {
        if self.localInfo.is_none() {
            self.localInfo.set_default();
        }
        self.localInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_localInfo(&mut self) -> super::hdfs::DatanodeLocalInfoProto {
        self.localInfo.take().unwrap_or_else(|| super::hdfs::DatanodeLocalInfoProto::new())
    }

    pub fn get_localInfo(&self) -> &super::hdfs::DatanodeLocalInfoProto {
        self.localInfo.as_ref().unwrap_or_else(|| super::hdfs::DatanodeLocalInfoProto::default_instance())
    }

    fn get_localInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto> {
        &self.localInfo
    }

    fn mut_localInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto> {
        &mut self.localInfo
    }
}

impl ::protobuf::Message for GetDatanodeInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.localInfo.is_none() {
            return false;
        }
        for v in &self.localInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.localInfo)?;
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
        if let Some(ref v) = self.localInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.localInfo.as_ref() {
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

impl ::protobuf::MessageStatic for GetDatanodeInfoResponseProto {
    fn new() -> GetDatanodeInfoResponseProto {
        GetDatanodeInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeLocalInfoProto>>(
                    "localInfo",
                    GetDatanodeInfoResponseProto::get_localInfo_for_reflect,
                    GetDatanodeInfoResponseProto::mut_localInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoResponseProto>(
                    "GetDatanodeInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoResponseProto {
    fn clear(&mut self) {
        self.clear_localInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDatanodeInfoResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVolumeReportRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVolumeReportRequestProto {}

impl GetVolumeReportRequestProto {
    pub fn new() -> GetVolumeReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVolumeReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetVolumeReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVolumeReportRequestProto,
        };
        unsafe {
            instance.get(GetVolumeReportRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetVolumeReportRequestProto {
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

impl ::protobuf::MessageStatic for GetVolumeReportRequestProto {
    fn new() -> GetVolumeReportRequestProto {
        GetVolumeReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVolumeReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetVolumeReportRequestProto>(
                    "GetVolumeReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVolumeReportRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVolumeReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVolumeReportRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetVolumeReportResponseProto {
    // message fields
    volumeInfo: ::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVolumeReportResponseProto {}

impl GetVolumeReportResponseProto {
    pub fn new() -> GetVolumeReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVolumeReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetVolumeReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVolumeReportResponseProto,
        };
        unsafe {
            instance.get(GetVolumeReportResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.DatanodeVolumeInfoProto volumeInfo = 1;

    pub fn clear_volumeInfo(&mut self) {
        self.volumeInfo.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeInfo(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto>) {
        self.volumeInfo = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeInfo(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto> {
        &mut self.volumeInfo
    }

    // Take field
    pub fn take_volumeInfo(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto> {
        ::std::mem::replace(&mut self.volumeInfo, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumeInfo(&self) -> &[super::hdfs::DatanodeVolumeInfoProto] {
        &self.volumeInfo
    }

    fn get_volumeInfo_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto> {
        &self.volumeInfo
    }

    fn mut_volumeInfo_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeVolumeInfoProto> {
        &mut self.volumeInfo
    }
}

impl ::protobuf::Message for GetVolumeReportResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.volumeInfo {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.volumeInfo)?;
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
        for value in &self.volumeInfo {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.volumeInfo {
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

impl ::protobuf::MessageStatic for GetVolumeReportResponseProto {
    fn new() -> GetVolumeReportResponseProto {
        GetVolumeReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVolumeReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeVolumeInfoProto>>(
                    "volumeInfo",
                    GetVolumeReportResponseProto::get_volumeInfo_for_reflect,
                    GetVolumeReportResponseProto::mut_volumeInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVolumeReportResponseProto>(
                    "GetVolumeReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVolumeReportResponseProto {
    fn clear(&mut self) {
        self.clear_volumeInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetVolumeReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetVolumeReportResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TriggerBlockReportRequestProto {
    // message fields
    incremental: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TriggerBlockReportRequestProto {}

impl TriggerBlockReportRequestProto {
    pub fn new() -> TriggerBlockReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TriggerBlockReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<TriggerBlockReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TriggerBlockReportRequestProto,
        };
        unsafe {
            instance.get(TriggerBlockReportRequestProto::new)
        }
    }

    // required bool incremental = 1;

    pub fn clear_incremental(&mut self) {
        self.incremental = ::std::option::Option::None;
    }

    pub fn has_incremental(&self) -> bool {
        self.incremental.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incremental(&mut self, v: bool) {
        self.incremental = ::std::option::Option::Some(v);
    }

    pub fn get_incremental(&self) -> bool {
        self.incremental.unwrap_or(false)
    }

    fn get_incremental_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.incremental
    }

    fn mut_incremental_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.incremental
    }
}

impl ::protobuf::Message for TriggerBlockReportRequestProto {
    fn is_initialized(&self) -> bool {
        if self.incremental.is_none() {
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
                    let tmp = is.read_bool()?;
                    self.incremental = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.incremental {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.incremental {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for TriggerBlockReportRequestProto {
    fn new() -> TriggerBlockReportRequestProto {
        TriggerBlockReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TriggerBlockReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "incremental",
                    TriggerBlockReportRequestProto::get_incremental_for_reflect,
                    TriggerBlockReportRequestProto::mut_incremental_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TriggerBlockReportRequestProto>(
                    "TriggerBlockReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TriggerBlockReportRequestProto {
    fn clear(&mut self) {
        self.clear_incremental();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TriggerBlockReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TriggerBlockReportRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TriggerBlockReportResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TriggerBlockReportResponseProto {}

impl TriggerBlockReportResponseProto {
    pub fn new() -> TriggerBlockReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TriggerBlockReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<TriggerBlockReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TriggerBlockReportResponseProto,
        };
        unsafe {
            instance.get(TriggerBlockReportResponseProto::new)
        }
    }
}

impl ::protobuf::Message for TriggerBlockReportResponseProto {
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

impl ::protobuf::MessageStatic for TriggerBlockReportResponseProto {
    fn new() -> TriggerBlockReportResponseProto {
        TriggerBlockReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TriggerBlockReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TriggerBlockReportResponseProto>(
                    "TriggerBlockReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TriggerBlockReportResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TriggerBlockReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TriggerBlockReportResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBalancerBandwidthRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBalancerBandwidthRequestProto {}

impl GetBalancerBandwidthRequestProto {
    pub fn new() -> GetBalancerBandwidthRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBalancerBandwidthRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBalancerBandwidthRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBalancerBandwidthRequestProto,
        };
        unsafe {
            instance.get(GetBalancerBandwidthRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetBalancerBandwidthRequestProto {
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

impl ::protobuf::MessageStatic for GetBalancerBandwidthRequestProto {
    fn new() -> GetBalancerBandwidthRequestProto {
        GetBalancerBandwidthRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBalancerBandwidthRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetBalancerBandwidthRequestProto>(
                    "GetBalancerBandwidthRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBalancerBandwidthRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBalancerBandwidthRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBalancerBandwidthRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBalancerBandwidthResponseProto {
    // message fields
    bandwidth: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBalancerBandwidthResponseProto {}

impl GetBalancerBandwidthResponseProto {
    pub fn new() -> GetBalancerBandwidthResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBalancerBandwidthResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBalancerBandwidthResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBalancerBandwidthResponseProto,
        };
        unsafe {
            instance.get(GetBalancerBandwidthResponseProto::new)
        }
    }

    // required uint64 bandwidth = 1;

    pub fn clear_bandwidth(&mut self) {
        self.bandwidth = ::std::option::Option::None;
    }

    pub fn has_bandwidth(&self) -> bool {
        self.bandwidth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bandwidth(&mut self, v: u64) {
        self.bandwidth = ::std::option::Option::Some(v);
    }

    pub fn get_bandwidth(&self) -> u64 {
        self.bandwidth.unwrap_or(0)
    }

    fn get_bandwidth_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bandwidth
    }

    fn mut_bandwidth_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bandwidth
    }
}

impl ::protobuf::Message for GetBalancerBandwidthResponseProto {
    fn is_initialized(&self) -> bool {
        if self.bandwidth.is_none() {
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
                    self.bandwidth = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.bandwidth {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bandwidth {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for GetBalancerBandwidthResponseProto {
    fn new() -> GetBalancerBandwidthResponseProto {
        GetBalancerBandwidthResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBalancerBandwidthResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bandwidth",
                    GetBalancerBandwidthResponseProto::get_bandwidth_for_reflect,
                    GetBalancerBandwidthResponseProto::mut_bandwidth_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBalancerBandwidthResponseProto>(
                    "GetBalancerBandwidthResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBalancerBandwidthResponseProto {
    fn clear(&mut self) {
        self.clear_bandwidth();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBalancerBandwidthResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBalancerBandwidthResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubmitDiskBalancerPlanRequestProto {
    // message fields
    planID: ::protobuf::SingularField<::std::string::String>,
    plan: ::protobuf::SingularField<::std::string::String>,
    planVersion: ::std::option::Option<u64>,
    ignoreDateCheck: ::std::option::Option<bool>,
    planFile: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubmitDiskBalancerPlanRequestProto {}

impl SubmitDiskBalancerPlanRequestProto {
    pub fn new() -> SubmitDiskBalancerPlanRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubmitDiskBalancerPlanRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<SubmitDiskBalancerPlanRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubmitDiskBalancerPlanRequestProto,
        };
        unsafe {
            instance.get(SubmitDiskBalancerPlanRequestProto::new)
        }
    }

    // required string planID = 1;

    pub fn clear_planID(&mut self) {
        self.planID.clear();
    }

    pub fn has_planID(&self) -> bool {
        self.planID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planID(&mut self, v: ::std::string::String) {
        self.planID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_planID(&mut self) -> &mut ::std::string::String {
        if self.planID.is_none() {
            self.planID.set_default();
        }
        self.planID.as_mut().unwrap()
    }

    // Take field
    pub fn take_planID(&mut self) -> ::std::string::String {
        self.planID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_planID(&self) -> &str {
        match self.planID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_planID_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.planID
    }

    fn mut_planID_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.planID
    }

    // required string plan = 2;

    pub fn clear_plan(&mut self) {
        self.plan.clear();
    }

    pub fn has_plan(&self) -> bool {
        self.plan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plan(&mut self) -> &mut ::std::string::String {
        if self.plan.is_none() {
            self.plan.set_default();
        }
        self.plan.as_mut().unwrap()
    }

    // Take field
    pub fn take_plan(&mut self) -> ::std::string::String {
        self.plan.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_plan(&self) -> &str {
        match self.plan.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_plan_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.plan
    }

    fn mut_plan_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.plan
    }

    // optional uint64 planVersion = 3;

    pub fn clear_planVersion(&mut self) {
        self.planVersion = ::std::option::Option::None;
    }

    pub fn has_planVersion(&self) -> bool {
        self.planVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planVersion(&mut self, v: u64) {
        self.planVersion = ::std::option::Option::Some(v);
    }

    pub fn get_planVersion(&self) -> u64 {
        self.planVersion.unwrap_or(0)
    }

    fn get_planVersion_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.planVersion
    }

    fn mut_planVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.planVersion
    }

    // optional bool ignoreDateCheck = 4;

    pub fn clear_ignoreDateCheck(&mut self) {
        self.ignoreDateCheck = ::std::option::Option::None;
    }

    pub fn has_ignoreDateCheck(&self) -> bool {
        self.ignoreDateCheck.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignoreDateCheck(&mut self, v: bool) {
        self.ignoreDateCheck = ::std::option::Option::Some(v);
    }

    pub fn get_ignoreDateCheck(&self) -> bool {
        self.ignoreDateCheck.unwrap_or(false)
    }

    fn get_ignoreDateCheck_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignoreDateCheck
    }

    fn mut_ignoreDateCheck_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignoreDateCheck
    }

    // required string planFile = 5;

    pub fn clear_planFile(&mut self) {
        self.planFile.clear();
    }

    pub fn has_planFile(&self) -> bool {
        self.planFile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planFile(&mut self, v: ::std::string::String) {
        self.planFile = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_planFile(&mut self) -> &mut ::std::string::String {
        if self.planFile.is_none() {
            self.planFile.set_default();
        }
        self.planFile.as_mut().unwrap()
    }

    // Take field
    pub fn take_planFile(&mut self) -> ::std::string::String {
        self.planFile.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_planFile(&self) -> &str {
        match self.planFile.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_planFile_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.planFile
    }

    fn mut_planFile_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.planFile
    }
}

impl ::protobuf::Message for SubmitDiskBalancerPlanRequestProto {
    fn is_initialized(&self) -> bool {
        if self.planID.is_none() {
            return false;
        }
        if self.plan.is_none() {
            return false;
        }
        if self.planFile.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.planID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.plan)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.planVersion = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignoreDateCheck = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.planFile)?;
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
        if let Some(ref v) = self.planID.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.plan.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.planVersion {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ignoreDateCheck {
            my_size += 2;
        }
        if let Some(ref v) = self.planFile.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.planID.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.plan.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.planVersion {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.ignoreDateCheck {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.planFile.as_ref() {
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

impl ::protobuf::MessageStatic for SubmitDiskBalancerPlanRequestProto {
    fn new() -> SubmitDiskBalancerPlanRequestProto {
        SubmitDiskBalancerPlanRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubmitDiskBalancerPlanRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "planID",
                    SubmitDiskBalancerPlanRequestProto::get_planID_for_reflect,
                    SubmitDiskBalancerPlanRequestProto::mut_planID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plan",
                    SubmitDiskBalancerPlanRequestProto::get_plan_for_reflect,
                    SubmitDiskBalancerPlanRequestProto::mut_plan_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "planVersion",
                    SubmitDiskBalancerPlanRequestProto::get_planVersion_for_reflect,
                    SubmitDiskBalancerPlanRequestProto::mut_planVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignoreDateCheck",
                    SubmitDiskBalancerPlanRequestProto::get_ignoreDateCheck_for_reflect,
                    SubmitDiskBalancerPlanRequestProto::mut_ignoreDateCheck_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "planFile",
                    SubmitDiskBalancerPlanRequestProto::get_planFile_for_reflect,
                    SubmitDiskBalancerPlanRequestProto::mut_planFile_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubmitDiskBalancerPlanRequestProto>(
                    "SubmitDiskBalancerPlanRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubmitDiskBalancerPlanRequestProto {
    fn clear(&mut self) {
        self.clear_planID();
        self.clear_plan();
        self.clear_planVersion();
        self.clear_ignoreDateCheck();
        self.clear_planFile();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubmitDiskBalancerPlanRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubmitDiskBalancerPlanRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubmitDiskBalancerPlanResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubmitDiskBalancerPlanResponseProto {}

impl SubmitDiskBalancerPlanResponseProto {
    pub fn new() -> SubmitDiskBalancerPlanResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubmitDiskBalancerPlanResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<SubmitDiskBalancerPlanResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubmitDiskBalancerPlanResponseProto,
        };
        unsafe {
            instance.get(SubmitDiskBalancerPlanResponseProto::new)
        }
    }
}

impl ::protobuf::Message for SubmitDiskBalancerPlanResponseProto {
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

impl ::protobuf::MessageStatic for SubmitDiskBalancerPlanResponseProto {
    fn new() -> SubmitDiskBalancerPlanResponseProto {
        SubmitDiskBalancerPlanResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubmitDiskBalancerPlanResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SubmitDiskBalancerPlanResponseProto>(
                    "SubmitDiskBalancerPlanResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubmitDiskBalancerPlanResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubmitDiskBalancerPlanResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubmitDiskBalancerPlanResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CancelPlanRequestProto {
    // message fields
    planID: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelPlanRequestProto {}

impl CancelPlanRequestProto {
    pub fn new() -> CancelPlanRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelPlanRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CancelPlanRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelPlanRequestProto,
        };
        unsafe {
            instance.get(CancelPlanRequestProto::new)
        }
    }

    // required string planID = 1;

    pub fn clear_planID(&mut self) {
        self.planID.clear();
    }

    pub fn has_planID(&self) -> bool {
        self.planID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planID(&mut self, v: ::std::string::String) {
        self.planID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_planID(&mut self) -> &mut ::std::string::String {
        if self.planID.is_none() {
            self.planID.set_default();
        }
        self.planID.as_mut().unwrap()
    }

    // Take field
    pub fn take_planID(&mut self) -> ::std::string::String {
        self.planID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_planID(&self) -> &str {
        match self.planID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_planID_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.planID
    }

    fn mut_planID_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.planID
    }
}

impl ::protobuf::Message for CancelPlanRequestProto {
    fn is_initialized(&self) -> bool {
        if self.planID.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.planID)?;
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
        if let Some(ref v) = self.planID.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.planID.as_ref() {
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

impl ::protobuf::MessageStatic for CancelPlanRequestProto {
    fn new() -> CancelPlanRequestProto {
        CancelPlanRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelPlanRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "planID",
                    CancelPlanRequestProto::get_planID_for_reflect,
                    CancelPlanRequestProto::mut_planID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CancelPlanRequestProto>(
                    "CancelPlanRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelPlanRequestProto {
    fn clear(&mut self) {
        self.clear_planID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CancelPlanRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelPlanRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CancelPlanResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelPlanResponseProto {}

impl CancelPlanResponseProto {
    pub fn new() -> CancelPlanResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelPlanResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CancelPlanResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelPlanResponseProto,
        };
        unsafe {
            instance.get(CancelPlanResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CancelPlanResponseProto {
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

impl ::protobuf::MessageStatic for CancelPlanResponseProto {
    fn new() -> CancelPlanResponseProto {
        CancelPlanResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelPlanResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CancelPlanResponseProto>(
                    "CancelPlanResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelPlanResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CancelPlanResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelPlanResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryPlanStatusRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryPlanStatusRequestProto {}

impl QueryPlanStatusRequestProto {
    pub fn new() -> QueryPlanStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryPlanStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<QueryPlanStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryPlanStatusRequestProto,
        };
        unsafe {
            instance.get(QueryPlanStatusRequestProto::new)
        }
    }
}

impl ::protobuf::Message for QueryPlanStatusRequestProto {
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

impl ::protobuf::MessageStatic for QueryPlanStatusRequestProto {
    fn new() -> QueryPlanStatusRequestProto {
        QueryPlanStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryPlanStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<QueryPlanStatusRequestProto>(
                    "QueryPlanStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryPlanStatusRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryPlanStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryPlanStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryPlanStatusResponseProto {
    // message fields
    result: ::std::option::Option<u32>,
    planID: ::protobuf::SingularField<::std::string::String>,
    currentStatus: ::protobuf::SingularField<::std::string::String>,
    planFile: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryPlanStatusResponseProto {}

impl QueryPlanStatusResponseProto {
    pub fn new() -> QueryPlanStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryPlanStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<QueryPlanStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryPlanStatusResponseProto,
        };
        unsafe {
            instance.get(QueryPlanStatusResponseProto::new)
        }
    }

    // optional uint32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.result
    }

    // optional string planID = 2;

    pub fn clear_planID(&mut self) {
        self.planID.clear();
    }

    pub fn has_planID(&self) -> bool {
        self.planID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planID(&mut self, v: ::std::string::String) {
        self.planID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_planID(&mut self) -> &mut ::std::string::String {
        if self.planID.is_none() {
            self.planID.set_default();
        }
        self.planID.as_mut().unwrap()
    }

    // Take field
    pub fn take_planID(&mut self) -> ::std::string::String {
        self.planID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_planID(&self) -> &str {
        match self.planID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_planID_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.planID
    }

    fn mut_planID_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.planID
    }

    // optional string currentStatus = 3;

    pub fn clear_currentStatus(&mut self) {
        self.currentStatus.clear();
    }

    pub fn has_currentStatus(&self) -> bool {
        self.currentStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentStatus(&mut self, v: ::std::string::String) {
        self.currentStatus = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentStatus(&mut self) -> &mut ::std::string::String {
        if self.currentStatus.is_none() {
            self.currentStatus.set_default();
        }
        self.currentStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentStatus(&mut self) -> ::std::string::String {
        self.currentStatus.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_currentStatus(&self) -> &str {
        match self.currentStatus.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_currentStatus_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.currentStatus
    }

    fn mut_currentStatus_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.currentStatus
    }

    // optional string planFile = 4;

    pub fn clear_planFile(&mut self) {
        self.planFile.clear();
    }

    pub fn has_planFile(&self) -> bool {
        self.planFile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_planFile(&mut self, v: ::std::string::String) {
        self.planFile = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_planFile(&mut self) -> &mut ::std::string::String {
        if self.planFile.is_none() {
            self.planFile.set_default();
        }
        self.planFile.as_mut().unwrap()
    }

    // Take field
    pub fn take_planFile(&mut self) -> ::std::string::String {
        self.planFile.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_planFile(&self) -> &str {
        match self.planFile.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_planFile_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.planFile
    }

    fn mut_planFile_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.planFile
    }
}

impl ::protobuf::Message for QueryPlanStatusResponseProto {
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
                    let tmp = is.read_uint32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.planID)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.currentStatus)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.planFile)?;
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.planID.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.currentStatus.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.planFile.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.planID.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.currentStatus.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.planFile.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for QueryPlanStatusResponseProto {
    fn new() -> QueryPlanStatusResponseProto {
        QueryPlanStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryPlanStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    QueryPlanStatusResponseProto::get_result_for_reflect,
                    QueryPlanStatusResponseProto::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "planID",
                    QueryPlanStatusResponseProto::get_planID_for_reflect,
                    QueryPlanStatusResponseProto::mut_planID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currentStatus",
                    QueryPlanStatusResponseProto::get_currentStatus_for_reflect,
                    QueryPlanStatusResponseProto::mut_currentStatus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "planFile",
                    QueryPlanStatusResponseProto::get_planFile_for_reflect,
                    QueryPlanStatusResponseProto::mut_planFile_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryPlanStatusResponseProto>(
                    "QueryPlanStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryPlanStatusResponseProto {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_planID();
        self.clear_currentStatus();
        self.clear_planFile();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryPlanStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryPlanStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiskBalancerSettingRequestProto {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiskBalancerSettingRequestProto {}

impl DiskBalancerSettingRequestProto {
    pub fn new() -> DiskBalancerSettingRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiskBalancerSettingRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DiskBalancerSettingRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiskBalancerSettingRequestProto,
        };
        unsafe {
            instance.get(DiskBalancerSettingRequestProto::new)
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
}

impl ::protobuf::Message for DiskBalancerSettingRequestProto {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
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

impl ::protobuf::MessageStatic for DiskBalancerSettingRequestProto {
    fn new() -> DiskBalancerSettingRequestProto {
        DiskBalancerSettingRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiskBalancerSettingRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    DiskBalancerSettingRequestProto::get_key_for_reflect,
                    DiskBalancerSettingRequestProto::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiskBalancerSettingRequestProto>(
                    "DiskBalancerSettingRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiskBalancerSettingRequestProto {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiskBalancerSettingRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiskBalancerSettingRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiskBalancerSettingResponseProto {
    // message fields
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiskBalancerSettingResponseProto {}

impl DiskBalancerSettingResponseProto {
    pub fn new() -> DiskBalancerSettingResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiskBalancerSettingResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DiskBalancerSettingResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiskBalancerSettingResponseProto,
        };
        unsafe {
            instance.get(DiskBalancerSettingResponseProto::new)
        }
    }

    // required string value = 1;

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

impl ::protobuf::Message for DiskBalancerSettingResponseProto {
    fn is_initialized(&self) -> bool {
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
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for DiskBalancerSettingResponseProto {
    fn new() -> DiskBalancerSettingResponseProto {
        DiskBalancerSettingResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiskBalancerSettingResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    DiskBalancerSettingResponseProto::get_value_for_reflect,
                    DiskBalancerSettingResponseProto::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiskBalancerSettingResponseProto>(
                    "DiskBalancerSettingResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiskBalancerSettingResponseProto {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiskBalancerSettingResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiskBalancerSettingResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cClientDatanodeProtocol.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.pr\
    oto\x1a\nhdfs.proto\x1a\x1dReconfigurationProtocol.proto\"\\\n#GetReplic\
    aVisibleLengthRequestProto\x125\n\x05block\x18\x01\x20\x02(\x0b2\x1f.had\
    oop.hdfs.ExtendedBlockProtoR\x05block\">\n$GetReplicaVisibleLengthRespon\
    seProto\x12\x16\n\x06length\x18\x01\x20\x02(\x04R\x06length\"\x1e\n\x1cR\
    efreshNamenodesRequestProto\"\x1f\n\x1dRefreshNamenodesResponseProto\"Q\
    \n\x1bDeleteBlockPoolRequestProto\x12\x1c\n\tblockPool\x18\x01\x20\x02(\
    \tR\tblockPool\x12\x14\n\x05force\x18\x02\x20\x02(\x08R\x05force\"\x1e\n\
    \x1cDeleteBlockPoolResponseProto\"\x8b\x01\n!GetBlockLocalPathInfoReques\
    tProto\x125\n\x05block\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlo\
    ckProtoR\x05block\x12/\n\x05token\x18\x02\x20\x02(\x0b2\x19.hadoop.commo\
    n.TokenProtoR\x05token\"\x9f\x01\n\"GetBlockLocalPathInfoResponseProto\
    \x125\n\x05block\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProt\
    oR\x05block\x12\x1c\n\tlocalPath\x18\x02\x20\x02(\tR\tlocalPath\x12$\n\r\
    localMetaPath\x18\x03\x20\x02(\tR\rlocalMetaPath\">\n\x1cShutdownDatanod\
    eRequestProto\x12\x1e\n\nforUpgrade\x18\x01\x20\x02(\x08R\nforUpgrade\"\
    \x1f\n\x1dShutdownDatanodeResponseProto\"\x1a\n\x18EvictWritersRequestPr\
    oto\"\x1b\n\x19EvictWritersResponseProto\"\x1d\n\x1bGetDatanodeInfoReque\
    stProto\"a\n\x1cGetDatanodeInfoResponseProto\x12A\n\tlocalInfo\x18\x01\
    \x20\x02(\x0b2#.hadoop.hdfs.DatanodeLocalInfoProtoR\tlocalInfo\"\x1d\n\
    \x1bGetVolumeReportRequestProto\"d\n\x1cGetVolumeReportResponseProto\x12\
    D\n\nvolumeInfo\x18\x01\x20\x03(\x0b2$.hadoop.hdfs.DatanodeVolumeInfoPro\
    toR\nvolumeInfo\"B\n\x1eTriggerBlockReportRequestProto\x12\x20\n\x0bincr\
    emental\x18\x01\x20\x02(\x08R\x0bincremental\"!\n\x1fTriggerBlockReportR\
    esponseProto\"\"\n\x20GetBalancerBandwidthRequestProto\"A\n!GetBalancerB\
    andwidthResponseProto\x12\x1c\n\tbandwidth\x18\x01\x20\x02(\x04R\tbandwi\
    dth\"\xb8\x01\n\"SubmitDiskBalancerPlanRequestProto\x12\x16\n\x06planID\
    \x18\x01\x20\x02(\tR\x06planID\x12\x12\n\x04plan\x18\x02\x20\x02(\tR\x04\
    plan\x12\x20\n\x0bplanVersion\x18\x03\x20\x01(\x04R\x0bplanVersion\x12(\
    \n\x0fignoreDateCheck\x18\x04\x20\x01(\x08R\x0fignoreDateCheck\x12\x1a\n\
    \x08planFile\x18\x05\x20\x02(\tR\x08planFile\"%\n#SubmitDiskBalancerPlan\
    ResponseProto\"0\n\x16CancelPlanRequestProto\x12\x16\n\x06planID\x18\x01\
    \x20\x02(\tR\x06planID\"\x19\n\x17CancelPlanResponseProto\"\x1d\n\x1bQue\
    ryPlanStatusRequestProto\"\x90\x01\n\x1cQueryPlanStatusResponseProto\x12\
    \x16\n\x06result\x18\x01\x20\x01(\rR\x06result\x12\x16\n\x06planID\x18\
    \x02\x20\x01(\tR\x06planID\x12$\n\rcurrentStatus\x18\x03\x20\x01(\tR\rcu\
    rrentStatus\x12\x1a\n\x08planFile\x18\x04\x20\x01(\tR\x08planFile\"3\n\
    \x1fDiskBalancerSettingRequestProto\x12\x10\n\x03key\x18\x01\x20\x02(\tR\
    \x03key\"8\n\x20DiskBalancerSettingResponseProto\x12\x14\n\x05value\x18\
    \x01\x20\x02(\tR\x05value2\xc0\x0f\n\x1dClientDatanodeProtocolService\
    \x12~\n\x17getReplicaVisibleLength\x120.hadoop.hdfs.GetReplicaVisibleLen\
    gthRequestProto\x1a1.hadoop.hdfs.GetReplicaVisibleLengthResponseProto\
    \x12i\n\x10refreshNamenodes\x12).hadoop.hdfs.RefreshNamenodesRequestProt\
    o\x1a*.hadoop.hdfs.RefreshNamenodesResponseProto\x12f\n\x0fdeleteBlockPo\
    ol\x12(.hadoop.hdfs.DeleteBlockPoolRequestProto\x1a).hadoop.hdfs.DeleteB\
    lockPoolResponseProto\x12x\n\x15getBlockLocalPathInfo\x12..hadoop.hdfs.G\
    etBlockLocalPathInfoRequestProto\x1a/.hadoop.hdfs.GetBlockLocalPathInfoR\
    esponseProto\x12i\n\x10shutdownDatanode\x12).hadoop.hdfs.ShutdownDatanod\
    eRequestProto\x1a*.hadoop.hdfs.ShutdownDatanodeResponseProto\x12]\n\x0ce\
    victWriters\x12%.hadoop.hdfs.EvictWritersRequestProto\x1a&.hadoop.hdfs.E\
    victWritersResponseProto\x12f\n\x0fgetDatanodeInfo\x12(.hadoop.hdfs.GetD\
    atanodeInfoRequestProto\x1a).hadoop.hdfs.GetDatanodeInfoResponseProto\
    \x12f\n\x0fgetVolumeReport\x12(.hadoop.hdfs.GetVolumeReportRequestProto\
    \x1a).hadoop.hdfs.GetVolumeReportResponseProto\x12\x81\x01\n\x18getRecon\
    figurationStatus\x121.hadoop.hdfs.GetReconfigurationStatusRequestProto\
    \x1a2.hadoop.hdfs.GetReconfigurationStatusResponseProto\x12u\n\x14startR\
    econfiguration\x12-.hadoop.hdfs.StartReconfigurationRequestProto\x1a..ha\
    doop.hdfs.StartReconfigurationResponseProto\x12\x8d\x01\n\x1clistReconfi\
    gurableProperties\x125.hadoop.hdfs.ListReconfigurablePropertiesRequestPr\
    oto\x1a6.hadoop.hdfs.ListReconfigurablePropertiesResponseProto\x12o\n\
    \x12triggerBlockReport\x12+.hadoop.hdfs.TriggerBlockReportRequestProto\
    \x1a,.hadoop.hdfs.TriggerBlockReportResponseProto\x12u\n\x14getBalancerB\
    andwidth\x12-.hadoop.hdfs.GetBalancerBandwidthRequestProto\x1a..hadoop.h\
    dfs.GetBalancerBandwidthResponseProto\x12{\n\x16submitDiskBalancerPlan\
    \x12/.hadoop.hdfs.SubmitDiskBalancerPlanRequestProto\x1a0.hadoop.hdfs.Su\
    bmitDiskBalancerPlanResponseProto\x12c\n\x16cancelDiskBalancerPlan\x12#.\
    hadoop.hdfs.CancelPlanRequestProto\x1a$.hadoop.hdfs.CancelPlanResponsePr\
    oto\x12l\n\x15queryDiskBalancerPlan\x12(.hadoop.hdfs.QueryPlanStatusRequ\
    estProto\x1a).hadoop.hdfs.QueryPlanStatusResponseProto\x12u\n\x16getDisk\
    BalancerSetting\x12,.hadoop.hdfs.DiskBalancerSettingRequestProto\x1a-.ha\
    doop.hdfs.DiskBalancerSettingResponseProtoBK\n%org.apache.hadoop.hdfs.pr\
    otocol.protoB\x1cClientDatanodeProtocolProtos\xa0\x01\x01\x88\x01\x01J\
    \x89<\n\x07\x12\x05\x1b\0\xb2\x02\x01\n\x08\n\x01\x08\x12\x03\x1b\0>\n\
    \xc1\x08\n\x04\x08\xe7\x07\0\x12\x03\x1b\0>2\x83\x06*\n\x20Licensed\x20t\
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
    erface.\n2\x80\x01\x20This\x20file\x20contains\x20protocol\x20buffers\
    \x20that\x20are\x20used\x20throughout\x20HDFS\x20--\x20i.e.\n\x20by\x20t\
    he\x20client,\x20server,\x20and\x20data\x20transfer\x20protocols.\n\n\
    \x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x1b\x07\x13\n\r\n\x06\x08\xe7\x07\0\
    \x02\0\x12\x03\x1b\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x1b\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x1b\x16=\n\x08\n\x01\
    \x08\x12\x03\x1c\0=\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x1c\0=\n\x0c\n\
    \x05\x08\xe7\x07\x01\x02\x12\x03\x1c\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x1c\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x1c\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x1c\x1e<\n\x08\n\
    \x01\x08\x12\x03\x1d\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1d\0$\n\x0c\
    \n\x05\x08\xe7\x07\x02\x02\x12\x03\x1d\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x1d\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x1d\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1d\x1f#\n\x08\n\
    \x01\x08\x12\x03\x1e\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1e\0,\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x1e\x07$\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x1e\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x1e\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1e'+\n\x08\n\x01\x02\
    \x12\x03\x1f\x08\x13\n\t\n\x02\x03\0\x12\x03!\x07\x17\n\t\n\x02\x03\x01\
    \x12\x03\"\x07\x13\n\t\n\x02\x03\x02\x12\x03#\x07&\nC\n\x02\x04\0\x12\
    \x04(\0*\x01\x1a7*\n\x20block\x20-\x20block\x20for\x20which\x20visible\
    \x20length\x20is\x20requested\n\n\n\n\x03\x04\0\x01\x12\x03(\x08+\n\x0b\
    \n\x04\x04\0\x02\0\x12\x03)\x02(\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03)\
    \x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03)\x0b\x1d\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03)\x1e#\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03)&'\n4\n\x02\
    \x04\x01\x12\x04/\01\x01\x1a(*\n\x20length\x20-\x20visible\x20length\x20\
    of\x20the\x20block\n\n\n\n\x03\x04\x01\x01\x12\x03/\x08,\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x030\x02\x1d\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x030\x02\n\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x030\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x030\x12\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x030\x1b\x1c\n\
    \x1c\n\x02\x04\x02\x12\x046\07\x01\x1a\x10*\n\x20void\x20request\n\n\n\n\
    \x03\x04\x02\x01\x12\x036\x08$\n\x1d\n\x02\x04\x03\x12\x04<\0=\x01\x1a\
    \x11*\n\x20void\x20response\n\n\n\n\x03\x04\x03\x01\x12\x03<\x08%\n\xb2\
    \x01\n\x02\x04\x04\x12\x04D\0G\x01\x1a\xa5\x01*\n\x20blockPool\x20-\x20b\
    lock\x20pool\x20to\x20be\x20deleted\n\x20force\x20-\x20if\x20false,\x20d\
    elete\x20the\x20block\x20pool\x20only\x20if\x20it\x20is\x20empty.\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20if\x20true,\x20delete\x20the\x20block\
    \x20pool\x20even\x20if\x20it\x20has\x20blocks.\n\n\n\n\x03\x04\x04\x01\
    \x12\x03D\x08#\n\x0b\n\x04\x04\x04\x02\0\x12\x03E\x02\x20\n\x0c\n\x05\
    \x04\x04\x02\0\x04\x12\x03E\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03E\
    \x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03E\x12\x1b\n\x0c\n\x05\x04\
    \x04\x02\0\x03\x12\x03E\x1e\x1f\n\x0b\n\x04\x04\x04\x02\x01\x12\x03F\x02\
    \x1a\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\x04\
    \x02\x01\x05\x12\x03F\x0b\x0f\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03F\
    \x10\x15\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03F\x18\x19\n\x1d\n\x02\
    \x04\x05\x12\x04L\0M\x01\x1a\x11*\n\x20void\x20response\n\n\n\n\x03\x04\
    \x05\x01\x12\x03L\x08$\n\xe5\x01\n\x02\x04\x06\x12\x04V\0Y\x01\x1a\xd8\
    \x01*\n\x20Gets\x20the\x20file\x20information\x20where\x20block\x20and\
    \x20its\x20metadata\x20is\x20stored\n\x20block\x20-\x20block\x20for\x20w\
    hich\x20path\x20information\x20is\x20being\x20requested\n\x20token\x20-\
    \x20block\x20token\n\n\x20This\x20message\x20is\x20deprecated\x20in\x20f\
    avor\x20of\x20file\x20descriptor\x20passing.\n\n\n\n\x03\x04\x06\x01\x12\
    \x03V\x08)\n\x0b\n\x04\x04\x06\x02\0\x12\x03W\x02(\n\x0c\n\x05\x04\x06\
    \x02\0\x04\x12\x03W\x02\n\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03W\x0b\x1d\
    \n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03W\x1e#\n\x0c\n\x05\x04\x06\x02\0\
    \x03\x12\x03W&'\n\x0b\n\x04\x04\x06\x02\x01\x12\x03X\x02.\n\x0c\n\x05\
    \x04\x06\x02\x01\x04\x12\x03X\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\
    \x03X\x0b#\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03X$)\n\x0c\n\x05\x04\
    \x06\x02\x01\x03\x12\x03X,-\n\x87\x02\n\x02\x04\x07\x12\x04b\0f\x01\x1a\
    \xfa\x01*\n\x20block\x20-\x20block\x20for\x20which\x20file\x20path\x20in\
    formation\x20is\x20being\x20returned\n\x20localPath\x20-\x20file\x20path\
    \x20where\x20the\x20block\x20data\x20is\x20stored\n\x20localMetaPath\x20\
    -\x20file\x20path\x20where\x20the\x20block\x20meta\x20data\x20is\x20stor\
    ed\n\n\x20This\x20message\x20is\x20deprecated\x20in\x20favor\x20of\x20fi\
    le\x20descriptor\x20passing.\n\n\n\n\x03\x04\x07\x01\x12\x03b\x08*\n\x0b\
    \n\x04\x04\x07\x02\0\x12\x03c\x02(\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03\
    c\x02\n\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03c\x0b\x1d\n\x0c\n\x05\x04\
    \x07\x02\0\x01\x12\x03c\x1e#\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03c&'\n\
    \x0b\n\x04\x04\x07\x02\x01\x12\x03d\x02\x20\n\x0c\n\x05\x04\x07\x02\x01\
    \x04\x12\x03d\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03d\x0b\x11\n\
    \x0c\n\x05\x04\x07\x02\x01\x01\x12\x03d\x12\x1b\n\x0c\n\x05\x04\x07\x02\
    \x01\x03\x12\x03d\x1e\x1f\n\x0b\n\x04\x04\x07\x02\x02\x12\x03e\x02$\n\
    \x0c\n\x05\x04\x07\x02\x02\x04\x12\x03e\x02\n\n\x0c\n\x05\x04\x07\x02\
    \x02\x05\x12\x03e\x0b\x11\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03e\x12\
    \x1f\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03e\"#\n\xc4\x01\n\x02\x04\x08\
    \x12\x04m\0o\x01\x1a\xb7\x01*\n\x20forUpgrade\x20-\x20if\x20true,\x20cli\
    ents\x20are\x20advised\x20to\x20wait\x20for\x20restart\x20and\x20quick\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20upgrade\x20resta\
    rt\x20is\x20instrumented.\x20Otherwise,\x20datanode\x20does\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20the\x20regular\x20shutdo\
    wn.\n\n\n\n\x03\x04\x08\x01\x12\x03m\x08$\n\x0b\n\x04\x04\x08\x02\0\x12\
    \x03n\x02\x1f\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03n\x02\n\n\x0c\n\x05\
    \x04\x08\x02\0\x05\x12\x03n\x0b\x0f\n\x0c\n\x05\x04\x08\x02\0\x01\x12\
    \x03n\x10\x1a\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03n\x1d\x1e\n\n\n\x02\
    \x04\t\x12\x04q\0r\x01\n\n\n\x03\x04\t\x01\x12\x03q\x08%\nE\n\x02\x04\n\
    \x12\x04u\0v\x01\x1a9*\x20Tell\x20datanode\x20to\x20evict\x20active\x20c\
    lients\x20that\x20are\x20writing\x20\n\n\n\x03\x04\n\x01\x12\x03u\x08\
    \x20\n\n\n\x02\x04\x0b\x12\x04x\0y\x01\n\n\n\x03\x04\x0b\x01\x12\x03x\
    \x08!\n9\n\x02\x04\x0c\x12\x04~\0\x7f\x01\x1a-*\n\x20Ping\x20datanode\
    \x20for\x20liveness\x20and\x20quick\x20info\n\n\n\n\x03\x04\x0c\x01\x12\
    \x03~\x08#\n\x0c\n\x02\x04\r\x12\x06\x81\x01\0\x83\x01\x01\n\x0b\n\x03\
    \x04\r\x01\x12\x04\x81\x01\x08$\n\x0c\n\x04\x04\r\x02\0\x12\x04\x82\x01\
    \x020\n\r\n\x05\x04\r\x02\0\x04\x12\x04\x82\x01\x02\n\n\r\n\x05\x04\r\
    \x02\0\x06\x12\x04\x82\x01\x0b!\n\r\n\x05\x04\r\x02\0\x01\x12\x04\x82\
    \x01\"+\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x82\x01./\n\x0c\n\x02\x04\x0e\
    \x12\x06\x85\x01\0\x86\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\x85\x01\
    \x08#\n\x0c\n\x02\x04\x0f\x12\x06\x88\x01\0\x8a\x01\x01\n\x0b\n\x03\x04\
    \x0f\x01\x12\x04\x88\x01\x08$\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\x89\x01\
    \x022\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\x89\x01\x02\n\n\r\n\x05\x04\
    \x0f\x02\0\x06\x12\x04\x89\x01\x0b\"\n\r\n\x05\x04\x0f\x02\0\x01\x12\x04\
    \x89\x01#-\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\x89\x0101\n\x0c\n\x02\x04\
    \x10\x12\x06\x8c\x01\0\x8e\x01\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\x8c\
    \x01\x08&\n\x0c\n\x04\x04\x10\x02\0\x12\x04\x8d\x01\x02\x20\n\r\n\x05\
    \x04\x10\x02\0\x04\x12\x04\x8d\x01\x02\n\n\r\n\x05\x04\x10\x02\0\x05\x12\
    \x04\x8d\x01\x0b\x0f\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\x8d\x01\x10\x1b\
    \n\r\n\x05\x04\x10\x02\0\x03\x12\x04\x8d\x01\x1e\x1f\n\x0c\n\x02\x04\x11\
    \x12\x06\x90\x01\0\x91\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\x90\x01\
    \x08'\n\x0c\n\x02\x04\x12\x12\x06\x93\x01\0\x94\x01\x01\n\x0b\n\x03\x04\
    \x12\x01\x12\x04\x93\x01\x08(\nG\n\x02\x04\x13\x12\x06\x99\x01\0\x9b\x01\
    \x01\x1a9*\n\x20bandwidth\x20-\x20balancer\x20bandwidth\x20value\x20of\
    \x20the\x20datanode.\n\n\x0b\n\x03\x04\x13\x01\x12\x04\x99\x01\x08)\n\
    \x0c\n\x04\x04\x13\x02\0\x12\x04\x9a\x01\x02\x20\n\r\n\x05\x04\x13\x02\0\
    \x04\x12\x04\x9a\x01\x02\n\n\r\n\x05\x04\x13\x02\0\x05\x12\x04\x9a\x01\
    \x0b\x11\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\x9a\x01\x12\x1b\n\r\n\x05\
    \x04\x13\x02\0\x03\x12\x04\x9a\x01\x1e\x1f\n^\n\x02\x04\x14\x12\x06\xa1\
    \x01\0\xa7\x01\x01\x1aP*\n\x20This\x20message\x20allows\x20a\x20client\
    \x20to\x20submit\x20a\x20disk\n\x20balancer\x20plan\x20to\x20a\x20data\
    \x20node.\n\n\x0b\n\x03\x04\x14\x01\x12\x04\xa1\x01\x08*\n-\n\x04\x04\
    \x14\x02\0\x12\x04\xa2\x01\x02\x1d\"\x1f\x20A\x20hash\x20of\x20the\x20pl\
    an\x20like\x20SHA-1\n\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\xa2\x01\x02\n\
    \n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xa2\x01\x0b\x11\n\r\n\x05\x04\x14\
    \x02\0\x01\x12\x04\xa2\x01\x12\x18\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\
    \xa2\x01\x1b\x1c\n-\n\x04\x04\x14\x02\x01\x12\x04\xa3\x01\x02\x1b\"\x1f\
    \x20Plan\x20file\x20data\x20in\x20Json\x20format\n\n\r\n\x05\x04\x14\x02\
    \x01\x04\x12\x04\xa3\x01\x02\n\n\r\n\x05\x04\x14\x02\x01\x05\x12\x04\xa3\
    \x01\x0b\x11\n\r\n\x05\x04\x14\x02\x01\x01\x12\x04\xa3\x01\x12\x16\n\r\n\
    \x05\x04\x14\x02\x01\x03\x12\x04\xa3\x01\x19\x1a\n#\n\x04\x04\x14\x02\
    \x02\x12\x04\xa4\x01\x02\"\"\x15\x20Plan\x20version\x20number\n\n\r\n\
    \x05\x04\x14\x02\x02\x04\x12\x04\xa4\x01\x02\n\n\r\n\x05\x04\x14\x02\x02\
    \x05\x12\x04\xa4\x01\x0b\x11\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\xa4\
    \x01\x12\x1d\n\r\n\x05\x04\x14\x02\x02\x03\x12\x04\xa4\x01\x20!\n0\n\x04\
    \x04\x14\x02\x03\x12\x04\xa5\x01\x02$\"\"\x20Ignore\x20date\x20checks\
    \x20on\x20this\x20plan.\n\n\r\n\x05\x04\x14\x02\x03\x04\x12\x04\xa5\x01\
    \x02\n\n\r\n\x05\x04\x14\x02\x03\x05\x12\x04\xa5\x01\x0b\x0f\n\r\n\x05\
    \x04\x14\x02\x03\x01\x12\x04\xa5\x01\x10\x1f\n\r\n\x05\x04\x14\x02\x03\
    \x03\x12\x04\xa5\x01\"#\n\x1e\n\x04\x04\x14\x02\x04\x12\x04\xa6\x01\x02\
    \x1f\"\x10\x20Plan\x20file\x20path\n\n\r\n\x05\x04\x14\x02\x04\x04\x12\
    \x04\xa6\x01\x02\n\n\r\n\x05\x04\x14\x02\x04\x05\x12\x04\xa6\x01\x0b\x11\
    \n\r\n\x05\x04\x14\x02\x04\x01\x12\x04\xa6\x01\x12\x1a\n\r\n\x05\x04\x14\
    \x02\x04\x03\x12\x04\xa6\x01\x1d\x1e\nC\n\x02\x04\x15\x12\x06\xac\x01\0\
    \xad\x01\x01\x1a5*\n\x20Response\x20from\x20the\x20DataNode\x20on\x20Pla\
    n\x20Submit\x20request\n\n\x0b\n\x03\x04\x15\x01\x12\x04\xac\x01\x08+\n_\
    \n\x02\x04\x16\x12\x06\xb3\x01\0\xb5\x01\x01\x1aQ*\n\x20This\x20message\
    \x20describes\x20a\x20request\x20to\x20cancel\x20an\n\x20outstanding\x20\
    disk\x20balancer\x20plan\n\n\x0b\n\x03\x04\x16\x01\x12\x04\xb3\x01\x08\
    \x1e\n\x0c\n\x04\x04\x16\x02\0\x12\x04\xb4\x01\x02\x1d\n\r\n\x05\x04\x16\
    \x02\0\x04\x12\x04\xb4\x01\x02\n\n\r\n\x05\x04\x16\x02\0\x05\x12\x04\xb4\
    \x01\x0b\x11\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xb4\x01\x12\x18\n\r\n\
    \x05\x04\x16\x02\0\x03\x12\x04\xb4\x01\x1b\x1c\nC\n\x02\x04\x17\x12\x06\
    \xba\x01\0\xbb\x01\x01\x1a5*\n\x20This\x20is\x20the\x20response\x20for\
    \x20the\x20cancellation\x20request\n\n\x0b\n\x03\x04\x17\x01\x12\x04\xba\
    \x01\x08\x1f\n\x8d\x01\n\x02\x04\x18\x12\x06\xc3\x01\0\xc4\x01\x01\x1a\
    \x7f*\n\x20This\x20message\x20allows\x20a\x20client\x20to\x20query\x20da\
    ta\x20node\x20to\x20see\n\x20if\x20a\x20disk\x20balancer\x20plan\x20is\
    \x20executing\x20and\x20if\x20so\x20what\x20is\n\x20the\x20status.\n\n\
    \x0b\n\x03\x04\x18\x01\x12\x04\xc3\x01\x08#\nD\n\x02\x04\x19\x12\x06\xc9\
    \x01\0\xce\x01\x01\x1a6*\n\x20This\x20message\x20describes\x20a\x20plan\
    \x20if\x20it\x20is\x20in\x20progress\n\n\x0b\n\x03\x04\x19\x01\x12\x04\
    \xc9\x01\x08$\n\x0c\n\x04\x04\x19\x02\0\x12\x04\xca\x01\x02\x1d\n\r\n\
    \x05\x04\x19\x02\0\x04\x12\x04\xca\x01\x02\n\n\r\n\x05\x04\x19\x02\0\x05\
    \x12\x04\xca\x01\x0b\x11\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xca\x01\x12\
    \x18\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xca\x01\x1b\x1c\n\x0c\n\x04\x04\
    \x19\x02\x01\x12\x04\xcb\x01\x02\x1d\n\r\n\x05\x04\x19\x02\x01\x04\x12\
    \x04\xcb\x01\x02\n\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\xcb\x01\x0b\x11\
    \n\r\n\x05\x04\x19\x02\x01\x01\x12\x04\xcb\x01\x12\x18\n\r\n\x05\x04\x19\
    \x02\x01\x03\x12\x04\xcb\x01\x1b\x1c\n\x0c\n\x04\x04\x19\x02\x02\x12\x04\
    \xcc\x01\x02$\n\r\n\x05\x04\x19\x02\x02\x04\x12\x04\xcc\x01\x02\n\n\r\n\
    \x05\x04\x19\x02\x02\x05\x12\x04\xcc\x01\x0b\x11\n\r\n\x05\x04\x19\x02\
    \x02\x01\x12\x04\xcc\x01\x12\x1f\n\r\n\x05\x04\x19\x02\x02\x03\x12\x04\
    \xcc\x01\"#\n\x0c\n\x04\x04\x19\x02\x03\x12\x04\xcd\x01\x02\x1f\n\r\n\
    \x05\x04\x19\x02\x03\x04\x12\x04\xcd\x01\x02\n\n\r\n\x05\x04\x19\x02\x03\
    \x05\x12\x04\xcd\x01\x0b\x11\n\r\n\x05\x04\x19\x02\x03\x01\x12\x04\xcd\
    \x01\x12\x1a\n\r\n\x05\x04\x19\x02\x03\x03\x12\x04\xcd\x01\x1d\x1e\nr\n\
    \x02\x04\x1a\x12\x06\xd4\x01\0\xd6\x01\x01\x1ad*\n\x20This\x20message\
    \x20sends\x20a\x20request\x20to\x20data\x20node\x20get\x20a\x20specific\
    \x20setting\n\x20that\x20is\x20used\x20by\x20disk\x20balancer.\n\n\x0b\n\
    \x03\x04\x1a\x01\x12\x04\xd4\x01\x08'\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\
    \xd5\x01\x02\x1a\n\r\n\x05\x04\x1a\x02\0\x04\x12\x04\xd5\x01\x02\n\n\r\n\
    \x05\x04\x1a\x02\0\x05\x12\x04\xd5\x01\x0b\x11\n\r\n\x05\x04\x1a\x02\0\
    \x01\x12\x04\xd5\x01\x12\x15\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xd5\x01\
    \x18\x19\nW\n\x02\x04\x1b\x12\x06\xdb\x01\0\xdd\x01\x01\x1aI*\n\x20Respo\
    nse\x20that\x20describes\x20the\x20value\x20of\x20requested\x20disk\x20b\
    alancer\x20setting.\n\n\x0b\n\x03\x04\x1b\x01\x12\x04\xdb\x01\x08(\n\x0c\
    \n\x04\x04\x1b\x02\0\x12\x04\xdc\x01\x02\x1c\n\r\n\x05\x04\x1b\x02\0\x04\
    \x12\x04\xdc\x01\x02\n\n\r\n\x05\x04\x1b\x02\0\x05\x12\x04\xdc\x01\x0b\
    \x11\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\xdc\x01\x12\x17\n\r\n\x05\x04\
    \x1b\x02\0\x03\x12\x04\xdc\x01\x1a\x1b\ns\n\x02\x06\0\x12\x06\xe3\x01\0\
    \xb2\x02\x01\x1ae*\n\x20Protocol\x20used\x20from\x20client\x20to\x20the\
    \x20Datanode.\n\x20See\x20the\x20request\x20and\x20response\x20for\x20de\
    tails\x20of\x20rpc\x20call.\n\n\x0b\n\x03\x06\0\x01\x12\x04\xe3\x01\x08%\
    \n=\n\x04\x06\0\x02\0\x12\x06\xe7\x01\x02\xe8\x014\x1a-*\n\x20Returns\
    \x20the\x20visible\x20length\x20of\x20the\x20replica\n\n\r\n\x05\x06\0\
    \x02\0\x01\x12\x04\xe7\x01\x06\x1d\n\r\n\x05\x06\0\x02\0\x02\x12\x04\xe7\
    \x01\x1eA\n\r\n\x05\x06\0\x02\0\x03\x12\x04\xe8\x01\x0e2\n\x8c\x01\n\x04\
    \x06\0\x02\x01\x12\x06\xee\x01\x02\xef\x01-\x1a|*\n\x20Refresh\x20the\
    \x20list\x20of\x20federated\x20namenodes\x20from\x20updated\x20configura\
    tion.\n\x20Adds\x20new\x20namenodes\x20and\x20stops\x20the\x20deleted\
    \x20namenodes.\n\n\r\n\x05\x06\0\x02\x01\x01\x12\x04\xee\x01\x06\x16\n\r\
    \n\x05\x06\0\x02\x01\x02\x12\x04\xee\x01\x173\n\r\n\x05\x06\0\x02\x01\
    \x03\x12\x04\xef\x01\x0e+\n<\n\x04\x06\0\x02\x02\x12\x06\xf4\x01\x02\xf5\
    \x01,\x1a,*\n\x20Delete\x20the\x20block\x20pool\x20from\x20the\x20datano\
    de.\n\n\r\n\x05\x06\0\x02\x02\x01\x12\x04\xf4\x01\x06\x15\n\r\n\x05\x06\
    \0\x02\x02\x02\x12\x04\xf4\x01\x161\n\r\n\x05\x06\0\x02\x02\x03\x12\x04\
    \xf5\x01\x0e*\nr\n\x04\x06\0\x02\x03\x12\x06\xfb\x01\x02\xfc\x012\x1ab*\
    \n\x20Retrieves\x20the\x20path\x20names\x20of\x20the\x20block\x20file\
    \x20and\x20metadata\x20file\x20stored\x20on\x20the\n\x20local\x20file\
    \x20system.\n\n\r\n\x05\x06\0\x02\x03\x01\x12\x04\xfb\x01\x06\x1b\n\r\n\
    \x05\x06\0\x02\x03\x02\x12\x04\xfb\x01\x1c=\n\r\n\x05\x06\0\x02\x03\x03\
    \x12\x04\xfc\x01\x0e0\n\x0e\n\x04\x06\0\x02\x04\x12\x06\xfe\x01\x02\xff\
    \x01-\n\r\n\x05\x06\0\x02\x04\x01\x12\x04\xfe\x01\x06\x16\n\r\n\x05\x06\
    \0\x02\x04\x02\x12\x04\xfe\x01\x173\n\r\n\x05\x06\0\x02\x04\x03\x12\x04\
    \xff\x01\x0e+\n\x0e\n\x04\x06\0\x02\x05\x12\x06\x81\x02\x02\x82\x02)\n\r\
    \n\x05\x06\0\x02\x05\x01\x12\x04\x81\x02\x06\x12\n\r\n\x05\x06\0\x02\x05\
    \x02\x12\x04\x81\x02\x13+\n\r\n\x05\x06\0\x02\x05\x03\x12\x04\x82\x02\
    \x0e'\n\x0e\n\x04\x06\0\x02\x06\x12\x06\x84\x02\x02\x85\x02,\n\r\n\x05\
    \x06\0\x02\x06\x01\x12\x04\x84\x02\x06\x15\n\r\n\x05\x06\0\x02\x06\x02\
    \x12\x04\x84\x02\x161\n\r\n\x05\x06\0\x02\x06\x03\x12\x04\x85\x02\x0e*\n\
    \x0e\n\x04\x06\0\x02\x07\x12\x06\x87\x02\x02\x88\x02,\n\r\n\x05\x06\0\
    \x02\x07\x01\x12\x04\x87\x02\x06\x15\n\r\n\x05\x06\0\x02\x07\x02\x12\x04\
    \x87\x02\x161\n\r\n\x05\x06\0\x02\x07\x03\x12\x04\x88\x02\x0e*\n\x0e\n\
    \x04\x06\0\x02\x08\x12\x06\x8a\x02\x02\x8b\x025\n\r\n\x05\x06\0\x02\x08\
    \x01\x12\x04\x8a\x02\x06\x1e\n\r\n\x05\x06\0\x02\x08\x02\x12\x04\x8a\x02\
    \x1fC\n\r\n\x05\x06\0\x02\x08\x03\x12\x04\x8b\x02\x0e3\n\x0e\n\x04\x06\0\
    \x02\t\x12\x06\x8d\x02\x02\x8e\x021\n\r\n\x05\x06\0\x02\t\x01\x12\x04\
    \x8d\x02\x06\x1a\n\r\n\x05\x06\0\x02\t\x02\x12\x04\x8d\x02\x1b;\n\r\n\
    \x05\x06\0\x02\t\x03\x12\x04\x8e\x02\x0e/\n\x0e\n\x04\x06\0\x02\n\x12\
    \x06\x90\x02\x02\x92\x029\n\r\n\x05\x06\0\x02\n\x01\x12\x04\x90\x02\x06\
    \"\n\r\n\x05\x06\0\x02\n\x02\x12\x04\x91\x02\x06.\n\r\n\x05\x06\0\x02\n\
    \x03\x12\x04\x92\x02\x0e7\n\x0e\n\x04\x06\0\x02\x0b\x12\x06\x94\x02\x02\
    \x95\x02/\n\r\n\x05\x06\0\x02\x0b\x01\x12\x04\x94\x02\x06\x18\n\r\n\x05\
    \x06\0\x02\x0b\x02\x12\x04\x94\x02\x197\n\r\n\x05\x06\0\x02\x0b\x03\x12\
    \x04\x95\x02\x0e-\nE\n\x04\x06\0\x02\x0c\x12\x06\x9a\x02\x02\x9b\x021\
    \x1a5*\n\x20Returns\x20the\x20balancer\x20bandwidth\x20value\x20of\x20da\
    tanode.\n\n\r\n\x05\x06\0\x02\x0c\x01\x12\x04\x9a\x02\x06\x1a\n\r\n\x05\
    \x06\0\x02\x0c\x02\x12\x04\x9a\x02\x1b;\n\r\n\x05\x06\0\x02\x0c\x03\x12\
    \x04\x9b\x02\x0e/\n=\n\x04\x06\0\x02\r\x12\x06\xa0\x02\x02\xa1\x024\x1a-\
    *\n\x20Submit\x20a\x20disk\x20balancer\x20plan\x20for\x20execution\n\n\r\
    \n\x05\x06\0\x02\r\x01\x12\x04\xa0\x02\x06\x1c\n\r\n\x05\x06\0\x02\r\x02\
    \x12\x04\xa0\x02\x1d?\n\r\n\x05\x06\0\x02\r\x03\x12\x04\xa1\x02\x0f2\n,\
    \n\x04\x06\0\x02\x0e\x12\x06\xa5\x02\x02\xa6\x02(\x1a\x1c*\n\x20Cancel\
    \x20an\x20executing\x20plan\n\n\r\n\x05\x06\0\x02\x0e\x01\x12\x04\xa5\
    \x02\x06\x1c\n\r\n\x05\x06\0\x02\x0e\x02\x12\x04\xa5\x02\x1d3\n\r\n\x05\
    \x06\0\x02\x0e\x03\x12\x04\xa6\x02\x0f&\n8\n\x04\x06\0\x02\x0f\x12\x06\
    \xab\x02\x02\xac\x02-\x1a(*\n\x20Gets\x20the\x20status\x20of\x20an\x20ex\
    ecuting\x20Plan\n\n\r\n\x05\x06\0\x02\x0f\x01\x12\x04\xab\x02\x06\x1b\n\
    \r\n\x05\x06\0\x02\x0f\x02\x12\x04\xab\x02\x1c7\n\r\n\x05\x06\0\x02\x0f\
    \x03\x12\x04\xac\x02\x0f+\n=\n\x04\x06\0\x02\x10\x12\x06\xb0\x02\x02\xb1\
    \x020\x1a-*\n\x20\x20Gets\x20run-time\x20settings\x20of\x20Disk\x20Balan\
    cer.\n\n\r\n\x05\x06\0\x02\x10\x01\x12\x04\xb0\x02\x06\x1c\n\r\n\x05\x06\
    \0\x02\x10\x02\x12\x04\xb0\x02\x1d<\n\r\n\x05\x06\0\x02\x10\x03\x12\x04\
    \xb1\x02\x0e.\
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
