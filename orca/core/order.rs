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
pub struct Order {
    // message fields
    pub kind: OrderKind,
    pub rate: ::protobuf::SingularPtrField<super::currency::CurrencyVolume>,
    pub volume: ::protobuf::SingularPtrField<super::currency::CurrencyVolume>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Order {}

impl Order {
    pub fn new() -> Order {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Order {
        static mut instance: ::protobuf::lazy::Lazy<Order> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Order,
        };
        unsafe {
            instance.get(Order::new)
        }
    }

    // .orca.OrderKind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = OrderKind::ASK;
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: OrderKind) {
        self.kind = v;
    }

    pub fn get_kind(&self) -> OrderKind {
        self.kind
    }

    fn get_kind_for_reflect(&self) -> &OrderKind {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut OrderKind {
        &mut self.kind
    }

    // .orca.CurrencyVolume rate = 2;

    pub fn clear_rate(&mut self) {
        self.rate.clear();
    }

    pub fn has_rate(&self) -> bool {
        self.rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rate(&mut self, v: super::currency::CurrencyVolume) {
        self.rate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rate(&mut self) -> &mut super::currency::CurrencyVolume {
        if self.rate.is_none() {
            self.rate.set_default();
        }
        self.rate.as_mut().unwrap()
    }

    // Take field
    pub fn take_rate(&mut self) -> super::currency::CurrencyVolume {
        self.rate.take().unwrap_or_else(|| super::currency::CurrencyVolume::new())
    }

    pub fn get_rate(&self) -> &super::currency::CurrencyVolume {
        self.rate.as_ref().unwrap_or_else(|| super::currency::CurrencyVolume::default_instance())
    }

    fn get_rate_for_reflect(&self) -> &::protobuf::SingularPtrField<super::currency::CurrencyVolume> {
        &self.rate
    }

    fn mut_rate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::currency::CurrencyVolume> {
        &mut self.rate
    }

    // .orca.CurrencyVolume volume = 3;

    pub fn clear_volume(&mut self) {
        self.volume.clear();
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: super::currency::CurrencyVolume) {
        self.volume = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_volume(&mut self) -> &mut super::currency::CurrencyVolume {
        if self.volume.is_none() {
            self.volume.set_default();
        }
        self.volume.as_mut().unwrap()
    }

    // Take field
    pub fn take_volume(&mut self) -> super::currency::CurrencyVolume {
        self.volume.take().unwrap_or_else(|| super::currency::CurrencyVolume::new())
    }

    pub fn get_volume(&self) -> &super::currency::CurrencyVolume {
        self.volume.as_ref().unwrap_or_else(|| super::currency::CurrencyVolume::default_instance())
    }

    fn get_volume_for_reflect(&self) -> &::protobuf::SingularPtrField<super::currency::CurrencyVolume> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::currency::CurrencyVolume> {
        &mut self.volume
    }
}

impl ::protobuf::Message for Order {
    fn is_initialized(&self) -> bool {
        for v in &self.rate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.volume {
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
                    self.kind = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.volume)?;
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
        if self.kind != OrderKind::ASK {
            my_size += ::protobuf::rt::enum_size(1, self.kind);
        }
        if let Some(ref v) = self.rate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.volume.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.kind != OrderKind::ASK {
            os.write_enum(1, self.kind.value())?;
        }
        if let Some(ref v) = self.rate.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.volume.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Order {
    fn new() -> Order {
        Order::new()
    }

    fn descriptor_static(_: ::std::option::Option<Order>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OrderKind>>(
                    "kind",
                    Order::get_kind_for_reflect,
                    Order::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::currency::CurrencyVolume>>(
                    "rate",
                    Order::get_rate_for_reflect,
                    Order::mut_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::currency::CurrencyVolume>>(
                    "volume",
                    Order::get_volume_for_reflect,
                    Order::mut_volume_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Order>(
                    "Order",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Order {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_rate();
        self.clear_volume();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Order {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Order {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawOrder {
    // message fields
    pub kind: OrderKind,
    pub rate: f64,
    pub volume: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawOrder {}

impl RawOrder {
    pub fn new() -> RawOrder {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawOrder {
        static mut instance: ::protobuf::lazy::Lazy<RawOrder> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawOrder,
        };
        unsafe {
            instance.get(RawOrder::new)
        }
    }

    // .orca.OrderKind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = OrderKind::ASK;
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: OrderKind) {
        self.kind = v;
    }

    pub fn get_kind(&self) -> OrderKind {
        self.kind
    }

    fn get_kind_for_reflect(&self) -> &OrderKind {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut OrderKind {
        &mut self.kind
    }

    // double rate = 2;

    pub fn clear_rate(&mut self) {
        self.rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rate(&mut self, v: f64) {
        self.rate = v;
    }

    pub fn get_rate(&self) -> f64 {
        self.rate
    }

    fn get_rate_for_reflect(&self) -> &f64 {
        &self.rate
    }

    fn mut_rate_for_reflect(&mut self) -> &mut f64 {
        &mut self.rate
    }

    // double volume = 3;

    pub fn clear_volume(&mut self) {
        self.volume = 0.;
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: f64) {
        self.volume = v;
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    fn get_volume_for_reflect(&self) -> &f64 {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut f64 {
        &mut self.volume
    }
}

impl ::protobuf::Message for RawOrder {
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
                    let tmp = is.read_enum()?;
                    self.kind = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.rate = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.volume = tmp;
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
        if self.kind != OrderKind::ASK {
            my_size += ::protobuf::rt::enum_size(1, self.kind);
        }
        if self.rate != 0. {
            my_size += 9;
        }
        if self.volume != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.kind != OrderKind::ASK {
            os.write_enum(1, self.kind.value())?;
        }
        if self.rate != 0. {
            os.write_double(2, self.rate)?;
        }
        if self.volume != 0. {
            os.write_double(3, self.volume)?;
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

impl ::protobuf::MessageStatic for RawOrder {
    fn new() -> RawOrder {
        RawOrder::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawOrder>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OrderKind>>(
                    "kind",
                    RawOrder::get_kind_for_reflect,
                    RawOrder::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "rate",
                    RawOrder::get_rate_for_reflect,
                    RawOrder::mut_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "volume",
                    RawOrder::get_volume_for_reflect,
                    RawOrder::mut_volume_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawOrder>(
                    "RawOrder",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawOrder {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_rate();
        self.clear_volume();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawOrder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OrderKind {
    ASK = 0,
    BID = 1,
}

impl ::protobuf::ProtobufEnum for OrderKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OrderKind> {
        match value {
            0 => ::std::option::Option::Some(OrderKind::ASK),
            1 => ::std::option::Option::Some(OrderKind::BID),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OrderKind] = &[
            OrderKind::ASK,
            OrderKind::BID,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OrderKind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OrderKind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OrderKind {
}

impl ::std::default::Default for OrderKind {
    fn default() -> Self {
        OrderKind::ASK
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderKind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15orca/core/order.proto\x12\x04orca\x1a\x18orca/core/currency.proto\
    \"\x84\x01\n\x05Order\x12#\n\x04kind\x18\x01\x20\x01(\x0e2\x0f.orca.Orde\
    rKindR\x04kind\x12(\n\x04rate\x18\x02\x20\x01(\x0b2\x14.orca.CurrencyVol\
    umeR\x04rate\x12,\n\x06volume\x18\x03\x20\x01(\x0b2\x14.orca.CurrencyVol\
    umeR\x06volume\"[\n\x08RawOrder\x12#\n\x04kind\x18\x01\x20\x01(\x0e2\x0f\
    .orca.OrderKindR\x04kind\x12\x12\n\x04rate\x18\x02\x20\x01(\x01R\x04rate\
    \x12\x16\n\x06volume\x18\x03\x20\x01(\x01R\x06volume*\x1d\n\tOrderKind\
    \x12\x07\n\x03ASK\x10\0\x12\x07\n\x03BID\x10\x01b\x06proto3\
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
