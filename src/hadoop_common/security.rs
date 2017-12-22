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
pub struct TokenProto {
    // message fields
    identifier: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    password: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    kind: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TokenProto {}

impl TokenProto {
    pub fn new() -> TokenProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TokenProto {
        static mut instance: ::protobuf::lazy::Lazy<TokenProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TokenProto,
        };
        unsafe {
            instance.get(TokenProto::new)
        }
    }

    // required bytes identifier = 1;

    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: ::std::vec::Vec<u8>) {
        self.identifier = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> ::std::vec::Vec<u8> {
        self.identifier.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_identifier(&self) -> &[u8] {
        match self.identifier.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_identifier_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.identifier
    }

    fn mut_identifier_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.identifier
    }

    // required bytes password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.password.is_none() {
            self.password.set_default();
        }
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        self.password.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        match self.password.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_password_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.password
    }

    // required string kind = 3;

    pub fn clear_kind(&mut self) {
        self.kind.clear();
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kind(&mut self) -> &mut ::std::string::String {
        if self.kind.is_none() {
            self.kind.set_default();
        }
        self.kind.as_mut().unwrap()
    }

    // Take field
    pub fn take_kind(&mut self) -> ::std::string::String {
        self.kind.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_kind(&self) -> &str {
        match self.kind.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_kind_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.kind
    }

    // required string service = 4;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }
}

impl ::protobuf::Message for TokenProto {
    fn is_initialized(&self) -> bool {
        if self.identifier.is_none() {
            return false;
        }
        if self.password.is_none() {
            return false;
        }
        if self.kind.is_none() {
            return false;
        }
        if self.service.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.identifier)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.password)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.kind)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
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
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.password.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.kind.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.identifier.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.password.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.kind.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.service.as_ref() {
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

impl ::protobuf::MessageStatic for TokenProto {
    fn new() -> TokenProto {
        TokenProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TokenProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "identifier",
                    TokenProto::get_identifier_for_reflect,
                    TokenProto::mut_identifier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    TokenProto::get_password_for_reflect,
                    TokenProto::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kind",
                    TokenProto::get_kind_for_reflect,
                    TokenProto::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    TokenProto::get_service_for_reflect,
                    TokenProto::mut_service_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TokenProto>(
                    "TokenProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TokenProto {
    fn clear(&mut self) {
        self.clear_identifier();
        self.clear_password();
        self.clear_kind();
        self.clear_service();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CredentialsKVProto {
    // message fields
    alias: ::protobuf::SingularField<::std::string::String>,
    token: ::protobuf::SingularPtrField<TokenProto>,
    secret: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CredentialsKVProto {}

impl CredentialsKVProto {
    pub fn new() -> CredentialsKVProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CredentialsKVProto {
        static mut instance: ::protobuf::lazy::Lazy<CredentialsKVProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CredentialsKVProto,
        };
        unsafe {
            instance.get(CredentialsKVProto::new)
        }
    }

    // required string alias = 1;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::std::string::String) {
        self.alias = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alias(&mut self) -> &mut ::std::string::String {
        if self.alias.is_none() {
            self.alias.set_default();
        }
        self.alias.as_mut().unwrap()
    }

    // Take field
    pub fn take_alias(&mut self) -> ::std::string::String {
        self.alias.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_alias(&self) -> &str {
        match self.alias.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_alias_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.alias
    }

    // optional .hadoop.common.TokenProto token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> TokenProto {
        self.token.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_token(&self) -> &TokenProto {
        self.token.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenProto> {
        &mut self.token
    }

    // optional bytes secret = 3;

    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    pub fn has_secret(&self) -> bool {
        self.secret.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.secret.is_none() {
            self.secret.set_default();
        }
        self.secret.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret(&mut self) -> ::std::vec::Vec<u8> {
        self.secret.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_secret(&self) -> &[u8] {
        match self.secret.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_secret_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.secret
    }

    fn mut_secret_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.secret
    }
}

impl ::protobuf::Message for CredentialsKVProto {
    fn is_initialized(&self) -> bool {
        if self.alias.is_none() {
            return false;
        }
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.alias)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.secret)?;
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
        if let Some(ref v) = self.alias.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.secret.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.alias.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.secret.as_ref() {
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

impl ::protobuf::MessageStatic for CredentialsKVProto {
    fn new() -> CredentialsKVProto {
        CredentialsKVProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CredentialsKVProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    CredentialsKVProto::get_alias_for_reflect,
                    CredentialsKVProto::mut_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenProto>>(
                    "token",
                    CredentialsKVProto::get_token_for_reflect,
                    CredentialsKVProto::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "secret",
                    CredentialsKVProto::get_secret_for_reflect,
                    CredentialsKVProto::mut_secret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CredentialsKVProto>(
                    "CredentialsKVProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CredentialsKVProto {
    fn clear(&mut self) {
        self.clear_alias();
        self.clear_token();
        self.clear_secret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CredentialsKVProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CredentialsKVProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CredentialsProto {
    // message fields
    tokens: ::protobuf::RepeatedField<CredentialsKVProto>,
    secrets: ::protobuf::RepeatedField<CredentialsKVProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CredentialsProto {}

impl CredentialsProto {
    pub fn new() -> CredentialsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CredentialsProto {
        static mut instance: ::protobuf::lazy::Lazy<CredentialsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CredentialsProto,
        };
        unsafe {
            instance.get(CredentialsProto::new)
        }
    }

    // repeated .hadoop.common.CredentialsKVProto tokens = 1;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<CredentialsKVProto>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens(&mut self) -> &mut ::protobuf::RepeatedField<CredentialsKVProto> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<CredentialsKVProto> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens(&self) -> &[CredentialsKVProto] {
        &self.tokens
    }

    fn get_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<CredentialsKVProto> {
        &self.tokens
    }

    fn mut_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CredentialsKVProto> {
        &mut self.tokens
    }

    // repeated .hadoop.common.CredentialsKVProto secrets = 2;

    pub fn clear_secrets(&mut self) {
        self.secrets.clear();
    }

    // Param is passed by value, moved
    pub fn set_secrets(&mut self, v: ::protobuf::RepeatedField<CredentialsKVProto>) {
        self.secrets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_secrets(&mut self) -> &mut ::protobuf::RepeatedField<CredentialsKVProto> {
        &mut self.secrets
    }

    // Take field
    pub fn take_secrets(&mut self) -> ::protobuf::RepeatedField<CredentialsKVProto> {
        ::std::mem::replace(&mut self.secrets, ::protobuf::RepeatedField::new())
    }

    pub fn get_secrets(&self) -> &[CredentialsKVProto] {
        &self.secrets
    }

    fn get_secrets_for_reflect(&self) -> &::protobuf::RepeatedField<CredentialsKVProto> {
        &self.secrets
    }

    fn mut_secrets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CredentialsKVProto> {
        &mut self.secrets
    }
}

impl ::protobuf::Message for CredentialsProto {
    fn is_initialized(&self) -> bool {
        for v in &self.tokens {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.secrets {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tokens)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.secrets)?;
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
        for value in &self.tokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.secrets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tokens {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.secrets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CredentialsProto {
    fn new() -> CredentialsProto {
        CredentialsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CredentialsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CredentialsKVProto>>(
                    "tokens",
                    CredentialsProto::get_tokens_for_reflect,
                    CredentialsProto::mut_tokens_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CredentialsKVProto>>(
                    "secrets",
                    CredentialsProto::get_secrets_for_reflect,
                    CredentialsProto::mut_secrets_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CredentialsProto>(
                    "CredentialsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CredentialsProto {
    fn clear(&mut self) {
        self.clear_tokens();
        self.clear_secrets();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CredentialsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CredentialsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDelegationTokenRequestProto {
    // message fields
    renewer: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDelegationTokenRequestProto {}

impl GetDelegationTokenRequestProto {
    pub fn new() -> GetDelegationTokenRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDelegationTokenRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDelegationTokenRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDelegationTokenRequestProto,
        };
        unsafe {
            instance.get(GetDelegationTokenRequestProto::new)
        }
    }

    // required string renewer = 1;

    pub fn clear_renewer(&mut self) {
        self.renewer.clear();
    }

    pub fn has_renewer(&self) -> bool {
        self.renewer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renewer(&mut self, v: ::std::string::String) {
        self.renewer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_renewer(&mut self) -> &mut ::std::string::String {
        if self.renewer.is_none() {
            self.renewer.set_default();
        }
        self.renewer.as_mut().unwrap()
    }

    // Take field
    pub fn take_renewer(&mut self) -> ::std::string::String {
        self.renewer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_renewer(&self) -> &str {
        match self.renewer.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_renewer_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.renewer
    }

    fn mut_renewer_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.renewer
    }
}

impl ::protobuf::Message for GetDelegationTokenRequestProto {
    fn is_initialized(&self) -> bool {
        if self.renewer.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.renewer)?;
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
        if let Some(ref v) = self.renewer.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.renewer.as_ref() {
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

impl ::protobuf::MessageStatic for GetDelegationTokenRequestProto {
    fn new() -> GetDelegationTokenRequestProto {
        GetDelegationTokenRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDelegationTokenRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "renewer",
                    GetDelegationTokenRequestProto::get_renewer_for_reflect,
                    GetDelegationTokenRequestProto::mut_renewer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDelegationTokenRequestProto>(
                    "GetDelegationTokenRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDelegationTokenRequestProto {
    fn clear(&mut self) {
        self.clear_renewer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDelegationTokenRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDelegationTokenRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDelegationTokenResponseProto {
    // message fields
    token: ::protobuf::SingularPtrField<TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDelegationTokenResponseProto {}

impl GetDelegationTokenResponseProto {
    pub fn new() -> GetDelegationTokenResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDelegationTokenResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDelegationTokenResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDelegationTokenResponseProto,
        };
        unsafe {
            instance.get(GetDelegationTokenResponseProto::new)
        }
    }

    // optional .hadoop.common.TokenProto token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> TokenProto {
        self.token.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_token(&self) -> &TokenProto {
        self.token.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenProto> {
        &mut self.token
    }
}

impl ::protobuf::Message for GetDelegationTokenResponseProto {
    fn is_initialized(&self) -> bool {
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
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
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

impl ::protobuf::MessageStatic for GetDelegationTokenResponseProto {
    fn new() -> GetDelegationTokenResponseProto {
        GetDelegationTokenResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDelegationTokenResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenProto>>(
                    "token",
                    GetDelegationTokenResponseProto::get_token_for_reflect,
                    GetDelegationTokenResponseProto::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDelegationTokenResponseProto>(
                    "GetDelegationTokenResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDelegationTokenResponseProto {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDelegationTokenResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDelegationTokenResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RenewDelegationTokenRequestProto {
    // message fields
    token: ::protobuf::SingularPtrField<TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RenewDelegationTokenRequestProto {}

impl RenewDelegationTokenRequestProto {
    pub fn new() -> RenewDelegationTokenRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RenewDelegationTokenRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RenewDelegationTokenRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RenewDelegationTokenRequestProto,
        };
        unsafe {
            instance.get(RenewDelegationTokenRequestProto::new)
        }
    }

    // required .hadoop.common.TokenProto token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> TokenProto {
        self.token.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_token(&self) -> &TokenProto {
        self.token.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenProto> {
        &mut self.token
    }
}

impl ::protobuf::Message for RenewDelegationTokenRequestProto {
    fn is_initialized(&self) -> bool {
        if self.token.is_none() {
            return false;
        }
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
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
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

impl ::protobuf::MessageStatic for RenewDelegationTokenRequestProto {
    fn new() -> RenewDelegationTokenRequestProto {
        RenewDelegationTokenRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RenewDelegationTokenRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenProto>>(
                    "token",
                    RenewDelegationTokenRequestProto::get_token_for_reflect,
                    RenewDelegationTokenRequestProto::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RenewDelegationTokenRequestProto>(
                    "RenewDelegationTokenRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RenewDelegationTokenRequestProto {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenewDelegationTokenRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenewDelegationTokenRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RenewDelegationTokenResponseProto {
    // message fields
    newExpiryTime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RenewDelegationTokenResponseProto {}

impl RenewDelegationTokenResponseProto {
    pub fn new() -> RenewDelegationTokenResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RenewDelegationTokenResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RenewDelegationTokenResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RenewDelegationTokenResponseProto,
        };
        unsafe {
            instance.get(RenewDelegationTokenResponseProto::new)
        }
    }

    // required uint64 newExpiryTime = 1;

    pub fn clear_newExpiryTime(&mut self) {
        self.newExpiryTime = ::std::option::Option::None;
    }

    pub fn has_newExpiryTime(&self) -> bool {
        self.newExpiryTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newExpiryTime(&mut self, v: u64) {
        self.newExpiryTime = ::std::option::Option::Some(v);
    }

    pub fn get_newExpiryTime(&self) -> u64 {
        self.newExpiryTime.unwrap_or(0)
    }

    fn get_newExpiryTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.newExpiryTime
    }

    fn mut_newExpiryTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.newExpiryTime
    }
}

impl ::protobuf::Message for RenewDelegationTokenResponseProto {
    fn is_initialized(&self) -> bool {
        if self.newExpiryTime.is_none() {
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
                    self.newExpiryTime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.newExpiryTime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.newExpiryTime {
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

impl ::protobuf::MessageStatic for RenewDelegationTokenResponseProto {
    fn new() -> RenewDelegationTokenResponseProto {
        RenewDelegationTokenResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RenewDelegationTokenResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "newExpiryTime",
                    RenewDelegationTokenResponseProto::get_newExpiryTime_for_reflect,
                    RenewDelegationTokenResponseProto::mut_newExpiryTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RenewDelegationTokenResponseProto>(
                    "RenewDelegationTokenResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RenewDelegationTokenResponseProto {
    fn clear(&mut self) {
        self.clear_newExpiryTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenewDelegationTokenResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenewDelegationTokenResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CancelDelegationTokenRequestProto {
    // message fields
    token: ::protobuf::SingularPtrField<TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelDelegationTokenRequestProto {}

impl CancelDelegationTokenRequestProto {
    pub fn new() -> CancelDelegationTokenRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelDelegationTokenRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CancelDelegationTokenRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelDelegationTokenRequestProto,
        };
        unsafe {
            instance.get(CancelDelegationTokenRequestProto::new)
        }
    }

    // required .hadoop.common.TokenProto token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> TokenProto {
        self.token.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_token(&self) -> &TokenProto {
        self.token.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenProto> {
        &mut self.token
    }
}

impl ::protobuf::Message for CancelDelegationTokenRequestProto {
    fn is_initialized(&self) -> bool {
        if self.token.is_none() {
            return false;
        }
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
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
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

impl ::protobuf::MessageStatic for CancelDelegationTokenRequestProto {
    fn new() -> CancelDelegationTokenRequestProto {
        CancelDelegationTokenRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelDelegationTokenRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenProto>>(
                    "token",
                    CancelDelegationTokenRequestProto::get_token_for_reflect,
                    CancelDelegationTokenRequestProto::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CancelDelegationTokenRequestProto>(
                    "CancelDelegationTokenRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelDelegationTokenRequestProto {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CancelDelegationTokenRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelDelegationTokenRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CancelDelegationTokenResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelDelegationTokenResponseProto {}

impl CancelDelegationTokenResponseProto {
    pub fn new() -> CancelDelegationTokenResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelDelegationTokenResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CancelDelegationTokenResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelDelegationTokenResponseProto,
        };
        unsafe {
            instance.get(CancelDelegationTokenResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CancelDelegationTokenResponseProto {
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

impl ::protobuf::MessageStatic for CancelDelegationTokenResponseProto {
    fn new() -> CancelDelegationTokenResponseProto {
        CancelDelegationTokenResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelDelegationTokenResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CancelDelegationTokenResponseProto>(
                    "CancelDelegationTokenResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelDelegationTokenResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CancelDelegationTokenResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CancelDelegationTokenResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eSecurity.proto\x12\rhadoop.common\"v\n\nTokenProto\x12\x1e\n\niden\
    tifier\x18\x01\x20\x02(\x0cR\nidentifier\x12\x1a\n\x08password\x18\x02\
    \x20\x02(\x0cR\x08password\x12\x12\n\x04kind\x18\x03\x20\x02(\tR\x04kind\
    \x12\x18\n\x07service\x18\x04\x20\x02(\tR\x07service\"s\n\x12Credentials\
    KVProto\x12\x14\n\x05alias\x18\x01\x20\x02(\tR\x05alias\x12/\n\x05token\
    \x18\x02\x20\x01(\x0b2\x19.hadoop.common.TokenProtoR\x05token\x12\x16\n\
    \x06secret\x18\x03\x20\x01(\x0cR\x06secret\"\x8a\x01\n\x10CredentialsPro\
    to\x129\n\x06tokens\x18\x01\x20\x03(\x0b2!.hadoop.common.CredentialsKVPr\
    otoR\x06tokens\x12;\n\x07secrets\x18\x02\x20\x03(\x0b2!.hadoop.common.Cr\
    edentialsKVProtoR\x07secrets\":\n\x1eGetDelegationTokenRequestProto\x12\
    \x18\n\x07renewer\x18\x01\x20\x02(\tR\x07renewer\"R\n\x1fGetDelegationTo\
    kenResponseProto\x12/\n\x05token\x18\x01\x20\x01(\x0b2\x19.hadoop.common\
    .TokenProtoR\x05token\"S\n\x20RenewDelegationTokenRequestProto\x12/\n\
    \x05token\x18\x01\x20\x02(\x0b2\x19.hadoop.common.TokenProtoR\x05token\"\
    I\n!RenewDelegationTokenResponseProto\x12$\n\rnewExpiryTime\x18\x01\x20\
    \x02(\x04R\rnewExpiryTime\"T\n!CancelDelegationTokenRequestProto\x12/\n\
    \x05token\x18\x01\x20\x02(\x0b2\x19.hadoop.common.TokenProtoR\x05token\"\
    $\n\"CancelDelegationTokenResponseProtoB8\n\x20org.apache.hadoop.securit\
    y.protoB\x0eSecurityProtos\xa0\x01\x01\x88\x01\x01J\xdc\x13\n\x06\x12\
    \x04\x18\0H\x01\n\x08\n\x01\x08\x12\x03\x18\09\n\xbe\x07\n\x04\x08\xe7\
    \x07\0\x12\x03\x18\092\x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\x20\
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
    are\x20allowed\x20for\x20a\x20*stable*\x20.proto\x20interface.\n\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x18\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x18\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\
    \x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x18\x168\n\x08\n\x01\x08\x12\
    \x03\x19\0/\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x19\0/\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x19\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x19\x1e.\n\x08\n\x01\x08\
    \x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x1a\x07\x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x1a\
    \x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x1a\x1f#\n\x08\n\x01\
    \x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1b\0,\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\n\r\n\x06\x08\xe7\x07\x03\x02\
    \0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x1b\
    \x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x1b'+\n\x08\n\x01\x02\x12\
    \x03\x1c\x08\x15\n)\n\x02\x04\0\x12\x04!\0&\x01\x1a\x1d*\n\x20Security\
    \x20token\x20identifier\n\n\n\n\x03\x04\0\x01\x12\x03!\x08\x12\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\"\x02\x20\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\"\
    \x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\"\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\"\x11\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\"\x1e\
    \x1f\n\x0b\n\x04\x04\0\x02\x01\x12\x03#\x02\x1e\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03#\x0b\x10\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03#\x11\x19\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03#\x1c\x1d\n\x0b\n\x04\x04\0\x02\x02\x12\x03$\x02\x1b\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03$\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03$\x12\x16\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03$\x19\x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03%\
    \x02\x1e\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\0\
    \x02\x03\x05\x12\x03%\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03%\x12\
    \x19\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03%\x1c\x1d\n\n\n\x02\x04\x01\
    \x12\x04(\0,\x01\n\n\n\x03\x04\x01\x01\x12\x03(\x08\x1a\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03)\x02\x1c\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03)\x02\n\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03)\x12\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03)\x1a\x1b\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03*\x02.\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03*\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03*\x0b#\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03*$)\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03*,-\n\x0b\n\x04\x04\x01\x02\x02\x12\x03+\x02\x1c\n\x0c\n\x05\x04\x01\
    \x02\x02\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03+\x0b\
    \x10\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03+\x11\x17\n\x0c\n\x05\x04\
    \x01\x02\x02\x03\x12\x03+\x1a\x1b\n\n\n\x02\x04\x02\x12\x04.\01\x01\n\n\
    \n\x03\x04\x02\x01\x12\x03.\x08\x18\n\x0b\n\x04\x04\x02\x02\0\x12\x03/\
    \x027\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\x02\
    \x02\0\x06\x12\x03/\x0b+\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03/,2\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03/56\n\x0b\n\x04\x04\x02\x02\x01\x12\x030\
    \x028\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x02\
    \x02\x01\x06\x12\x030\x0b+\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x030,3\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03067\n\n\n\x02\x04\x03\x12\x043\05\
    \x01\n\n\n\x03\x04\x03\x01\x12\x033\x08&\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x034\x02\x1e\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x034\x02\n\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x034\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x034\x12\x19\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x034\x1c\x1d\n\n\n\x02\
    \x04\x04\x12\x047\09\x01\n\n\n\x03\x04\x04\x01\x12\x037\x08'\n\x0b\n\x04\
    \x04\x04\x02\0\x12\x038\x02.\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x038\x02\
    \n\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x038\x0b#\n\x0c\n\x05\x04\x04\x02\0\
    \x01\x12\x038$)\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x038,-\n\n\n\x02\x04\
    \x05\x12\x04;\0=\x01\n\n\n\x03\x04\x05\x01\x12\x03;\x08(\n\x0b\n\x04\x04\
    \x05\x02\0\x12\x03<\x02.\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03<\x02\n\n\
    \x0c\n\x05\x04\x05\x02\0\x06\x12\x03<\x0b#\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03<$)\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03<,-\n\n\n\x02\x04\x06\
    \x12\x04?\0A\x01\n\n\n\x03\x04\x06\x01\x12\x03?\x08)\n\x0b\n\x04\x04\x06\
    \x02\0\x12\x03@\x02$\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03@\x02\n\n\x0c\
    \n\x05\x04\x06\x02\0\x05\x12\x03@\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x03@\x12\x1f\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03@\"#\n\n\n\x02\
    \x04\x07\x12\x04C\0E\x01\n\n\n\x03\x04\x07\x01\x12\x03C\x08)\n\x0b\n\x04\
    \x04\x07\x02\0\x12\x03D\x02.\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03D\x02\
    \n\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03D\x0b#\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x03D$)\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03D,-\n\x1b\n\x02\x04\
    \x08\x12\x04G\0H\x01\"\x0f\x20void\x20response\n\n\n\n\x03\x04\x08\x01\
    \x12\x03G\x08*\
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
