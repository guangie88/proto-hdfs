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
pub struct StartReconfigurationRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartReconfigurationRequestProto {}

impl StartReconfigurationRequestProto {
    pub fn new() -> StartReconfigurationRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationRequestProto,
        };
        unsafe {
            instance.get(StartReconfigurationRequestProto::new)
        }
    }
}

impl ::protobuf::Message for StartReconfigurationRequestProto {
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

impl ::protobuf::MessageStatic for StartReconfigurationRequestProto {
    fn new() -> StartReconfigurationRequestProto {
        StartReconfigurationRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationRequestProto>(
                    "StartReconfigurationRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartReconfigurationRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartReconfigurationRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartReconfigurationResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartReconfigurationResponseProto {}

impl StartReconfigurationResponseProto {
    pub fn new() -> StartReconfigurationResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationResponseProto,
        };
        unsafe {
            instance.get(StartReconfigurationResponseProto::new)
        }
    }
}

impl ::protobuf::Message for StartReconfigurationResponseProto {
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

impl ::protobuf::MessageStatic for StartReconfigurationResponseProto {
    fn new() -> StartReconfigurationResponseProto {
        StartReconfigurationResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationResponseProto>(
                    "StartReconfigurationResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartReconfigurationResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartReconfigurationResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusRequestProto {}

impl GetReconfigurationStatusRequestProto {
    pub fn new() -> GetReconfigurationStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusRequestProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetReconfigurationStatusRequestProto {
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusRequestProto {
    fn new() -> GetReconfigurationStatusRequestProto {
        GetReconfigurationStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusRequestProto>(
                    "GetReconfigurationStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusConfigChangeProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    oldValue: ::protobuf::SingularField<::std::string::String>,
    newValue: ::protobuf::SingularField<::std::string::String>,
    errorMessage: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusConfigChangeProto {}

impl GetReconfigurationStatusConfigChangeProto {
    pub fn new() -> GetReconfigurationStatusConfigChangeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusConfigChangeProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusConfigChangeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusConfigChangeProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusConfigChangeProto::new)
        }
    }

    // required string name = 1;

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

    // required string oldValue = 2;

    pub fn clear_oldValue(&mut self) {
        self.oldValue.clear();
    }

    pub fn has_oldValue(&self) -> bool {
        self.oldValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldValue(&mut self, v: ::std::string::String) {
        self.oldValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldValue(&mut self) -> &mut ::std::string::String {
        if self.oldValue.is_none() {
            self.oldValue.set_default();
        }
        self.oldValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldValue(&mut self) -> ::std::string::String {
        self.oldValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oldValue(&self) -> &str {
        match self.oldValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_oldValue_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.oldValue
    }

    fn mut_oldValue_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.oldValue
    }

    // optional string newValue = 3;

    pub fn clear_newValue(&mut self) {
        self.newValue.clear();
    }

    pub fn has_newValue(&self) -> bool {
        self.newValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newValue(&mut self, v: ::std::string::String) {
        self.newValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newValue(&mut self) -> &mut ::std::string::String {
        if self.newValue.is_none() {
            self.newValue.set_default();
        }
        self.newValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_newValue(&mut self) -> ::std::string::String {
        self.newValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_newValue(&self) -> &str {
        match self.newValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_newValue_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.newValue
    }

    fn mut_newValue_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.newValue
    }

    // optional string errorMessage = 4;

    pub fn clear_errorMessage(&mut self) {
        self.errorMessage.clear();
    }

    pub fn has_errorMessage(&self) -> bool {
        self.errorMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMessage(&mut self, v: ::std::string::String) {
        self.errorMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMessage(&mut self) -> &mut ::std::string::String {
        if self.errorMessage.is_none() {
            self.errorMessage.set_default();
        }
        self.errorMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMessage(&mut self) -> ::std::string::String {
        self.errorMessage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMessage(&self) -> &str {
        match self.errorMessage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_errorMessage_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.errorMessage
    }

    fn mut_errorMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.errorMessage
    }
}

impl ::protobuf::Message for GetReconfigurationStatusConfigChangeProto {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.oldValue.is_none() {
            return false;
        }
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.oldValue)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.newValue)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.errorMessage)?;
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
        if let Some(ref v) = self.oldValue.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.newValue.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.errorMessage.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.oldValue.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.newValue.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.errorMessage.as_ref() {
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusConfigChangeProto {
    fn new() -> GetReconfigurationStatusConfigChangeProto {
        GetReconfigurationStatusConfigChangeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusConfigChangeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GetReconfigurationStatusConfigChangeProto::get_name_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oldValue",
                    GetReconfigurationStatusConfigChangeProto::get_oldValue_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_oldValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "newValue",
                    GetReconfigurationStatusConfigChangeProto::get_newValue_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_newValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "errorMessage",
                    GetReconfigurationStatusConfigChangeProto::get_errorMessage_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_errorMessage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusConfigChangeProto>(
                    "GetReconfigurationStatusConfigChangeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusConfigChangeProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_oldValue();
        self.clear_newValue();
        self.clear_errorMessage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusConfigChangeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusConfigChangeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusResponseProto {
    // message fields
    startTime: ::std::option::Option<i64>,
    endTime: ::std::option::Option<i64>,
    changes: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusResponseProto {}

impl GetReconfigurationStatusResponseProto {
    pub fn new() -> GetReconfigurationStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusResponseProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusResponseProto::new)
        }
    }

    // required int64 startTime = 1;

    pub fn clear_startTime(&mut self) {
        self.startTime = ::std::option::Option::None;
    }

    pub fn has_startTime(&self) -> bool {
        self.startTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTime(&mut self, v: i64) {
        self.startTime = ::std::option::Option::Some(v);
    }

    pub fn get_startTime(&self) -> i64 {
        self.startTime.unwrap_or(0)
    }

    fn get_startTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.startTime
    }

    fn mut_startTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.startTime
    }

    // optional int64 endTime = 2;

    pub fn clear_endTime(&mut self) {
        self.endTime = ::std::option::Option::None;
    }

    pub fn has_endTime(&self) -> bool {
        self.endTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endTime(&mut self, v: i64) {
        self.endTime = ::std::option::Option::Some(v);
    }

    pub fn get_endTime(&self) -> i64 {
        self.endTime.unwrap_or(0)
    }

    fn get_endTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.endTime
    }

    fn mut_endTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.endTime
    }

    // repeated .hadoop.hdfs.GetReconfigurationStatusConfigChangeProto changes = 3;

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    // Param is passed by value, moved
    pub fn set_changes(&mut self, v: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>) {
        self.changes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_changes(&mut self) -> &mut ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &mut self.changes
    }

    // Take field
    pub fn take_changes(&mut self) -> ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        ::std::mem::replace(&mut self.changes, ::protobuf::RepeatedField::new())
    }

    pub fn get_changes(&self) -> &[GetReconfigurationStatusConfigChangeProto] {
        &self.changes
    }

    fn get_changes_for_reflect(&self) -> &::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &self.changes
    }

    fn mut_changes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &mut self.changes
    }
}

impl ::protobuf::Message for GetReconfigurationStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.startTime.is_none() {
            return false;
        }
        for v in &self.changes {
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
                    let tmp = is.read_int64()?;
                    self.startTime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.endTime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.changes)?;
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
        if let Some(v) = self.startTime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.endTime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.changes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.startTime {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.endTime {
            os.write_int64(2, v)?;
        }
        for v in &self.changes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusResponseProto {
    fn new() -> GetReconfigurationStatusResponseProto {
        GetReconfigurationStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "startTime",
                    GetReconfigurationStatusResponseProto::get_startTime_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_startTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "endTime",
                    GetReconfigurationStatusResponseProto::get_endTime_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_endTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetReconfigurationStatusConfigChangeProto>>(
                    "changes",
                    GetReconfigurationStatusResponseProto::get_changes_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_changes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusResponseProto>(
                    "GetReconfigurationStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusResponseProto {
    fn clear(&mut self) {
        self.clear_startTime();
        self.clear_endTime();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListReconfigurablePropertiesRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListReconfigurablePropertiesRequestProto {}

impl ListReconfigurablePropertiesRequestProto {
    pub fn new() -> ListReconfigurablePropertiesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListReconfigurablePropertiesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListReconfigurablePropertiesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListReconfigurablePropertiesRequestProto,
        };
        unsafe {
            instance.get(ListReconfigurablePropertiesRequestProto::new)
        }
    }
}

impl ::protobuf::Message for ListReconfigurablePropertiesRequestProto {
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

impl ::protobuf::MessageStatic for ListReconfigurablePropertiesRequestProto {
    fn new() -> ListReconfigurablePropertiesRequestProto {
        ListReconfigurablePropertiesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListReconfigurablePropertiesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListReconfigurablePropertiesRequestProto>(
                    "ListReconfigurablePropertiesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListReconfigurablePropertiesRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListReconfigurablePropertiesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListReconfigurablePropertiesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListReconfigurablePropertiesResponseProto {
    // message fields
    name: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListReconfigurablePropertiesResponseProto {}

impl ListReconfigurablePropertiesResponseProto {
    pub fn new() -> ListReconfigurablePropertiesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListReconfigurablePropertiesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListReconfigurablePropertiesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListReconfigurablePropertiesResponseProto,
        };
        unsafe {
            instance.get(ListReconfigurablePropertiesResponseProto::new)
        }
    }

    // repeated string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.name, ::protobuf::RepeatedField::new())
    }

    pub fn get_name(&self) -> &[::std::string::String] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for ListReconfigurablePropertiesResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.name)?;
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.name {
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

impl ::protobuf::MessageStatic for ListReconfigurablePropertiesResponseProto {
    fn new() -> ListReconfigurablePropertiesResponseProto {
        ListReconfigurablePropertiesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListReconfigurablePropertiesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ListReconfigurablePropertiesResponseProto::get_name_for_reflect,
                    ListReconfigurablePropertiesResponseProto::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListReconfigurablePropertiesResponseProto>(
                    "ListReconfigurablePropertiesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListReconfigurablePropertiesResponseProto {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListReconfigurablePropertiesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListReconfigurablePropertiesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dReconfigurationProtocol.proto\x12\x0bhadoop.hdfs\"\"\n\x20StartRec\
    onfigurationRequestProto\"#\n!StartReconfigurationResponseProto\"&\n$Get\
    ReconfigurationStatusRequestProto\"\x9b\x01\n)GetReconfigurationStatusCo\
    nfigChangeProto\x12\x12\n\x04name\x18\x01\x20\x02(\tR\x04name\x12\x1a\n\
    \x08oldValue\x18\x02\x20\x02(\tR\x08oldValue\x12\x1a\n\x08newValue\x18\
    \x03\x20\x01(\tR\x08newValue\x12\"\n\x0cerrorMessage\x18\x04\x20\x01(\tR\
    \x0cerrorMessage\"\xb1\x01\n%GetReconfigurationStatusResponseProto\x12\
    \x1c\n\tstartTime\x18\x01\x20\x02(\x03R\tstartTime\x12\x18\n\x07endTime\
    \x18\x02\x20\x01(\x03R\x07endTime\x12P\n\x07changes\x18\x03\x20\x03(\x0b\
    26.hadoop.hdfs.GetReconfigurationStatusConfigChangeProtoR\x07changes\"*\
    \n(ListReconfigurablePropertiesRequestProto\"?\n)ListReconfigurablePrope\
    rtiesResponseProto\x12\x12\n\x04name\x18\x01\x20\x03(\tR\x04name2\xab\
    \x03\n\x1eReconfigurationProtocolService\x12\x81\x01\n\x18getReconfigura\
    tionStatus\x121.hadoop.hdfs.GetReconfigurationStatusRequestProto\x1a2.ha\
    doop.hdfs.GetReconfigurationStatusResponseProto\x12u\n\x14startReconfigu\
    ration\x12-.hadoop.hdfs.StartReconfigurationRequestProto\x1a..hadoop.hdf\
    s.StartReconfigurationResponseProto\x12\x8d\x01\n\x1clistReconfigurableP\
    roperties\x125.hadoop.hdfs.ListReconfigurablePropertiesRequestProto\x1a6\
    .hadoop.hdfs.ListReconfigurablePropertiesResponseProtoBL\n%org.apache.ha\
    doop.hdfs.protocol.protoB\x1dReconfigurationProtocolProtos\xa0\x01\x01\
    \x88\x01\x01J\xf1\x12\n\x06\x12\x04\x15\0I\x01\n\x08\n\x01\x08\x12\x03\
    \x15\0>\n\xfb\x06\n\x04\x08\xe7\x07\0\x12\x03\x15\0>2\x83\x06*\n\x20Lice\
    nsed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\x20under\
    \x20one\n\x20or\x20more\x20contributor\x20license\x20agreements.\x20\x20\
    See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\x20work\
    \x20for\x20additional\x20information\n\x20regarding\x20copyright\x20owne\
    rship.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20you\x20u\
    nder\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\n\x20\"Licen\
    se\");\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20co\
    mpliance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20obtain\x20a\
    \x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www\
    .apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20appl\
    icable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20d\
    istributed\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\
    \x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITION\
    S\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\
    \x20the\x20License\x20for\x20the\x20specific\x20language\x20governing\
    \x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\n2h\
    \x20This\x20file\x20contains\x20protocol\x20buffers\x20that\x20are\x20us\
    ed\x20to\x20reconfigure\x20NameNode\n\x20and\x20DataNode\x20by\x20HDFS\
    \x20admin.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x15\x07\x13\n\r\n\x06\
    \x08\xe7\x07\0\x02\0\x12\x03\x15\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\x15\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x15\x16=\
    \n\x08\n\x01\x08\x12\x03\x16\0>\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x16\
    \0>\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x16\x07\x1b\n\r\n\x06\x08\
    \xe7\x07\x01\x02\0\x12\x03\x16\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\
    \x01\x12\x03\x16\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x16\x1e\
    =\n\x08\n\x01\x08\x12\x03\x17\0$\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x17\
    \0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x17\x07\x1c\n\r\n\x06\x08\
    \xe7\x07\x02\x02\0\x12\x03\x17\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\
    \x01\x12\x03\x17\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x17\x1f\
    #\n\x08\n\x01\x08\x12\x03\x18\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x18\
    \0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x18\x07$\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\x18\x07$\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\x18\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x18'+\n\x08\n\
    \x01\x02\x12\x03\x19\x08\x13\n7\n\x02\x04\0\x12\x04\x1c\0\x1d\x01\x1a+*\
    \x20Asks\x20NN/DN\x20to\x20reload\x20configuration\x20file.\x20\n\n\n\
    \x03\x04\0\x01\x12\x03\x1c\x08(\n\n\n\x02\x04\x01\x12\x04\x1f\0\x20\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x1f\x08)\nB\n\x02\x04\x02\x12\x04#\0$\x01\
    \x1a6*\x20Query\x20the\x20running\x20status\x20of\x20reconfiguration\x20\
    process\x20\n\n\n\x03\x04\x02\x01\x12\x03#\x08,\n\n\n\x02\x04\x03\x12\
    \x04&\0+\x01\n\n\n\x03\x04\x03\x01\x12\x03&\x081\n\x0b\n\x04\x04\x03\x02\
    \0\x12\x03'\x02\x1b\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03'\x02\n\n\x0c\n\
    \x05\x04\x03\x02\0\x05\x12\x03'\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03'\x12\x16\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03'\x19\x1a\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x03(\x02\x1f\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x03(\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03(\x0b\x11\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03(\x12\x1a\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03(\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x02\x12\x03)\x02\x1f\n\x0c\
    \n\x05\x04\x03\x02\x02\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x03\x02\x02\
    \x05\x12\x03)\x0b\x11\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03)\x12\x1a\n\
    \x0c\n\x05\x04\x03\x02\x02\x03\x12\x03)\x1d\x1e\n&\n\x04\x04\x03\x02\x03\
    \x12\x03*\x02#\"\x19\x20It\x20is\x20empty\x20if\x20success.\n\n\x0c\n\
    \x05\x04\x03\x02\x03\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\x03\x02\x03\x05\
    \x12\x03*\x0b\x11\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03*\x12\x1e\n\x0c\
    \n\x05\x04\x03\x02\x03\x03\x12\x03*!\"\n\n\n\x02\x04\x04\x12\x04-\01\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03-\x08-\n\x0b\n\x04\x04\x04\x02\0\x12\x03.\
    \x02\x1f\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\
    \x04\x02\0\x05\x12\x03.\x0b\x10\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03.\
    \x11\x1a\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03.\x1d\x1e\n\x0b\n\x04\x04\
    \x04\x02\x01\x12\x03/\x02\x1d\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03/\
    \x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03/\x0b\x10\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03/\x11\x18\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03/\x1b\x1c\n\x0b\n\x04\x04\x04\x02\x02\x12\x030\x02A\n\x0c\n\x05\x04\
    \x04\x02\x02\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x06\x12\x030\
    \x0b4\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x0305<\n\x0c\n\x05\x04\x04\x02\
    \x02\x03\x12\x030?@\n<\n\x02\x04\x05\x12\x044\05\x01\x1a0*\x20Query\x20t\
    he\x20reconfigurable\x20properties\x20on\x20NN/DN.\x20\n\n\n\x03\x04\x05\
    \x01\x12\x034\x080\n\n\n\x02\x04\x06\x12\x047\09\x01\n\n\n\x03\x04\x06\
    \x01\x12\x037\x081\n\x0b\n\x04\x04\x06\x02\0\x12\x038\x02\x1b\n\x0c\n\
    \x05\x04\x06\x02\0\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\
    \x038\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x038\x12\x16\n\x0c\n\x05\
    \x04\x06\x02\0\x03\x12\x038\x19\x1a\nn\n\x02\x06\0\x12\x04?\0I\x01\x1ab*\
    \n\x20Protocol\x20used\x20from\x20client\x20to\x20the\x20NN/DN.\n\x20See\
    \x20the\x20request\x20and\x20response\x20for\x20details\x20of\x20rpc\x20\
    call.\n\n\n\n\x03\x06\0\x01\x12\x03?\x08&\n\x0c\n\x04\x06\0\x02\0\x12\
    \x04@\x02A5\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03@\x06\x1e\n\x0c\n\x05\x06\
    \0\x02\0\x02\x12\x03@\x1fC\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03A\x0e3\n\
    \x0c\n\x04\x06\0\x02\x01\x12\x04C\x02D1\n\x0c\n\x05\x06\0\x02\x01\x01\
    \x12\x03C\x06\x1a\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03C\x1b;\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x03D\x0e/\n\x0c\n\x04\x06\0\x02\x02\x12\x04F\
    \x02H9\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03F\x06\"\n\x0c\n\x05\x06\0\
    \x02\x02\x02\x12\x03G\x06.\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03H\x0e7\
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
