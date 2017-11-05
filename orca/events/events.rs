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
pub struct LogEvent {
    // message oneof groups
    event: ::std::option::Option<LogEvent_oneof_event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogEvent {}

#[derive(Clone,PartialEq)]
pub enum LogEvent_oneof_event {
    order(::markets::Order),
    trade(::markets::Trade),
    book(::markets::OrderBook),
}

impl LogEvent {
    pub fn new() -> LogEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEvent {
        static mut instance: ::protobuf::lazy::Lazy<LogEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEvent,
        };
        unsafe {
            instance.get(LogEvent::new)
        }
    }

    // .orca.markets.Order order = 1;

    pub fn clear_order(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_order(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::order(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: ::markets::Order) {
        self.event = ::std::option::Option::Some(LogEvent_oneof_event::order(v))
    }

    // Mutable pointer to the field.
    pub fn mut_order(&mut self) -> &mut ::markets::Order {
        if let ::std::option::Option::Some(LogEvent_oneof_event::order(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(LogEvent_oneof_event::order(::markets::Order::new()));
        }
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::order(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_order(&mut self) -> ::markets::Order {
        if self.has_order() {
            match self.event.take() {
                ::std::option::Option::Some(LogEvent_oneof_event::order(v)) => v,
                _ => panic!(),
            }
        } else {
            ::markets::Order::new()
        }
    }

    pub fn get_order(&self) -> &::markets::Order {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::order(ref v)) => v,
            _ => ::markets::Order::default_instance(),
        }
    }

    // .orca.markets.Trade trade = 2;

    pub fn clear_trade(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_trade(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::trade(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_trade(&mut self, v: ::markets::Trade) {
        self.event = ::std::option::Option::Some(LogEvent_oneof_event::trade(v))
    }

    // Mutable pointer to the field.
    pub fn mut_trade(&mut self) -> &mut ::markets::Trade {
        if let ::std::option::Option::Some(LogEvent_oneof_event::trade(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(LogEvent_oneof_event::trade(::markets::Trade::new()));
        }
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::trade(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_trade(&mut self) -> ::markets::Trade {
        if self.has_trade() {
            match self.event.take() {
                ::std::option::Option::Some(LogEvent_oneof_event::trade(v)) => v,
                _ => panic!(),
            }
        } else {
            ::markets::Trade::new()
        }
    }

    pub fn get_trade(&self) -> &::markets::Trade {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::trade(ref v)) => v,
            _ => ::markets::Trade::default_instance(),
        }
    }

    // .orca.markets.OrderBook book = 3;

    pub fn clear_book(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_book(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::book(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_book(&mut self, v: ::markets::OrderBook) {
        self.event = ::std::option::Option::Some(LogEvent_oneof_event::book(v))
    }

    // Mutable pointer to the field.
    pub fn mut_book(&mut self) -> &mut ::markets::OrderBook {
        if let ::std::option::Option::Some(LogEvent_oneof_event::book(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(LogEvent_oneof_event::book(::markets::OrderBook::new()));
        }
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::book(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_book(&mut self) -> ::markets::OrderBook {
        if self.has_book() {
            match self.event.take() {
                ::std::option::Option::Some(LogEvent_oneof_event::book(v)) => v,
                _ => panic!(),
            }
        } else {
            ::markets::OrderBook::new()
        }
    }

    pub fn get_book(&self) -> &::markets::OrderBook {
        match self.event {
            ::std::option::Option::Some(LogEvent_oneof_event::book(ref v)) => v,
            _ => ::markets::OrderBook::default_instance(),
        }
    }
}

impl ::protobuf::Message for LogEvent {
    fn is_initialized(&self) -> bool {
        if let Some(LogEvent_oneof_event::order(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(LogEvent_oneof_event::trade(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(LogEvent_oneof_event::book(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(LogEvent_oneof_event::order(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(LogEvent_oneof_event::trade(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(LogEvent_oneof_event::book(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &LogEvent_oneof_event::order(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LogEvent_oneof_event::trade(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LogEvent_oneof_event::book(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &LogEvent_oneof_event::order(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LogEvent_oneof_event::trade(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LogEvent_oneof_event::book(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for LogEvent {
    fn new() -> LogEvent {
        LogEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::markets::Order>(
                    "order",
                    LogEvent::has_order,
                    LogEvent::get_order,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::markets::Trade>(
                    "trade",
                    LogEvent::has_trade,
                    LogEvent::get_trade,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::markets::OrderBook>(
                    "book",
                    LogEvent::has_book,
                    LogEvent::get_book,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEvent>(
                    "LogEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEvent {
    fn clear(&mut self) {
        self.clear_order();
        self.clear_trade();
        self.clear_book();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogBody {
    // message fields
    pub events: ::protobuf::RepeatedField<LogEvent>,
    pub timestamp: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogBody {}

impl LogBody {
    pub fn new() -> LogBody {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogBody {
        static mut instance: ::protobuf::lazy::Lazy<LogBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogBody,
        };
        unsafe {
            instance.get(LogBody::new)
        }
    }

    // repeated .orca.events.LogEvent events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<LogEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<LogEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<LogEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[LogEvent] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<LogEvent> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LogEvent> {
        &mut self.events
    }

    // int64 timestamp = 2;

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

impl ::protobuf::Message for LogBody {
    fn is_initialized(&self) -> bool {
        for v in &self.events {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                2 => {
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.timestamp != 0 {
            os.write_int64(2, self.timestamp)?;
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

impl ::protobuf::MessageStatic for LogBody {
    fn new() -> LogBody {
        LogBody::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogBody>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LogEvent>>(
                    "events",
                    LogBody::get_events_for_reflect,
                    LogBody::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    LogBody::get_timestamp_for_reflect,
                    LogBody::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogBody>(
                    "LogBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogBody {
    fn clear(&mut self) {
        self.clear_events();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18orca/events/events.proto\x12\x0borca.events\x1a\x1aorca/markets/ma\
    rkets.proto\"\x9c\x01\n\x08LogEvent\x12+\n\x05order\x18\x01\x20\x01(\x0b\
    2\x13.orca.markets.OrderH\0R\x05order\x12+\n\x05trade\x18\x02\x20\x01(\
    \x0b2\x13.orca.markets.TradeH\0R\x05trade\x12-\n\x04book\x18\x03\x20\x01\
    (\x0b2\x17.orca.markets.OrderBookH\0R\x04bookB\x07\n\x05event\"V\n\x07Lo\
    gBody\x12-\n\x06events\x18\x01\x20\x03(\x0b2\x15.orca.events.LogEventR\
    \x06events\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x03R\ttimestampb\x06pro\
    to3\
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
