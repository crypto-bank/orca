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
pub struct Event {
    // message oneof groups
    event: ::std::option::Option<Event_oneof_event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event {}

#[derive(Clone,PartialEq)]
pub enum Event_oneof_event {
    order(super::order::RawOrder),
    trade(super::trade::RawTrade),
}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(Event::new)
        }
    }

    // .orca.core.RawOrder order = 1;

    pub fn clear_order(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_order(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::order(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: super::order::RawOrder) {
        self.event = ::std::option::Option::Some(Event_oneof_event::order(v))
    }

    // Mutable pointer to the field.
    pub fn mut_order(&mut self) -> &mut super::order::RawOrder {
        if let ::std::option::Option::Some(Event_oneof_event::order(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(Event_oneof_event::order(super::order::RawOrder::new()));
        }
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::order(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_order(&mut self) -> super::order::RawOrder {
        if self.has_order() {
            match self.event.take() {
                ::std::option::Option::Some(Event_oneof_event::order(v)) => v,
                _ => panic!(),
            }
        } else {
            super::order::RawOrder::new()
        }
    }

    pub fn get_order(&self) -> &super::order::RawOrder {
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::order(ref v)) => v,
            _ => super::order::RawOrder::default_instance(),
        }
    }

    // .orca.core.RawTrade trade = 2;

    pub fn clear_trade(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_trade(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::trade(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_trade(&mut self, v: super::trade::RawTrade) {
        self.event = ::std::option::Option::Some(Event_oneof_event::trade(v))
    }

    // Mutable pointer to the field.
    pub fn mut_trade(&mut self) -> &mut super::trade::RawTrade {
        if let ::std::option::Option::Some(Event_oneof_event::trade(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(Event_oneof_event::trade(super::trade::RawTrade::new()));
        }
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::trade(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_trade(&mut self) -> super::trade::RawTrade {
        if self.has_trade() {
            match self.event.take() {
                ::std::option::Option::Some(Event_oneof_event::trade(v)) => v,
                _ => panic!(),
            }
        } else {
            super::trade::RawTrade::new()
        }
    }

    pub fn get_trade(&self) -> &super::trade::RawTrade {
        match self.event {
            ::std::option::Option::Some(Event_oneof_event::trade(ref v)) => v,
            _ => super::trade::RawTrade::default_instance(),
        }
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        if let Some(Event_oneof_event::order(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Event_oneof_event::trade(ref v)) = self.event {
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
                    self.event = ::std::option::Option::Some(Event_oneof_event::order(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(Event_oneof_event::trade(is.read_message()?));
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
                &Event_oneof_event::order(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Event_oneof_event::trade(ref v) => {
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
                &Event_oneof_event::order(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Event_oneof_event::trade(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::order::RawOrder>(
                    "order",
                    Event::has_order,
                    Event::get_order,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::trade::RawTrade>(
                    "trade",
                    Event::has_trade,
                    Event::get_trade,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_order();
        self.clear_trade();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Events {
    // message fields
    pub events: ::protobuf::RepeatedField<Event>,
    pub reset: bool,
    pub timestamp: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Events {}

impl Events {
    pub fn new() -> Events {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Events {
        static mut instance: ::protobuf::lazy::Lazy<Events> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Events,
        };
        unsafe {
            instance.get(Events::new)
        }
    }

    // repeated .orca.storage.Event events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<Event>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<Event> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<Event> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[Event] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<Event> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Event> {
        &mut self.events
    }

    // bool reset = 2;

    pub fn clear_reset(&mut self) {
        self.reset = false;
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: bool) {
        self.reset = v;
    }

    pub fn get_reset(&self) -> bool {
        self.reset
    }

    fn get_reset_for_reflect(&self) -> &bool {
        &self.reset
    }

    fn mut_reset_for_reflect(&mut self) -> &mut bool {
        &mut self.reset
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

impl ::protobuf::Message for Events {
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
                    let tmp = is.read_bool()?;
                    self.reset = tmp;
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.reset != false {
            my_size += 2;
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
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
        if self.reset != false {
            os.write_bool(2, self.reset)?;
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

impl ::protobuf::MessageStatic for Events {
    fn new() -> Events {
        Events::new()
    }

    fn descriptor_static(_: ::std::option::Option<Events>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event>>(
                    "events",
                    Events::get_events_for_reflect,
                    Events::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reset",
                    Events::get_reset_for_reflect,
                    Events::mut_reset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    Events::get_timestamp_for_reflect,
                    Events::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Events>(
                    "Events",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Events {
    fn clear(&mut self) {
        self.clear_events();
        self.clear_reset();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Events {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Events {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19orca/storage/events.proto\x12\x0corca.storage\x1a\x15orca/core/ord\
    er.proto\x1a\x15orca/core/trade.proto\"j\n\x05Event\x12+\n\x05order\x18\
    \x01\x20\x01(\x0b2\x13.orca.core.RawOrderH\0R\x05order\x12+\n\x05trade\
    \x18\x02\x20\x01(\x0b2\x13.orca.core.RawTradeH\0R\x05tradeB\x07\n\x05eve\
    nt\"i\n\x06Events\x12+\n\x06events\x18\x01\x20\x03(\x0b2\x13.orca.storag\
    e.EventR\x06events\x12\x14\n\x05reset\x18\x02\x20\x01(\x08R\x05reset\x12\
    \x1c\n\ttimestamp\x18\x03\x20\x01(\x03R\ttimestampb\x06proto3\
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
