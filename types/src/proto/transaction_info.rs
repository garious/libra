// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `transaction_info.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct TransactionInfo {
    // message fields
    pub signed_transaction_hash: ::std::vec::Vec<u8>,
    pub state_root_hash: ::std::vec::Vec<u8>,
    pub event_root_hash: ::std::vec::Vec<u8>,
    pub gas_used: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TransactionInfo {
    fn default() -> &'a TransactionInfo {
        <TransactionInfo as ::protobuf::Message>::default_instance()
    }
}

impl TransactionInfo {
    pub fn new() -> TransactionInfo {
        ::std::default::Default::default()
    }

    // bytes signed_transaction_hash = 1;


    pub fn get_signed_transaction_hash(&self) -> &[u8] {
        &self.signed_transaction_hash
    }
    pub fn clear_signed_transaction_hash(&mut self) {
        self.signed_transaction_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_signed_transaction_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.signed_transaction_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signed_transaction_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signed_transaction_hash
    }

    // Take field
    pub fn take_signed_transaction_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signed_transaction_hash, ::std::vec::Vec::new())
    }

    // bytes state_root_hash = 2;


    pub fn get_state_root_hash(&self) -> &[u8] {
        &self.state_root_hash
    }
    pub fn clear_state_root_hash(&mut self) {
        self.state_root_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_root_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_root_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root_hash
    }

    // Take field
    pub fn take_state_root_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.state_root_hash, ::std::vec::Vec::new())
    }

    // bytes event_root_hash = 3;


    pub fn get_event_root_hash(&self) -> &[u8] {
        &self.event_root_hash
    }
    pub fn clear_event_root_hash(&mut self) {
        self.event_root_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_root_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.event_root_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_root_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.event_root_hash
    }

    // Take field
    pub fn take_event_root_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.event_root_hash, ::std::vec::Vec::new())
    }

    // uint64 gas_used = 4;


    pub fn get_gas_used(&self) -> u64 {
        self.gas_used
    }
    pub fn clear_gas_used(&mut self) {
        self.gas_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: u64) {
        self.gas_used = v;
    }
}

impl ::protobuf::Message for TransactionInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signed_transaction_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.event_root_hash)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gas_used = tmp;
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
        if !self.signed_transaction_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.signed_transaction_hash);
        }
        if !self.state_root_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.state_root_hash);
        }
        if !self.event_root_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.event_root_hash);
        }
        if self.gas_used != 0 {
            my_size += ::protobuf::rt::value_size(4, self.gas_used, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.signed_transaction_hash.is_empty() {
            os.write_bytes(1, &self.signed_transaction_hash)?;
        }
        if !self.state_root_hash.is_empty() {
            os.write_bytes(2, &self.state_root_hash)?;
        }
        if !self.event_root_hash.is_empty() {
            os.write_bytes(3, &self.event_root_hash)?;
        }
        if self.gas_used != 0 {
            os.write_uint64(4, self.gas_used)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TransactionInfo {
        TransactionInfo::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signed_transaction_hash",
                    |m: &TransactionInfo| { &m.signed_transaction_hash },
                    |m: &mut TransactionInfo| { &mut m.signed_transaction_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root_hash",
                    |m: &TransactionInfo| { &m.state_root_hash },
                    |m: &mut TransactionInfo| { &mut m.state_root_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "event_root_hash",
                    |m: &TransactionInfo| { &m.event_root_hash },
                    |m: &mut TransactionInfo| { &mut m.event_root_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gas_used",
                    |m: &TransactionInfo| { &m.gas_used },
                    |m: &mut TransactionInfo| { &mut m.gas_used },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionInfo>(
                    "TransactionInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TransactionInfo {
        static mut instance: ::protobuf::lazy::Lazy<TransactionInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionInfo,
        };
        unsafe {
            instance.get(TransactionInfo::new)
        }
    }
}

impl ::protobuf::Clear for TransactionInfo {
    fn clear(&mut self) {
        self.signed_transaction_hash.clear();
        self.state_root_hash.clear();
        self.event_root_hash.clear();
        self.gas_used = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16transaction_info.proto\x12\x05types\"\xb4\x01\n\x0fTransactionInfo\
    \x126\n\x17signed_transaction_hash\x18\x01\x20\x01(\x0cR\x15signedTransa\
    ctionHash\x12&\n\x0fstate_root_hash\x18\x02\x20\x01(\x0cR\rstateRootHash\
    \x12&\n\x0fevent_root_hash\x18\x03\x20\x01(\x0cR\reventRootHash\x12\x19\
    \n\x08gas_used\x18\x04\x20\x01(\x04R\x07gasUsedb\x06proto3\
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
