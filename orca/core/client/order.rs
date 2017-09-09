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
pub struct OrderPlacedResult {
    // message fields
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OrderPlacedResult {}

impl OrderPlacedResult {
    pub fn new() -> OrderPlacedResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OrderPlacedResult {
        static mut instance: ::protobuf::lazy::Lazy<OrderPlacedResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderPlacedResult,
        };
        unsafe {
            instance.get(OrderPlacedResult::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for OrderPlacedResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
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

impl ::protobuf::MessageStatic for OrderPlacedResult {
    fn new() -> OrderPlacedResult {
        OrderPlacedResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<OrderPlacedResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    OrderPlacedResult::get_id_for_reflect,
                    OrderPlacedResult::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderPlacedResult>(
                    "OrderPlacedResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OrderPlacedResult {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderPlacedResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderPlacedResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderFilledResult {
    // message fields
    pub trades: ::protobuf::RepeatedField<super::trade::RawTrade>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OrderFilledResult {}

impl OrderFilledResult {
    pub fn new() -> OrderFilledResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OrderFilledResult {
        static mut instance: ::protobuf::lazy::Lazy<OrderFilledResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderFilledResult,
        };
        unsafe {
            instance.get(OrderFilledResult::new)
        }
    }

    // repeated .orca.RawTrade trades = 1;

    pub fn clear_trades(&mut self) {
        self.trades.clear();
    }

    // Param is passed by value, moved
    pub fn set_trades(&mut self, v: ::protobuf::RepeatedField<super::trade::RawTrade>) {
        self.trades = v;
    }

    // Mutable pointer to the field.
    pub fn mut_trades(&mut self) -> &mut ::protobuf::RepeatedField<super::trade::RawTrade> {
        &mut self.trades
    }

    // Take field
    pub fn take_trades(&mut self) -> ::protobuf::RepeatedField<super::trade::RawTrade> {
        ::std::mem::replace(&mut self.trades, ::protobuf::RepeatedField::new())
    }

    pub fn get_trades(&self) -> &[super::trade::RawTrade] {
        &self.trades
    }

    fn get_trades_for_reflect(&self) -> &::protobuf::RepeatedField<super::trade::RawTrade> {
        &self.trades
    }

    fn mut_trades_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::trade::RawTrade> {
        &mut self.trades
    }
}

impl ::protobuf::Message for OrderFilledResult {
    fn is_initialized(&self) -> bool {
        for v in &self.trades {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.trades)?;
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
        for value in &self.trades {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.trades {
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

impl ::protobuf::MessageStatic for OrderFilledResult {
    fn new() -> OrderFilledResult {
        OrderFilledResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<OrderFilledResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::trade::RawTrade>>(
                    "trades",
                    OrderFilledResult::get_trades_for_reflect,
                    OrderFilledResult::mut_trades_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderFilledResult>(
                    "OrderFilledResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OrderFilledResult {
    fn clear(&mut self) {
        self.clear_trades();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderFilledResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderFilledResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderPartiallyFilledResult {
    // message fields
    pub id: ::std::string::String,
    pub trades: ::protobuf::RepeatedField<super::trade::RawTrade>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OrderPartiallyFilledResult {}

impl OrderPartiallyFilledResult {
    pub fn new() -> OrderPartiallyFilledResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OrderPartiallyFilledResult {
        static mut instance: ::protobuf::lazy::Lazy<OrderPartiallyFilledResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderPartiallyFilledResult,
        };
        unsafe {
            instance.get(OrderPartiallyFilledResult::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // repeated .orca.RawTrade trades = 2;

    pub fn clear_trades(&mut self) {
        self.trades.clear();
    }

    // Param is passed by value, moved
    pub fn set_trades(&mut self, v: ::protobuf::RepeatedField<super::trade::RawTrade>) {
        self.trades = v;
    }

    // Mutable pointer to the field.
    pub fn mut_trades(&mut self) -> &mut ::protobuf::RepeatedField<super::trade::RawTrade> {
        &mut self.trades
    }

    // Take field
    pub fn take_trades(&mut self) -> ::protobuf::RepeatedField<super::trade::RawTrade> {
        ::std::mem::replace(&mut self.trades, ::protobuf::RepeatedField::new())
    }

    pub fn get_trades(&self) -> &[super::trade::RawTrade] {
        &self.trades
    }

    fn get_trades_for_reflect(&self) -> &::protobuf::RepeatedField<super::trade::RawTrade> {
        &self.trades
    }

    fn mut_trades_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::trade::RawTrade> {
        &mut self.trades
    }
}

impl ::protobuf::Message for OrderPartiallyFilledResult {
    fn is_initialized(&self) -> bool {
        for v in &self.trades {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.trades)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        for value in &self.trades {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        for v in &self.trades {
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

impl ::protobuf::MessageStatic for OrderPartiallyFilledResult {
    fn new() -> OrderPartiallyFilledResult {
        OrderPartiallyFilledResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<OrderPartiallyFilledResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    OrderPartiallyFilledResult::get_id_for_reflect,
                    OrderPartiallyFilledResult::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::trade::RawTrade>>(
                    "trades",
                    OrderPartiallyFilledResult::get_trades_for_reflect,
                    OrderPartiallyFilledResult::mut_trades_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderPartiallyFilledResult>(
                    "OrderPartiallyFilledResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OrderPartiallyFilledResult {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_trades();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderPartiallyFilledResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderPartiallyFilledResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderResult {
    // message oneof groups
    result: ::std::option::Option<OrderResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OrderResult {}

#[derive(Clone,PartialEq)]
pub enum OrderResult_oneof_result {
    placed(OrderPlacedResult),
    filled(OrderFilledResult),
    partial(OrderPartiallyFilledResult),
}

impl OrderResult {
    pub fn new() -> OrderResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OrderResult {
        static mut instance: ::protobuf::lazy::Lazy<OrderResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderResult,
        };
        unsafe {
            instance.get(OrderResult::new)
        }
    }

    // .orca.client.OrderPlacedResult placed = 1;

    pub fn clear_placed(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_placed(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::placed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_placed(&mut self, v: OrderPlacedResult) {
        self.result = ::std::option::Option::Some(OrderResult_oneof_result::placed(v))
    }

    // Mutable pointer to the field.
    pub fn mut_placed(&mut self) -> &mut OrderPlacedResult {
        if let ::std::option::Option::Some(OrderResult_oneof_result::placed(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(OrderResult_oneof_result::placed(OrderPlacedResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::placed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_placed(&mut self) -> OrderPlacedResult {
        if self.has_placed() {
            match self.result.take() {
                ::std::option::Option::Some(OrderResult_oneof_result::placed(v)) => v,
                _ => panic!(),
            }
        } else {
            OrderPlacedResult::new()
        }
    }

    pub fn get_placed(&self) -> &OrderPlacedResult {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::placed(ref v)) => v,
            _ => OrderPlacedResult::default_instance(),
        }
    }

    // .orca.client.OrderFilledResult filled = 2;

    pub fn clear_filled(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_filled(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::filled(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filled(&mut self, v: OrderFilledResult) {
        self.result = ::std::option::Option::Some(OrderResult_oneof_result::filled(v))
    }

    // Mutable pointer to the field.
    pub fn mut_filled(&mut self) -> &mut OrderFilledResult {
        if let ::std::option::Option::Some(OrderResult_oneof_result::filled(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(OrderResult_oneof_result::filled(OrderFilledResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::filled(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_filled(&mut self) -> OrderFilledResult {
        if self.has_filled() {
            match self.result.take() {
                ::std::option::Option::Some(OrderResult_oneof_result::filled(v)) => v,
                _ => panic!(),
            }
        } else {
            OrderFilledResult::new()
        }
    }

    pub fn get_filled(&self) -> &OrderFilledResult {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::filled(ref v)) => v,
            _ => OrderFilledResult::default_instance(),
        }
    }

    // .orca.client.OrderPartiallyFilledResult partial = 3;

    pub fn clear_partial(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_partial(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::partial(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_partial(&mut self, v: OrderPartiallyFilledResult) {
        self.result = ::std::option::Option::Some(OrderResult_oneof_result::partial(v))
    }

    // Mutable pointer to the field.
    pub fn mut_partial(&mut self) -> &mut OrderPartiallyFilledResult {
        if let ::std::option::Option::Some(OrderResult_oneof_result::partial(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(OrderResult_oneof_result::partial(OrderPartiallyFilledResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::partial(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_partial(&mut self) -> OrderPartiallyFilledResult {
        if self.has_partial() {
            match self.result.take() {
                ::std::option::Option::Some(OrderResult_oneof_result::partial(v)) => v,
                _ => panic!(),
            }
        } else {
            OrderPartiallyFilledResult::new()
        }
    }

    pub fn get_partial(&self) -> &OrderPartiallyFilledResult {
        match self.result {
            ::std::option::Option::Some(OrderResult_oneof_result::partial(ref v)) => v,
            _ => OrderPartiallyFilledResult::default_instance(),
        }
    }
}

impl ::protobuf::Message for OrderResult {
    fn is_initialized(&self) -> bool {
        if let Some(OrderResult_oneof_result::placed(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(OrderResult_oneof_result::filled(ref v)) = self.result {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(OrderResult_oneof_result::partial(ref v)) = self.result {
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
                    self.result = ::std::option::Option::Some(OrderResult_oneof_result::placed(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(OrderResult_oneof_result::filled(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.result = ::std::option::Option::Some(OrderResult_oneof_result::partial(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &OrderResult_oneof_result::placed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &OrderResult_oneof_result::filled(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &OrderResult_oneof_result::partial(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &OrderResult_oneof_result::placed(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &OrderResult_oneof_result::filled(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &OrderResult_oneof_result::partial(ref v) => {
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

impl ::protobuf::MessageStatic for OrderResult {
    fn new() -> OrderResult {
        OrderResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<OrderResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OrderPlacedResult>(
                    "placed",
                    OrderResult::has_placed,
                    OrderResult::get_placed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OrderFilledResult>(
                    "filled",
                    OrderResult::has_filled,
                    OrderResult::get_filled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OrderPartiallyFilledResult>(
                    "partial",
                    OrderResult::has_partial,
                    OrderResult::get_partial,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderResult>(
                    "OrderResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OrderResult {
    fn clear(&mut self) {
        self.clear_placed();
        self.clear_filled();
        self.clear_partial();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OrderStrategy {
    FILL_OR_KILL = 0,
    IMMEDIATE_OR_CANCEL = 1,
    POST_ONLY = 2,
}

impl ::protobuf::ProtobufEnum for OrderStrategy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OrderStrategy> {
        match value {
            0 => ::std::option::Option::Some(OrderStrategy::FILL_OR_KILL),
            1 => ::std::option::Option::Some(OrderStrategy::IMMEDIATE_OR_CANCEL),
            2 => ::std::option::Option::Some(OrderStrategy::POST_ONLY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OrderStrategy] = &[
            OrderStrategy::FILL_OR_KILL,
            OrderStrategy::IMMEDIATE_OR_CANCEL,
            OrderStrategy::POST_ONLY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OrderStrategy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OrderStrategy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OrderStrategy {
}

impl ::std::default::Default for OrderStrategy {
    fn default() -> Self {
        OrderStrategy::FILL_OR_KILL
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1corca/core/client/order.proto\x12\x0borca.client\x1a\x15orca/core/t\
    rade.proto\"#\n\x11OrderPlacedResult\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\
    \x02id\";\n\x11OrderFilledResult\x12&\n\x06trades\x18\x01\x20\x03(\x0b2\
    \x0e.orca.RawTradeR\x06trades\"T\n\x1aOrderPartiallyFilledResult\x12\x0e\
    \n\x02id\x18\x01\x20\x01(\tR\x02id\x12&\n\x06trades\x18\x02\x20\x03(\x0b\
    2\x0e.orca.RawTradeR\x06trades\"\xd0\x01\n\x0bOrderResult\x128\n\x06plac\
    ed\x18\x01\x20\x01(\x0b2\x1e.orca.client.OrderPlacedResultH\0R\x06placed\
    \x128\n\x06filled\x18\x02\x20\x01(\x0b2\x1e.orca.client.OrderFilledResul\
    tH\0R\x06filled\x12C\n\x07partial\x18\x03\x20\x01(\x0b2'.orca.client.Ord\
    erPartiallyFilledResultH\0R\x07partialB\x08\n\x06result*I\n\rOrderStrate\
    gy\x12\x10\n\x0cFILL_OR_KILL\x10\0\x12\x17\n\x13IMMEDIATE_OR_CANCEL\x10\
    \x01\x12\r\n\tPOST_ONLY\x10\x02b\x06proto3\
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
