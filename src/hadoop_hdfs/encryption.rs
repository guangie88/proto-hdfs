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
pub struct CreateEncryptionZoneRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEncryptionZoneRequestProto {}

impl CreateEncryptionZoneRequestProto {
    pub fn new() -> CreateEncryptionZoneRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneRequestProto,
        };
        unsafe {
            instance.get(CreateEncryptionZoneRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }

    // optional string keyName = 2;

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
}

impl ::protobuf::Message for CreateEncryptionZoneRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
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

impl ::protobuf::MessageStatic for CreateEncryptionZoneRequestProto {
    fn new() -> CreateEncryptionZoneRequestProto {
        CreateEncryptionZoneRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    CreateEncryptionZoneRequestProto::get_src_for_reflect,
                    CreateEncryptionZoneRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    CreateEncryptionZoneRequestProto::get_keyName_for_reflect,
                    CreateEncryptionZoneRequestProto::mut_keyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneRequestProto>(
                    "CreateEncryptionZoneRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEncryptionZoneRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateEncryptionZoneResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEncryptionZoneResponseProto {}

impl CreateEncryptionZoneResponseProto {
    pub fn new() -> CreateEncryptionZoneResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneResponseProto,
        };
        unsafe {
            instance.get(CreateEncryptionZoneResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CreateEncryptionZoneResponseProto {
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

impl ::protobuf::MessageStatic for CreateEncryptionZoneResponseProto {
    fn new() -> CreateEncryptionZoneResponseProto {
        CreateEncryptionZoneResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneResponseProto>(
                    "CreateEncryptionZoneResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEncryptionZoneResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListEncryptionZonesRequestProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListEncryptionZonesRequestProto {}

impl ListEncryptionZonesRequestProto {
    pub fn new() -> ListEncryptionZonesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesRequestProto,
        };
        unsafe {
            instance.get(ListEncryptionZonesRequestProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
    }
}

impl ::protobuf::Message for ListEncryptionZonesRequestProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for ListEncryptionZonesRequestProto {
    fn new() -> ListEncryptionZonesRequestProto {
        ListEncryptionZonesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    ListEncryptionZonesRequestProto::get_id_for_reflect,
                    ListEncryptionZonesRequestProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesRequestProto>(
                    "ListEncryptionZonesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesRequestProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListEncryptionZonesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EncryptionZoneProto {
    // message fields
    id: ::std::option::Option<i64>,
    path: ::protobuf::SingularField<::std::string::String>,
    suite: ::std::option::Option<super::hdfs::CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<super::hdfs::CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncryptionZoneProto {}

impl EncryptionZoneProto {
    pub fn new() -> EncryptionZoneProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncryptionZoneProto {
        static mut instance: ::protobuf::lazy::Lazy<EncryptionZoneProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptionZoneProto,
        };
        unsafe {
            instance.get(EncryptionZoneProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
    }

    // required string path = 2;

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

    // required .hadoop.hdfs.CipherSuiteProto suite = 3;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: super::hdfs::CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> super::hdfs::CipherSuiteProto {
        self.suite.unwrap_or(super::hdfs::CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<super::hdfs::CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 4;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: super::hdfs::CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> super::hdfs::CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(super::hdfs::CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<super::hdfs::CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
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
}

impl ::protobuf::Message for EncryptionZoneProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
        if self.suite.is_none() {
            return false;
        }
        if self.cryptoProtocolVersion.is_none() {
            return false;
        }
        if self.keyName.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.suite = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.suite {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
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

impl ::protobuf::MessageStatic for EncryptionZoneProto {
    fn new() -> EncryptionZoneProto {
        EncryptionZoneProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncryptionZoneProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    EncryptionZoneProto::get_id_for_reflect,
                    EncryptionZoneProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    EncryptionZoneProto::get_path_for_reflect,
                    EncryptionZoneProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::CipherSuiteProto>>(
                    "suite",
                    EncryptionZoneProto::get_suite_for_reflect,
                    EncryptionZoneProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    EncryptionZoneProto::get_cryptoProtocolVersion_for_reflect,
                    EncryptionZoneProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    EncryptionZoneProto::get_keyName_for_reflect,
                    EncryptionZoneProto::mut_keyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptionZoneProto>(
                    "EncryptionZoneProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncryptionZoneProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_path();
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EncryptionZoneProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EncryptionZoneProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListEncryptionZonesResponseProto {
    // message fields
    zones: ::protobuf::RepeatedField<EncryptionZoneProto>,
    hasMore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListEncryptionZonesResponseProto {}

impl ListEncryptionZonesResponseProto {
    pub fn new() -> ListEncryptionZonesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesResponseProto,
        };
        unsafe {
            instance.get(ListEncryptionZonesResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.EncryptionZoneProto zones = 1;

    pub fn clear_zones(&mut self) {
        self.zones.clear();
    }

    // Param is passed by value, moved
    pub fn set_zones(&mut self, v: ::protobuf::RepeatedField<EncryptionZoneProto>) {
        self.zones = v;
    }

    // Mutable pointer to the field.
    pub fn mut_zones(&mut self) -> &mut ::protobuf::RepeatedField<EncryptionZoneProto> {
        &mut self.zones
    }

    // Take field
    pub fn take_zones(&mut self) -> ::protobuf::RepeatedField<EncryptionZoneProto> {
        ::std::mem::replace(&mut self.zones, ::protobuf::RepeatedField::new())
    }

    pub fn get_zones(&self) -> &[EncryptionZoneProto] {
        &self.zones
    }

    fn get_zones_for_reflect(&self) -> &::protobuf::RepeatedField<EncryptionZoneProto> {
        &self.zones
    }

    fn mut_zones_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EncryptionZoneProto> {
        &mut self.zones
    }

    // required bool hasMore = 2;

    pub fn clear_hasMore(&mut self) {
        self.hasMore = ::std::option::Option::None;
    }

    pub fn has_hasMore(&self) -> bool {
        self.hasMore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasMore(&mut self, v: bool) {
        self.hasMore = ::std::option::Option::Some(v);
    }

    pub fn get_hasMore(&self) -> bool {
        self.hasMore.unwrap_or(false)
    }

    fn get_hasMore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hasMore
    }

    fn mut_hasMore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hasMore
    }
}

impl ::protobuf::Message for ListEncryptionZonesResponseProto {
    fn is_initialized(&self) -> bool {
        if self.hasMore.is_none() {
            return false;
        }
        for v in &self.zones {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.zones)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hasMore = ::std::option::Option::Some(tmp);
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
        for value in &self.zones {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hasMore {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.zones {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hasMore {
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

impl ::protobuf::MessageStatic for ListEncryptionZonesResponseProto {
    fn new() -> ListEncryptionZonesResponseProto {
        ListEncryptionZonesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EncryptionZoneProto>>(
                    "zones",
                    ListEncryptionZonesResponseProto::get_zones_for_reflect,
                    ListEncryptionZonesResponseProto::mut_zones_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hasMore",
                    ListEncryptionZonesResponseProto::get_hasMore_for_reflect,
                    ListEncryptionZonesResponseProto::mut_hasMore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesResponseProto>(
                    "ListEncryptionZonesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesResponseProto {
    fn clear(&mut self) {
        self.clear_zones();
        self.clear_hasMore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListEncryptionZonesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReencryptEncryptionZoneRequestProto {
    // message fields
    action: ::std::option::Option<ReencryptActionProto>,
    zone: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReencryptEncryptionZoneRequestProto {}

impl ReencryptEncryptionZoneRequestProto {
    pub fn new() -> ReencryptEncryptionZoneRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReencryptEncryptionZoneRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ReencryptEncryptionZoneRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReencryptEncryptionZoneRequestProto,
        };
        unsafe {
            instance.get(ReencryptEncryptionZoneRequestProto::new)
        }
    }

    // required .hadoop.hdfs.ReencryptActionProto action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: ReencryptActionProto) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> ReencryptActionProto {
        self.action.unwrap_or(ReencryptActionProto::CANCEL_REENCRYPT)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<ReencryptActionProto> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<ReencryptActionProto> {
        &mut self.action
    }

    // required string zone = 2;

    pub fn clear_zone(&mut self) {
        self.zone.clear();
    }

    pub fn has_zone(&self) -> bool {
        self.zone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zone(&mut self, v: ::std::string::String) {
        self.zone = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zone(&mut self) -> &mut ::std::string::String {
        if self.zone.is_none() {
            self.zone.set_default();
        }
        self.zone.as_mut().unwrap()
    }

    // Take field
    pub fn take_zone(&mut self) -> ::std::string::String {
        self.zone.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_zone(&self) -> &str {
        match self.zone.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_zone_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.zone
    }

    fn mut_zone_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.zone
    }
}

impl ::protobuf::Message for ReencryptEncryptionZoneRequestProto {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        }
        if self.zone.is_none() {
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
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.zone)?;
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.zone.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.zone.as_ref() {
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

impl ::protobuf::MessageStatic for ReencryptEncryptionZoneRequestProto {
    fn new() -> ReencryptEncryptionZoneRequestProto {
        ReencryptEncryptionZoneRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReencryptEncryptionZoneRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReencryptActionProto>>(
                    "action",
                    ReencryptEncryptionZoneRequestProto::get_action_for_reflect,
                    ReencryptEncryptionZoneRequestProto::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "zone",
                    ReencryptEncryptionZoneRequestProto::get_zone_for_reflect,
                    ReencryptEncryptionZoneRequestProto::mut_zone_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReencryptEncryptionZoneRequestProto>(
                    "ReencryptEncryptionZoneRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReencryptEncryptionZoneRequestProto {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_zone();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReencryptEncryptionZoneRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReencryptEncryptionZoneRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReencryptEncryptionZoneResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReencryptEncryptionZoneResponseProto {}

impl ReencryptEncryptionZoneResponseProto {
    pub fn new() -> ReencryptEncryptionZoneResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReencryptEncryptionZoneResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ReencryptEncryptionZoneResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReencryptEncryptionZoneResponseProto,
        };
        unsafe {
            instance.get(ReencryptEncryptionZoneResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ReencryptEncryptionZoneResponseProto {
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

impl ::protobuf::MessageStatic for ReencryptEncryptionZoneResponseProto {
    fn new() -> ReencryptEncryptionZoneResponseProto {
        ReencryptEncryptionZoneResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReencryptEncryptionZoneResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ReencryptEncryptionZoneResponseProto>(
                    "ReencryptEncryptionZoneResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReencryptEncryptionZoneResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReencryptEncryptionZoneResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReencryptEncryptionZoneResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListReencryptionStatusRequestProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListReencryptionStatusRequestProto {}

impl ListReencryptionStatusRequestProto {
    pub fn new() -> ListReencryptionStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListReencryptionStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListReencryptionStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListReencryptionStatusRequestProto,
        };
        unsafe {
            instance.get(ListReencryptionStatusRequestProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
    }
}

impl ::protobuf::Message for ListReencryptionStatusRequestProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for ListReencryptionStatusRequestProto {
    fn new() -> ListReencryptionStatusRequestProto {
        ListReencryptionStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListReencryptionStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    ListReencryptionStatusRequestProto::get_id_for_reflect,
                    ListReencryptionStatusRequestProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListReencryptionStatusRequestProto>(
                    "ListReencryptionStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListReencryptionStatusRequestProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListReencryptionStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListReencryptionStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZoneReencryptionStatusProto {
    // message fields
    id: ::std::option::Option<i64>,
    path: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<ReencryptionStateProto>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    submissionTime: ::std::option::Option<i64>,
    canceled: ::std::option::Option<bool>,
    numReencrypted: ::std::option::Option<i64>,
    numFailures: ::std::option::Option<i64>,
    completionTime: ::std::option::Option<i64>,
    lastFile: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZoneReencryptionStatusProto {}

impl ZoneReencryptionStatusProto {
    pub fn new() -> ZoneReencryptionStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZoneReencryptionStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<ZoneReencryptionStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZoneReencryptionStatusProto,
        };
        unsafe {
            instance.get(ZoneReencryptionStatusProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
    }

    // required string path = 2;

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

    // required .hadoop.hdfs.ReencryptionStateProto state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ReencryptionStateProto) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> ReencryptionStateProto {
        self.state.unwrap_or(ReencryptionStateProto::SUBMITTED)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<ReencryptionStateProto> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<ReencryptionStateProto> {
        &mut self.state
    }

    // required string ezKeyVersionName = 4;

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

    // required int64 submissionTime = 5;

    pub fn clear_submissionTime(&mut self) {
        self.submissionTime = ::std::option::Option::None;
    }

    pub fn has_submissionTime(&self) -> bool {
        self.submissionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_submissionTime(&mut self, v: i64) {
        self.submissionTime = ::std::option::Option::Some(v);
    }

    pub fn get_submissionTime(&self) -> i64 {
        self.submissionTime.unwrap_or(0)
    }

    fn get_submissionTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.submissionTime
    }

    fn mut_submissionTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.submissionTime
    }

    // required bool canceled = 6;

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

    // required int64 numReencrypted = 7;

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

    // required int64 numFailures = 8;

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

    // optional int64 completionTime = 9;

    pub fn clear_completionTime(&mut self) {
        self.completionTime = ::std::option::Option::None;
    }

    pub fn has_completionTime(&self) -> bool {
        self.completionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_completionTime(&mut self, v: i64) {
        self.completionTime = ::std::option::Option::Some(v);
    }

    pub fn get_completionTime(&self) -> i64 {
        self.completionTime.unwrap_or(0)
    }

    fn get_completionTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.completionTime
    }

    fn mut_completionTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.completionTime
    }

    // optional string lastFile = 10;

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

impl ::protobuf::Message for ZoneReencryptionStatusProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
        if self.state.is_none() {
            return false;
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.submissionTime = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.canceled = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.numReencrypted = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.numFailures = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.completionTime = ::std::option::Option::Some(tmp);
                },
                10 => {
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.submissionTime {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.canceled {
            my_size += 2;
        }
        if let Some(v) = self.numReencrypted {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numFailures {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.completionTime {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.lastFile.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.submissionTime {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.canceled {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.numReencrypted {
            os.write_int64(7, v)?;
        }
        if let Some(v) = self.numFailures {
            os.write_int64(8, v)?;
        }
        if let Some(v) = self.completionTime {
            os.write_int64(9, v)?;
        }
        if let Some(ref v) = self.lastFile.as_ref() {
            os.write_string(10, &v)?;
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

impl ::protobuf::MessageStatic for ZoneReencryptionStatusProto {
    fn new() -> ZoneReencryptionStatusProto {
        ZoneReencryptionStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZoneReencryptionStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    ZoneReencryptionStatusProto::get_id_for_reflect,
                    ZoneReencryptionStatusProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    ZoneReencryptionStatusProto::get_path_for_reflect,
                    ZoneReencryptionStatusProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReencryptionStateProto>>(
                    "state",
                    ZoneReencryptionStatusProto::get_state_for_reflect,
                    ZoneReencryptionStatusProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    ZoneReencryptionStatusProto::get_ezKeyVersionName_for_reflect,
                    ZoneReencryptionStatusProto::mut_ezKeyVersionName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "submissionTime",
                    ZoneReencryptionStatusProto::get_submissionTime_for_reflect,
                    ZoneReencryptionStatusProto::mut_submissionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canceled",
                    ZoneReencryptionStatusProto::get_canceled_for_reflect,
                    ZoneReencryptionStatusProto::mut_canceled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "numReencrypted",
                    ZoneReencryptionStatusProto::get_numReencrypted_for_reflect,
                    ZoneReencryptionStatusProto::mut_numReencrypted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "numFailures",
                    ZoneReencryptionStatusProto::get_numFailures_for_reflect,
                    ZoneReencryptionStatusProto::mut_numFailures_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "completionTime",
                    ZoneReencryptionStatusProto::get_completionTime_for_reflect,
                    ZoneReencryptionStatusProto::mut_completionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lastFile",
                    ZoneReencryptionStatusProto::get_lastFile_for_reflect,
                    ZoneReencryptionStatusProto::mut_lastFile_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZoneReencryptionStatusProto>(
                    "ZoneReencryptionStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZoneReencryptionStatusProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_path();
        self.clear_state();
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

impl ::std::fmt::Debug for ZoneReencryptionStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZoneReencryptionStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListReencryptionStatusResponseProto {
    // message fields
    statuses: ::protobuf::RepeatedField<ZoneReencryptionStatusProto>,
    hasMore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListReencryptionStatusResponseProto {}

impl ListReencryptionStatusResponseProto {
    pub fn new() -> ListReencryptionStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListReencryptionStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListReencryptionStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListReencryptionStatusResponseProto,
        };
        unsafe {
            instance.get(ListReencryptionStatusResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.ZoneReencryptionStatusProto statuses = 1;

    pub fn clear_statuses(&mut self) {
        self.statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_statuses(&mut self, v: ::protobuf::RepeatedField<ZoneReencryptionStatusProto>) {
        self.statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statuses(&mut self) -> &mut ::protobuf::RepeatedField<ZoneReencryptionStatusProto> {
        &mut self.statuses
    }

    // Take field
    pub fn take_statuses(&mut self) -> ::protobuf::RepeatedField<ZoneReencryptionStatusProto> {
        ::std::mem::replace(&mut self.statuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_statuses(&self) -> &[ZoneReencryptionStatusProto] {
        &self.statuses
    }

    fn get_statuses_for_reflect(&self) -> &::protobuf::RepeatedField<ZoneReencryptionStatusProto> {
        &self.statuses
    }

    fn mut_statuses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ZoneReencryptionStatusProto> {
        &mut self.statuses
    }

    // required bool hasMore = 2;

    pub fn clear_hasMore(&mut self) {
        self.hasMore = ::std::option::Option::None;
    }

    pub fn has_hasMore(&self) -> bool {
        self.hasMore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasMore(&mut self, v: bool) {
        self.hasMore = ::std::option::Option::Some(v);
    }

    pub fn get_hasMore(&self) -> bool {
        self.hasMore.unwrap_or(false)
    }

    fn get_hasMore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hasMore
    }

    fn mut_hasMore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hasMore
    }
}

impl ::protobuf::Message for ListReencryptionStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.hasMore.is_none() {
            return false;
        }
        for v in &self.statuses {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.statuses)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hasMore = ::std::option::Option::Some(tmp);
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
        for value in &self.statuses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hasMore {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.statuses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hasMore {
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

impl ::protobuf::MessageStatic for ListReencryptionStatusResponseProto {
    fn new() -> ListReencryptionStatusResponseProto {
        ListReencryptionStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListReencryptionStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ZoneReencryptionStatusProto>>(
                    "statuses",
                    ListReencryptionStatusResponseProto::get_statuses_for_reflect,
                    ListReencryptionStatusResponseProto::mut_statuses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hasMore",
                    ListReencryptionStatusResponseProto::get_hasMore_for_reflect,
                    ListReencryptionStatusResponseProto::mut_hasMore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListReencryptionStatusResponseProto>(
                    "ListReencryptionStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListReencryptionStatusResponseProto {
    fn clear(&mut self) {
        self.clear_statuses();
        self.clear_hasMore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListReencryptionStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListReencryptionStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEZForPathRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEZForPathRequestProto {}

impl GetEZForPathRequestProto {
    pub fn new() -> GetEZForPathRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathRequestProto,
        };
        unsafe {
            instance.get(GetEZForPathRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }
}

impl ::protobuf::Message for GetEZForPathRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
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

impl ::protobuf::MessageStatic for GetEZForPathRequestProto {
    fn new() -> GetEZForPathRequestProto {
        GetEZForPathRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    GetEZForPathRequestProto::get_src_for_reflect,
                    GetEZForPathRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathRequestProto>(
                    "GetEZForPathRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEZForPathRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEZForPathRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEZForPathResponseProto {
    // message fields
    zone: ::protobuf::SingularPtrField<EncryptionZoneProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEZForPathResponseProto {}

impl GetEZForPathResponseProto {
    pub fn new() -> GetEZForPathResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathResponseProto,
        };
        unsafe {
            instance.get(GetEZForPathResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.EncryptionZoneProto zone = 1;

    pub fn clear_zone(&mut self) {
        self.zone.clear();
    }

    pub fn has_zone(&self) -> bool {
        self.zone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zone(&mut self, v: EncryptionZoneProto) {
        self.zone = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zone(&mut self) -> &mut EncryptionZoneProto {
        if self.zone.is_none() {
            self.zone.set_default();
        }
        self.zone.as_mut().unwrap()
    }

    // Take field
    pub fn take_zone(&mut self) -> EncryptionZoneProto {
        self.zone.take().unwrap_or_else(|| EncryptionZoneProto::new())
    }

    pub fn get_zone(&self) -> &EncryptionZoneProto {
        self.zone.as_ref().unwrap_or_else(|| EncryptionZoneProto::default_instance())
    }

    fn get_zone_for_reflect(&self) -> &::protobuf::SingularPtrField<EncryptionZoneProto> {
        &self.zone
    }

    fn mut_zone_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EncryptionZoneProto> {
        &mut self.zone
    }
}

impl ::protobuf::Message for GetEZForPathResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.zone {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.zone)?;
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
        if let Some(ref v) = self.zone.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.zone.as_ref() {
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

impl ::protobuf::MessageStatic for GetEZForPathResponseProto {
    fn new() -> GetEZForPathResponseProto {
        GetEZForPathResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EncryptionZoneProto>>(
                    "zone",
                    GetEZForPathResponseProto::get_zone_for_reflect,
                    GetEZForPathResponseProto::mut_zone_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathResponseProto>(
                    "GetEZForPathResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathResponseProto {
    fn clear(&mut self) {
        self.clear_zone();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEZForPathResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEZForPathResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReencryptActionProto {
    CANCEL_REENCRYPT = 1,
    START_REENCRYPT = 2,
}

impl ::protobuf::ProtobufEnum for ReencryptActionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReencryptActionProto> {
        match value {
            1 => ::std::option::Option::Some(ReencryptActionProto::CANCEL_REENCRYPT),
            2 => ::std::option::Option::Some(ReencryptActionProto::START_REENCRYPT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReencryptActionProto] = &[
            ReencryptActionProto::CANCEL_REENCRYPT,
            ReencryptActionProto::START_REENCRYPT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReencryptActionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReencryptActionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReencryptActionProto {
}

impl ::protobuf::reflect::ProtobufValue for ReencryptActionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReencryptionStateProto {
    SUBMITTED = 1,
    PROCESSING = 2,
    COMPLETED = 3,
}

impl ::protobuf::ProtobufEnum for ReencryptionStateProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReencryptionStateProto> {
        match value {
            1 => ::std::option::Option::Some(ReencryptionStateProto::SUBMITTED),
            2 => ::std::option::Option::Some(ReencryptionStateProto::PROCESSING),
            3 => ::std::option::Option::Some(ReencryptionStateProto::COMPLETED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReencryptionStateProto] = &[
            ReencryptionStateProto::SUBMITTED,
            ReencryptionStateProto::PROCESSING,
            ReencryptionStateProto::COMPLETED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReencryptionStateProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReencryptionStateProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReencryptionStateProto {
}

impl ::protobuf::reflect::ProtobufValue for ReencryptionStateProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10encryption.proto\x12\x0bhadoop.hdfs\x1a\nhdfs.proto\"N\n\x20Create\
    EncryptionZoneRequestProto\x12\x10\n\x03src\x18\x01\x20\x02(\tR\x03src\
    \x12\x18\n\x07keyName\x18\x02\x20\x01(\tR\x07keyName\"#\n!CreateEncrypti\
    onZoneResponseProto\"1\n\x1fListEncryptionZonesRequestProto\x12\x0e\n\
    \x02id\x18\x01\x20\x02(\x03R\x02id\"\xe7\x01\n\x13EncryptionZoneProto\
    \x12\x0e\n\x02id\x18\x01\x20\x02(\x03R\x02id\x12\x12\n\x04path\x18\x02\
    \x20\x02(\tR\x04path\x123\n\x05suite\x18\x03\x20\x02(\x0e2\x1d.hadoop.hd\
    fs.CipherSuiteProtoR\x05suite\x12]\n\x15cryptoProtocolVersion\x18\x04\
    \x20\x02(\x0e2'.hadoop.hdfs.CryptoProtocolVersionProtoR\x15cryptoProtoco\
    lVersion\x12\x18\n\x07keyName\x18\x05\x20\x02(\tR\x07keyName\"t\n\x20Lis\
    tEncryptionZonesResponseProto\x126\n\x05zones\x18\x01\x20\x03(\x0b2\x20.\
    hadoop.hdfs.EncryptionZoneProtoR\x05zones\x12\x18\n\x07hasMore\x18\x02\
    \x20\x02(\x08R\x07hasMore\"t\n#ReencryptEncryptionZoneRequestProto\x129\
    \n\x06action\x18\x01\x20\x02(\x0e2!.hadoop.hdfs.ReencryptActionProtoR\
    \x06action\x12\x12\n\x04zone\x18\x02\x20\x02(\tR\x04zone\"&\n$ReencryptE\
    ncryptionZoneResponseProto\"4\n\"ListReencryptionStatusRequestProto\x12\
    \x0e\n\x02id\x18\x01\x20\x02(\x03R\x02id\"\xfa\x02\n\x1bZoneReencryption\
    StatusProto\x12\x0e\n\x02id\x18\x01\x20\x02(\x03R\x02id\x12\x12\n\x04pat\
    h\x18\x02\x20\x02(\tR\x04path\x129\n\x05state\x18\x03\x20\x02(\x0e2#.had\
    oop.hdfs.ReencryptionStateProtoR\x05state\x12*\n\x10ezKeyVersionName\x18\
    \x04\x20\x02(\tR\x10ezKeyVersionName\x12&\n\x0esubmissionTime\x18\x05\
    \x20\x02(\x03R\x0esubmissionTime\x12\x1a\n\x08canceled\x18\x06\x20\x02(\
    \x08R\x08canceled\x12&\n\x0enumReencrypted\x18\x07\x20\x02(\x03R\x0enumR\
    eencrypted\x12\x20\n\x0bnumFailures\x18\x08\x20\x02(\x03R\x0bnumFailures\
    \x12&\n\x0ecompletionTime\x18\t\x20\x01(\x03R\x0ecompletionTime\x12\x1a\
    \n\x08lastFile\x18\n\x20\x01(\tR\x08lastFile\"\x85\x01\n#ListReencryptio\
    nStatusResponseProto\x12D\n\x08statuses\x18\x01\x20\x03(\x0b2(.hadoop.hd\
    fs.ZoneReencryptionStatusProtoR\x08statuses\x12\x18\n\x07hasMore\x18\x02\
    \x20\x02(\x08R\x07hasMore\",\n\x18GetEZForPathRequestProto\x12\x10\n\x03\
    src\x18\x01\x20\x02(\tR\x03src\"Q\n\x19GetEZForPathResponseProto\x124\n\
    \x04zone\x18\x01\x20\x01(\x0b2\x20.hadoop.hdfs.EncryptionZoneProtoR\x04z\
    one*A\n\x14ReencryptActionProto\x12\x14\n\x10CANCEL_REENCRYPT\x10\x01\
    \x12\x13\n\x0fSTART_REENCRYPT\x10\x02*F\n\x16ReencryptionStateProto\x12\
    \r\n\tSUBMITTED\x10\x01\x12\x0e\n\nPROCESSING\x10\x02\x12\r\n\tCOMPLETED\
    \x10\x03BA\n%org.apache.hadoop.hdfs.protocol.protoB\x15EncryptionZonesPr\
    otos\xa0\x01\x01J\xae\x1d\n\x06\x12\x04\x1c\0k\x01\n\x08\n\x01\x08\x12\
    \x03\x1c\0>\n\xc1\x08\n\x04\x08\xe7\x07\0\x12\x03\x1c\0>2\x83\x06*\n\x20\
    Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\x20un\
    der\x20one\n\x20or\x20more\x20contributor\x20license\x20agreements.\x20\
    \x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\x20wor\
    k\x20for\x20additional\x20information\n\x20regarding\x20copyright\x20own\
    ership.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20you\x20\
    under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\n\x20\"Lice\
    nse\");\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20c\
    ompliance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20obtain\x20a\
    \x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www\
    .apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20appl\
    icable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20d\
    istributed\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\
    \x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITION\
    S\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\
    \x20the\x20License\x20for\x20the\x20specific\x20language\x20governing\
    \x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\n2\
    \xaa\x01*\n\x20These\x20.proto\x20interfaces\x20are\x20private\x20and\
    \x20stable.\n\x20Please\x20see\x20http://wiki.apache.org/hadoop/Compatib\
    ility\n\x20for\x20what\x20changes\x20are\x20allowed\x20for\x20a\x20*stab\
    le*\x20.proto\x20interface.\n2\x80\x01\x20This\x20file\x20contains\x20pr\
    otocol\x20buffers\x20that\x20are\x20used\x20throughout\x20HDFS\x20--\x20\
    i.e.\n\x20by\x20the\x20client,\x20server,\x20and\x20data\x20transfer\x20\
    protocols.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x1c\x07\x13\n\r\n\x06\
    \x08\xe7\x07\0\x02\0\x12\x03\x1c\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\x1c\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x1c\x16=\
    \n\x08\n\x01\x08\x12\x03\x1d\06\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x1d\
    \06\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x1d\x07\x1b\n\r\n\x06\x08\
    \xe7\x07\x01\x02\0\x12\x03\x1d\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\
    \x01\x12\x03\x1d\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x1d\x1e\
    5\n\x08\n\x01\x08\x12\x03\x1e\0,\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1e\
    \0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1e\x07$\n\r\n\x06\x08\xe7\
    \x07\x02\x02\0\x12\x03\x1e\x07$\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\
    \x12\x03\x1e\x07$\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1e'+\n\x08\n\
    \x01\x02\x12\x03\x1f\x08\x13\n\t\n\x02\x03\0\x12\x03!\x07\x13\n\n\n\x02\
    \x04\0\x12\x04#\0&\x01\n\n\n\x03\x04\0\x01\x12\x03#\x08(\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03$\x02\x1a\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03$\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03$\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03$\x12\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03$\x18\x19\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03%\x02\x1e\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x03%\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03%\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03%\x12\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03%\x1c\x1d\n\n\n\x02\x04\x01\x12\x04(\0)\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03(\x08)\n\n\n\x02\x04\x02\x12\x04+\0-\x01\n\n\n\x03\x04\x02\x01\
    \x12\x03+\x08'\n\x0b\n\x04\x04\x02\x02\0\x12\x03,\x02\x18\n\x0c\n\x05\
    \x04\x02\x02\0\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03,\
    \x0b\x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03,\x11\x13\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03,\x16\x17\n\n\n\x02\x04\x03\x12\x04/\05\x01\n\n\n\
    \x03\x04\x03\x01\x12\x03/\x08\x1b\n\x0b\n\x04\x04\x03\x02\0\x12\x030\x02\
    \x18\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x03\
    \x02\0\x05\x12\x030\x0b\x10\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x030\x11\
    \x13\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x030\x16\x17\n\x0b\n\x04\x04\x03\
    \x02\x01\x12\x031\x02\x1b\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x031\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x031\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x01\x01\x12\x031\x12\x16\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x031\
    \x19\x1a\n\x0b\n\x04\x04\x03\x02\x02\x12\x032\x02&\n\x0c\n\x05\x04\x03\
    \x02\x02\x04\x12\x032\x02\n\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x032\x0b\
    \x1b\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x032\x1c!\n\x0c\n\x05\x04\x03\
    \x02\x02\x03\x12\x032$%\n\x0b\n\x04\x04\x03\x02\x03\x12\x033\x02@\n\x0c\
    \n\x05\x04\x03\x02\x03\x04\x12\x033\x02\n\n\x0c\n\x05\x04\x03\x02\x03\
    \x06\x12\x033\x0b%\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x033&;\n\x0c\n\
    \x05\x04\x03\x02\x03\x03\x12\x033>?\n\x0b\n\x04\x04\x03\x02\x04\x12\x034\
    \x02\x1e\n\x0c\n\x05\x04\x03\x02\x04\x04\x12\x034\x02\n\n\x0c\n\x05\x04\
    \x03\x02\x04\x05\x12\x034\x0b\x11\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\
    \x034\x12\x19\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x034\x1c\x1d\n\n\n\x02\
    \x04\x04\x12\x047\0:\x01\n\n\n\x03\x04\x04\x01\x12\x037\x08(\n\x0b\n\x04\
    \x04\x04\x02\0\x12\x038\x02)\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x038\x02\
    \n\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x038\x0b\x1e\n\x0c\n\x05\x04\x04\
    \x02\0\x01\x12\x038\x1f$\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x038'(\n\x0b\
    \n\x04\x04\x04\x02\x01\x12\x039\x02\x1c\n\x0c\n\x05\x04\x04\x02\x01\x04\
    \x12\x039\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x039\x0b\x0f\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x039\x10\x17\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x039\x1a\x1b\n\n\n\x02\x05\0\x12\x04<\0?\x01\n\n\n\x03\x05\0\
    \x01\x12\x03<\x05\x19\n\x0b\n\x04\x05\0\x02\0\x12\x03=\x02\x17\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03=\x02\x12\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03=\x15\x16\n\x0b\n\x04\x05\0\x02\x01\x12\x03>\x02\x16\n\x0c\n\x05\x05\
    \0\x02\x01\x01\x12\x03>\x02\x11\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03>\
    \x14\x15\n\n\n\x02\x04\x05\x12\x04A\0D\x01\n\n\n\x03\x04\x05\x01\x12\x03\
    A\x08+\n\x0b\n\x04\x04\x05\x02\0\x12\x03B\x02+\n\x0c\n\x05\x04\x05\x02\0\
    \x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03B\x0b\x1f\n\x0c\
    \n\x05\x04\x05\x02\0\x01\x12\x03B\x20&\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03B)*\n\x0b\n\x04\x04\x05\x02\x01\x12\x03C\x02\x1b\n\x0c\n\x05\x04\x05\
    \x02\x01\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03C\x0b\
    \x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03C\x12\x16\n\x0c\n\x05\x04\
    \x05\x02\x01\x03\x12\x03C\x19\x1a\n\n\n\x02\x04\x06\x12\x04F\0G\x01\n\n\
    \n\x03\x04\x06\x01\x12\x03F\x08,\n\n\n\x02\x04\x07\x12\x04I\0K\x01\n\n\n\
    \x03\x04\x07\x01\x12\x03I\x08*\n\x0b\n\x04\x04\x07\x02\0\x12\x03J\x02\
    \x18\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\x07\
    \x02\0\x05\x12\x03J\x0b\x10\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03J\x11\
    \x13\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03J\x16\x17\n\n\n\x02\x05\x01\
    \x12\x04M\0Q\x01\n\n\n\x03\x05\x01\x01\x12\x03M\x05\x1b\n\x0b\n\x04\x05\
    \x01\x02\0\x12\x03N\x02\x10\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03N\x02\
    \x0b\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03N\x0e\x0f\n\x0b\n\x04\x05\x01\
    \x02\x01\x12\x03O\x02\x11\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03O\x02\
    \x0c\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03O\x0f\x10\n\x0b\n\x04\x05\
    \x01\x02\x02\x12\x03P\x02\x10\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03P\
    \x02\x0b\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03P\x0e\x0f\n\n\n\x02\x04\
    \x08\x12\x04S\0^\x01\n\n\n\x03\x04\x08\x01\x12\x03S\x08#\n\x0b\n\x04\x04\
    \x08\x02\0\x12\x03T\x02\x18\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03T\x02\n\
    \n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03T\x0b\x10\n\x0c\n\x05\x04\x08\x02\
    \0\x01\x12\x03T\x11\x13\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03T\x16\x17\n\
    \x0b\n\x04\x04\x08\x02\x01\x12\x03U\x02\x1b\n\x0c\n\x05\x04\x08\x02\x01\
    \x04\x12\x03U\x02\n\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03U\x0b\x11\n\
    \x0c\n\x05\x04\x08\x02\x01\x01\x12\x03U\x12\x16\n\x0c\n\x05\x04\x08\x02\
    \x01\x03\x12\x03U\x19\x1a\n\x0b\n\x04\x04\x08\x02\x02\x12\x03V\x02,\n\
    \x0c\n\x05\x04\x08\x02\x02\x04\x12\x03V\x02\n\n\x0c\n\x05\x04\x08\x02\
    \x02\x06\x12\x03V\x0b!\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03V\"'\n\x0c\
    \n\x05\x04\x08\x02\x02\x03\x12\x03V*+\n\x0b\n\x04\x04\x08\x02\x03\x12\
    \x03W\x02'\n\x0c\n\x05\x04\x08\x02\x03\x04\x12\x03W\x02\n\n\x0c\n\x05\
    \x04\x08\x02\x03\x05\x12\x03W\x0b\x11\n\x0c\n\x05\x04\x08\x02\x03\x01\
    \x12\x03W\x12\"\n\x0c\n\x05\x04\x08\x02\x03\x03\x12\x03W%&\n\x0b\n\x04\
    \x04\x08\x02\x04\x12\x03X\x02$\n\x0c\n\x05\x04\x08\x02\x04\x04\x12\x03X\
    \x02\n\n\x0c\n\x05\x04\x08\x02\x04\x05\x12\x03X\x0b\x10\n\x0c\n\x05\x04\
    \x08\x02\x04\x01\x12\x03X\x11\x1f\n\x0c\n\x05\x04\x08\x02\x04\x03\x12\
    \x03X\"#\n\x0b\n\x04\x04\x08\x02\x05\x12\x03Y\x02\x1d\n\x0c\n\x05\x04\
    \x08\x02\x05\x04\x12\x03Y\x02\n\n\x0c\n\x05\x04\x08\x02\x05\x05\x12\x03Y\
    \x0b\x0f\n\x0c\n\x05\x04\x08\x02\x05\x01\x12\x03Y\x10\x18\n\x0c\n\x05\
    \x04\x08\x02\x05\x03\x12\x03Y\x1b\x1c\n\x0b\n\x04\x04\x08\x02\x06\x12\
    \x03Z\x02$\n\x0c\n\x05\x04\x08\x02\x06\x04\x12\x03Z\x02\n\n\x0c\n\x05\
    \x04\x08\x02\x06\x05\x12\x03Z\x0b\x10\n\x0c\n\x05\x04\x08\x02\x06\x01\
    \x12\x03Z\x11\x1f\n\x0c\n\x05\x04\x08\x02\x06\x03\x12\x03Z\"#\n\x0b\n\
    \x04\x04\x08\x02\x07\x12\x03[\x02!\n\x0c\n\x05\x04\x08\x02\x07\x04\x12\
    \x03[\x02\n\n\x0c\n\x05\x04\x08\x02\x07\x05\x12\x03[\x0b\x10\n\x0c\n\x05\
    \x04\x08\x02\x07\x01\x12\x03[\x11\x1c\n\x0c\n\x05\x04\x08\x02\x07\x03\
    \x12\x03[\x1f\x20\n\x0b\n\x04\x04\x08\x02\x08\x12\x03\\\x02$\n\x0c\n\x05\
    \x04\x08\x02\x08\x04\x12\x03\\\x02\n\n\x0c\n\x05\x04\x08\x02\x08\x05\x12\
    \x03\\\x0b\x10\n\x0c\n\x05\x04\x08\x02\x08\x01\x12\x03\\\x11\x1f\n\x0c\n\
    \x05\x04\x08\x02\x08\x03\x12\x03\\\"#\n\x0b\n\x04\x04\x08\x02\t\x12\x03]\
    \x02\x20\n\x0c\n\x05\x04\x08\x02\t\x04\x12\x03]\x02\n\n\x0c\n\x05\x04\
    \x08\x02\t\x05\x12\x03]\x0b\x11\n\x0c\n\x05\x04\x08\x02\t\x01\x12\x03]\
    \x12\x1a\n\x0c\n\x05\x04\x08\x02\t\x03\x12\x03]\x1d\x1f\n\n\n\x02\x04\t\
    \x12\x04`\0c\x01\n\n\n\x03\x04\t\x01\x12\x03`\x08+\n\x0b\n\x04\x04\t\x02\
    \0\x12\x03a\x024\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03a\x02\n\n\x0c\n\x05\
    \x04\t\x02\0\x06\x12\x03a\x0b&\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03a'/\n\
    \x0c\n\x05\x04\t\x02\0\x03\x12\x03a23\n\x0b\n\x04\x04\t\x02\x01\x12\x03b\
    \x02\x1c\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03b\x02\n\n\x0c\n\x05\x04\t\
    \x02\x01\x05\x12\x03b\x0b\x0f\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03b\x10\
    \x17\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03b\x1a\x1b\n\n\n\x02\x04\n\x12\
    \x04e\0g\x01\n\n\n\x03\x04\n\x01\x12\x03e\x08\x20\n\x0b\n\x04\x04\n\x02\
    \0\x12\x03f\x04\x1c\n\x0c\n\x05\x04\n\x02\0\x04\x12\x03f\x04\x0c\n\x0c\n\
    \x05\x04\n\x02\0\x05\x12\x03f\r\x13\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03f\
    \x14\x17\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03f\x1a\x1b\n\n\n\x02\x04\x0b\
    \x12\x04i\0k\x01\n\n\n\x03\x04\x0b\x01\x12\x03i\x08!\n\x0b\n\x04\x04\x0b\
    \x02\0\x12\x03j\x04*\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03j\x04\x0c\n\
    \x0c\n\x05\x04\x0b\x02\0\x06\x12\x03j\r\x20\n\x0c\n\x05\x04\x0b\x02\0\
    \x01\x12\x03j!%\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03j()\
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
