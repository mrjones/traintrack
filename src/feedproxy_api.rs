// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct FeedProxyResponse {
    // message fields
    feed: ::protobuf::SingularPtrField<super::gtfs_realtime::FeedMessage>,
    last_good_fetch_timestamp: ::std::option::Option<i64>,
    last_attempted_fetch_timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeedProxyResponse {}

impl FeedProxyResponse {
    pub fn new() -> FeedProxyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeedProxyResponse {
        static mut instance: ::protobuf::lazy::Lazy<FeedProxyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeedProxyResponse,
        };
        unsafe {
            instance.get(|| {
                FeedProxyResponse {
                    feed: ::protobuf::SingularPtrField::none(),
                    last_good_fetch_timestamp: ::std::option::Option::None,
                    last_attempted_fetch_timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .transit_realtime.FeedMessage feed = 1;

    pub fn clear_feed(&mut self) {
        self.feed.clear();
    }

    pub fn has_feed(&self) -> bool {
        self.feed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feed(&mut self, v: super::gtfs_realtime::FeedMessage) {
        self.feed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feed(&mut self) -> &mut super::gtfs_realtime::FeedMessage {
        if self.feed.is_none() {
            self.feed.set_default();
        };
        self.feed.as_mut().unwrap()
    }

    // Take field
    pub fn take_feed(&mut self) -> super::gtfs_realtime::FeedMessage {
        self.feed.take().unwrap_or_else(|| super::gtfs_realtime::FeedMessage::new())
    }

    pub fn get_feed(&self) -> &super::gtfs_realtime::FeedMessage {
        self.feed.as_ref().unwrap_or_else(|| super::gtfs_realtime::FeedMessage::default_instance())
    }

    // optional int64 last_good_fetch_timestamp = 2;

    pub fn clear_last_good_fetch_timestamp(&mut self) {
        self.last_good_fetch_timestamp = ::std::option::Option::None;
    }

    pub fn has_last_good_fetch_timestamp(&self) -> bool {
        self.last_good_fetch_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_good_fetch_timestamp(&mut self, v: i64) {
        self.last_good_fetch_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_last_good_fetch_timestamp(&self) -> i64 {
        self.last_good_fetch_timestamp.unwrap_or(0)
    }

    // optional int64 last_attempted_fetch_timestamp = 3;

    pub fn clear_last_attempted_fetch_timestamp(&mut self) {
        self.last_attempted_fetch_timestamp = ::std::option::Option::None;
    }

    pub fn has_last_attempted_fetch_timestamp(&self) -> bool {
        self.last_attempted_fetch_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_attempted_fetch_timestamp(&mut self, v: i64) {
        self.last_attempted_fetch_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_last_attempted_fetch_timestamp(&self) -> i64 {
        self.last_attempted_fetch_timestamp.unwrap_or(0)
    }
}

impl ::protobuf::Message for FeedProxyResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feed));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.last_good_fetch_timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.last_attempted_fetch_timestamp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.feed {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.last_good_fetch_timestamp {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.last_attempted_fetch_timestamp {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.feed.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.last_good_fetch_timestamp {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.last_attempted_fetch_timestamp {
            try!(os.write_int64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FeedProxyResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FeedProxyResponse {
    fn new() -> FeedProxyResponse {
        FeedProxyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeedProxyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "feed",
                    FeedProxyResponse::has_feed,
                    FeedProxyResponse::get_feed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_good_fetch_timestamp",
                    FeedProxyResponse::has_last_good_fetch_timestamp,
                    FeedProxyResponse::get_last_good_fetch_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_attempted_fetch_timestamp",
                    FeedProxyResponse::has_last_attempted_fetch_timestamp,
                    FeedProxyResponse::get_last_attempted_fetch_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeedProxyResponse>(
                    "FeedProxyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeedProxyResponse {
    fn clear(&mut self) {
        self.clear_feed();
        self.clear_last_good_fetch_timestamp();
        self.clear_last_attempted_fetch_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FeedProxyResponse {
    fn eq(&self, other: &FeedProxyResponse) -> bool {
        self.feed == other.feed &&
        self.last_good_fetch_timestamp == other.last_good_fetch_timestamp &&
        self.last_attempted_fetch_timestamp == other.last_attempted_fetch_timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FeedProxyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x66, 0x65, 0x65, 0x64, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x67, 0x74, 0x66, 0x73, 0x2d, 0x72, 0x65, 0x61, 0x6c,
    0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc6, 0x01, 0x0a, 0x11, 0x46,
    0x65, 0x65, 0x64, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x31, 0x0a, 0x04, 0x66, 0x65, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d,
    0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x5f, 0x72, 0x65, 0x61, 0x6c, 0x74, 0x69, 0x6d,
    0x65, 0x2e, 0x46, 0x65, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x04, 0x66,
    0x65, 0x65, 0x64, 0x12, 0x39, 0x0a, 0x19, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x67, 0x6f, 0x6f, 0x64,
    0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x6c, 0x61, 0x73, 0x74, 0x47, 0x6f, 0x6f, 0x64,
    0x46, 0x65, 0x74, 0x63, 0x68, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x43,
    0x0a, 0x1e, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65, 0x64,
    0x5f, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x1b, 0x6c, 0x61, 0x73, 0x74, 0x41, 0x74, 0x74, 0x65,
    0x6d, 0x70, 0x74, 0x65, 0x64, 0x46, 0x65, 0x74, 0x63, 0x68, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x4a, 0x84, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x02, 0x07, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x08, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x05, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x28, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x06, 0x11, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x06, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07,
    0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x32, 0x33,
];

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
