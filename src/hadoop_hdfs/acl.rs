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
pub struct FsPermissionProto {
    // message fields
    perm: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FsPermissionProto {}

impl FsPermissionProto {
    pub fn new() -> FsPermissionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsPermissionProto {
        static mut instance: ::protobuf::lazy::Lazy<FsPermissionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsPermissionProto,
        };
        unsafe {
            instance.get(FsPermissionProto::new)
        }
    }

    // required uint32 perm = 1;

    pub fn clear_perm(&mut self) {
        self.perm = ::std::option::Option::None;
    }

    pub fn has_perm(&self) -> bool {
        self.perm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: u32) {
        self.perm = ::std::option::Option::Some(v);
    }

    pub fn get_perm(&self) -> u32 {
        self.perm.unwrap_or(0)
    }

    fn get_perm_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.perm
    }

    fn mut_perm_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.perm
    }
}

impl ::protobuf::Message for FsPermissionProto {
    fn is_initialized(&self) -> bool {
        if self.perm.is_none() {
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
                    self.perm = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.perm {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.perm {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for FsPermissionProto {
    fn new() -> FsPermissionProto {
        FsPermissionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsPermissionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "perm",
                    FsPermissionProto::get_perm_for_reflect,
                    FsPermissionProto::mut_perm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsPermissionProto>(
                    "FsPermissionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsPermissionProto {
    fn clear(&mut self) {
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FsPermissionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FsPermissionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AclEntryProto {
    // message fields
    field_type: ::std::option::Option<AclEntryProto_AclEntryTypeProto>,
    scope: ::std::option::Option<AclEntryProto_AclEntryScopeProto>,
    permissions: ::std::option::Option<AclEntryProto_FsActionProto>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AclEntryProto {}

impl AclEntryProto {
    pub fn new() -> AclEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AclEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<AclEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AclEntryProto,
        };
        unsafe {
            instance.get(AclEntryProto::new)
        }
    }

    // required .hadoop.hdfs.AclEntryProto.AclEntryTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: AclEntryProto_AclEntryTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> AclEntryProto_AclEntryTypeProto {
        self.field_type.unwrap_or(AclEntryProto_AclEntryTypeProto::USER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<AclEntryProto_AclEntryTypeProto> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<AclEntryProto_AclEntryTypeProto> {
        &mut self.field_type
    }

    // required .hadoop.hdfs.AclEntryProto.AclEntryScopeProto scope = 2;

    pub fn clear_scope(&mut self) {
        self.scope = ::std::option::Option::None;
    }

    pub fn has_scope(&self) -> bool {
        self.scope.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scope(&mut self, v: AclEntryProto_AclEntryScopeProto) {
        self.scope = ::std::option::Option::Some(v);
    }

    pub fn get_scope(&self) -> AclEntryProto_AclEntryScopeProto {
        self.scope.unwrap_or(AclEntryProto_AclEntryScopeProto::ACCESS)
    }

    fn get_scope_for_reflect(&self) -> &::std::option::Option<AclEntryProto_AclEntryScopeProto> {
        &self.scope
    }

    fn mut_scope_for_reflect(&mut self) -> &mut ::std::option::Option<AclEntryProto_AclEntryScopeProto> {
        &mut self.scope
    }

    // required .hadoop.hdfs.AclEntryProto.FsActionProto permissions = 3;

    pub fn clear_permissions(&mut self) {
        self.permissions = ::std::option::Option::None;
    }

    pub fn has_permissions(&self) -> bool {
        self.permissions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: AclEntryProto_FsActionProto) {
        self.permissions = ::std::option::Option::Some(v);
    }

    pub fn get_permissions(&self) -> AclEntryProto_FsActionProto {
        self.permissions.unwrap_or(AclEntryProto_FsActionProto::NONE)
    }

    fn get_permissions_for_reflect(&self) -> &::std::option::Option<AclEntryProto_FsActionProto> {
        &self.permissions
    }

    fn mut_permissions_for_reflect(&mut self) -> &mut ::std::option::Option<AclEntryProto_FsActionProto> {
        &mut self.permissions
    }

    // optional string name = 4;

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
}

impl ::protobuf::Message for AclEntryProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.scope.is_none() {
            return false;
        }
        if self.permissions.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.scope = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.permissions = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.scope {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.permissions {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.scope {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.permissions {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for AclEntryProto {
    fn new() -> AclEntryProto {
        AclEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AclEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AclEntryProto_AclEntryTypeProto>>(
                    "type",
                    AclEntryProto::get_field_type_for_reflect,
                    AclEntryProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AclEntryProto_AclEntryScopeProto>>(
                    "scope",
                    AclEntryProto::get_scope_for_reflect,
                    AclEntryProto::mut_scope_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AclEntryProto_FsActionProto>>(
                    "permissions",
                    AclEntryProto::get_permissions_for_reflect,
                    AclEntryProto::mut_permissions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AclEntryProto::get_name_for_reflect,
                    AclEntryProto::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AclEntryProto>(
                    "AclEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AclEntryProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_scope();
        self.clear_permissions();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AclEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AclEntryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_AclEntryScopeProto {
    ACCESS = 0,
    DEFAULT = 1,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_AclEntryScopeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_AclEntryScopeProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_AclEntryScopeProto::ACCESS),
            1 => ::std::option::Option::Some(AclEntryProto_AclEntryScopeProto::DEFAULT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AclEntryProto_AclEntryScopeProto] = &[
            AclEntryProto_AclEntryScopeProto::ACCESS,
            AclEntryProto_AclEntryScopeProto::DEFAULT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AclEntryProto_AclEntryScopeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_AclEntryScopeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_AclEntryScopeProto {
}

impl ::protobuf::reflect::ProtobufValue for AclEntryProto_AclEntryScopeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_AclEntryTypeProto {
    USER = 0,
    GROUP = 1,
    MASK = 2,
    OTHER = 3,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_AclEntryTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_AclEntryTypeProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::USER),
            1 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::GROUP),
            2 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::MASK),
            3 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::OTHER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AclEntryProto_AclEntryTypeProto] = &[
            AclEntryProto_AclEntryTypeProto::USER,
            AclEntryProto_AclEntryTypeProto::GROUP,
            AclEntryProto_AclEntryTypeProto::MASK,
            AclEntryProto_AclEntryTypeProto::OTHER,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AclEntryProto_AclEntryTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_AclEntryTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_AclEntryTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for AclEntryProto_AclEntryTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_FsActionProto {
    NONE = 0,
    EXECUTE = 1,
    WRITE = 2,
    WRITE_EXECUTE = 3,
    READ = 4,
    READ_EXECUTE = 5,
    READ_WRITE = 6,
    PERM_ALL = 7,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_FsActionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_FsActionProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_FsActionProto::NONE),
            1 => ::std::option::Option::Some(AclEntryProto_FsActionProto::EXECUTE),
            2 => ::std::option::Option::Some(AclEntryProto_FsActionProto::WRITE),
            3 => ::std::option::Option::Some(AclEntryProto_FsActionProto::WRITE_EXECUTE),
            4 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ),
            5 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ_EXECUTE),
            6 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ_WRITE),
            7 => ::std::option::Option::Some(AclEntryProto_FsActionProto::PERM_ALL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AclEntryProto_FsActionProto] = &[
            AclEntryProto_FsActionProto::NONE,
            AclEntryProto_FsActionProto::EXECUTE,
            AclEntryProto_FsActionProto::WRITE,
            AclEntryProto_FsActionProto::WRITE_EXECUTE,
            AclEntryProto_FsActionProto::READ,
            AclEntryProto_FsActionProto::READ_EXECUTE,
            AclEntryProto_FsActionProto::READ_WRITE,
            AclEntryProto_FsActionProto::PERM_ALL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AclEntryProto_FsActionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_FsActionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_FsActionProto {
}

impl ::protobuf::reflect::ProtobufValue for AclEntryProto_FsActionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AclStatusProto {
    // message fields
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    sticky: ::std::option::Option<bool>,
    entries: ::protobuf::RepeatedField<AclEntryProto>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AclStatusProto {}

impl AclStatusProto {
    pub fn new() -> AclStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AclStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<AclStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AclStatusProto,
        };
        unsafe {
            instance.get(AclStatusProto::new)
        }
    }

    // required string owner = 1;

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

    // required string group = 2;

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

    // required bool sticky = 3;

    pub fn clear_sticky(&mut self) {
        self.sticky = ::std::option::Option::None;
    }

    pub fn has_sticky(&self) -> bool {
        self.sticky.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sticky(&mut self, v: bool) {
        self.sticky = ::std::option::Option::Some(v);
    }

    pub fn get_sticky(&self) -> bool {
        self.sticky.unwrap_or(false)
    }

    fn get_sticky_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sticky
    }

    fn mut_sticky_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sticky
    }

    // repeated .hadoop.hdfs.AclEntryProto entries = 4;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[AclEntryProto] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<AclEntryProto> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.entries
    }

    // optional .hadoop.hdfs.FsPermissionProto permission = 5;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> FsPermissionProto {
        self.permission.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_permission(&self) -> &FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<FsPermissionProto> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FsPermissionProto> {
        &mut self.permission
    }
}

impl ::protobuf::Message for AclStatusProto {
    fn is_initialized(&self) -> bool {
        if self.owner.is_none() {
            return false;
        }
        if self.group.is_none() {
            return false;
        }
        if self.sticky.is_none() {
            return false;
        }
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.sticky = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
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
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.sticky {
            my_size += 2;
        }
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.sticky {
            os.write_bool(3, v)?;
        }
        for v in &self.entries {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.permission.as_ref() {
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

impl ::protobuf::MessageStatic for AclStatusProto {
    fn new() -> AclStatusProto {
        AclStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AclStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    AclStatusProto::get_owner_for_reflect,
                    AclStatusProto::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    AclStatusProto::get_group_for_reflect,
                    AclStatusProto::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sticky",
                    AclStatusProto::get_sticky_for_reflect,
                    AclStatusProto::mut_sticky_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AclEntryProto>>(
                    "entries",
                    AclStatusProto::get_entries_for_reflect,
                    AclStatusProto::mut_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FsPermissionProto>>(
                    "permission",
                    AclStatusProto::get_permission_for_reflect,
                    AclStatusProto::mut_permission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AclStatusProto>(
                    "AclStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AclStatusProto {
    fn clear(&mut self) {
        self.clear_owner();
        self.clear_group();
        self.clear_sticky();
        self.clear_entries();
        self.clear_permission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AclStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AclStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ModifyAclEntriesRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ModifyAclEntriesRequestProto {}

impl ModifyAclEntriesRequestProto {
    pub fn new() -> ModifyAclEntriesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModifyAclEntriesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ModifyAclEntriesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModifyAclEntriesRequestProto,
        };
        unsafe {
            instance.get(ModifyAclEntriesRequestProto::new)
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec(&self) -> &[AclEntryProto] {
        &self.aclSpec
    }

    fn get_aclSpec_for_reflect(&self) -> &::protobuf::RepeatedField<AclEntryProto> {
        &self.aclSpec
    }

    fn mut_aclSpec_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }
}

impl ::protobuf::Message for ModifyAclEntriesRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.aclSpec {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec)?;
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
        for value in &self.aclSpec {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.aclSpec {
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

impl ::protobuf::MessageStatic for ModifyAclEntriesRequestProto {
    fn new() -> ModifyAclEntriesRequestProto {
        ModifyAclEntriesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModifyAclEntriesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    ModifyAclEntriesRequestProto::get_src_for_reflect,
                    ModifyAclEntriesRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AclEntryProto>>(
                    "aclSpec",
                    ModifyAclEntriesRequestProto::get_aclSpec_for_reflect,
                    ModifyAclEntriesRequestProto::mut_aclSpec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModifyAclEntriesRequestProto>(
                    "ModifyAclEntriesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModifyAclEntriesRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ModifyAclEntriesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModifyAclEntriesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ModifyAclEntriesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ModifyAclEntriesResponseProto {}

impl ModifyAclEntriesResponseProto {
    pub fn new() -> ModifyAclEntriesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModifyAclEntriesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ModifyAclEntriesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModifyAclEntriesResponseProto,
        };
        unsafe {
            instance.get(ModifyAclEntriesResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ModifyAclEntriesResponseProto {
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

impl ::protobuf::MessageStatic for ModifyAclEntriesResponseProto {
    fn new() -> ModifyAclEntriesResponseProto {
        ModifyAclEntriesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModifyAclEntriesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ModifyAclEntriesResponseProto>(
                    "ModifyAclEntriesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModifyAclEntriesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ModifyAclEntriesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModifyAclEntriesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveAclRequestProto {}

impl RemoveAclRequestProto {
    pub fn new() -> RemoveAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclRequestProto,
        };
        unsafe {
            instance.get(RemoveAclRequestProto::new)
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

impl ::protobuf::Message for RemoveAclRequestProto {
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

impl ::protobuf::MessageStatic for RemoveAclRequestProto {
    fn new() -> RemoveAclRequestProto {
        RemoveAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    RemoveAclRequestProto::get_src_for_reflect,
                    RemoveAclRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclRequestProto>(
                    "RemoveAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveAclRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveAclResponseProto {}

impl RemoveAclResponseProto {
    pub fn new() -> RemoveAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclResponseProto,
        };
        unsafe {
            instance.get(RemoveAclResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveAclResponseProto {
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

impl ::protobuf::MessageStatic for RemoveAclResponseProto {
    fn new() -> RemoveAclResponseProto {
        RemoveAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclResponseProto>(
                    "RemoveAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveAclResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveAclEntriesRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveAclEntriesRequestProto {}

impl RemoveAclEntriesRequestProto {
    pub fn new() -> RemoveAclEntriesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclEntriesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclEntriesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclEntriesRequestProto,
        };
        unsafe {
            instance.get(RemoveAclEntriesRequestProto::new)
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec(&self) -> &[AclEntryProto] {
        &self.aclSpec
    }

    fn get_aclSpec_for_reflect(&self) -> &::protobuf::RepeatedField<AclEntryProto> {
        &self.aclSpec
    }

    fn mut_aclSpec_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }
}

impl ::protobuf::Message for RemoveAclEntriesRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.aclSpec {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec)?;
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
        for value in &self.aclSpec {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.aclSpec {
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

impl ::protobuf::MessageStatic for RemoveAclEntriesRequestProto {
    fn new() -> RemoveAclEntriesRequestProto {
        RemoveAclEntriesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclEntriesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    RemoveAclEntriesRequestProto::get_src_for_reflect,
                    RemoveAclEntriesRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AclEntryProto>>(
                    "aclSpec",
                    RemoveAclEntriesRequestProto::get_aclSpec_for_reflect,
                    RemoveAclEntriesRequestProto::mut_aclSpec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclEntriesRequestProto>(
                    "RemoveAclEntriesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclEntriesRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveAclEntriesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveAclEntriesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveAclEntriesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveAclEntriesResponseProto {}

impl RemoveAclEntriesResponseProto {
    pub fn new() -> RemoveAclEntriesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclEntriesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclEntriesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclEntriesResponseProto,
        };
        unsafe {
            instance.get(RemoveAclEntriesResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveAclEntriesResponseProto {
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

impl ::protobuf::MessageStatic for RemoveAclEntriesResponseProto {
    fn new() -> RemoveAclEntriesResponseProto {
        RemoveAclEntriesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclEntriesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclEntriesResponseProto>(
                    "RemoveAclEntriesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclEntriesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveAclEntriesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveAclEntriesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveDefaultAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveDefaultAclRequestProto {}

impl RemoveDefaultAclRequestProto {
    pub fn new() -> RemoveDefaultAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveDefaultAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveDefaultAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveDefaultAclRequestProto,
        };
        unsafe {
            instance.get(RemoveDefaultAclRequestProto::new)
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

impl ::protobuf::Message for RemoveDefaultAclRequestProto {
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

impl ::protobuf::MessageStatic for RemoveDefaultAclRequestProto {
    fn new() -> RemoveDefaultAclRequestProto {
        RemoveDefaultAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveDefaultAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    RemoveDefaultAclRequestProto::get_src_for_reflect,
                    RemoveDefaultAclRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveDefaultAclRequestProto>(
                    "RemoveDefaultAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveDefaultAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveDefaultAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveDefaultAclRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveDefaultAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveDefaultAclResponseProto {}

impl RemoveDefaultAclResponseProto {
    pub fn new() -> RemoveDefaultAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveDefaultAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveDefaultAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveDefaultAclResponseProto,
        };
        unsafe {
            instance.get(RemoveDefaultAclResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveDefaultAclResponseProto {
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

impl ::protobuf::MessageStatic for RemoveDefaultAclResponseProto {
    fn new() -> RemoveDefaultAclResponseProto {
        RemoveDefaultAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveDefaultAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveDefaultAclResponseProto>(
                    "RemoveDefaultAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveDefaultAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveDefaultAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveDefaultAclResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetAclRequestProto {}

impl SetAclRequestProto {
    pub fn new() -> SetAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<SetAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAclRequestProto,
        };
        unsafe {
            instance.get(SetAclRequestProto::new)
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec(&self) -> &[AclEntryProto] {
        &self.aclSpec
    }

    fn get_aclSpec_for_reflect(&self) -> &::protobuf::RepeatedField<AclEntryProto> {
        &self.aclSpec
    }

    fn mut_aclSpec_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }
}

impl ::protobuf::Message for SetAclRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.aclSpec {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec)?;
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
        for value in &self.aclSpec {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.aclSpec {
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

impl ::protobuf::MessageStatic for SetAclRequestProto {
    fn new() -> SetAclRequestProto {
        SetAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    SetAclRequestProto::get_src_for_reflect,
                    SetAclRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AclEntryProto>>(
                    "aclSpec",
                    SetAclRequestProto::get_aclSpec_for_reflect,
                    SetAclRequestProto::mut_aclSpec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetAclRequestProto>(
                    "SetAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetAclRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetAclResponseProto {}

impl SetAclResponseProto {
    pub fn new() -> SetAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<SetAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAclResponseProto,
        };
        unsafe {
            instance.get(SetAclResponseProto::new)
        }
    }
}

impl ::protobuf::Message for SetAclResponseProto {
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

impl ::protobuf::MessageStatic for SetAclResponseProto {
    fn new() -> SetAclResponseProto {
        SetAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetAclResponseProto>(
                    "SetAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetAclResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetAclStatusRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAclStatusRequestProto {}

impl GetAclStatusRequestProto {
    pub fn new() -> GetAclStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAclStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetAclStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAclStatusRequestProto,
        };
        unsafe {
            instance.get(GetAclStatusRequestProto::new)
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

impl ::protobuf::Message for GetAclStatusRequestProto {
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

impl ::protobuf::MessageStatic for GetAclStatusRequestProto {
    fn new() -> GetAclStatusRequestProto {
        GetAclStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAclStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    GetAclStatusRequestProto::get_src_for_reflect,
                    GetAclStatusRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAclStatusRequestProto>(
                    "GetAclStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAclStatusRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetAclStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAclStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetAclStatusResponseProto {
    // message fields
    result: ::protobuf::SingularPtrField<AclStatusProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAclStatusResponseProto {}

impl GetAclStatusResponseProto {
    pub fn new() -> GetAclStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAclStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetAclStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAclStatusResponseProto,
        };
        unsafe {
            instance.get(GetAclStatusResponseProto::new)
        }
    }

    // required .hadoop.hdfs.AclStatusProto result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: AclStatusProto) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut AclStatusProto {
        if self.result.is_none() {
            self.result.set_default();
        }
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> AclStatusProto {
        self.result.take().unwrap_or_else(|| AclStatusProto::new())
    }

    pub fn get_result(&self) -> &AclStatusProto {
        self.result.as_ref().unwrap_or_else(|| AclStatusProto::default_instance())
    }

    fn get_result_for_reflect(&self) -> &::protobuf::SingularPtrField<AclStatusProto> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AclStatusProto> {
        &mut self.result
    }
}

impl ::protobuf::Message for GetAclStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        for v in &self.result {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result)?;
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
        if let Some(ref v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.result.as_ref() {
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

impl ::protobuf::MessageStatic for GetAclStatusResponseProto {
    fn new() -> GetAclStatusResponseProto {
        GetAclStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAclStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AclStatusProto>>(
                    "result",
                    GetAclStatusResponseProto::get_result_for_reflect,
                    GetAclStatusResponseProto::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAclStatusResponseProto>(
                    "GetAclStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAclStatusResponseProto {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetAclStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAclStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tacl.proto\x12\x0bhadoop.hdfs\"'\n\x11FsPermissionProto\x12\x12\n\x04\
    perm\x18\x01\x20\x02(\rR\x04perm\"\xe4\x03\n\rAclEntryProto\x12@\n\x04ty\
    pe\x18\x01\x20\x02(\x0e2,.hadoop.hdfs.AclEntryProto.AclEntryTypeProtoR\
    \x04type\x12C\n\x05scope\x18\x02\x20\x02(\x0e2-.hadoop.hdfs.AclEntryProt\
    o.AclEntryScopeProtoR\x05scope\x12J\n\x0bpermissions\x18\x03\x20\x02(\
    \x0e2(.hadoop.hdfs.AclEntryProto.FsActionProtoR\x0bpermissions\x12\x12\n\
    \x04name\x18\x04\x20\x01(\tR\x04name\"-\n\x12AclEntryScopeProto\x12\n\n\
    \x06ACCESS\x10\0\x12\x0b\n\x07DEFAULT\x10\x01\"=\n\x11AclEntryTypeProto\
    \x12\x08\n\x04USER\x10\0\x12\t\n\x05GROUP\x10\x01\x12\x08\n\x04MASK\x10\
    \x02\x12\t\n\x05OTHER\x10\x03\"~\n\rFsActionProto\x12\x08\n\x04NONE\x10\
    \0\x12\x0b\n\x07EXECUTE\x10\x01\x12\t\n\x05WRITE\x10\x02\x12\x11\n\rWRIT\
    E_EXECUTE\x10\x03\x12\x08\n\x04READ\x10\x04\x12\x10\n\x0cREAD_EXECUTE\
    \x10\x05\x12\x0e\n\nREAD_WRITE\x10\x06\x12\x0c\n\x08PERM_ALL\x10\x07\"\
    \xca\x01\n\x0eAclStatusProto\x12\x14\n\x05owner\x18\x01\x20\x02(\tR\x05o\
    wner\x12\x14\n\x05group\x18\x02\x20\x02(\tR\x05group\x12\x16\n\x06sticky\
    \x18\x03\x20\x02(\x08R\x06sticky\x124\n\x07entries\x18\x04\x20\x03(\x0b2\
    \x1a.hadoop.hdfs.AclEntryProtoR\x07entries\x12>\n\npermission\x18\x05\
    \x20\x01(\x0b2\x1e.hadoop.hdfs.FsPermissionProtoR\npermission\"f\n\x1cMo\
    difyAclEntriesRequestProto\x12\x10\n\x03src\x18\x01\x20\x02(\tR\x03src\
    \x124\n\x07aclSpec\x18\x02\x20\x03(\x0b2\x1a.hadoop.hdfs.AclEntryProtoR\
    \x07aclSpec\"\x1f\n\x1dModifyAclEntriesResponseProto\")\n\x15RemoveAclRe\
    questProto\x12\x10\n\x03src\x18\x01\x20\x02(\tR\x03src\"\x18\n\x16Remove\
    AclResponseProto\"f\n\x1cRemoveAclEntriesRequestProto\x12\x10\n\x03src\
    \x18\x01\x20\x02(\tR\x03src\x124\n\x07aclSpec\x18\x02\x20\x03(\x0b2\x1a.\
    hadoop.hdfs.AclEntryProtoR\x07aclSpec\"\x1f\n\x1dRemoveAclEntriesRespons\
    eProto\"0\n\x1cRemoveDefaultAclRequestProto\x12\x10\n\x03src\x18\x01\x20\
    \x02(\tR\x03src\"\x1f\n\x1dRemoveDefaultAclResponseProto\"\\\n\x12SetAcl\
    RequestProto\x12\x10\n\x03src\x18\x01\x20\x02(\tR\x03src\x124\n\x07aclSp\
    ec\x18\x02\x20\x03(\x0b2\x1a.hadoop.hdfs.AclEntryProtoR\x07aclSpec\"\x15\
    \n\x13SetAclResponseProto\",\n\x18GetAclStatusRequestProto\x12\x10\n\x03\
    src\x18\x01\x20\x02(\tR\x03src\"P\n\x19GetAclStatusResponseProto\x123\n\
    \x06result\x18\x01\x20\x02(\x0b2\x1b.hadoop.hdfs.AclStatusProtoR\x06resu\
    ltB5\n%org.apache.hadoop.hdfs.protocol.protoB\tAclProtos\xa0\x01\x01J\
    \x9e\x1c\n\x06\x12\x04\x12\0p\x01\n\x08\n\x01\x08\x12\x03\x12\0>\n\x91\
    \x06\n\x04\x08\xe7\x07\0\x12\x03\x12\0>2\x83\x06*\n\x20Licensed\x20to\
    \x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\x20under\x20one\n\
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
    \x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x12\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x12\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x12\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x12\x16=\n\x08\n\x01\x08\x12\x03\
    \x13\0*\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x13\0*\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x13\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x13\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x13\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x13\x1e)\n\x08\n\x01\x08\x12\x03\
    \x14\0,\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x14\0,\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x14\x07$\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x14\x07$\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x14\x07$\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\x14'+\n\x08\n\x01\x02\x12\x03\x15\x08\
    \x13\n@\n\x02\x04\0\x12\x04\x1a\0\x1c\x01\x1a4*\n\x20File\x20or\x20Direc\
    tory\x20permision\x20-\x20same\x20spec\x20as\x20posix\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x1a\x08\x19\n2\n\x04\x04\0\x02\0\x12\x03\x1b\x02\x1b\"%\x20\
    Actually\x20a\x20short\x20-\x20only\x2016bits\x20used\n\n\x0c\n\x05\x04\
    \0\x02\0\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1b\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1b\x12\x16\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x1b\x19\x1a\n\n\n\x02\x04\x01\x12\x04\x1e\0:\x01\n\
    \n\n\x03\x04\x01\x01\x12\x03\x1e\x08\x15\n\x0c\n\x04\x04\x01\x04\0\x12\
    \x04\x1f\x02\"\x03\n\x0c\n\x05\x04\x01\x04\0\x01\x12\x03\x1f\x07\x19\n\r\
    \n\x06\x04\x01\x04\0\x02\0\x12\x03\x20\x04\x12\n\x0e\n\x07\x04\x01\x04\0\
    \x02\0\x01\x12\x03\x20\x04\n\n\x0e\n\x07\x04\x01\x04\0\x02\0\x02\x12\x03\
    \x20\x0e\x11\n\r\n\x06\x04\x01\x04\0\x02\x01\x12\x03!\x04\x12\n\x0e\n\
    \x07\x04\x01\x04\0\x02\x01\x01\x12\x03!\x04\x0b\n\x0e\n\x07\x04\x01\x04\
    \0\x02\x01\x02\x12\x03!\x0e\x11\n\x0c\n\x04\x04\x01\x04\x01\x12\x04$\x02\
    )\x03\n\x0c\n\x05\x04\x01\x04\x01\x01\x12\x03$\x07\x18\n\r\n\x06\x04\x01\
    \x04\x01\x02\0\x12\x03%\x04\x10\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x01\
    \x12\x03%\x04\x08\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x02\x12\x03%\x0c\x0f\
    \n\r\n\x06\x04\x01\x04\x01\x02\x01\x12\x03&\x04\x10\n\x0e\n\x07\x04\x01\
    \x04\x01\x02\x01\x01\x12\x03&\x04\t\n\x0e\n\x07\x04\x01\x04\x01\x02\x01\
    \x02\x12\x03&\x0c\x0f\n\r\n\x06\x04\x01\x04\x01\x02\x02\x12\x03'\x04\x10\
    \n\x0e\n\x07\x04\x01\x04\x01\x02\x02\x01\x12\x03'\x04\x08\n\x0e\n\x07\
    \x04\x01\x04\x01\x02\x02\x02\x12\x03'\x0c\x0f\n\r\n\x06\x04\x01\x04\x01\
    \x02\x03\x12\x03(\x04\x10\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\x01\x12\
    \x03(\x04\t\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\x02\x12\x03(\x0c\x0f\n\
    \x0c\n\x04\x04\x01\x04\x02\x12\x04+\x024\x03\n\x0c\n\x05\x04\x01\x04\x02\
    \x01\x12\x03+\x07\x14\n\r\n\x06\x04\x01\x04\x02\x02\0\x12\x03,\x04\x18\n\
    \x0e\n\x07\x04\x01\x04\x02\x02\0\x01\x12\x03,\x04\x08\n\x0e\n\x07\x04\
    \x01\x04\x02\x02\0\x02\x12\x03,\x14\x17\n\r\n\x06\x04\x01\x04\x02\x02\
    \x01\x12\x03-\x04\x18\n\x0e\n\x07\x04\x01\x04\x02\x02\x01\x01\x12\x03-\
    \x04\x0b\n\x0e\n\x07\x04\x01\x04\x02\x02\x01\x02\x12\x03-\x14\x17\n\r\n\
    \x06\x04\x01\x04\x02\x02\x02\x12\x03.\x04\x18\n\x0e\n\x07\x04\x01\x04\
    \x02\x02\x02\x01\x12\x03.\x04\t\n\x0e\n\x07\x04\x01\x04\x02\x02\x02\x02\
    \x12\x03.\x14\x17\n\r\n\x06\x04\x01\x04\x02\x02\x03\x12\x03/\x04\x18\n\
    \x0e\n\x07\x04\x01\x04\x02\x02\x03\x01\x12\x03/\x04\x11\n\x0e\n\x07\x04\
    \x01\x04\x02\x02\x03\x02\x12\x03/\x14\x17\n\r\n\x06\x04\x01\x04\x02\x02\
    \x04\x12\x030\x04\x18\n\x0e\n\x07\x04\x01\x04\x02\x02\x04\x01\x12\x030\
    \x04\x08\n\x0e\n\x07\x04\x01\x04\x02\x02\x04\x02\x12\x030\x14\x17\n\r\n\
    \x06\x04\x01\x04\x02\x02\x05\x12\x031\x04\x18\n\x0e\n\x07\x04\x01\x04\
    \x02\x02\x05\x01\x12\x031\x04\x10\n\x0e\n\x07\x04\x01\x04\x02\x02\x05\
    \x02\x12\x031\x14\x17\n\r\n\x06\x04\x01\x04\x02\x02\x06\x12\x032\x04\x18\
    \n\x0e\n\x07\x04\x01\x04\x02\x02\x06\x01\x12\x032\x04\x0e\n\x0e\n\x07\
    \x04\x01\x04\x02\x02\x06\x02\x12\x032\x14\x17\n\r\n\x06\x04\x01\x04\x02\
    \x02\x07\x12\x033\x04\x18\n\x0e\n\x07\x04\x01\x04\x02\x02\x07\x01\x12\
    \x033\x04\x0c\n\x0e\n\x07\x04\x01\x04\x02\x02\x07\x02\x12\x033\x14\x17\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x036\x02)\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x036\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x036\x0b\x1c\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x036\x1d!\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x036'\
    (\n\x0b\n\x04\x04\x01\x02\x01\x12\x037\x02)\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x037\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x037\x0b\x1d\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x037\x1e#\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x037'(\n\x0b\n\x04\x04\x01\x02\x02\x12\x038\x02)\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\
    \x038\x0b\x18\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x038\x19$\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x038'(\n\x0b\n\x04\x04\x01\x02\x03\x12\x039\x02\
    )\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x039\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x03\x05\x12\x039\x0b\x11\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x039\x12\
    \x16\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x039'(\n\n\n\x02\x04\x02\x12\
    \x04<\0B\x01\n\n\n\x03\x04\x02\x01\x12\x03<\x08\x16\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03=\x02%\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03=\x02\n\n\x0c\
    \n\x05\x04\x02\x02\0\x05\x12\x03=\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03=\x12\x17\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03=#$\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03>\x02%\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03>\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03>\x0b\x11\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03>\x12\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03>#$\n\x0b\n\x04\x04\x02\x02\x02\x12\x03?\x02%\n\x0c\n\x05\x04\x02\
    \x02\x02\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03?\x0b\
    \x0f\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03?\x10\x16\n\x0c\n\x05\x04\
    \x02\x02\x02\x03\x12\x03?#$\n\x0b\n\x04\x04\x02\x02\x03\x12\x03@\x02%\n\
    \x0c\n\x05\x04\x02\x02\x03\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x03\x06\x12\x03@\x0b\x18\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03@\x19\
    \x20\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03@#$\n\x0b\n\x04\x04\x02\x02\
    \x04\x12\x03A\x02,\n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03A\x02\n\n\x0c\
    \n\x05\x04\x02\x02\x04\x06\x12\x03A\x0b\x1c\n\x0c\n\x05\x04\x02\x02\x04\
    \x01\x12\x03A\x1d'\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03A*+\n\n\n\x02\
    \x04\x03\x12\x04D\0G\x01\n\n\n\x03\x04\x03\x01\x12\x03D\x08$\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03E\x02%\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03E\x02\
    \n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03E\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\0\x01\x12\x03E\x12\x15\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03E#$\n\
    \x0b\n\x04\x04\x03\x02\x01\x12\x03F\x02%\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x03F\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03F\x0b\x18\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03F\x19\x20\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03F#$\n\n\n\x02\x04\x04\x12\x04I\0J\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03I\x08%\n\n\n\x02\x04\x05\x12\x04L\0N\x01\n\n\n\x03\x04\x05\x01\
    \x12\x03L\x08\x1d\n\x0b\n\x04\x04\x05\x02\0\x12\x03M\x02\x1a\n\x0c\n\x05\
    \x04\x05\x02\0\x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03M\
    \x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03M\x12\x15\n\x0c\n\x05\x04\
    \x05\x02\0\x03\x12\x03M\x18\x19\n\n\n\x02\x04\x06\x12\x04P\0Q\x01\n\n\n\
    \x03\x04\x06\x01\x12\x03P\x08\x1e\n\n\n\x02\x04\x07\x12\x04S\0V\x01\n\n\
    \n\x03\x04\x07\x01\x12\x03S\x08$\n\x0b\n\x04\x04\x07\x02\0\x12\x03T\x02%\
    \n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03T\x02\n\n\x0c\n\x05\x04\x07\x02\0\
    \x05\x12\x03T\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03T\x12\x15\n\
    \x0c\n\x05\x04\x07\x02\0\x03\x12\x03T#$\n\x0b\n\x04\x04\x07\x02\x01\x12\
    \x03U\x02%\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03U\x02\n\n\x0c\n\x05\
    \x04\x07\x02\x01\x06\x12\x03U\x0b\x18\n\x0c\n\x05\x04\x07\x02\x01\x01\
    \x12\x03U\x19\x20\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03U#$\n\n\n\x02\
    \x04\x08\x12\x04X\0Y\x01\n\n\n\x03\x04\x08\x01\x12\x03X\x08%\n\n\n\x02\
    \x04\t\x12\x04[\0]\x01\n\n\n\x03\x04\t\x01\x12\x03[\x08$\n\x0b\n\x04\x04\
    \t\x02\0\x12\x03\\\x02\x1a\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03\\\x02\n\n\
    \x0c\n\x05\x04\t\x02\0\x05\x12\x03\\\x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\
    \x12\x03\\\x12\x15\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03\\\x18\x19\n\n\n\
    \x02\x04\n\x12\x04_\0`\x01\n\n\n\x03\x04\n\x01\x12\x03_\x08%\n\n\n\x02\
    \x04\x0b\x12\x04b\0e\x01\n\n\n\x03\x04\x0b\x01\x12\x03b\x08\x1a\n\x0b\n\
    \x04\x04\x0b\x02\0\x12\x03c\x02%\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03c\
    \x02\n\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03c\x0b\x11\n\x0c\n\x05\x04\
    \x0b\x02\0\x01\x12\x03c\x12\x15\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03c#$\
    \n\x0b\n\x04\x04\x0b\x02\x01\x12\x03d\x02%\n\x0c\n\x05\x04\x0b\x02\x01\
    \x04\x12\x03d\x02\n\n\x0c\n\x05\x04\x0b\x02\x01\x06\x12\x03d\x0b\x18\n\
    \x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03d\x19\x20\n\x0c\n\x05\x04\x0b\x02\
    \x01\x03\x12\x03d#$\n\n\n\x02\x04\x0c\x12\x04g\0h\x01\n\n\n\x03\x04\x0c\
    \x01\x12\x03g\x08\x1b\n\n\n\x02\x04\r\x12\x04j\0l\x01\n\n\n\x03\x04\r\
    \x01\x12\x03j\x08\x20\n\x0b\n\x04\x04\r\x02\0\x12\x03k\x02\x1a\n\x0c\n\
    \x05\x04\r\x02\0\x04\x12\x03k\x02\n\n\x0c\n\x05\x04\r\x02\0\x05\x12\x03k\
    \x0b\x11\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03k\x12\x15\n\x0c\n\x05\x04\r\
    \x02\0\x03\x12\x03k\x18\x19\n\n\n\x02\x04\x0e\x12\x04n\0p\x01\n\n\n\x03\
    \x04\x0e\x01\x12\x03n\x08!\n\x0b\n\x04\x04\x0e\x02\0\x12\x03o\x02%\n\x0c\
    \n\x05\x04\x0e\x02\0\x04\x12\x03o\x02\n\n\x0c\n\x05\x04\x0e\x02\0\x06\
    \x12\x03o\x0b\x19\n\x0c\n\x05\x04\x0e\x02\0\x01\x12\x03o\x1a\x20\n\x0c\n\
    \x05\x04\x0e\x02\0\x03\x12\x03o#$\
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
