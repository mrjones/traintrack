// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
pub struct FeedProxyResponse {
    // message fields
    feed: ::protobuf::SingularPtrField<super::gtfs_realtime::FeedMessage>,
    last_good_fetch_timestamp: ::std::option::Option<i64>,
    last_attempted_fetch_timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(FeedProxyResponse::new)
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
        }
        self.feed.as_mut().unwrap()
    }

    // Take field
    pub fn take_feed(&mut self) -> super::gtfs_realtime::FeedMessage {
        self.feed.take().unwrap_or_else(|| super::gtfs_realtime::FeedMessage::new())
    }

    pub fn get_feed(&self) -> &super::gtfs_realtime::FeedMessage {
        self.feed.as_ref().unwrap_or_else(|| super::gtfs_realtime::FeedMessage::default_instance())
    }

    fn get_feed_for_reflect(&self) -> &::protobuf::SingularPtrField<super::gtfs_realtime::FeedMessage> {
        &self.feed
    }

    fn mut_feed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::gtfs_realtime::FeedMessage> {
        &mut self.feed
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

    fn get_last_good_fetch_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_good_fetch_timestamp
    }

    fn mut_last_good_fetch_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_good_fetch_timestamp
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

    fn get_last_attempted_fetch_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_attempted_fetch_timestamp
    }

    fn mut_last_attempted_fetch_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_attempted_fetch_timestamp
    }
}

impl ::protobuf::Message for FeedProxyResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.feed {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feed)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_good_fetch_timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_attempted_fetch_timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.feed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.last_good_fetch_timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_attempted_fetch_timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.feed.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.last_good_fetch_timestamp {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.last_attempted_fetch_timestamp {
            os.write_int64(3, v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::gtfs_realtime::FeedMessage>>(
                    "feed",
                    FeedProxyResponse::get_feed_for_reflect,
                    FeedProxyResponse::mut_feed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_good_fetch_timestamp",
                    FeedProxyResponse::get_last_good_fetch_timestamp_for_reflect,
                    FeedProxyResponse::mut_last_good_fetch_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_attempted_fetch_timestamp",
                    FeedProxyResponse::get_last_attempted_fetch_timestamp_for_reflect,
                    FeedProxyResponse::mut_last_attempted_fetch_timestamp_for_reflect,
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

impl ::std::fmt::Debug for FeedProxyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeedProxyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    
feedproxy_api.protogtfs-realtime.proto\"Æ
Feed\
    ProxyResponse1
feed (2.transit_realtime.\
    FeedMessageRfeed9
last_good_fetch_timestamp \
    (RlastGoodFetchTimestampC
last_attempted_fetch_timestam\
    p (RlastAttemptedFetchTimestampJ„
\
      

  
	
 \
    


  


 \
    

  1
\
    
  


  \
    '

  (,

\
      /0

 /
\
    
 


 \
    

 *\
    

 -.

 \
    4

 

\
    
 

 \
    /

 23\
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
