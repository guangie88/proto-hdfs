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
pub struct SetErasureCodingPolicyRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    ecPolicyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetErasureCodingPolicyRequestProto {}

impl SetErasureCodingPolicyRequestProto {
    pub fn new() -> SetErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<SetErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(SetErasureCodingPolicyRequestProto::new)
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

    // optional string ecPolicyName = 2;

    pub fn clear_ecPolicyName(&mut self) {
        self.ecPolicyName.clear();
    }

    pub fn has_ecPolicyName(&self) -> bool {
        self.ecPolicyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicyName(&mut self, v: ::std::string::String) {
        self.ecPolicyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicyName(&mut self) -> &mut ::std::string::String {
        if self.ecPolicyName.is_none() {
            self.ecPolicyName.set_default();
        }
        self.ecPolicyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicyName(&mut self) -> ::std::string::String {
        self.ecPolicyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ecPolicyName(&self) -> &str {
        match self.ecPolicyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ecPolicyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ecPolicyName
    }

    fn mut_ecPolicyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ecPolicyName
    }
}

impl ::protobuf::Message for SetErasureCodingPolicyRequestProto {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ecPolicyName)?;
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
        if let Some(ref v) = self.ecPolicyName.as_ref() {
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
        if let Some(ref v) = self.ecPolicyName.as_ref() {
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

impl ::protobuf::MessageStatic for SetErasureCodingPolicyRequestProto {
    fn new() -> SetErasureCodingPolicyRequestProto {
        SetErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    SetErasureCodingPolicyRequestProto::get_src_for_reflect,
                    SetErasureCodingPolicyRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ecPolicyName",
                    SetErasureCodingPolicyRequestProto::get_ecPolicyName_for_reflect,
                    SetErasureCodingPolicyRequestProto::mut_ecPolicyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetErasureCodingPolicyRequestProto>(
                    "SetErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_ecPolicyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetErasureCodingPolicyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetErasureCodingPolicyResponseProto {}

impl SetErasureCodingPolicyResponseProto {
    pub fn new() -> SetErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<SetErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(SetErasureCodingPolicyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for SetErasureCodingPolicyResponseProto {
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

impl ::protobuf::MessageStatic for SetErasureCodingPolicyResponseProto {
    fn new() -> SetErasureCodingPolicyResponseProto {
        SetErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetErasureCodingPolicyResponseProto>(
                    "SetErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingPoliciesRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingPoliciesRequestProto {}

impl GetErasureCodingPoliciesRequestProto {
    pub fn new() -> GetErasureCodingPoliciesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingPoliciesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingPoliciesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingPoliciesRequestProto,
        };
        unsafe {
            instance.get(GetErasureCodingPoliciesRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetErasureCodingPoliciesRequestProto {
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

impl ::protobuf::MessageStatic for GetErasureCodingPoliciesRequestProto {
    fn new() -> GetErasureCodingPoliciesRequestProto {
        GetErasureCodingPoliciesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingPoliciesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingPoliciesRequestProto>(
                    "GetErasureCodingPoliciesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingPoliciesRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingPoliciesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingPoliciesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingPoliciesResponseProto {
    // message fields
    ecPolicies: ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingPoliciesResponseProto {}

impl GetErasureCodingPoliciesResponseProto {
    pub fn new() -> GetErasureCodingPoliciesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingPoliciesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingPoliciesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingPoliciesResponseProto,
        };
        unsafe {
            instance.get(GetErasureCodingPoliciesResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.ErasureCodingPolicyProto ecPolicies = 1;

    pub fn clear_ecPolicies(&mut self) {
        self.ecPolicies.clear();
    }

    // Param is passed by value, moved
    pub fn set_ecPolicies(&mut self, v: ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto>) {
        self.ecPolicies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ecPolicies(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &mut self.ecPolicies
    }

    // Take field
    pub fn take_ecPolicies(&mut self) -> ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        ::std::mem::replace(&mut self.ecPolicies, ::protobuf::RepeatedField::new())
    }

    pub fn get_ecPolicies(&self) -> &[super::hdfs::ErasureCodingPolicyProto] {
        &self.ecPolicies
    }

    fn get_ecPolicies_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &self.ecPolicies
    }

    fn mut_ecPolicies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &mut self.ecPolicies
    }
}

impl ::protobuf::Message for GetErasureCodingPoliciesResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.ecPolicies {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ecPolicies)?;
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
        for value in &self.ecPolicies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ecPolicies {
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

impl ::protobuf::MessageStatic for GetErasureCodingPoliciesResponseProto {
    fn new() -> GetErasureCodingPoliciesResponseProto {
        GetErasureCodingPoliciesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingPoliciesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ErasureCodingPolicyProto>>(
                    "ecPolicies",
                    GetErasureCodingPoliciesResponseProto::get_ecPolicies_for_reflect,
                    GetErasureCodingPoliciesResponseProto::mut_ecPolicies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingPoliciesResponseProto>(
                    "GetErasureCodingPoliciesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingPoliciesResponseProto {
    fn clear(&mut self) {
        self.clear_ecPolicies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingPoliciesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingPoliciesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingCodecsRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingCodecsRequestProto {}

impl GetErasureCodingCodecsRequestProto {
    pub fn new() -> GetErasureCodingCodecsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingCodecsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingCodecsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingCodecsRequestProto,
        };
        unsafe {
            instance.get(GetErasureCodingCodecsRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetErasureCodingCodecsRequestProto {
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

impl ::protobuf::MessageStatic for GetErasureCodingCodecsRequestProto {
    fn new() -> GetErasureCodingCodecsRequestProto {
        GetErasureCodingCodecsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingCodecsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingCodecsRequestProto>(
                    "GetErasureCodingCodecsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingCodecsRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingCodecsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingCodecsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingCodecsResponseProto {
    // message fields
    codec: ::protobuf::RepeatedField<CodecProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingCodecsResponseProto {}

impl GetErasureCodingCodecsResponseProto {
    pub fn new() -> GetErasureCodingCodecsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingCodecsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingCodecsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingCodecsResponseProto,
        };
        unsafe {
            instance.get(GetErasureCodingCodecsResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.CodecProto codec = 1;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::protobuf::RepeatedField<CodecProto>) {
        self.codec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_codec(&mut self) -> &mut ::protobuf::RepeatedField<CodecProto> {
        &mut self.codec
    }

    // Take field
    pub fn take_codec(&mut self) -> ::protobuf::RepeatedField<CodecProto> {
        ::std::mem::replace(&mut self.codec, ::protobuf::RepeatedField::new())
    }

    pub fn get_codec(&self) -> &[CodecProto] {
        &self.codec
    }

    fn get_codec_for_reflect(&self) -> &::protobuf::RepeatedField<CodecProto> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CodecProto> {
        &mut self.codec
    }
}

impl ::protobuf::Message for GetErasureCodingCodecsResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.codec {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.codec)?;
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
        for value in &self.codec {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.codec {
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

impl ::protobuf::MessageStatic for GetErasureCodingCodecsResponseProto {
    fn new() -> GetErasureCodingCodecsResponseProto {
        GetErasureCodingCodecsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingCodecsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CodecProto>>(
                    "codec",
                    GetErasureCodingCodecsResponseProto::get_codec_for_reflect,
                    GetErasureCodingCodecsResponseProto::mut_codec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingCodecsResponseProto>(
                    "GetErasureCodingCodecsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingCodecsResponseProto {
    fn clear(&mut self) {
        self.clear_codec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingCodecsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingCodecsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingPolicyRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingPolicyRequestProto {}

impl GetErasureCodingPolicyRequestProto {
    pub fn new() -> GetErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(GetErasureCodingPolicyRequestProto::new)
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

impl ::protobuf::Message for GetErasureCodingPolicyRequestProto {
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

impl ::protobuf::MessageStatic for GetErasureCodingPolicyRequestProto {
    fn new() -> GetErasureCodingPolicyRequestProto {
        GetErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    GetErasureCodingPolicyRequestProto::get_src_for_reflect,
                    GetErasureCodingPolicyRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingPolicyRequestProto>(
                    "GetErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetErasureCodingPolicyResponseProto {
    // message fields
    ecPolicy: ::protobuf::SingularPtrField<super::hdfs::ErasureCodingPolicyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetErasureCodingPolicyResponseProto {}

impl GetErasureCodingPolicyResponseProto {
    pub fn new() -> GetErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(GetErasureCodingPolicyResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.ErasureCodingPolicyProto ecPolicy = 1;

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
}

impl ::protobuf::Message for GetErasureCodingPolicyResponseProto {
    fn is_initialized(&self) -> bool {
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
        if let Some(ref v) = self.ecPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ecPolicy.as_ref() {
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

impl ::protobuf::MessageStatic for GetErasureCodingPolicyResponseProto {
    fn new() -> GetErasureCodingPolicyResponseProto {
        GetErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ErasureCodingPolicyProto>>(
                    "ecPolicy",
                    GetErasureCodingPolicyResponseProto::get_ecPolicy_for_reflect,
                    GetErasureCodingPolicyResponseProto::mut_ecPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetErasureCodingPolicyResponseProto>(
                    "GetErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.clear_ecPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddErasureCodingPoliciesRequestProto {
    // message fields
    ecPolicies: ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddErasureCodingPoliciesRequestProto {}

impl AddErasureCodingPoliciesRequestProto {
    pub fn new() -> AddErasureCodingPoliciesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddErasureCodingPoliciesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<AddErasureCodingPoliciesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddErasureCodingPoliciesRequestProto,
        };
        unsafe {
            instance.get(AddErasureCodingPoliciesRequestProto::new)
        }
    }

    // repeated .hadoop.hdfs.ErasureCodingPolicyProto ecPolicies = 1;

    pub fn clear_ecPolicies(&mut self) {
        self.ecPolicies.clear();
    }

    // Param is passed by value, moved
    pub fn set_ecPolicies(&mut self, v: ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto>) {
        self.ecPolicies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ecPolicies(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &mut self.ecPolicies
    }

    // Take field
    pub fn take_ecPolicies(&mut self) -> ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        ::std::mem::replace(&mut self.ecPolicies, ::protobuf::RepeatedField::new())
    }

    pub fn get_ecPolicies(&self) -> &[super::hdfs::ErasureCodingPolicyProto] {
        &self.ecPolicies
    }

    fn get_ecPolicies_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &self.ecPolicies
    }

    fn mut_ecPolicies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::ErasureCodingPolicyProto> {
        &mut self.ecPolicies
    }
}

impl ::protobuf::Message for AddErasureCodingPoliciesRequestProto {
    fn is_initialized(&self) -> bool {
        for v in &self.ecPolicies {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ecPolicies)?;
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
        for value in &self.ecPolicies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ecPolicies {
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

impl ::protobuf::MessageStatic for AddErasureCodingPoliciesRequestProto {
    fn new() -> AddErasureCodingPoliciesRequestProto {
        AddErasureCodingPoliciesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddErasureCodingPoliciesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ErasureCodingPolicyProto>>(
                    "ecPolicies",
                    AddErasureCodingPoliciesRequestProto::get_ecPolicies_for_reflect,
                    AddErasureCodingPoliciesRequestProto::mut_ecPolicies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddErasureCodingPoliciesRequestProto>(
                    "AddErasureCodingPoliciesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddErasureCodingPoliciesRequestProto {
    fn clear(&mut self) {
        self.clear_ecPolicies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddErasureCodingPoliciesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddErasureCodingPoliciesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddErasureCodingPoliciesResponseProto {
    // message fields
    responses: ::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddErasureCodingPoliciesResponseProto {}

impl AddErasureCodingPoliciesResponseProto {
    pub fn new() -> AddErasureCodingPoliciesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddErasureCodingPoliciesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<AddErasureCodingPoliciesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddErasureCodingPoliciesResponseProto,
        };
        unsafe {
            instance.get(AddErasureCodingPoliciesResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.AddErasureCodingPolicyResponseProto responses = 1;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[super::hdfs::AddErasureCodingPolicyResponseProto] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::AddErasureCodingPolicyResponseProto> {
        &mut self.responses
    }
}

impl ::protobuf::Message for AddErasureCodingPoliciesResponseProto {
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

impl ::protobuf::MessageStatic for AddErasureCodingPoliciesResponseProto {
    fn new() -> AddErasureCodingPoliciesResponseProto {
        AddErasureCodingPoliciesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddErasureCodingPoliciesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::AddErasureCodingPolicyResponseProto>>(
                    "responses",
                    AddErasureCodingPoliciesResponseProto::get_responses_for_reflect,
                    AddErasureCodingPoliciesResponseProto::mut_responses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddErasureCodingPoliciesResponseProto>(
                    "AddErasureCodingPoliciesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddErasureCodingPoliciesResponseProto {
    fn clear(&mut self) {
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddErasureCodingPoliciesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddErasureCodingPoliciesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveErasureCodingPolicyRequestProto {
    // message fields
    ecPolicyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveErasureCodingPolicyRequestProto {}

impl RemoveErasureCodingPolicyRequestProto {
    pub fn new() -> RemoveErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(RemoveErasureCodingPolicyRequestProto::new)
        }
    }

    // required string ecPolicyName = 1;

    pub fn clear_ecPolicyName(&mut self) {
        self.ecPolicyName.clear();
    }

    pub fn has_ecPolicyName(&self) -> bool {
        self.ecPolicyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicyName(&mut self, v: ::std::string::String) {
        self.ecPolicyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicyName(&mut self) -> &mut ::std::string::String {
        if self.ecPolicyName.is_none() {
            self.ecPolicyName.set_default();
        }
        self.ecPolicyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicyName(&mut self) -> ::std::string::String {
        self.ecPolicyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ecPolicyName(&self) -> &str {
        match self.ecPolicyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ecPolicyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ecPolicyName
    }

    fn mut_ecPolicyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ecPolicyName
    }
}

impl ::protobuf::Message for RemoveErasureCodingPolicyRequestProto {
    fn is_initialized(&self) -> bool {
        if self.ecPolicyName.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ecPolicyName)?;
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
        if let Some(ref v) = self.ecPolicyName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ecPolicyName.as_ref() {
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

impl ::protobuf::MessageStatic for RemoveErasureCodingPolicyRequestProto {
    fn new() -> RemoveErasureCodingPolicyRequestProto {
        RemoveErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ecPolicyName",
                    RemoveErasureCodingPolicyRequestProto::get_ecPolicyName_for_reflect,
                    RemoveErasureCodingPolicyRequestProto::mut_ecPolicyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveErasureCodingPolicyRequestProto>(
                    "RemoveErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_ecPolicyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveErasureCodingPolicyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveErasureCodingPolicyResponseProto {}

impl RemoveErasureCodingPolicyResponseProto {
    pub fn new() -> RemoveErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(RemoveErasureCodingPolicyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveErasureCodingPolicyResponseProto {
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

impl ::protobuf::MessageStatic for RemoveErasureCodingPolicyResponseProto {
    fn new() -> RemoveErasureCodingPolicyResponseProto {
        RemoveErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveErasureCodingPolicyResponseProto>(
                    "RemoveErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnableErasureCodingPolicyRequestProto {
    // message fields
    ecPolicyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnableErasureCodingPolicyRequestProto {}

impl EnableErasureCodingPolicyRequestProto {
    pub fn new() -> EnableErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnableErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<EnableErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnableErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(EnableErasureCodingPolicyRequestProto::new)
        }
    }

    // required string ecPolicyName = 1;

    pub fn clear_ecPolicyName(&mut self) {
        self.ecPolicyName.clear();
    }

    pub fn has_ecPolicyName(&self) -> bool {
        self.ecPolicyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicyName(&mut self, v: ::std::string::String) {
        self.ecPolicyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicyName(&mut self) -> &mut ::std::string::String {
        if self.ecPolicyName.is_none() {
            self.ecPolicyName.set_default();
        }
        self.ecPolicyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicyName(&mut self) -> ::std::string::String {
        self.ecPolicyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ecPolicyName(&self) -> &str {
        match self.ecPolicyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ecPolicyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ecPolicyName
    }

    fn mut_ecPolicyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ecPolicyName
    }
}

impl ::protobuf::Message for EnableErasureCodingPolicyRequestProto {
    fn is_initialized(&self) -> bool {
        if self.ecPolicyName.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ecPolicyName)?;
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
        if let Some(ref v) = self.ecPolicyName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ecPolicyName.as_ref() {
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

impl ::protobuf::MessageStatic for EnableErasureCodingPolicyRequestProto {
    fn new() -> EnableErasureCodingPolicyRequestProto {
        EnableErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnableErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ecPolicyName",
                    EnableErasureCodingPolicyRequestProto::get_ecPolicyName_for_reflect,
                    EnableErasureCodingPolicyRequestProto::mut_ecPolicyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnableErasureCodingPolicyRequestProto>(
                    "EnableErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnableErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_ecPolicyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnableErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnableErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnableErasureCodingPolicyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnableErasureCodingPolicyResponseProto {}

impl EnableErasureCodingPolicyResponseProto {
    pub fn new() -> EnableErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnableErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<EnableErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnableErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(EnableErasureCodingPolicyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for EnableErasureCodingPolicyResponseProto {
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

impl ::protobuf::MessageStatic for EnableErasureCodingPolicyResponseProto {
    fn new() -> EnableErasureCodingPolicyResponseProto {
        EnableErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnableErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EnableErasureCodingPolicyResponseProto>(
                    "EnableErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnableErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnableErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnableErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisableErasureCodingPolicyRequestProto {
    // message fields
    ecPolicyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisableErasureCodingPolicyRequestProto {}

impl DisableErasureCodingPolicyRequestProto {
    pub fn new() -> DisableErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisableErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DisableErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisableErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(DisableErasureCodingPolicyRequestProto::new)
        }
    }

    // required string ecPolicyName = 1;

    pub fn clear_ecPolicyName(&mut self) {
        self.ecPolicyName.clear();
    }

    pub fn has_ecPolicyName(&self) -> bool {
        self.ecPolicyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ecPolicyName(&mut self, v: ::std::string::String) {
        self.ecPolicyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ecPolicyName(&mut self) -> &mut ::std::string::String {
        if self.ecPolicyName.is_none() {
            self.ecPolicyName.set_default();
        }
        self.ecPolicyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ecPolicyName(&mut self) -> ::std::string::String {
        self.ecPolicyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ecPolicyName(&self) -> &str {
        match self.ecPolicyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ecPolicyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ecPolicyName
    }

    fn mut_ecPolicyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ecPolicyName
    }
}

impl ::protobuf::Message for DisableErasureCodingPolicyRequestProto {
    fn is_initialized(&self) -> bool {
        if self.ecPolicyName.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ecPolicyName)?;
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
        if let Some(ref v) = self.ecPolicyName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ecPolicyName.as_ref() {
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

impl ::protobuf::MessageStatic for DisableErasureCodingPolicyRequestProto {
    fn new() -> DisableErasureCodingPolicyRequestProto {
        DisableErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisableErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ecPolicyName",
                    DisableErasureCodingPolicyRequestProto::get_ecPolicyName_for_reflect,
                    DisableErasureCodingPolicyRequestProto::mut_ecPolicyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DisableErasureCodingPolicyRequestProto>(
                    "DisableErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisableErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_ecPolicyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisableErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisableErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisableErasureCodingPolicyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisableErasureCodingPolicyResponseProto {}

impl DisableErasureCodingPolicyResponseProto {
    pub fn new() -> DisableErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisableErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DisableErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisableErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(DisableErasureCodingPolicyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DisableErasureCodingPolicyResponseProto {
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

impl ::protobuf::MessageStatic for DisableErasureCodingPolicyResponseProto {
    fn new() -> DisableErasureCodingPolicyResponseProto {
        DisableErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisableErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DisableErasureCodingPolicyResponseProto>(
                    "DisableErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisableErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisableErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisableErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnsetErasureCodingPolicyRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsetErasureCodingPolicyRequestProto {}

impl UnsetErasureCodingPolicyRequestProto {
    pub fn new() -> UnsetErasureCodingPolicyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsetErasureCodingPolicyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<UnsetErasureCodingPolicyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsetErasureCodingPolicyRequestProto,
        };
        unsafe {
            instance.get(UnsetErasureCodingPolicyRequestProto::new)
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

impl ::protobuf::Message for UnsetErasureCodingPolicyRequestProto {
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

impl ::protobuf::MessageStatic for UnsetErasureCodingPolicyRequestProto {
    fn new() -> UnsetErasureCodingPolicyRequestProto {
        UnsetErasureCodingPolicyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsetErasureCodingPolicyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    UnsetErasureCodingPolicyRequestProto::get_src_for_reflect,
                    UnsetErasureCodingPolicyRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnsetErasureCodingPolicyRequestProto>(
                    "UnsetErasureCodingPolicyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsetErasureCodingPolicyRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnsetErasureCodingPolicyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnsetErasureCodingPolicyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnsetErasureCodingPolicyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsetErasureCodingPolicyResponseProto {}

impl UnsetErasureCodingPolicyResponseProto {
    pub fn new() -> UnsetErasureCodingPolicyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsetErasureCodingPolicyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<UnsetErasureCodingPolicyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsetErasureCodingPolicyResponseProto,
        };
        unsafe {
            instance.get(UnsetErasureCodingPolicyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for UnsetErasureCodingPolicyResponseProto {
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

impl ::protobuf::MessageStatic for UnsetErasureCodingPolicyResponseProto {
    fn new() -> UnsetErasureCodingPolicyResponseProto {
        UnsetErasureCodingPolicyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsetErasureCodingPolicyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UnsetErasureCodingPolicyResponseProto>(
                    "UnsetErasureCodingPolicyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsetErasureCodingPolicyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnsetErasureCodingPolicyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnsetErasureCodingPolicyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockECReconstructionInfoProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    sourceDnInfos: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto>,
    targetDnInfos: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto>,
    targetStorageUuids: ::protobuf::SingularPtrField<super::hdfs::StorageUuidsProto>,
    targetStorageTypes: ::protobuf::SingularPtrField<super::hdfs::StorageTypesProto>,
    liveBlockIndices: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ecPolicy: ::protobuf::SingularPtrField<super::hdfs::ErasureCodingPolicyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockECReconstructionInfoProto {}

impl BlockECReconstructionInfoProto {
    pub fn new() -> BlockECReconstructionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockECReconstructionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockECReconstructionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockECReconstructionInfoProto,
        };
        unsafe {
            instance.get(BlockECReconstructionInfoProto::new)
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

    // required .hadoop.hdfs.DatanodeInfosProto sourceDnInfos = 2;

    pub fn clear_sourceDnInfos(&mut self) {
        self.sourceDnInfos.clear();
    }

    pub fn has_sourceDnInfos(&self) -> bool {
        self.sourceDnInfos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sourceDnInfos(&mut self, v: super::hdfs::DatanodeInfosProto) {
        self.sourceDnInfos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sourceDnInfos(&mut self) -> &mut super::hdfs::DatanodeInfosProto {
        if self.sourceDnInfos.is_none() {
            self.sourceDnInfos.set_default();
        }
        self.sourceDnInfos.as_mut().unwrap()
    }

    // Take field
    pub fn take_sourceDnInfos(&mut self) -> super::hdfs::DatanodeInfosProto {
        self.sourceDnInfos.take().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::new())
    }

    pub fn get_sourceDnInfos(&self) -> &super::hdfs::DatanodeInfosProto {
        self.sourceDnInfos.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::default_instance())
    }

    fn get_sourceDnInfos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &self.sourceDnInfos
    }

    fn mut_sourceDnInfos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &mut self.sourceDnInfos
    }

    // required .hadoop.hdfs.DatanodeInfosProto targetDnInfos = 3;

    pub fn clear_targetDnInfos(&mut self) {
        self.targetDnInfos.clear();
    }

    pub fn has_targetDnInfos(&self) -> bool {
        self.targetDnInfos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetDnInfos(&mut self, v: super::hdfs::DatanodeInfosProto) {
        self.targetDnInfos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetDnInfos(&mut self) -> &mut super::hdfs::DatanodeInfosProto {
        if self.targetDnInfos.is_none() {
            self.targetDnInfos.set_default();
        }
        self.targetDnInfos.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetDnInfos(&mut self) -> super::hdfs::DatanodeInfosProto {
        self.targetDnInfos.take().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::new())
    }

    pub fn get_targetDnInfos(&self) -> &super::hdfs::DatanodeInfosProto {
        self.targetDnInfos.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfosProto::default_instance())
    }

    fn get_targetDnInfos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &self.targetDnInfos
    }

    fn mut_targetDnInfos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfosProto> {
        &mut self.targetDnInfos
    }

    // required .hadoop.hdfs.StorageUuidsProto targetStorageUuids = 4;

    pub fn clear_targetStorageUuids(&mut self) {
        self.targetStorageUuids.clear();
    }

    pub fn has_targetStorageUuids(&self) -> bool {
        self.targetStorageUuids.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetStorageUuids(&mut self, v: super::hdfs::StorageUuidsProto) {
        self.targetStorageUuids = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetStorageUuids(&mut self) -> &mut super::hdfs::StorageUuidsProto {
        if self.targetStorageUuids.is_none() {
            self.targetStorageUuids.set_default();
        }
        self.targetStorageUuids.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetStorageUuids(&mut self) -> super::hdfs::StorageUuidsProto {
        self.targetStorageUuids.take().unwrap_or_else(|| super::hdfs::StorageUuidsProto::new())
    }

    pub fn get_targetStorageUuids(&self) -> &super::hdfs::StorageUuidsProto {
        self.targetStorageUuids.as_ref().unwrap_or_else(|| super::hdfs::StorageUuidsProto::default_instance())
    }

    fn get_targetStorageUuids_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageUuidsProto> {
        &self.targetStorageUuids
    }

    fn mut_targetStorageUuids_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageUuidsProto> {
        &mut self.targetStorageUuids
    }

    // required .hadoop.hdfs.StorageTypesProto targetStorageTypes = 5;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    pub fn has_targetStorageTypes(&self) -> bool {
        self.targetStorageTypes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: super::hdfs::StorageTypesProto) {
        self.targetStorageTypes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetStorageTypes(&mut self) -> &mut super::hdfs::StorageTypesProto {
        if self.targetStorageTypes.is_none() {
            self.targetStorageTypes.set_default();
        }
        self.targetStorageTypes.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> super::hdfs::StorageTypesProto {
        self.targetStorageTypes.take().unwrap_or_else(|| super::hdfs::StorageTypesProto::new())
    }

    pub fn get_targetStorageTypes(&self) -> &super::hdfs::StorageTypesProto {
        self.targetStorageTypes.as_ref().unwrap_or_else(|| super::hdfs::StorageTypesProto::default_instance())
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageTypesProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageTypesProto> {
        &mut self.targetStorageTypes
    }

    // required bytes liveBlockIndices = 6;

    pub fn clear_liveBlockIndices(&mut self) {
        self.liveBlockIndices.clear();
    }

    pub fn has_liveBlockIndices(&self) -> bool {
        self.liveBlockIndices.is_some()
    }

    // Param is passed by value, moved
    pub fn set_liveBlockIndices(&mut self, v: ::std::vec::Vec<u8>) {
        self.liveBlockIndices = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_liveBlockIndices(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.liveBlockIndices.is_none() {
            self.liveBlockIndices.set_default();
        }
        self.liveBlockIndices.as_mut().unwrap()
    }

    // Take field
    pub fn take_liveBlockIndices(&mut self) -> ::std::vec::Vec<u8> {
        self.liveBlockIndices.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_liveBlockIndices(&self) -> &[u8] {
        match self.liveBlockIndices.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_liveBlockIndices_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.liveBlockIndices
    }

    fn mut_liveBlockIndices_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.liveBlockIndices
    }

    // required .hadoop.hdfs.ErasureCodingPolicyProto ecPolicy = 7;

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
}

impl ::protobuf::Message for BlockECReconstructionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.sourceDnInfos.is_none() {
            return false;
        }
        if self.targetDnInfos.is_none() {
            return false;
        }
        if self.targetStorageUuids.is_none() {
            return false;
        }
        if self.targetStorageTypes.is_none() {
            return false;
        }
        if self.liveBlockIndices.is_none() {
            return false;
        }
        if self.ecPolicy.is_none() {
            return false;
        }
        for v in &self.block {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sourceDnInfos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetDnInfos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetStorageUuids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetStorageTypes {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sourceDnInfos)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetDnInfos)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetStorageUuids)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetStorageTypes)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.liveBlockIndices)?;
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.sourceDnInfos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.targetDnInfos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.targetStorageUuids.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.targetStorageTypes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.liveBlockIndices.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
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
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.sourceDnInfos.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.targetDnInfos.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.targetStorageUuids.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.targetStorageTypes.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.liveBlockIndices.as_ref() {
            os.write_bytes(6, &v)?;
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

impl ::protobuf::MessageStatic for BlockECReconstructionInfoProto {
    fn new() -> BlockECReconstructionInfoProto {
        BlockECReconstructionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockECReconstructionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    BlockECReconstructionInfoProto::get_block_for_reflect,
                    BlockECReconstructionInfoProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfosProto>>(
                    "sourceDnInfos",
                    BlockECReconstructionInfoProto::get_sourceDnInfos_for_reflect,
                    BlockECReconstructionInfoProto::mut_sourceDnInfos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfosProto>>(
                    "targetDnInfos",
                    BlockECReconstructionInfoProto::get_targetDnInfos_for_reflect,
                    BlockECReconstructionInfoProto::mut_targetDnInfos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageUuidsProto>>(
                    "targetStorageUuids",
                    BlockECReconstructionInfoProto::get_targetStorageUuids_for_reflect,
                    BlockECReconstructionInfoProto::mut_targetStorageUuids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageTypesProto>>(
                    "targetStorageTypes",
                    BlockECReconstructionInfoProto::get_targetStorageTypes_for_reflect,
                    BlockECReconstructionInfoProto::mut_targetStorageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "liveBlockIndices",
                    BlockECReconstructionInfoProto::get_liveBlockIndices_for_reflect,
                    BlockECReconstructionInfoProto::mut_liveBlockIndices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ErasureCodingPolicyProto>>(
                    "ecPolicy",
                    BlockECReconstructionInfoProto::get_ecPolicy_for_reflect,
                    BlockECReconstructionInfoProto::mut_ecPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockECReconstructionInfoProto>(
                    "BlockECReconstructionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockECReconstructionInfoProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_sourceDnInfos();
        self.clear_targetDnInfos();
        self.clear_targetStorageUuids();
        self.clear_targetStorageTypes();
        self.clear_liveBlockIndices();
        self.clear_ecPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockECReconstructionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockECReconstructionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CodecProto {
    // message fields
    codec: ::protobuf::SingularField<::std::string::String>,
    coders: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CodecProto {}

impl CodecProto {
    pub fn new() -> CodecProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CodecProto {
        static mut instance: ::protobuf::lazy::Lazy<CodecProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CodecProto,
        };
        unsafe {
            instance.get(CodecProto::new)
        }
    }

    // required string codec = 1;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::std::string::String) {
        self.codec = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec(&mut self) -> &mut ::std::string::String {
        if self.codec.is_none() {
            self.codec.set_default();
        }
        self.codec.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec(&mut self) -> ::std::string::String {
        self.codec.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codec(&self) -> &str {
        match self.codec.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_codec_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.codec
    }

    // required string coders = 2;

    pub fn clear_coders(&mut self) {
        self.coders.clear();
    }

    pub fn has_coders(&self) -> bool {
        self.coders.is_some()
    }

    // Param is passed by value, moved
    pub fn set_coders(&mut self, v: ::std::string::String) {
        self.coders = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_coders(&mut self) -> &mut ::std::string::String {
        if self.coders.is_none() {
            self.coders.set_default();
        }
        self.coders.as_mut().unwrap()
    }

    // Take field
    pub fn take_coders(&mut self) -> ::std::string::String {
        self.coders.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_coders(&self) -> &str {
        match self.coders.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_coders_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.coders
    }

    fn mut_coders_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.coders
    }
}

impl ::protobuf::Message for CodecProto {
    fn is_initialized(&self) -> bool {
        if self.codec.is_none() {
            return false;
        }
        if self.coders.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codec)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.coders)?;
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
        if let Some(ref v) = self.codec.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.coders.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.codec.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.coders.as_ref() {
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

impl ::protobuf::MessageStatic for CodecProto {
    fn new() -> CodecProto {
        CodecProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CodecProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "codec",
                    CodecProto::get_codec_for_reflect,
                    CodecProto::mut_codec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "coders",
                    CodecProto::get_coders_for_reflect,
                    CodecProto::mut_coders_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CodecProto>(
                    "CodecProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CodecProto {
    fn clear(&mut self) {
        self.clear_codec();
        self.clear_coders();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CodecProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodecProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13erasurecoding.proto\x12\x0bhadoop.hdfs\x1a\nhdfs.proto\"Z\n\"SetEr\
    asureCodingPolicyRequestProto\x12\x10\n\x03src\x18\x01\x20\x02(\tR\x03sr\
    c\x12\"\n\x0cecPolicyName\x18\x02\x20\x01(\tR\x0cecPolicyName\"%\n#SetEr\
    asureCodingPolicyResponseProto\"&\n$GetErasureCodingPoliciesRequestProto\
    \"n\n%GetErasureCodingPoliciesResponseProto\x12E\n\necPolicies\x18\x01\
    \x20\x03(\x0b2%.hadoop.hdfs.ErasureCodingPolicyProtoR\necPolicies\"$\n\"\
    GetErasureCodingCodecsRequestProto\"T\n#GetErasureCodingCodecsResponsePr\
    oto\x12-\n\x05codec\x18\x01\x20\x03(\x0b2\x17.hadoop.hdfs.CodecProtoR\
    \x05codec\"6\n\"GetErasureCodingPolicyRequestProto\x12\x10\n\x03src\x18\
    \x01\x20\x02(\tR\x03src\"h\n#GetErasureCodingPolicyResponseProto\x12A\n\
    \x08ecPolicy\x18\x01\x20\x01(\x0b2%.hadoop.hdfs.ErasureCodingPolicyProto\
    R\x08ecPolicy\"m\n$AddErasureCodingPoliciesRequestProto\x12E\n\necPolici\
    es\x18\x01\x20\x03(\x0b2%.hadoop.hdfs.ErasureCodingPolicyProtoR\necPolic\
    ies\"w\n%AddErasureCodingPoliciesResponseProto\x12N\n\tresponses\x18\x01\
    \x20\x03(\x0b20.hadoop.hdfs.AddErasureCodingPolicyResponseProtoR\trespon\
    ses\"K\n%RemoveErasureCodingPolicyRequestProto\x12\"\n\x0cecPolicyName\
    \x18\x01\x20\x02(\tR\x0cecPolicyName\"(\n&RemoveErasureCodingPolicyRespo\
    nseProto\"K\n%EnableErasureCodingPolicyRequestProto\x12\"\n\x0cecPolicyN\
    ame\x18\x01\x20\x02(\tR\x0cecPolicyName\"(\n&EnableErasureCodingPolicyRe\
    sponseProto\"L\n&DisableErasureCodingPolicyRequestProto\x12\"\n\x0cecPol\
    icyName\x18\x01\x20\x02(\tR\x0cecPolicyName\")\n'DisableErasureCodingPol\
    icyResponseProto\"8\n$UnsetErasureCodingPolicyRequestProto\x12\x10\n\x03\
    src\x18\x01\x20\x02(\tR\x03src\"'\n%UnsetErasureCodingPolicyResponseProt\
    o\"\xf4\x03\n\x1eBlockECReconstructionInfoProto\x125\n\x05block\x18\x01\
    \x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProtoR\x05block\x12E\n\rsour\
    ceDnInfos\x18\x02\x20\x02(\x0b2\x1f.hadoop.hdfs.DatanodeInfosProtoR\rsou\
    rceDnInfos\x12E\n\rtargetDnInfos\x18\x03\x20\x02(\x0b2\x1f.hadoop.hdfs.D\
    atanodeInfosProtoR\rtargetDnInfos\x12N\n\x12targetStorageUuids\x18\x04\
    \x20\x02(\x0b2\x1e.hadoop.hdfs.StorageUuidsProtoR\x12targetStorageUuids\
    \x12N\n\x12targetStorageTypes\x18\x05\x20\x02(\x0b2\x1e.hadoop.hdfs.Stor\
    ageTypesProtoR\x12targetStorageTypes\x12*\n\x10liveBlockIndices\x18\x06\
    \x20\x02(\x0cR\x10liveBlockIndices\x12A\n\x08ecPolicy\x18\x07\x20\x02(\
    \x0b2%.hadoop.hdfs.ErasureCodingPolicyProtoR\x08ecPolicy\":\n\nCodecProt\
    o\x12\x14\n\x05codec\x18\x01\x20\x02(\tR\x05codec\x12\x16\n\x06coders\
    \x18\x02\x20\x02(\tR\x06codersB?\n%org.apache.hadoop.hdfs.protocol.proto\
    B\x13ErasureCodingProtos\xa0\x01\x01J\xb9\x18\n\x06\x12\x04\x12\0n\x01\n\
    \x08\n\x01\x08\x12\x03\x12\0>\n\x91\x06\n\x04\x08\xe7\x07\0\x12\x03\x12\
    \0>2\x83\x06*\n\x20Licensed\x20to\x20the\x20Apache\x20Software\x20Founda\
    tion\x20(ASF)\x20under\x20one\n\x20or\x20more\x20contributor\x20license\
    \x20agreements.\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20\
    with\x20this\x20work\x20for\x20additional\x20information\n\x20regarding\
    \x20copyright\x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20fil\
    e\n\x20to\x20you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
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
    e\x20License.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x12\x07\x13\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\x12\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x12\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x12\
    \x16=\n\x08\n\x01\x08\x12\x03\x13\04\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\
    \x13\04\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x13\x07\x1b\n\r\n\x06\
    \x08\xe7\x07\x01\x02\0\x12\x03\x13\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03\x13\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\
    \x13\x1e3\n\x08\n\x01\x08\x12\x03\x14\0,\n\x0b\n\x04\x08\xe7\x07\x02\x12\
    \x03\x14\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x14\x07$\n\r\n\x06\
    \x08\xe7\x07\x02\x02\0\x12\x03\x14\x07$\n\x0e\n\x07\x08\xe7\x07\x02\x02\
    \0\x01\x12\x03\x14\x07$\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x14'+\n\
    \x08\n\x01\x02\x12\x03\x15\x08\x13\n\t\n\x02\x03\0\x12\x03\x17\x07\x13\n\
    \n\n\x02\x04\0\x12\x04\x19\0\x1c\x01\n\n\n\x03\x04\0\x01\x12\x03\x19\x08\
    *\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1a\x02\x1a\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1a\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1a\x12\x15\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x1a\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x1b\x02#\n\
    \x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x1b\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1b\x12\
    \x1e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1b!\"\n\n\n\x02\x04\x01\x12\
    \x04\x1e\0\x1f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1e\x08+\n\x1a\n\x02\
    \x04\x02\x12\x04!\0\"\x01\"\x0e\x20void\x20request\n\n\n\n\x03\x04\x02\
    \x01\x12\x03!\x08,\n\n\n\x02\x04\x03\x12\x04$\0&\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03$\x08-\n\x0b\n\x04\x04\x03\x02\0\x12\x03%\x023\n\x0c\n\x05\
    \x04\x03\x02\0\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03%\
    \x0b#\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03%$.\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03%12\n\x1a\n\x02\x04\x04\x12\x04(\0)\x01\"\x0e\x20void\x20req\
    uest\n\n\n\n\x03\x04\x04\x01\x12\x03(\x08*\n\n\n\x02\x04\x05\x12\x04+\0-\
    \x01\n\n\n\x03\x04\x05\x01\x12\x03+\x08+\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x03,\x02\x20\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03,\x02\n\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x03,\x0b\x15\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03,\x16\x1b\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03,\x1e\x1f\n\n\n\x02\
    \x04\x06\x12\x04/\01\x01\n\n\n\x03\x04\x06\x01\x12\x03/\x08*\n*\n\x04\
    \x04\x06\x02\0\x12\x030\x02\x1a\"\x1d\x20path\x20to\x20get\x20the\x20pol\
    icy\x20info\n\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\
    \x04\x06\x02\0\x05\x12\x030\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x030\x12\x15\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x030\x18\x19\n\n\n\x02\
    \x04\x07\x12\x043\05\x01\n\n\n\x03\x04\x07\x01\x12\x033\x08+\n\x0b\n\x04\
    \x04\x07\x02\0\x12\x034\x021\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x034\x02\
    \n\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x034\x0b#\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x034$,\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x034/0\n\n\n\x02\x04\
    \x08\x12\x047\09\x01\n\n\n\x03\x04\x08\x01\x12\x037\x08,\n\x0b\n\x04\x04\
    \x08\x02\0\x12\x038\x023\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x038\x02\n\n\
    \x0c\n\x05\x04\x08\x02\0\x06\x12\x038\x0b#\n\x0c\n\x05\x04\x08\x02\0\x01\
    \x12\x038$.\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03812\n\n\n\x02\x04\t\x12\
    \x04;\0=\x01\n\n\n\x03\x04\t\x01\x12\x03;\x08-\n\x0b\n\x04\x04\t\x02\0\
    \x12\x03<\x02=\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03<\x02\n\n\x0c\n\x05\
    \x04\t\x02\0\x06\x12\x03<\x0b.\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03</8\n\
    \x0c\n\x05\x04\t\x02\0\x03\x12\x03<;<\n\n\n\x02\x04\n\x12\x04?\0A\x01\n\
    \n\n\x03\x04\n\x01\x12\x03?\x08-\n\x0b\n\x04\x04\n\x02\0\x12\x03@\x02#\n\
    \x0c\n\x05\x04\n\x02\0\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\n\x02\0\x05\
    \x12\x03@\x0b\x11\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03@\x12\x1e\n\x0c\n\
    \x05\x04\n\x02\0\x03\x12\x03@!\"\n\n\n\x02\x04\x0b\x12\x04C\0D\x01\n\n\n\
    \x03\x04\x0b\x01\x12\x03C\x08.\n\n\n\x02\x04\x0c\x12\x04F\0H\x01\n\n\n\
    \x03\x04\x0c\x01\x12\x03F\x08-\n\x0b\n\x04\x04\x0c\x02\0\x12\x03G\x02#\n\
    \x0c\n\x05\x04\x0c\x02\0\x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x0c\x02\0\
    \x05\x12\x03G\x0b\x11\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03G\x12\x1e\n\
    \x0c\n\x05\x04\x0c\x02\0\x03\x12\x03G!\"\n\n\n\x02\x04\r\x12\x04J\0K\x01\
    \n\n\n\x03\x04\r\x01\x12\x03J\x08.\n\n\n\x02\x04\x0e\x12\x04M\0O\x01\n\n\
    \n\x03\x04\x0e\x01\x12\x03M\x08.\n\x0b\n\x04\x04\x0e\x02\0\x12\x03N\x02#\
    \n\x0c\n\x05\x04\x0e\x02\0\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x0e\x02\0\
    \x05\x12\x03N\x0b\x11\n\x0c\n\x05\x04\x0e\x02\0\x01\x12\x03N\x12\x1e\n\
    \x0c\n\x05\x04\x0e\x02\0\x03\x12\x03N!\"\n\n\n\x02\x04\x0f\x12\x04Q\0R\
    \x01\n\n\n\x03\x04\x0f\x01\x12\x03Q\x08/\n\n\n\x02\x04\x10\x12\x04T\0V\
    \x01\n\n\n\x03\x04\x10\x01\x12\x03T\x08,\n\x0b\n\x04\x04\x10\x02\0\x12\
    \x03U\x02\x1a\n\x0c\n\x05\x04\x10\x02\0\x04\x12\x03U\x02\n\n\x0c\n\x05\
    \x04\x10\x02\0\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x10\x02\0\x01\x12\
    \x03U\x12\x15\n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03U\x18\x19\n\n\n\x02\
    \x04\x11\x12\x04X\0Y\x01\n\n\n\x03\x04\x11\x01\x12\x03X\x08-\n8\n\x02\
    \x04\x12\x12\x04^\0f\x01\x1a,*\n\x20Block\x20erasure\x20coding\x20recons\
    truction\x20info\n\n\n\n\x03\x04\x12\x01\x12\x03^\x08&\n\x0b\n\x04\x04\
    \x12\x02\0\x12\x03_\x02(\n\x0c\n\x05\x04\x12\x02\0\x04\x12\x03_\x02\n\n\
    \x0c\n\x05\x04\x12\x02\0\x06\x12\x03_\x0b\x1d\n\x0c\n\x05\x04\x12\x02\0\
    \x01\x12\x03_\x1e#\n\x0c\n\x05\x04\x12\x02\0\x03\x12\x03_&'\n\x0b\n\x04\
    \x04\x12\x02\x01\x12\x03`\x020\n\x0c\n\x05\x04\x12\x02\x01\x04\x12\x03`\
    \x02\n\n\x0c\n\x05\x04\x12\x02\x01\x06\x12\x03`\x0b\x1d\n\x0c\n\x05\x04\
    \x12\x02\x01\x01\x12\x03`\x1e+\n\x0c\n\x05\x04\x12\x02\x01\x03\x12\x03`.\
    /\n\x0b\n\x04\x04\x12\x02\x02\x12\x03a\x020\n\x0c\n\x05\x04\x12\x02\x02\
    \x04\x12\x03a\x02\n\n\x0c\n\x05\x04\x12\x02\x02\x06\x12\x03a\x0b\x1d\n\
    \x0c\n\x05\x04\x12\x02\x02\x01\x12\x03a\x1e+\n\x0c\n\x05\x04\x12\x02\x02\
    \x03\x12\x03a./\n\x0b\n\x04\x04\x12\x02\x03\x12\x03b\x024\n\x0c\n\x05\
    \x04\x12\x02\x03\x04\x12\x03b\x02\n\n\x0c\n\x05\x04\x12\x02\x03\x06\x12\
    \x03b\x0b\x1c\n\x0c\n\x05\x04\x12\x02\x03\x01\x12\x03b\x1d/\n\x0c\n\x05\
    \x04\x12\x02\x03\x03\x12\x03b23\n\x0b\n\x04\x04\x12\x02\x04\x12\x03c\x02\
    4\n\x0c\n\x05\x04\x12\x02\x04\x04\x12\x03c\x02\n\n\x0c\n\x05\x04\x12\x02\
    \x04\x06\x12\x03c\x0b\x1c\n\x0c\n\x05\x04\x12\x02\x04\x01\x12\x03c\x1d/\
    \n\x0c\n\x05\x04\x12\x02\x04\x03\x12\x03c23\n\x0b\n\x04\x04\x12\x02\x05\
    \x12\x03d\x02&\n\x0c\n\x05\x04\x12\x02\x05\x04\x12\x03d\x02\n\n\x0c\n\
    \x05\x04\x12\x02\x05\x05\x12\x03d\x0b\x10\n\x0c\n\x05\x04\x12\x02\x05\
    \x01\x12\x03d\x11!\n\x0c\n\x05\x04\x12\x02\x05\x03\x12\x03d$%\n\x0b\n\
    \x04\x04\x12\x02\x06\x12\x03e\x021\n\x0c\n\x05\x04\x12\x02\x06\x04\x12\
    \x03e\x02\n\n\x0c\n\x05\x04\x12\x02\x06\x06\x12\x03e\x0b#\n\x0c\n\x05\
    \x04\x12\x02\x06\x01\x12\x03e$,\n\x0c\n\x05\x04\x12\x02\x06\x03\x12\x03e\
    /0\n3\n\x02\x04\x13\x12\x04k\0n\x01\x1a'*\n\x20Codec\x20and\x20it's\x20c\
    orresponding\x20coders\n\n\n\n\x03\x04\x13\x01\x12\x03k\x08\x12\n\x0b\n\
    \x04\x04\x13\x02\0\x12\x03l\x02\x1d\n\x0c\n\x05\x04\x13\x02\0\x04\x12\
    \x03l\x02\n\n\x0c\n\x05\x04\x13\x02\0\x05\x12\x03l\x0b\x11\n\x0c\n\x05\
    \x04\x13\x02\0\x01\x12\x03l\x12\x17\n\x0c\n\x05\x04\x13\x02\0\x03\x12\
    \x03l\x1b\x1c\n\x0b\n\x04\x04\x13\x02\x01\x12\x03m\x02\x1d\n\x0c\n\x05\
    \x04\x13\x02\x01\x04\x12\x03m\x02\n\n\x0c\n\x05\x04\x13\x02\x01\x05\x12\
    \x03m\x0b\x11\n\x0c\n\x05\x04\x13\x02\x01\x01\x12\x03m\x12\x18\n\x0c\n\
    \x05\x04\x13\x02\x01\x03\x12\x03m\x1b\x1c\
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
