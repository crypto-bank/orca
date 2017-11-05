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
    pub rate: f64,
    pub volume: f64,
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

    // .orca.markets.OrderKind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = OrderKind::Ask;
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

impl ::protobuf::Message for Order {
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
        if self.kind != OrderKind::Ask {
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
        if self.kind != OrderKind::Ask {
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "rate",
                    Order::get_rate_for_reflect,
                    Order::mut_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
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
pub struct Trade {
    // message fields
    pub id: i64,
    pub order: ::protobuf::SingularPtrField<Order>,
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

    // .orca.markets.Order order = 2;

    pub fn clear_order(&mut self) {
        self.order.clear();
    }

    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: Order) {
        self.order = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_order(&mut self) -> &mut Order {
        if self.order.is_none() {
            self.order.set_default();
        }
        self.order.as_mut().unwrap()
    }

    // Take field
    pub fn take_order(&mut self) -> Order {
        self.order.take().unwrap_or_else(|| Order::new())
    }

    pub fn get_order(&self) -> &Order {
        self.order.as_ref().unwrap_or_else(|| Order::default_instance())
    }

    fn get_order_for_reflect(&self) -> &::protobuf::SingularPtrField<Order> {
        &self.order
    }

    fn mut_order_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Order> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Order>>(
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
pub struct OrderBook {
    // message fields
    pub orders: ::protobuf::RepeatedField<Order>,
    pub pair: ::protobuf::SingularPtrField<::currency::Pair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OrderBook {}

impl OrderBook {
    pub fn new() -> OrderBook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OrderBook {
        static mut instance: ::protobuf::lazy::Lazy<OrderBook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderBook,
        };
        unsafe {
            instance.get(OrderBook::new)
        }
    }

    // repeated .orca.markets.Order orders = 1;

    pub fn clear_orders(&mut self) {
        self.orders.clear();
    }

    // Param is passed by value, moved
    pub fn set_orders(&mut self, v: ::protobuf::RepeatedField<Order>) {
        self.orders = v;
    }

    // Mutable pointer to the field.
    pub fn mut_orders(&mut self) -> &mut ::protobuf::RepeatedField<Order> {
        &mut self.orders
    }

    // Take field
    pub fn take_orders(&mut self) -> ::protobuf::RepeatedField<Order> {
        ::std::mem::replace(&mut self.orders, ::protobuf::RepeatedField::new())
    }

    pub fn get_orders(&self) -> &[Order] {
        &self.orders
    }

    fn get_orders_for_reflect(&self) -> &::protobuf::RepeatedField<Order> {
        &self.orders
    }

    fn mut_orders_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Order> {
        &mut self.orders
    }

    // .orca.currency.Pair pair = 2;

    pub fn clear_pair(&mut self) {
        self.pair.clear();
    }

    pub fn has_pair(&self) -> bool {
        self.pair.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pair(&mut self, v: ::currency::Pair) {
        self.pair = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pair(&mut self) -> &mut ::currency::Pair {
        if self.pair.is_none() {
            self.pair.set_default();
        }
        self.pair.as_mut().unwrap()
    }

    // Take field
    pub fn take_pair(&mut self) -> ::currency::Pair {
        self.pair.take().unwrap_or_else(|| ::currency::Pair::new())
    }

    pub fn get_pair(&self) -> &::currency::Pair {
        self.pair.as_ref().unwrap_or_else(|| ::currency::Pair::default_instance())
    }

    fn get_pair_for_reflect(&self) -> &::protobuf::SingularPtrField<::currency::Pair> {
        &self.pair
    }

    fn mut_pair_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::currency::Pair> {
        &mut self.pair
    }
}

impl ::protobuf::Message for OrderBook {
    fn is_initialized(&self) -> bool {
        for v in &self.orders {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pair {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.orders)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pair)?;
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
        for value in &self.orders {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.pair.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.orders {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.pair.as_ref() {
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

impl ::protobuf::MessageStatic for OrderBook {
    fn new() -> OrderBook {
        OrderBook::new()
    }

    fn descriptor_static(_: ::std::option::Option<OrderBook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Order>>(
                    "orders",
                    OrderBook::get_orders_for_reflect,
                    OrderBook::mut_orders_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::currency::Pair>>(
                    "pair",
                    OrderBook::get_pair_for_reflect,
                    OrderBook::mut_pair_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderBook>(
                    "OrderBook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OrderBook {
    fn clear(&mut self) {
        self.clear_orders();
        self.clear_pair();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderBook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderBook {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Market {
    Invalid = 0,
    Poloniex = 1,
    Bitfinex = 2,
}

impl ::protobuf::ProtobufEnum for Market {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Market> {
        match value {
            0 => ::std::option::Option::Some(Market::Invalid),
            1 => ::std::option::Option::Some(Market::Poloniex),
            2 => ::std::option::Option::Some(Market::Bitfinex),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Market] = &[
            Market::Invalid,
            Market::Poloniex,
            Market::Bitfinex,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Market>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Market", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Market {
}

impl ::std::default::Default for Market {
    fn default() -> Self {
        Market::Invalid
    }
}

impl ::protobuf::reflect::ProtobufValue for Market {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OrderKind {
    Ask = 0,
    Bid = 1,
}

impl ::protobuf::ProtobufEnum for OrderKind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OrderKind> {
        match value {
            0 => ::std::option::Option::Some(OrderKind::Ask),
            1 => ::std::option::Option::Some(OrderKind::Bid),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OrderKind] = &[
            OrderKind::Ask,
            OrderKind::Bid,
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
        OrderKind::Ask
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderKind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aorca/markets/markets.proto\x12\x0corca.markets\x1a\x1corca/currenc\
    y/currency.proto\"`\n\x05Order\x12+\n\x04kind\x18\x01\x20\x01(\x0e2\x17.\
    orca.markets.OrderKindR\x04kind\x12\x12\n\x04rate\x18\x02\x20\x01(\x01R\
    \x04rate\x12\x16\n\x06volume\x18\x03\x20\x01(\x01R\x06volume\"`\n\x05Tra\
    de\x12\x0e\n\x02id\x18\x01\x20\x01(\x03R\x02id\x12)\n\x05order\x18\x02\
    \x20\x01(\x0b2\x13.orca.markets.OrderR\x05order\x12\x1c\n\ttimestamp\x18\
    \x03\x20\x01(\x03R\ttimestamp\"a\n\tOrderBook\x12+\n\x06orders\x18\x01\
    \x20\x03(\x0b2\x13.orca.markets.OrderR\x06orders\x12'\n\x04pair\x18\x02\
    \x20\x01(\x0b2\x13.orca.currency.PairR\x04pair*1\n\x06Market\x12\x0b\n\
    \x07Invalid\x10\0\x12\x0c\n\x08Poloniex\x10\x01\x12\x0c\n\x08Bitfinex\
    \x10\x02*\x1d\n\tOrderKind\x12\x07\n\x03Ask\x10\0\x12\x07\n\x03Bid\x10\
    \x01b\x06proto3\
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
