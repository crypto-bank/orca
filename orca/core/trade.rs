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
pub struct Trade {
    // message fields
    pub id: i64,
    pub order: ::protobuf::SingularPtrField<super::order::Order>,
    pub timestamp: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Trade {}

impl Trade {
    pub fn new() -> Trade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Trade {
        static mut instance: ::protobuf::lazy::Lazy<Trade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Trade,
        };
        unsafe {
            instance.get(Trade::new)
        }
    }

    // int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = v;
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.id
    }

    // .orca.core.Order order = 2;

    pub fn clear_order(&mut self) {
        self.order.clear();
    }

    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: super::order::Order) {
        self.order = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_order(&mut self) -> &mut super::order::Order {
        if self.order.is_none() {
            self.order.set_default();
        }
        self.order.as_mut().unwrap()
    }

    // Take field
    pub fn take_order(&mut self) -> super::order::Order {
        self.order.take().unwrap_or_else(|| super::order::Order::new())
    }

    pub fn get_order(&self) -> &super::order::Order {
        self.order.as_ref().unwrap_or_else(|| super::order::Order::default_instance())
    }

    fn get_order_for_reflect(&self) -> &::protobuf::SingularPtrField<super::order::Order> {
        &self.order
    }

    fn mut_order_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::order::Order> {
        &mut self.order
    }

    // int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &i64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for Trade {
    fn is_initialized(&self) -> bool {
        for v in &self.order {
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
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.order)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.order.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int64(1, self.id)?;
        }
        if let Some(ref v) = self.order.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.timestamp != 0 {
            os.write_int64(3, self.timestamp)?;
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

impl ::protobuf::MessageStatic for Trade {
    fn new() -> Trade {
        Trade::new()
    }

    fn descriptor_static(_: ::std::option::Option<Trade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    Trade::get_id_for_reflect,
                    Trade::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::order::Order>>(
                    "order",
                    Trade::get_order_for_reflect,
                    Trade::mut_order_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    Trade::get_timestamp_for_reflect,
                    Trade::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Trade>(
                    "Trade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Trade {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_order();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Trade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Trade {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawTrade {
    // message fields
    pub id: i64,
    pub order: ::protobuf::SingularPtrField<super::order::RawOrder>,
    pub timestamp: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawTrade {}

impl RawTrade {
    pub fn new() -> RawTrade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawTrade {
        static mut instance: ::protobuf::lazy::Lazy<RawTrade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawTrade,
        };
        unsafe {
            instance.get(RawTrade::new)
        }
    }

    // int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = v;
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.id
    }

    // .orca.core.RawOrder order = 2;

    pub fn clear_order(&mut self) {
        self.order.clear();
    }

    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: super::order::RawOrder) {
        self.order = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_order(&mut self) -> &mut super::order::RawOrder {
        if self.order.is_none() {
            self.order.set_default();
        }
        self.order.as_mut().unwrap()
    }

    // Take field
    pub fn take_order(&mut self) -> super::order::RawOrder {
        self.order.take().unwrap_or_else(|| super::order::RawOrder::new())
    }

    pub fn get_order(&self) -> &super::order::RawOrder {
        self.order.as_ref().unwrap_or_else(|| super::order::RawOrder::default_instance())
    }

    fn get_order_for_reflect(&self) -> &::protobuf::SingularPtrField<super::order::RawOrder> {
        &self.order
    }

    fn mut_order_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::order::RawOrder> {
        &mut self.order
    }

    // int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &i64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for RawTrade {
    fn is_initialized(&self) -> bool {
        for v in &self.order {
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
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.order)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.order.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int64(1, self.id)?;
        }
        if let Some(ref v) = self.order.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.timestamp != 0 {
            os.write_int64(3, self.timestamp)?;
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

impl ::protobuf::MessageStatic for RawTrade {
    fn new() -> RawTrade {
        RawTrade::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawTrade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    RawTrade::get_id_for_reflect,
                    RawTrade::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::order::RawOrder>>(
                    "order",
                    RawTrade::get_order_for_reflect,
                    RawTrade::mut_order_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    RawTrade::get_timestamp_for_reflect,
                    RawTrade::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawTrade>(
                    "RawTrade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawTrade {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_order();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawTrade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawTrade {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15orca/core/trade.proto\x12\torca.core\x1a\x15orca/core/order.proto\
    \"]\n\x05Trade\x12\x0e\n\x02id\x18\x01\x20\x01(\x03R\x02id\x12&\n\x05ord\
    er\x18\x02\x20\x01(\x0b2\x10.orca.core.OrderR\x05order\x12\x1c\n\ttimest\
    amp\x18\x03\x20\x01(\x03R\ttimestamp\"c\n\x08RawTrade\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\x03R\x02id\x12)\n\x05order\x18\x02\x20\x01(\x0b2\x13.o\
    rca.core.RawOrderR\x05order\x12\x1c\n\ttimestamp\x18\x03\x20\x01(\x03R\t\
    timestampb\x06proto3\
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
