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
pub struct CoinPair {
    // message fields
    pub first: ::std::string::String,
    pub second: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CoinPair {}

impl CoinPair {
    pub fn new() -> CoinPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CoinPair {
        static mut instance: ::protobuf::lazy::Lazy<CoinPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoinPair,
        };
        unsafe {
            instance.get(CoinPair::new)
        }
    }

    // string first = 1;

    pub fn clear_first(&mut self) {
        self.first.clear();
    }

    // Param is passed by value, moved
    pub fn set_first(&mut self, v: ::std::string::String) {
        self.first = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first(&mut self) -> &mut ::std::string::String {
        &mut self.first
    }

    // Take field
    pub fn take_first(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.first, ::std::string::String::new())
    }

    pub fn get_first(&self) -> &str {
        &self.first
    }

    fn get_first_for_reflect(&self) -> &::std::string::String {
        &self.first
    }

    fn mut_first_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.first
    }

    // string second = 2;

    pub fn clear_second(&mut self) {
        self.second.clear();
    }

    // Param is passed by value, moved
    pub fn set_second(&mut self, v: ::std::string::String) {
        self.second = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_second(&mut self) -> &mut ::std::string::String {
        &mut self.second
    }

    // Take field
    pub fn take_second(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.second, ::std::string::String::new())
    }

    pub fn get_second(&self) -> &str {
        &self.second
    }

    fn get_second_for_reflect(&self) -> &::std::string::String {
        &self.second
    }

    fn mut_second_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.second
    }
}

impl ::protobuf::Message for CoinPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.first)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.second)?;
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
        if !self.first.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.first);
        }
        if !self.second.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.second);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.first.is_empty() {
            os.write_string(1, &self.first)?;
        }
        if !self.second.is_empty() {
            os.write_string(2, &self.second)?;
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

impl ::protobuf::MessageStatic for CoinPair {
    fn new() -> CoinPair {
        CoinPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<CoinPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "first",
                    CoinPair::get_first_for_reflect,
                    CoinPair::mut_first_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "second",
                    CoinPair::get_second_for_reflect,
                    CoinPair::mut_second_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CoinPair>(
                    "CoinPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CoinPair {
    fn clear(&mut self) {
        self.clear_first();
        self.clear_second();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CoinPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoinPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CoinVolume {
    // message fields
    pub coin: ::std::string::String,
    pub amount: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CoinVolume {}

impl CoinVolume {
    pub fn new() -> CoinVolume {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CoinVolume {
        static mut instance: ::protobuf::lazy::Lazy<CoinVolume> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoinVolume,
        };
        unsafe {
            instance.get(CoinVolume::new)
        }
    }

    // string coin = 1;

    pub fn clear_coin(&mut self) {
        self.coin.clear();
    }

    // Param is passed by value, moved
    pub fn set_coin(&mut self, v: ::std::string::String) {
        self.coin = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_coin(&mut self) -> &mut ::std::string::String {
        &mut self.coin
    }

    // Take field
    pub fn take_coin(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.coin, ::std::string::String::new())
    }

    pub fn get_coin(&self) -> &str {
        &self.coin
    }

    fn get_coin_for_reflect(&self) -> &::std::string::String {
        &self.coin
    }

    fn mut_coin_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.coin
    }

    // uint64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: u64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &u64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut u64 {
        &mut self.amount
    }
}

impl ::protobuf::Message for CoinVolume {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.coin)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.amount = tmp;
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
        if !self.coin.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.coin);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.coin.is_empty() {
            os.write_string(1, &self.coin)?;
        }
        if self.amount != 0 {
            os.write_uint64(2, self.amount)?;
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

impl ::protobuf::MessageStatic for CoinVolume {
    fn new() -> CoinVolume {
        CoinVolume::new()
    }

    fn descriptor_static(_: ::std::option::Option<CoinVolume>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "coin",
                    CoinVolume::get_coin_for_reflect,
                    CoinVolume::mut_coin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "amount",
                    CoinVolume::get_amount_for_reflect,
                    CoinVolume::mut_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CoinVolume>(
                    "CoinVolume",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CoinVolume {
    fn clear(&mut self) {
        self.clear_coin();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CoinVolume {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoinVolume {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14orca/core/coin.proto\x12\x04orca\"8\n\x08CoinPair\x12\x14\n\x05fir\
    st\x18\x01\x20\x01(\tR\x05first\x12\x16\n\x06second\x18\x02\x20\x01(\tR\
    \x06second\"8\n\nCoinVolume\x12\x12\n\x04coin\x18\x01\x20\x01(\tR\x04coi\
    n\x12\x16\n\x06amount\x18\x02\x20\x01(\x04R\x06amountb\x06proto3\
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
