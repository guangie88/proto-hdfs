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
pub struct GetProtocolVersionsRequestProto {
    // message fields
    protocol: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProtocolVersionsRequestProto {}

impl GetProtocolVersionsRequestProto {
    pub fn new() -> GetProtocolVersionsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProtocolVersionsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetProtocolVersionsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProtocolVersionsRequestProto,
        };
        unsafe {
            instance.get(GetProtocolVersionsRequestProto::new)
        }
    }

    // required string protocol = 1;

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

impl ::protobuf::Message for GetProtocolVersionsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.protocol.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.protocol.as_ref() {
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

impl ::protobuf::MessageStatic for GetProtocolVersionsRequestProto {
    fn new() -> GetProtocolVersionsRequestProto {
        GetProtocolVersionsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProtocolVersionsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    GetProtocolVersionsRequestProto::get_protocol_for_reflect,
                    GetProtocolVersionsRequestProto::mut_protocol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProtocolVersionsRequestProto>(
                    "GetProtocolVersionsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProtocolVersionsRequestProto {
    fn clear(&mut self) {
        self.clear_protocol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetProtocolVersionsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetProtocolVersionsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProtocolVersionProto {
    // message fields
    rpcKind: ::protobuf::SingularField<::std::string::String>,
    versions: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProtocolVersionProto {}

impl ProtocolVersionProto {
    pub fn new() -> ProtocolVersionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProtocolVersionProto {
        static mut instance: ::protobuf::lazy::Lazy<ProtocolVersionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProtocolVersionProto,
        };
        unsafe {
            instance.get(ProtocolVersionProto::new)
        }
    }

    // required string rpcKind = 1;

    pub fn clear_rpcKind(&mut self) {
        self.rpcKind.clear();
    }

    pub fn has_rpcKind(&self) -> bool {
        self.rpcKind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcKind(&mut self, v: ::std::string::String) {
        self.rpcKind = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpcKind(&mut self) -> &mut ::std::string::String {
        if self.rpcKind.is_none() {
            self.rpcKind.set_default();
        }
        self.rpcKind.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpcKind(&mut self) -> ::std::string::String {
        self.rpcKind.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rpcKind(&self) -> &str {
        match self.rpcKind.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rpcKind_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rpcKind
    }

    fn mut_rpcKind_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rpcKind
    }

    // repeated uint64 versions = 2;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: ::std::vec::Vec<u64>) {
        self.versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_versions(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.versions
    }

    // Take field
    pub fn take_versions(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.versions, ::std::vec::Vec::new())
    }

    pub fn get_versions(&self) -> &[u64] {
        &self.versions
    }

    fn get_versions_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.versions
    }

    fn mut_versions_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.versions
    }
}

impl ::protobuf::Message for ProtocolVersionProto {
    fn is_initialized(&self) -> bool {
        if self.rpcKind.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rpcKind)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.versions)?;
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
        if let Some(ref v) = self.rpcKind.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.versions {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rpcKind.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.versions {
            os.write_uint64(2, *v)?;
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

impl ::protobuf::MessageStatic for ProtocolVersionProto {
    fn new() -> ProtocolVersionProto {
        ProtocolVersionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProtocolVersionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rpcKind",
                    ProtocolVersionProto::get_rpcKind_for_reflect,
                    ProtocolVersionProto::mut_rpcKind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "versions",
                    ProtocolVersionProto::get_versions_for_reflect,
                    ProtocolVersionProto::mut_versions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProtocolVersionProto>(
                    "ProtocolVersionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProtocolVersionProto {
    fn clear(&mut self) {
        self.clear_rpcKind();
        self.clear_versions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProtocolVersionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtocolVersionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetProtocolVersionsResponseProto {
    // message fields
    protocolVersions: ::protobuf::RepeatedField<ProtocolVersionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProtocolVersionsResponseProto {}

impl GetProtocolVersionsResponseProto {
    pub fn new() -> GetProtocolVersionsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProtocolVersionsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetProtocolVersionsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProtocolVersionsResponseProto,
        };
        unsafe {
            instance.get(GetProtocolVersionsResponseProto::new)
        }
    }

    // repeated .hadoop.common.ProtocolVersionProto protocolVersions = 1;

    pub fn clear_protocolVersions(&mut self) {
        self.protocolVersions.clear();
    }

    // Param is passed by value, moved
    pub fn set_protocolVersions(&mut self, v: ::protobuf::RepeatedField<ProtocolVersionProto>) {
        self.protocolVersions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_protocolVersions(&mut self) -> &mut ::protobuf::RepeatedField<ProtocolVersionProto> {
        &mut self.protocolVersions
    }

    // Take field
    pub fn take_protocolVersions(&mut self) -> ::protobuf::RepeatedField<ProtocolVersionProto> {
        ::std::mem::replace(&mut self.protocolVersions, ::protobuf::RepeatedField::new())
    }

    pub fn get_protocolVersions(&self) -> &[ProtocolVersionProto] {
        &self.protocolVersions
    }

    fn get_protocolVersions_for_reflect(&self) -> &::protobuf::RepeatedField<ProtocolVersionProto> {
        &self.protocolVersions
    }

    fn mut_protocolVersions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ProtocolVersionProto> {
        &mut self.protocolVersions
    }
}

impl ::protobuf::Message for GetProtocolVersionsResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.protocolVersions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.protocolVersions)?;
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
        for value in &self.protocolVersions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.protocolVersions {
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

impl ::protobuf::MessageStatic for GetProtocolVersionsResponseProto {
    fn new() -> GetProtocolVersionsResponseProto {
        GetProtocolVersionsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProtocolVersionsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProtocolVersionProto>>(
                    "protocolVersions",
                    GetProtocolVersionsResponseProto::get_protocolVersions_for_reflect,
                    GetProtocolVersionsResponseProto::mut_protocolVersions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProtocolVersionsResponseProto>(
                    "GetProtocolVersionsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProtocolVersionsResponseProto {
    fn clear(&mut self) {
        self.clear_protocolVersions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetProtocolVersionsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetProtocolVersionsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetProtocolSignatureRequestProto {
    // message fields
    protocol: ::protobuf::SingularField<::std::string::String>,
    rpcKind: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProtocolSignatureRequestProto {}

impl GetProtocolSignatureRequestProto {
    pub fn new() -> GetProtocolSignatureRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProtocolSignatureRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetProtocolSignatureRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProtocolSignatureRequestProto,
        };
        unsafe {
            instance.get(GetProtocolSignatureRequestProto::new)
        }
    }

    // required string protocol = 1;

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

    // required string rpcKind = 2;

    pub fn clear_rpcKind(&mut self) {
        self.rpcKind.clear();
    }

    pub fn has_rpcKind(&self) -> bool {
        self.rpcKind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcKind(&mut self, v: ::std::string::String) {
        self.rpcKind = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpcKind(&mut self) -> &mut ::std::string::String {
        if self.rpcKind.is_none() {
            self.rpcKind.set_default();
        }
        self.rpcKind.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpcKind(&mut self) -> ::std::string::String {
        self.rpcKind.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rpcKind(&self) -> &str {
        match self.rpcKind.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rpcKind_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rpcKind
    }

    fn mut_rpcKind_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rpcKind
    }
}

impl ::protobuf::Message for GetProtocolSignatureRequestProto {
    fn is_initialized(&self) -> bool {
        if self.protocol.is_none() {
            return false;
        }
        if self.rpcKind.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rpcKind)?;
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
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.rpcKind.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.protocol.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.rpcKind.as_ref() {
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

impl ::protobuf::MessageStatic for GetProtocolSignatureRequestProto {
    fn new() -> GetProtocolSignatureRequestProto {
        GetProtocolSignatureRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProtocolSignatureRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    GetProtocolSignatureRequestProto::get_protocol_for_reflect,
                    GetProtocolSignatureRequestProto::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rpcKind",
                    GetProtocolSignatureRequestProto::get_rpcKind_for_reflect,
                    GetProtocolSignatureRequestProto::mut_rpcKind_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProtocolSignatureRequestProto>(
                    "GetProtocolSignatureRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProtocolSignatureRequestProto {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_rpcKind();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetProtocolSignatureRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetProtocolSignatureRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetProtocolSignatureResponseProto {
    // message fields
    protocolSignature: ::protobuf::RepeatedField<ProtocolSignatureProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProtocolSignatureResponseProto {}

impl GetProtocolSignatureResponseProto {
    pub fn new() -> GetProtocolSignatureResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProtocolSignatureResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetProtocolSignatureResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProtocolSignatureResponseProto,
        };
        unsafe {
            instance.get(GetProtocolSignatureResponseProto::new)
        }
    }

    // repeated .hadoop.common.ProtocolSignatureProto protocolSignature = 1;

    pub fn clear_protocolSignature(&mut self) {
        self.protocolSignature.clear();
    }

    // Param is passed by value, moved
    pub fn set_protocolSignature(&mut self, v: ::protobuf::RepeatedField<ProtocolSignatureProto>) {
        self.protocolSignature = v;
    }

    // Mutable pointer to the field.
    pub fn mut_protocolSignature(&mut self) -> &mut ::protobuf::RepeatedField<ProtocolSignatureProto> {
        &mut self.protocolSignature
    }

    // Take field
    pub fn take_protocolSignature(&mut self) -> ::protobuf::RepeatedField<ProtocolSignatureProto> {
        ::std::mem::replace(&mut self.protocolSignature, ::protobuf::RepeatedField::new())
    }

    pub fn get_protocolSignature(&self) -> &[ProtocolSignatureProto] {
        &self.protocolSignature
    }

    fn get_protocolSignature_for_reflect(&self) -> &::protobuf::RepeatedField<ProtocolSignatureProto> {
        &self.protocolSignature
    }

    fn mut_protocolSignature_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ProtocolSignatureProto> {
        &mut self.protocolSignature
    }
}

impl ::protobuf::Message for GetProtocolSignatureResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.protocolSignature {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.protocolSignature)?;
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
        for value in &self.protocolSignature {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.protocolSignature {
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

impl ::protobuf::MessageStatic for GetProtocolSignatureResponseProto {
    fn new() -> GetProtocolSignatureResponseProto {
        GetProtocolSignatureResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProtocolSignatureResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProtocolSignatureProto>>(
                    "protocolSignature",
                    GetProtocolSignatureResponseProto::get_protocolSignature_for_reflect,
                    GetProtocolSignatureResponseProto::mut_protocolSignature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProtocolSignatureResponseProto>(
                    "GetProtocolSignatureResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProtocolSignatureResponseProto {
    fn clear(&mut self) {
        self.clear_protocolSignature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetProtocolSignatureResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetProtocolSignatureResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProtocolSignatureProto {
    // message fields
    version: ::std::option::Option<u64>,
    methods: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProtocolSignatureProto {}

impl ProtocolSignatureProto {
    pub fn new() -> ProtocolSignatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProtocolSignatureProto {
        static mut instance: ::protobuf::lazy::Lazy<ProtocolSignatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProtocolSignatureProto,
        };
        unsafe {
            instance.get(ProtocolSignatureProto::new)
        }
    }

    // required uint64 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // repeated uint32 methods = 2;

    pub fn clear_methods(&mut self) {
        self.methods.clear();
    }

    // Param is passed by value, moved
    pub fn set_methods(&mut self, v: ::std::vec::Vec<u32>) {
        self.methods = v;
    }

    // Mutable pointer to the field.
    pub fn mut_methods(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.methods
    }

    // Take field
    pub fn take_methods(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.methods, ::std::vec::Vec::new())
    }

    pub fn get_methods(&self) -> &[u32] {
        &self.methods
    }

    fn get_methods_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.methods
    }

    fn mut_methods_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.methods
    }
}

impl ::protobuf::Message for ProtocolSignatureProto {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.methods)?;
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.methods {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint64(1, v)?;
        }
        for v in &self.methods {
            os.write_uint32(2, *v)?;
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

impl ::protobuf::MessageStatic for ProtocolSignatureProto {
    fn new() -> ProtocolSignatureProto {
        ProtocolSignatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProtocolSignatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    ProtocolSignatureProto::get_version_for_reflect,
                    ProtocolSignatureProto::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "methods",
                    ProtocolSignatureProto::get_methods_for_reflect,
                    ProtocolSignatureProto::mut_methods_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProtocolSignatureProto>(
                    "ProtocolSignatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProtocolSignatureProto {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_methods();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProtocolSignatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtocolSignatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12ProtocolInfo.proto\x12\rhadoop.common\"=\n\x1fGetProtocolVersionsR\
    equestProto\x12\x1a\n\x08protocol\x18\x01\x20\x02(\tR\x08protocol\"L\n\
    \x14ProtocolVersionProto\x12\x18\n\x07rpcKind\x18\x01\x20\x02(\tR\x07rpc\
    Kind\x12\x1a\n\x08versions\x18\x02\x20\x03(\x04R\x08versions\"s\n\x20Get\
    ProtocolVersionsResponseProto\x12O\n\x10protocolVersions\x18\x01\x20\x03\
    (\x0b2#.hadoop.common.ProtocolVersionProtoR\x10protocolVersions\"X\n\x20\
    GetProtocolSignatureRequestProto\x12\x1a\n\x08protocol\x18\x01\x20\x02(\
    \tR\x08protocol\x12\x18\n\x07rpcKind\x18\x02\x20\x02(\tR\x07rpcKind\"x\n\
    !GetProtocolSignatureResponseProto\x12S\n\x11protocolSignature\x18\x01\
    \x20\x03(\x0b2%.hadoop.common.ProtocolSignatureProtoR\x11protocolSignatu\
    re\"L\n\x16ProtocolSignatureProto\x12\x18\n\x07version\x18\x01\x20\x02(\
    \x04R\x07version\x12\x18\n\x07methods\x18\x02\x20\x03(\rR\x07methods2\
    \x88\x02\n\x13ProtocolInfoService\x12v\n\x13getProtocolVersions\x12..had\
    oop.common.GetProtocolVersionsRequestProto\x1a/.hadoop.common.GetProtoco\
    lVersionsResponseProto\x12y\n\x14getProtocolSignature\x12/.hadoop.common\
    .GetProtocolSignatureRequestProto\x1a0.hadoop.common.GetProtocolSignatur\
    eResponseProtoB:\n\x1eorg.apache.hadoop.ipc.protobufB\x12ProtocolInfoPro\
    tos\xa0\x01\x01\x88\x01\x01J\xb6\x15\n\x06\x12\x04\x18\0X\x01\n\x08\n\
    \x01\x08\x12\x03\x18\07\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\072\
    \x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\
    \x20(ASF)\x20under\x20one\n\x20or\x20more\x20contributor\x20license\x20a\
    greements.\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\
    \x20this\x20work\x20for\x20additional\x20information\n\x20regarding\x20c\
    opyright\x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\
    \x20to\x20you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\n\x20\"License\");\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\n\x20with\x20the\x20License.\x20\x20You\
    \x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\
    \x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20\
    required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writi\
    ng,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20dis\
    tributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIE\
    S\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n2\xaa\x01*\n\x20These\x20.proto\x20interfaces\x20are\x20p\
    rivate\x20and\x20stable.\n\x20Please\x20see\x20http://wiki.apache.org/ha\
    doop/Compatibility\n\x20for\x20what\x20changes\x20are\x20allowed\x20for\
    \x20a\x20*stable*\x20.proto\x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\
    \x02\x12\x03\x18\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\x05\
    \x08\xe7\x07\0\x07\x12\x03\x18\x166\n\x08\n\x01\x08\x12\x03\x19\03\n\x0b\
    \n\x04\x08\xe7\x07\x01\x12\x03\x19\03\n\x0c\n\x05\x08\xe7\x07\x01\x02\
    \x12\x03\x19\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\
    \n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\
    \x08\xe7\x07\x01\x07\x12\x03\x19\x1e2\n\x08\n\x01\x08\x12\x03\x1a\0$\n\
    \x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x1a\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07\
    \x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1a\x07\x1c\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\x1a\x1f#\n\x08\n\x01\x08\x12\x03\x1b\0,\
    \n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1b\0,\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x1b\x07$\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x1b\x07$\n\
    \x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x1b\x07$\n\x0c\n\x05\x08\
    \xe7\x07\x03\x03\x12\x03\x1b'+\n\x08\n\x01\x02\x12\x03\x1c\x08\x15\nM\n\
    \x02\x04\0\x12\x04!\0#\x01\x1aA*\n\x20Request\x20to\x20get\x20protocol\
    \x20versions\x20for\x20all\x20supported\x20rpc\x20kinds.\n\n\n\n\x03\x04\
    \0\x01\x12\x03!\x08'\n\x1c\n\x04\x04\0\x02\0\x12\x03\"\x02\x1f\"\x0f\x20\
    Protocol\x20name\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\"\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\"\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\"\x12\x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\"\x1d\x1e\n=\n\x02\
    \x04\x01\x12\x04(\0+\x01\x1a1*\n\x20Protocol\x20version\x20with\x20corre\
    sponding\x20rpc\x20kind.\n\n\n\n\x03\x04\x01\x01\x12\x03(\x08\x1c\n\x16\
    \n\x04\x04\x01\x02\0\x12\x03)\x02\x1e\"\tRPC\x20kind\n\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03)\x12\x19\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03)\x1c\x1d\n=\n\x04\x04\x01\x02\x01\x12\x03*\x02\x1f\"0\
    Protocol\x20version\x20corresponding\x20to\x20the\x20rpc\x20kind.\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03*\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03*\x12\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03*\x1d\x1e\n.\n\x02\x04\x02\
    \x12\x040\02\x01\x1a\"*\n\x20Get\x20protocol\x20version\x20response.\n\n\
    \n\n\x03\x04\x02\x01\x12\x030\x08(\n\x0b\n\x04\x04\x02\x02\0\x12\x031\
    \x025\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x031\x02\n\n\x0c\n\x05\x04\x02\
    \x02\0\x06\x12\x031\x0b\x1f\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x031\x200\
    \n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03134\n/\n\x02\x04\x03\x12\x047\0:\
    \x01\x1a#*\n\x20Get\x20protocol\x20signature\x20request.\n\n\n\n\x03\x04\
    \x03\x01\x12\x037\x08(\n\x1c\n\x04\x04\x03\x02\0\x12\x038\x02\x1f\"\x0f\
    \x20Protocol\x20name\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x038\x02\n\n\
    \x0c\n\x05\x04\x03\x02\0\x05\x12\x038\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x038\x12\x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x038\x1d\x1e\n\
    \x17\n\x04\x04\x03\x02\x01\x12\x039\x02\x1e\"\n\x20RPC\x20kind\n\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x039\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x039\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x039\x12\x19\n\x0c\
    \n\x05\x04\x03\x02\x01\x03\x12\x039\x1c\x1d\n0\n\x02\x04\x04\x12\x04?\0A\
    \x01\x1a$*\n\x20Get\x20protocol\x20signature\x20response.\n\n\n\n\x03\
    \x04\x04\x01\x12\x03?\x08)\n\x0b\n\x04\x04\x04\x02\0\x12\x03@\x028\n\x0c\
    \n\x05\x04\x04\x02\0\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x04\x02\0\x06\
    \x12\x03@\x0b!\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03@\"3\n\x0c\n\x05\x04\
    \x04\x02\0\x03\x12\x03@67\n\n\n\x02\x04\x05\x12\x04C\0F\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03C\x08\x1e\n\x0b\n\x04\x04\x05\x02\0\x12\x03D\x02\x1e\
    \n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03D\x02\n\n\x0c\n\x05\x04\x05\x02\0\
    \x05\x12\x03D\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03D\x12\x19\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03D\x1c\x1d\n\x0b\n\x04\x04\x05\x02\
    \x01\x12\x03E\x02\x1e\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03E\x02\n\n\
    \x0c\n\x05\x04\x05\x02\x01\x05\x12\x03E\x0b\x11\n\x0c\n\x05\x04\x05\x02\
    \x01\x01\x12\x03E\x12\x19\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03E\x1c\
    \x1d\n<\n\x02\x06\0\x12\x04K\0X\x01\x1a0*\n\x20Protocol\x20to\x20get\x20\
    information\x20about\x20protocols.\n\n\n\n\x03\x06\0\x01\x12\x03K\x08\
    \x1b\nk\n\x04\x06\0\x02\0\x12\x04P\x02Q1\x1a]*\n\x20Return\x20protocol\
    \x20version\x20corresponding\x20to\x20protocol\x20interface\x20for\x20ea\
    ch\n\x20supported\x20rpc\x20kind.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03P\
    \x06\x19\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03P\x1a9\n\x0c\n\x05\x06\0\x02\
    \0\x03\x12\x03Q\x0f/\nN\n\x04\x06\0\x02\x01\x12\x04V\x02W2\x1a@*\n\x20Re\
    turn\x20protocol\x20version\x20corresponding\x20to\x20protocol\x20interf\
    ace.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03V\x06\x1a\n\x0c\n\x05\x06\0\
    \x02\x01\x02\x12\x03V\x1b;\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03W\x0f0\
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
