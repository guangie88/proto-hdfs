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
pub struct ListSpanReceiversRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListSpanReceiversRequestProto {}

impl ListSpanReceiversRequestProto {
    pub fn new() -> ListSpanReceiversRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListSpanReceiversRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListSpanReceiversRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListSpanReceiversRequestProto,
        };
        unsafe {
            instance.get(ListSpanReceiversRequestProto::new)
        }
    }
}

impl ::protobuf::Message for ListSpanReceiversRequestProto {
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

impl ::protobuf::MessageStatic for ListSpanReceiversRequestProto {
    fn new() -> ListSpanReceiversRequestProto {
        ListSpanReceiversRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListSpanReceiversRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListSpanReceiversRequestProto>(
                    "ListSpanReceiversRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListSpanReceiversRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListSpanReceiversRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListSpanReceiversRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SpanReceiverListInfo {
    // message fields
    id: ::std::option::Option<i64>,
    className: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpanReceiverListInfo {}

impl SpanReceiverListInfo {
    pub fn new() -> SpanReceiverListInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpanReceiverListInfo {
        static mut instance: ::protobuf::lazy::Lazy<SpanReceiverListInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpanReceiverListInfo,
        };
        unsafe {
            instance.get(SpanReceiverListInfo::new)
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

    // required string className = 2;

    pub fn clear_className(&mut self) {
        self.className.clear();
    }

    pub fn has_className(&self) -> bool {
        self.className.is_some()
    }

    // Param is passed by value, moved
    pub fn set_className(&mut self, v: ::std::string::String) {
        self.className = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_className(&mut self) -> &mut ::std::string::String {
        if self.className.is_none() {
            self.className.set_default();
        }
        self.className.as_mut().unwrap()
    }

    // Take field
    pub fn take_className(&mut self) -> ::std::string::String {
        self.className.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_className(&self) -> &str {
        match self.className.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_className_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.className
    }

    fn mut_className_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.className
    }
}

impl ::protobuf::Message for SpanReceiverListInfo {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.className.is_none() {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.className)?;
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
        if let Some(ref v) = self.className.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.className.as_ref() {
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

impl ::protobuf::MessageStatic for SpanReceiverListInfo {
    fn new() -> SpanReceiverListInfo {
        SpanReceiverListInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpanReceiverListInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    SpanReceiverListInfo::get_id_for_reflect,
                    SpanReceiverListInfo::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "className",
                    SpanReceiverListInfo::get_className_for_reflect,
                    SpanReceiverListInfo::mut_className_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpanReceiverListInfo>(
                    "SpanReceiverListInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpanReceiverListInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_className();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SpanReceiverListInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpanReceiverListInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListSpanReceiversResponseProto {
    // message fields
    descriptions: ::protobuf::RepeatedField<SpanReceiverListInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListSpanReceiversResponseProto {}

impl ListSpanReceiversResponseProto {
    pub fn new() -> ListSpanReceiversResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListSpanReceiversResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListSpanReceiversResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListSpanReceiversResponseProto,
        };
        unsafe {
            instance.get(ListSpanReceiversResponseProto::new)
        }
    }

    // repeated .hadoop.common.SpanReceiverListInfo descriptions = 1;

    pub fn clear_descriptions(&mut self) {
        self.descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptions(&mut self, v: ::protobuf::RepeatedField<SpanReceiverListInfo>) {
        self.descriptions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptions(&mut self) -> &mut ::protobuf::RepeatedField<SpanReceiverListInfo> {
        &mut self.descriptions
    }

    // Take field
    pub fn take_descriptions(&mut self) -> ::protobuf::RepeatedField<SpanReceiverListInfo> {
        ::std::mem::replace(&mut self.descriptions, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptions(&self) -> &[SpanReceiverListInfo] {
        &self.descriptions
    }

    fn get_descriptions_for_reflect(&self) -> &::protobuf::RepeatedField<SpanReceiverListInfo> {
        &self.descriptions
    }

    fn mut_descriptions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SpanReceiverListInfo> {
        &mut self.descriptions
    }
}

impl ::protobuf::Message for ListSpanReceiversResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.descriptions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptions)?;
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
        for value in &self.descriptions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.descriptions {
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

impl ::protobuf::MessageStatic for ListSpanReceiversResponseProto {
    fn new() -> ListSpanReceiversResponseProto {
        ListSpanReceiversResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListSpanReceiversResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SpanReceiverListInfo>>(
                    "descriptions",
                    ListSpanReceiversResponseProto::get_descriptions_for_reflect,
                    ListSpanReceiversResponseProto::mut_descriptions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListSpanReceiversResponseProto>(
                    "ListSpanReceiversResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListSpanReceiversResponseProto {
    fn clear(&mut self) {
        self.clear_descriptions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListSpanReceiversResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListSpanReceiversResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigPair {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigPair {}

impl ConfigPair {
    pub fn new() -> ConfigPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigPair {
        static mut instance: ::protobuf::lazy::Lazy<ConfigPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigPair,
        };
        unsafe {
            instance.get(ConfigPair::new)
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

impl ::protobuf::Message for ConfigPair {
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

impl ::protobuf::MessageStatic for ConfigPair {
    fn new() -> ConfigPair {
        ConfigPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    ConfigPair::get_key_for_reflect,
                    ConfigPair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    ConfigPair::get_value_for_reflect,
                    ConfigPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigPair>(
                    "ConfigPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigPair {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddSpanReceiverRequestProto {
    // message fields
    className: ::protobuf::SingularField<::std::string::String>,
    config: ::protobuf::RepeatedField<ConfigPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddSpanReceiverRequestProto {}

impl AddSpanReceiverRequestProto {
    pub fn new() -> AddSpanReceiverRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddSpanReceiverRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<AddSpanReceiverRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddSpanReceiverRequestProto,
        };
        unsafe {
            instance.get(AddSpanReceiverRequestProto::new)
        }
    }

    // required string className = 1;

    pub fn clear_className(&mut self) {
        self.className.clear();
    }

    pub fn has_className(&self) -> bool {
        self.className.is_some()
    }

    // Param is passed by value, moved
    pub fn set_className(&mut self, v: ::std::string::String) {
        self.className = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_className(&mut self) -> &mut ::std::string::String {
        if self.className.is_none() {
            self.className.set_default();
        }
        self.className.as_mut().unwrap()
    }

    // Take field
    pub fn take_className(&mut self) -> ::std::string::String {
        self.className.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_className(&self) -> &str {
        match self.className.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_className_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.className
    }

    fn mut_className_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.className
    }

    // repeated .hadoop.common.ConfigPair config = 2;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::protobuf::RepeatedField<ConfigPair>) {
        self.config = v;
    }

    // Mutable pointer to the field.
    pub fn mut_config(&mut self) -> &mut ::protobuf::RepeatedField<ConfigPair> {
        &mut self.config
    }

    // Take field
    pub fn take_config(&mut self) -> ::protobuf::RepeatedField<ConfigPair> {
        ::std::mem::replace(&mut self.config, ::protobuf::RepeatedField::new())
    }

    pub fn get_config(&self) -> &[ConfigPair] {
        &self.config
    }

    fn get_config_for_reflect(&self) -> &::protobuf::RepeatedField<ConfigPair> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ConfigPair> {
        &mut self.config
    }
}

impl ::protobuf::Message for AddSpanReceiverRequestProto {
    fn is_initialized(&self) -> bool {
        if self.className.is_none() {
            return false;
        }
        for v in &self.config {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.className)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.config)?;
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
        if let Some(ref v) = self.className.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.className.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.config {
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

impl ::protobuf::MessageStatic for AddSpanReceiverRequestProto {
    fn new() -> AddSpanReceiverRequestProto {
        AddSpanReceiverRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddSpanReceiverRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "className",
                    AddSpanReceiverRequestProto::get_className_for_reflect,
                    AddSpanReceiverRequestProto::mut_className_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConfigPair>>(
                    "config",
                    AddSpanReceiverRequestProto::get_config_for_reflect,
                    AddSpanReceiverRequestProto::mut_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddSpanReceiverRequestProto>(
                    "AddSpanReceiverRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddSpanReceiverRequestProto {
    fn clear(&mut self) {
        self.clear_className();
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddSpanReceiverRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddSpanReceiverRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddSpanReceiverResponseProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddSpanReceiverResponseProto {}

impl AddSpanReceiverResponseProto {
    pub fn new() -> AddSpanReceiverResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddSpanReceiverResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<AddSpanReceiverResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddSpanReceiverResponseProto,
        };
        unsafe {
            instance.get(AddSpanReceiverResponseProto::new)
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

impl ::protobuf::Message for AddSpanReceiverResponseProto {
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

impl ::protobuf::MessageStatic for AddSpanReceiverResponseProto {
    fn new() -> AddSpanReceiverResponseProto {
        AddSpanReceiverResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddSpanReceiverResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    AddSpanReceiverResponseProto::get_id_for_reflect,
                    AddSpanReceiverResponseProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddSpanReceiverResponseProto>(
                    "AddSpanReceiverResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddSpanReceiverResponseProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddSpanReceiverResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddSpanReceiverResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveSpanReceiverRequestProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveSpanReceiverRequestProto {}

impl RemoveSpanReceiverRequestProto {
    pub fn new() -> RemoveSpanReceiverRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveSpanReceiverRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveSpanReceiverRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveSpanReceiverRequestProto,
        };
        unsafe {
            instance.get(RemoveSpanReceiverRequestProto::new)
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

impl ::protobuf::Message for RemoveSpanReceiverRequestProto {
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

impl ::protobuf::MessageStatic for RemoveSpanReceiverRequestProto {
    fn new() -> RemoveSpanReceiverRequestProto {
        RemoveSpanReceiverRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveSpanReceiverRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    RemoveSpanReceiverRequestProto::get_id_for_reflect,
                    RemoveSpanReceiverRequestProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveSpanReceiverRequestProto>(
                    "RemoveSpanReceiverRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveSpanReceiverRequestProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveSpanReceiverRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveSpanReceiverRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveSpanReceiverResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveSpanReceiverResponseProto {}

impl RemoveSpanReceiverResponseProto {
    pub fn new() -> RemoveSpanReceiverResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveSpanReceiverResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveSpanReceiverResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveSpanReceiverResponseProto,
        };
        unsafe {
            instance.get(RemoveSpanReceiverResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveSpanReceiverResponseProto {
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

impl ::protobuf::MessageStatic for RemoveSpanReceiverResponseProto {
    fn new() -> RemoveSpanReceiverResponseProto {
        RemoveSpanReceiverResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveSpanReceiverResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveSpanReceiverResponseProto>(
                    "RemoveSpanReceiverResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveSpanReceiverResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveSpanReceiverResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveSpanReceiverResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10TraceAdmin.proto\x12\rhadoop.common\"\x1f\n\x1dListSpanReceiversRe\
    questProto\"D\n\x14SpanReceiverListInfo\x12\x0e\n\x02id\x18\x01\x20\x02(\
    \x03R\x02id\x12\x1c\n\tclassName\x18\x02\x20\x02(\tR\tclassName\"i\n\x1e\
    ListSpanReceiversResponseProto\x12G\n\x0cdescriptions\x18\x01\x20\x03(\
    \x0b2#.hadoop.common.SpanReceiverListInfoR\x0cdescriptions\"4\n\nConfigP\
    air\x12\x10\n\x03key\x18\x01\x20\x02(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x02(\tR\x05value\"n\n\x1bAddSpanReceiverRequestProto\x12\x1c\n\
    \tclassName\x18\x01\x20\x02(\tR\tclassName\x121\n\x06config\x18\x02\x20\
    \x03(\x0b2\x19.hadoop.common.ConfigPairR\x06config\".\n\x1cAddSpanReceiv\
    erResponseProto\x12\x0e\n\x02id\x18\x01\x20\x02(\x03R\x02id\"0\n\x1eRemo\
    veSpanReceiverRequestProto\x12\x0e\n\x02id\x18\x01\x20\x02(\x03R\x02id\"\
    !\n\x1fRemoveSpanReceiverResponseProto2\xe6\x02\n\x11TraceAdminService\
    \x12p\n\x11listSpanReceivers\x12,.hadoop.common.ListSpanReceiversRequest\
    Proto\x1a-.hadoop.common.ListSpanReceiversResponseProto\x12j\n\x0faddSpa\
    nReceiver\x12*.hadoop.common.AddSpanReceiverRequestProto\x1a+.hadoop.com\
    mon.AddSpanReceiverResponseProto\x12s\n\x12removeSpanReceiver\x12-.hadoo\
    p.common.RemoveSpanReceiverRequestProto\x1a..hadoop.common.RemoveSpanRec\
    eiverResponseProtoB/\n\x19org.apache.hadoop.tracingB\x0cTraceAdminPB\xa0\
    \x01\x01\x88\x01\x01J\xfb\x11\n\x06\x12\x04\x18\0H\x01\n\x08\n\x01\x08\
    \x12\x03\x18\02\n\xbe\x07\n\x04\x08\xe7\x07\0\x12\x03\x18\022\x83\x06*\n\
    \x20Licensed\x20to\x20the\x20Apache\x20Software\x20Foundation\x20(ASF)\
    \x20under\x20one\n\x20or\x20more\x20contributor\x20license\x20agreements\
    .\x20\x20See\x20the\x20NOTICE\x20file\n\x20distributed\x20with\x20this\
    \x20work\x20for\x20additional\x20information\n\x20regarding\x20copyright\
    \x20ownership.\x20\x20The\x20ASF\x20licenses\x20this\x20file\n\x20to\x20\
    you\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\n\
    \x20\"License\");\x20you\x20may\x20not\x20use\x20this\x20file\x20except\
    \x20in\x20compliance\n\x20with\x20the\x20License.\x20\x20You\x20may\x20o\
    btain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20\
    http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20\
    by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20softw\
    are\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\x20\
    on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20C\
    ONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\
    \x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20gover\
    ning\x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\
    \n2\xaa\x01*\n\x20These\x20.proto\x20interfaces\x20are\x20private\x20and\
    \x20stable.\n\x20Please\x20see\x20http://wiki.apache.org/hadoop/Compatib\
    ility\n\x20for\x20what\x20changes\x20are\x20allowed\x20for\x20a\x20*stab\
    le*\x20.proto\x20interface.\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x18\
    \x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\
    \x07\x12\x03\x18\x161\n\x08\n\x01\x08\x12\x03\x19\0-\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\x19\0-\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x19\x07\
    \x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\
    \xe7\x07\x01\x02\0\x01\x12\x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x07\x12\x03\x19\x1e,\n\x08\n\x01\x08\x12\x03\x1a\0$\n\x0b\n\x04\x08\xe7\
    \x07\x02\x12\x03\x1a\0$\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x1a\x07\
    \x1c\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x1a\x07\x1c\n\x0e\n\x07\x08\
    \xe7\x07\x02\x02\0\x01\x12\x03\x1a\x07\x1c\n\x0c\n\x05\x08\xe7\x07\x02\
    \x03\x12\x03\x1a\x1f#\n\x08\n\x01\x08\x12\x03\x1b\0,\n\x0b\n\x04\x08\xe7\
    \x07\x03\x12\x03\x1b\0,\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x1b\x07$\
    \n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x1b\x07$\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\x1b\x07$\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\
    \x03\x1b'+\n\x08\n\x01\x02\x12\x03\x1c\x08\x15\n\n\n\x02\x04\0\x12\x04\
    \x1e\0\x1f\x01\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08%\n\n\n\x02\x04\x01\
    \x12\x04!\0$\x01\n\n\n\x03\x04\x01\x01\x12\x03!\x08\x1c\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03\"\x02\x18\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\"\x02\
    \n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\"\x0b\x10\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\"\x11\x13\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\"\x16\
    \x17\n\x0b\n\x04\x04\x01\x02\x01\x12\x03#\x02\x20\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03#\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03#\x12\x1b\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03#\x1e\x1f\n\n\n\x02\x04\x02\x12\x04&\0(\x01\n\n\
    \n\x03\x04\x02\x01\x12\x03&\x08&\n\x0b\n\x04\x04\x02\x02\0\x12\x03'\x021\
    \n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03'\x0b\x1f\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03'\x20,\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03'/0\n\n\n\x02\x04\x03\x12\x04*\0-\x01\n\
    \n\n\x03\x04\x03\x01\x12\x03*\x08\x12\n\x0b\n\x04\x04\x03\x02\0\x12\x03+\
    \x02\x1a\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\
    \x03\x02\0\x05\x12\x03+\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03+\
    \x12\x15\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03+\x18\x19\n\x0b\n\x04\x04\
    \x03\x02\x01\x12\x03,\x02\x1c\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03,\
    \x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03,\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\x01\x01\x12\x03,\x12\x17\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\
    \x03,\x1a\x1b\n\n\n\x02\x04\x04\x12\x04/\02\x01\n\n\n\x03\x04\x04\x01\
    \x12\x03/\x08#\n\x0b\n\x04\x04\x04\x02\0\x12\x030\x02\x20\n\x0c\n\x05\
    \x04\x04\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x030\
    \x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x030\x12\x1b\n\x0c\n\x05\x04\
    \x04\x02\0\x03\x12\x030\x1e\x1f\n\x0b\n\x04\x04\x04\x02\x01\x12\x031\x02\
    !\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x031\x02\n\n\x0c\n\x05\x04\x04\x02\
    \x01\x06\x12\x031\x0b\x15\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x031\x16\
    \x1c\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x031\x1f\x20\n\n\n\x02\x04\x05\
    \x12\x044\06\x01\n\n\n\x03\x04\x05\x01\x12\x034\x08$\n\x0b\n\x04\x04\x05\
    \x02\0\x12\x035\x02\x18\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x035\x02\n\n\
    \x0c\n\x05\x04\x05\x02\0\x05\x12\x035\x0b\x10\n\x0c\n\x05\x04\x05\x02\0\
    \x01\x12\x035\x11\x13\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x035\x16\x17\n\n\
    \n\x02\x04\x06\x12\x048\0:\x01\n\n\n\x03\x04\x06\x01\x12\x038\x08&\n\x0b\
    \n\x04\x04\x06\x02\0\x12\x039\x02\x18\n\x0c\n\x05\x04\x06\x02\0\x04\x12\
    \x039\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x039\x0b\x10\n\x0c\n\x05\
    \x04\x06\x02\0\x01\x12\x039\x11\x13\n\x0c\n\x05\x04\x06\x02\0\x03\x12\
    \x039\x16\x17\n\n\n\x02\x04\x07\x12\x04<\0=\x01\n\n\n\x03\x04\x07\x01\
    \x12\x03<\x08'\n\n\n\x02\x06\0\x12\x04?\0H\x01\n\n\n\x03\x06\0\x01\x12\
    \x03?\x08\x19\n\x0c\n\x04\x06\0\x02\0\x12\x04@\x02A.\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03@\x06\x17\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03@\x185\n\
    \x0c\n\x05\x06\0\x02\0\x03\x12\x03A\x0e,\n\x0c\n\x04\x06\0\x02\x01\x12\
    \x04C\x02D,\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03C\x06\x15\n\x0c\n\x05\
    \x06\0\x02\x01\x02\x12\x03C\x161\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03D\
    \x0e*\n\x0c\n\x04\x06\0\x02\x02\x12\x04F\x02G/\n\x0c\n\x05\x06\0\x02\x02\
    \x01\x12\x03F\x06\x18\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03F\x197\n\x0c\
    \n\x05\x06\0\x02\x02\x03\x12\x03G\x0e-\
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
