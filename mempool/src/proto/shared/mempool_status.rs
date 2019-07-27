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
//! Generated file from `mempool_status.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct MempoolAddTransactionStatus {
    // message fields
    pub code: MempoolAddTransactionStatusCode,
    pub message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MempoolAddTransactionStatus {
    fn default() -> &'a MempoolAddTransactionStatus {
        <MempoolAddTransactionStatus as ::protobuf::Message>::default_instance()
    }
}

impl MempoolAddTransactionStatus {
    pub fn new() -> MempoolAddTransactionStatus {
        ::std::default::Default::default()
    }

    // .mempool.MempoolAddTransactionStatusCode code = 1;


    pub fn get_code(&self) -> MempoolAddTransactionStatusCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = MempoolAddTransactionStatusCode::Valid;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: MempoolAddTransactionStatusCode) {
        self.code = v;
    }

    // string message = 2;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for MempoolAddTransactionStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if self.code != MempoolAddTransactionStatusCode::Valid {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != MempoolAddTransactionStatusCode::Valid {
            os.write_enum(1, self.code.value())?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
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

    fn new() -> MempoolAddTransactionStatus {
        MempoolAddTransactionStatus::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MempoolAddTransactionStatusCode>>(
                    "code",
                    |m: &MempoolAddTransactionStatus| { &m.code },
                    |m: &mut MempoolAddTransactionStatus| { &mut m.code },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    |m: &MempoolAddTransactionStatus| { &m.message },
                    |m: &mut MempoolAddTransactionStatus| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MempoolAddTransactionStatus>(
                    "MempoolAddTransactionStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static MempoolAddTransactionStatus {
        static mut instance: ::protobuf::lazy::Lazy<MempoolAddTransactionStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MempoolAddTransactionStatus,
        };
        unsafe {
            instance.get(MempoolAddTransactionStatus::new)
        }
    }
}

impl ::protobuf::Clear for MempoolAddTransactionStatus {
    fn clear(&mut self) {
        self.code = MempoolAddTransactionStatusCode::Valid;
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MempoolAddTransactionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MempoolAddTransactionStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MempoolAddTransactionStatusCode {
    Valid = 0,
    InsufficientBalance = 1,
    InvalidSeqNumber = 2,
    MempoolIsFull = 3,
    TooManyTransactions = 4,
    InvalidUpdate = 5,
}

impl ::protobuf::ProtobufEnum for MempoolAddTransactionStatusCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MempoolAddTransactionStatusCode> {
        match value {
            0 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::Valid),
            1 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::InsufficientBalance),
            2 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::InvalidSeqNumber),
            3 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::MempoolIsFull),
            4 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::TooManyTransactions),
            5 => ::std::option::Option::Some(MempoolAddTransactionStatusCode::InvalidUpdate),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MempoolAddTransactionStatusCode] = &[
            MempoolAddTransactionStatusCode::Valid,
            MempoolAddTransactionStatusCode::InsufficientBalance,
            MempoolAddTransactionStatusCode::InvalidSeqNumber,
            MempoolAddTransactionStatusCode::MempoolIsFull,
            MempoolAddTransactionStatusCode::TooManyTransactions,
            MempoolAddTransactionStatusCode::InvalidUpdate,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MempoolAddTransactionStatusCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MempoolAddTransactionStatusCode {
}

impl ::std::default::Default for MempoolAddTransactionStatusCode {
    fn default() -> Self {
        MempoolAddTransactionStatusCode::Valid
    }
}

impl ::protobuf::reflect::ProtobufValue for MempoolAddTransactionStatusCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14mempool_status.proto\x12\x07mempool\"u\n\x1bMempoolAddTransactionS\
    tatus\x12<\n\x04code\x18\x01\x20\x01(\x0e2(.mempool.MempoolAddTransactio\
    nStatusCodeR\x04code\x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\
    *\x9a\x01\n\x1fMempoolAddTransactionStatusCode\x12\t\n\x05Valid\x10\0\
    \x12\x17\n\x13InsufficientBalance\x10\x01\x12\x14\n\x10InvalidSeqNumber\
    \x10\x02\x12\x11\n\rMempoolIsFull\x10\x03\x12\x17\n\x13TooManyTransactio\
    ns\x10\x04\x12\x11\n\rInvalidUpdate\x10\x05b\x06proto3\
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
