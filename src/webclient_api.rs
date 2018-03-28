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
pub struct DebugInfo {
    // message fields
    processing_time_ms: ::std::option::Option<i64>,
    build_version: ::protobuf::SingularField<::std::string::String>,
    build_timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugInfo {}

impl DebugInfo {
    pub fn new() -> DebugInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugInfo {
        static mut instance: ::protobuf::lazy::Lazy<DebugInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugInfo,
        };
        unsafe {
            instance.get(DebugInfo::new)
        }
    }

    // optional int64 processing_time_ms = 1;

    pub fn clear_processing_time_ms(&mut self) {
        self.processing_time_ms = ::std::option::Option::None;
    }

    pub fn has_processing_time_ms(&self) -> bool {
        self.processing_time_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_processing_time_ms(&mut self, v: i64) {
        self.processing_time_ms = ::std::option::Option::Some(v);
    }

    pub fn get_processing_time_ms(&self) -> i64 {
        self.processing_time_ms.unwrap_or(0)
    }

    fn get_processing_time_ms_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.processing_time_ms
    }

    fn mut_processing_time_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.processing_time_ms
    }

    // optional string build_version = 2;

    pub fn clear_build_version(&mut self) {
        self.build_version.clear();
    }

    pub fn has_build_version(&self) -> bool {
        self.build_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_version(&mut self, v: ::std::string::String) {
        self.build_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_version(&mut self) -> &mut ::std::string::String {
        if self.build_version.is_none() {
            self.build_version.set_default();
        }
        self.build_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_build_version(&mut self) -> ::std::string::String {
        self.build_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_build_version(&self) -> &str {
        match self.build_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_build_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.build_version
    }

    fn mut_build_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.build_version
    }

    // optional int64 build_timestamp = 3;

    pub fn clear_build_timestamp(&mut self) {
        self.build_timestamp = ::std::option::Option::None;
    }

    pub fn has_build_timestamp(&self) -> bool {
        self.build_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_timestamp(&mut self, v: i64) {
        self.build_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_build_timestamp(&self) -> i64 {
        self.build_timestamp.unwrap_or(0)
    }

    fn get_build_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.build_timestamp
    }

    fn mut_build_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.build_timestamp
    }
}

impl ::protobuf::Message for DebugInfo {
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
                    let tmp = is.read_int64()?;
                    self.processing_time_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.build_version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.build_timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.processing_time_ms {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.build_version.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.build_timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.processing_time_ms {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.build_version.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.build_timestamp {
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

impl ::protobuf::MessageStatic for DebugInfo {
    fn new() -> DebugInfo {
        DebugInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "processing_time_ms",
                    DebugInfo::get_processing_time_ms_for_reflect,
                    DebugInfo::mut_processing_time_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "build_version",
                    DebugInfo::get_build_version_for_reflect,
                    DebugInfo::mut_build_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "build_timestamp",
                    DebugInfo::get_build_timestamp_for_reflect,
                    DebugInfo::mut_build_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugInfo>(
                    "DebugInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugInfo {
    fn clear(&mut self) {
        self.clear_processing_time_ms();
        self.clear_build_version();
        self.clear_build_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LineArrival {
    // message fields
    timestamp: ::std::option::Option<i64>,
    trip_id: ::protobuf::SingularField<::std::string::String>,
    headsign: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LineArrival {}

impl LineArrival {
    pub fn new() -> LineArrival {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LineArrival {
        static mut instance: ::protobuf::lazy::Lazy<LineArrival> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LineArrival,
        };
        unsafe {
            instance.get(LineArrival::new)
        }
    }

    // optional int64 timestamp = 1;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }

    // optional string trip_id = 2;

    pub fn clear_trip_id(&mut self) {
        self.trip_id.clear();
    }

    pub fn has_trip_id(&self) -> bool {
        self.trip_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trip_id(&mut self, v: ::std::string::String) {
        self.trip_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_trip_id(&mut self) -> &mut ::std::string::String {
        if self.trip_id.is_none() {
            self.trip_id.set_default();
        }
        self.trip_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_trip_id(&mut self) -> ::std::string::String {
        self.trip_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_trip_id(&self) -> &str {
        match self.trip_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_trip_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.trip_id
    }

    fn mut_trip_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.trip_id
    }

    // optional string headsign = 3;

    pub fn clear_headsign(&mut self) {
        self.headsign.clear();
    }

    pub fn has_headsign(&self) -> bool {
        self.headsign.is_some()
    }

    // Param is passed by value, moved
    pub fn set_headsign(&mut self, v: ::std::string::String) {
        self.headsign = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_headsign(&mut self) -> &mut ::std::string::String {
        if self.headsign.is_none() {
            self.headsign.set_default();
        }
        self.headsign.as_mut().unwrap()
    }

    // Take field
    pub fn take_headsign(&mut self) -> ::std::string::String {
        self.headsign.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_headsign(&self) -> &str {
        match self.headsign.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_headsign_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.headsign
    }

    fn mut_headsign_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.headsign
    }
}

impl ::protobuf::Message for LineArrival {
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
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.trip_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.headsign)?;
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
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.trip_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.headsign.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.trip_id.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.headsign.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for LineArrival {
    fn new() -> LineArrival {
        LineArrival::new()
    }

    fn descriptor_static(_: ::std::option::Option<LineArrival>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    LineArrival::get_timestamp_for_reflect,
                    LineArrival::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "trip_id",
                    LineArrival::get_trip_id_for_reflect,
                    LineArrival::mut_trip_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "headsign",
                    LineArrival::get_headsign_for_reflect,
                    LineArrival::mut_headsign_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LineArrival>(
                    "LineArrival",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LineArrival {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_trip_id();
        self.clear_headsign();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LineArrival {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineArrival {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LineArrivals {
    // message fields
    line: ::protobuf::SingularField<::std::string::String>,
    direction: ::std::option::Option<Direction>,
    line_color_hex: ::protobuf::SingularField<::std::string::String>,
    arrivals: ::protobuf::RepeatedField<LineArrival>,
    debug_info: ::protobuf::SingularPtrField<DebugInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LineArrivals {}

impl LineArrivals {
    pub fn new() -> LineArrivals {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LineArrivals {
        static mut instance: ::protobuf::lazy::Lazy<LineArrivals> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LineArrivals,
        };
        unsafe {
            instance.get(LineArrivals::new)
        }
    }

    // optional string line = 1;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    pub fn has_line(&self) -> bool {
        self.line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::std::string::String) {
        self.line = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut ::std::string::String {
        if self.line.is_none() {
            self.line.set_default();
        }
        self.line.as_mut().unwrap()
    }

    // Take field
    pub fn take_line(&mut self) -> ::std::string::String {
        self.line.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_line(&self) -> &str {
        match self.line.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_line_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.line
    }

    // optional .Direction direction = 2;

    pub fn clear_direction(&mut self) {
        self.direction = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: Direction) {
        self.direction = ::std::option::Option::Some(v);
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.unwrap_or(Direction::UPTOWN)
    }

    fn get_direction_for_reflect(&self) -> &::std::option::Option<Direction> {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut ::std::option::Option<Direction> {
        &mut self.direction
    }

    // optional string line_color_hex = 4;

    pub fn clear_line_color_hex(&mut self) {
        self.line_color_hex.clear();
    }

    pub fn has_line_color_hex(&self) -> bool {
        self.line_color_hex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line_color_hex(&mut self, v: ::std::string::String) {
        self.line_color_hex = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line_color_hex(&mut self) -> &mut ::std::string::String {
        if self.line_color_hex.is_none() {
            self.line_color_hex.set_default();
        }
        self.line_color_hex.as_mut().unwrap()
    }

    // Take field
    pub fn take_line_color_hex(&mut self) -> ::std::string::String {
        self.line_color_hex.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_line_color_hex(&self) -> &str {
        match self.line_color_hex.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_line_color_hex_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.line_color_hex
    }

    fn mut_line_color_hex_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.line_color_hex
    }

    // repeated .LineArrival arrivals = 6;

    pub fn clear_arrivals(&mut self) {
        self.arrivals.clear();
    }

    // Param is passed by value, moved
    pub fn set_arrivals(&mut self, v: ::protobuf::RepeatedField<LineArrival>) {
        self.arrivals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arrivals(&mut self) -> &mut ::protobuf::RepeatedField<LineArrival> {
        &mut self.arrivals
    }

    // Take field
    pub fn take_arrivals(&mut self) -> ::protobuf::RepeatedField<LineArrival> {
        ::std::mem::replace(&mut self.arrivals, ::protobuf::RepeatedField::new())
    }

    pub fn get_arrivals(&self) -> &[LineArrival] {
        &self.arrivals
    }

    fn get_arrivals_for_reflect(&self) -> &::protobuf::RepeatedField<LineArrival> {
        &self.arrivals
    }

    fn mut_arrivals_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LineArrival> {
        &mut self.arrivals
    }

    // optional .DebugInfo debug_info = 5;

    pub fn clear_debug_info(&mut self) {
        self.debug_info.clear();
    }

    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_info(&mut self, v: DebugInfo) {
        self.debug_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_info(&mut self) -> &mut DebugInfo {
        if self.debug_info.is_none() {
            self.debug_info.set_default();
        }
        self.debug_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_info(&mut self) -> DebugInfo {
        self.debug_info.take().unwrap_or_else(|| DebugInfo::new())
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.debug_info.as_ref().unwrap_or_else(|| DebugInfo::default_instance())
    }

    fn get_debug_info_for_reflect(&self) -> &::protobuf::SingularPtrField<DebugInfo> {
        &self.debug_info
    }

    fn mut_debug_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DebugInfo> {
        &mut self.debug_info
    }
}

impl ::protobuf::Message for LineArrivals {
    fn is_initialized(&self) -> bool {
        for v in &self.arrivals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.debug_info {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.direction = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line_color_hex)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.arrivals)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_info)?;
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
        if let Some(ref v) = self.line.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.direction {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.line_color_hex.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        for value in &self.arrivals {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.line.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.direction {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.line_color_hex.as_ref() {
            os.write_string(4, &v)?;
        }
        for v in &self.arrivals {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LineArrivals {
    fn new() -> LineArrivals {
        LineArrivals::new()
    }

    fn descriptor_static(_: ::std::option::Option<LineArrivals>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line",
                    LineArrivals::get_line_for_reflect,
                    LineArrivals::mut_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Direction>>(
                    "direction",
                    LineArrivals::get_direction_for_reflect,
                    LineArrivals::mut_direction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line_color_hex",
                    LineArrivals::get_line_color_hex_for_reflect,
                    LineArrivals::mut_line_color_hex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LineArrival>>(
                    "arrivals",
                    LineArrivals::get_arrivals_for_reflect,
                    LineArrivals::mut_arrivals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugInfo>>(
                    "debug_info",
                    LineArrivals::get_debug_info_for_reflect,
                    LineArrivals::mut_debug_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LineArrivals>(
                    "LineArrivals",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LineArrivals {
    fn clear(&mut self) {
        self.clear_line();
        self.clear_direction();
        self.clear_line_color_hex();
        self.clear_arrivals();
        self.clear_debug_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LineArrivals {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineArrivals {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrainItineraryArrival {
    // message fields
    timestamp: ::std::option::Option<i64>,
    station: ::protobuf::SingularPtrField<Station>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TrainItineraryArrival {}

impl TrainItineraryArrival {
    pub fn new() -> TrainItineraryArrival {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TrainItineraryArrival {
        static mut instance: ::protobuf::lazy::Lazy<TrainItineraryArrival> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TrainItineraryArrival,
        };
        unsafe {
            instance.get(TrainItineraryArrival::new)
        }
    }

    // optional int64 timestamp = 1;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }

    // optional .Station station = 2;

    pub fn clear_station(&mut self) {
        self.station.clear();
    }

    pub fn has_station(&self) -> bool {
        self.station.is_some()
    }

    // Param is passed by value, moved
    pub fn set_station(&mut self, v: Station) {
        self.station = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_station(&mut self) -> &mut Station {
        if self.station.is_none() {
            self.station.set_default();
        }
        self.station.as_mut().unwrap()
    }

    // Take field
    pub fn take_station(&mut self) -> Station {
        self.station.take().unwrap_or_else(|| Station::new())
    }

    pub fn get_station(&self) -> &Station {
        self.station.as_ref().unwrap_or_else(|| Station::default_instance())
    }

    fn get_station_for_reflect(&self) -> &::protobuf::SingularPtrField<Station> {
        &self.station
    }

    fn mut_station_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Station> {
        &mut self.station
    }
}

impl ::protobuf::Message for TrainItineraryArrival {
    fn is_initialized(&self) -> bool {
        for v in &self.station {
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
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.station)?;
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
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.station.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.station.as_ref() {
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

impl ::protobuf::MessageStatic for TrainItineraryArrival {
    fn new() -> TrainItineraryArrival {
        TrainItineraryArrival::new()
    }

    fn descriptor_static(_: ::std::option::Option<TrainItineraryArrival>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    TrainItineraryArrival::get_timestamp_for_reflect,
                    TrainItineraryArrival::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Station>>(
                    "station",
                    TrainItineraryArrival::get_station_for_reflect,
                    TrainItineraryArrival::mut_station_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TrainItineraryArrival>(
                    "TrainItineraryArrival",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TrainItineraryArrival {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_station();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrainItineraryArrival {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainItineraryArrival {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrainItinerary {
    // message fields
    line: ::protobuf::SingularField<::std::string::String>,
    direction: ::std::option::Option<Direction>,
    line_color_hex: ::protobuf::SingularField<::std::string::String>,
    arrival: ::protobuf::RepeatedField<TrainItineraryArrival>,
    data_timestamp: ::std::option::Option<i64>,
    debug_info: ::protobuf::SingularPtrField<DebugInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TrainItinerary {}

impl TrainItinerary {
    pub fn new() -> TrainItinerary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TrainItinerary {
        static mut instance: ::protobuf::lazy::Lazy<TrainItinerary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TrainItinerary,
        };
        unsafe {
            instance.get(TrainItinerary::new)
        }
    }

    // optional string line = 1;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    pub fn has_line(&self) -> bool {
        self.line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::std::string::String) {
        self.line = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut ::std::string::String {
        if self.line.is_none() {
            self.line.set_default();
        }
        self.line.as_mut().unwrap()
    }

    // Take field
    pub fn take_line(&mut self) -> ::std::string::String {
        self.line.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_line(&self) -> &str {
        match self.line.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_line_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.line
    }

    // optional .Direction direction = 2;

    pub fn clear_direction(&mut self) {
        self.direction = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: Direction) {
        self.direction = ::std::option::Option::Some(v);
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.unwrap_or(Direction::UPTOWN)
    }

    fn get_direction_for_reflect(&self) -> &::std::option::Option<Direction> {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut ::std::option::Option<Direction> {
        &mut self.direction
    }

    // optional string line_color_hex = 3;

    pub fn clear_line_color_hex(&mut self) {
        self.line_color_hex.clear();
    }

    pub fn has_line_color_hex(&self) -> bool {
        self.line_color_hex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line_color_hex(&mut self, v: ::std::string::String) {
        self.line_color_hex = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line_color_hex(&mut self) -> &mut ::std::string::String {
        if self.line_color_hex.is_none() {
            self.line_color_hex.set_default();
        }
        self.line_color_hex.as_mut().unwrap()
    }

    // Take field
    pub fn take_line_color_hex(&mut self) -> ::std::string::String {
        self.line_color_hex.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_line_color_hex(&self) -> &str {
        match self.line_color_hex.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_line_color_hex_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.line_color_hex
    }

    fn mut_line_color_hex_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.line_color_hex
    }

    // repeated .TrainItineraryArrival arrival = 4;

    pub fn clear_arrival(&mut self) {
        self.arrival.clear();
    }

    // Param is passed by value, moved
    pub fn set_arrival(&mut self, v: ::protobuf::RepeatedField<TrainItineraryArrival>) {
        self.arrival = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arrival(&mut self) -> &mut ::protobuf::RepeatedField<TrainItineraryArrival> {
        &mut self.arrival
    }

    // Take field
    pub fn take_arrival(&mut self) -> ::protobuf::RepeatedField<TrainItineraryArrival> {
        ::std::mem::replace(&mut self.arrival, ::protobuf::RepeatedField::new())
    }

    pub fn get_arrival(&self) -> &[TrainItineraryArrival] {
        &self.arrival
    }

    fn get_arrival_for_reflect(&self) -> &::protobuf::RepeatedField<TrainItineraryArrival> {
        &self.arrival
    }

    fn mut_arrival_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TrainItineraryArrival> {
        &mut self.arrival
    }

    // optional int64 data_timestamp = 6;

    pub fn clear_data_timestamp(&mut self) {
        self.data_timestamp = ::std::option::Option::None;
    }

    pub fn has_data_timestamp(&self) -> bool {
        self.data_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_timestamp(&mut self, v: i64) {
        self.data_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_data_timestamp(&self) -> i64 {
        self.data_timestamp.unwrap_or(0)
    }

    fn get_data_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.data_timestamp
    }

    fn mut_data_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.data_timestamp
    }

    // optional .DebugInfo debug_info = 5;

    pub fn clear_debug_info(&mut self) {
        self.debug_info.clear();
    }

    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_info(&mut self, v: DebugInfo) {
        self.debug_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_info(&mut self) -> &mut DebugInfo {
        if self.debug_info.is_none() {
            self.debug_info.set_default();
        }
        self.debug_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_info(&mut self) -> DebugInfo {
        self.debug_info.take().unwrap_or_else(|| DebugInfo::new())
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.debug_info.as_ref().unwrap_or_else(|| DebugInfo::default_instance())
    }

    fn get_debug_info_for_reflect(&self) -> &::protobuf::SingularPtrField<DebugInfo> {
        &self.debug_info
    }

    fn mut_debug_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DebugInfo> {
        &mut self.debug_info
    }
}

impl ::protobuf::Message for TrainItinerary {
    fn is_initialized(&self) -> bool {
        for v in &self.arrival {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.debug_info {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.direction = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line_color_hex)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.arrival)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.data_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_info)?;
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
        if let Some(ref v) = self.line.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.direction {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.line_color_hex.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.arrival {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.data_timestamp {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.debug_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.line.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.direction {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.line_color_hex.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.arrival {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.data_timestamp {
            os.write_int64(6, v)?;
        }
        if let Some(ref v) = self.debug_info.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TrainItinerary {
    fn new() -> TrainItinerary {
        TrainItinerary::new()
    }

    fn descriptor_static(_: ::std::option::Option<TrainItinerary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line",
                    TrainItinerary::get_line_for_reflect,
                    TrainItinerary::mut_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Direction>>(
                    "direction",
                    TrainItinerary::get_direction_for_reflect,
                    TrainItinerary::mut_direction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line_color_hex",
                    TrainItinerary::get_line_color_hex_for_reflect,
                    TrainItinerary::mut_line_color_hex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TrainItineraryArrival>>(
                    "arrival",
                    TrainItinerary::get_arrival_for_reflect,
                    TrainItinerary::mut_arrival_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "data_timestamp",
                    TrainItinerary::get_data_timestamp_for_reflect,
                    TrainItinerary::mut_data_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugInfo>>(
                    "debug_info",
                    TrainItinerary::get_debug_info_for_reflect,
                    TrainItinerary::mut_debug_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TrainItinerary>(
                    "TrainItinerary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TrainItinerary {
    fn clear(&mut self) {
        self.clear_line();
        self.clear_direction();
        self.clear_line_color_hex();
        self.clear_arrival();
        self.clear_data_timestamp();
        self.clear_debug_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrainItinerary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainItinerary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StationStatus {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularField<::std::string::String>,
    line: ::protobuf::RepeatedField<LineArrivals>,
    data_timestamp: ::std::option::Option<i64>,
    debug_info: ::protobuf::SingularPtrField<DebugInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StationStatus {}

impl StationStatus {
    pub fn new() -> StationStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StationStatus {
        static mut instance: ::protobuf::lazy::Lazy<StationStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StationStatus,
        };
        unsafe {
            instance.get(StationStatus::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string id = 5;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }

    // repeated .LineArrivals line = 2;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::protobuf::RepeatedField<LineArrivals>) {
        self.line = v;
    }

    // Mutable pointer to the field.
    pub fn mut_line(&mut self) -> &mut ::protobuf::RepeatedField<LineArrivals> {
        &mut self.line
    }

    // Take field
    pub fn take_line(&mut self) -> ::protobuf::RepeatedField<LineArrivals> {
        ::std::mem::replace(&mut self.line, ::protobuf::RepeatedField::new())
    }

    pub fn get_line(&self) -> &[LineArrivals] {
        &self.line
    }

    fn get_line_for_reflect(&self) -> &::protobuf::RepeatedField<LineArrivals> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LineArrivals> {
        &mut self.line
    }

    // optional int64 data_timestamp = 3;

    pub fn clear_data_timestamp(&mut self) {
        self.data_timestamp = ::std::option::Option::None;
    }

    pub fn has_data_timestamp(&self) -> bool {
        self.data_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_timestamp(&mut self, v: i64) {
        self.data_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_data_timestamp(&self) -> i64 {
        self.data_timestamp.unwrap_or(0)
    }

    fn get_data_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.data_timestamp
    }

    fn mut_data_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.data_timestamp
    }

    // optional .DebugInfo debug_info = 4;

    pub fn clear_debug_info(&mut self) {
        self.debug_info.clear();
    }

    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_info(&mut self, v: DebugInfo) {
        self.debug_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_info(&mut self) -> &mut DebugInfo {
        if self.debug_info.is_none() {
            self.debug_info.set_default();
        }
        self.debug_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_info(&mut self) -> DebugInfo {
        self.debug_info.take().unwrap_or_else(|| DebugInfo::new())
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.debug_info.as_ref().unwrap_or_else(|| DebugInfo::default_instance())
    }

    fn get_debug_info_for_reflect(&self) -> &::protobuf::SingularPtrField<DebugInfo> {
        &self.debug_info
    }

    fn mut_debug_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DebugInfo> {
        &mut self.debug_info
    }
}

impl ::protobuf::Message for StationStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.line {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.debug_info {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.line)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.data_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_info)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        for value in &self.line {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.data_timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.debug_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.id.as_ref() {
            os.write_string(5, &v)?;
        }
        for v in &self.line {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.data_timestamp {
            os.write_int64(3, v)?;
        }
        if let Some(ref v) = self.debug_info.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StationStatus {
    fn new() -> StationStatus {
        StationStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<StationStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    StationStatus::get_name_for_reflect,
                    StationStatus::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    StationStatus::get_id_for_reflect,
                    StationStatus::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LineArrivals>>(
                    "line",
                    StationStatus::get_line_for_reflect,
                    StationStatus::mut_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "data_timestamp",
                    StationStatus::get_data_timestamp_for_reflect,
                    StationStatus::mut_data_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugInfo>>(
                    "debug_info",
                    StationStatus::get_debug_info_for_reflect,
                    StationStatus::mut_debug_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StationStatus>(
                    "StationStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StationStatus {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_line();
        self.clear_data_timestamp();
        self.clear_debug_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StationStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Station {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    lines: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Station {}

impl Station {
    pub fn new() -> Station {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Station {
        static mut instance: ::protobuf::lazy::Lazy<Station> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Station,
        };
        unsafe {
            instance.get(Station::new)
        }
    }

    // optional string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated string lines = 3;

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    // Param is passed by value, moved
    pub fn set_lines(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.lines = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lines(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.lines
    }

    // Take field
    pub fn take_lines(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.lines, ::protobuf::RepeatedField::new())
    }

    pub fn get_lines(&self) -> &[::std::string::String] {
        &self.lines
    }

    fn get_lines_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.lines
    }

    fn mut_lines_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.lines
    }
}

impl ::protobuf::Message for Station {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.lines)?;
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
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.lines {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.lines {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for Station {
    fn new() -> Station {
        Station::new()
    }

    fn descriptor_static(_: ::std::option::Option<Station>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Station::get_id_for_reflect,
                    Station::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Station::get_name_for_reflect,
                    Station::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lines",
                    Station::get_lines_for_reflect,
                    Station::mut_lines_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Station>(
                    "Station",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Station {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_lines();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Station {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Station {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StationList {
    // message fields
    station: ::protobuf::RepeatedField<Station>,
    debug_info: ::protobuf::SingularPtrField<DebugInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StationList {}

impl StationList {
    pub fn new() -> StationList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StationList {
        static mut instance: ::protobuf::lazy::Lazy<StationList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StationList,
        };
        unsafe {
            instance.get(StationList::new)
        }
    }

    // repeated .Station station = 1;

    pub fn clear_station(&mut self) {
        self.station.clear();
    }

    // Param is passed by value, moved
    pub fn set_station(&mut self, v: ::protobuf::RepeatedField<Station>) {
        self.station = v;
    }

    // Mutable pointer to the field.
    pub fn mut_station(&mut self) -> &mut ::protobuf::RepeatedField<Station> {
        &mut self.station
    }

    // Take field
    pub fn take_station(&mut self) -> ::protobuf::RepeatedField<Station> {
        ::std::mem::replace(&mut self.station, ::protobuf::RepeatedField::new())
    }

    pub fn get_station(&self) -> &[Station] {
        &self.station
    }

    fn get_station_for_reflect(&self) -> &::protobuf::RepeatedField<Station> {
        &self.station
    }

    fn mut_station_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Station> {
        &mut self.station
    }

    // optional .DebugInfo debug_info = 5;

    pub fn clear_debug_info(&mut self) {
        self.debug_info.clear();
    }

    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_info(&mut self, v: DebugInfo) {
        self.debug_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_info(&mut self) -> &mut DebugInfo {
        if self.debug_info.is_none() {
            self.debug_info.set_default();
        }
        self.debug_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_info(&mut self) -> DebugInfo {
        self.debug_info.take().unwrap_or_else(|| DebugInfo::new())
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.debug_info.as_ref().unwrap_or_else(|| DebugInfo::default_instance())
    }

    fn get_debug_info_for_reflect(&self) -> &::protobuf::SingularPtrField<DebugInfo> {
        &self.debug_info
    }

    fn mut_debug_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DebugInfo> {
        &mut self.debug_info
    }
}

impl ::protobuf::Message for StationList {
    fn is_initialized(&self) -> bool {
        for v in &self.station {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.debug_info {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.station)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_info)?;
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
        for value in &self.station {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.station {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StationList {
    fn new() -> StationList {
        StationList::new()
    }

    fn descriptor_static(_: ::std::option::Option<StationList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Station>>(
                    "station",
                    StationList::get_station_for_reflect,
                    StationList::mut_station_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugInfo>>(
                    "debug_info",
                    StationList::get_debug_info_for_reflect,
                    StationList::mut_debug_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StationList>(
                    "StationList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StationList {
    fn clear(&mut self) {
        self.clear_station();
        self.clear_debug_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StationList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StationList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Line {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    color_hex: ::protobuf::SingularField<::std::string::String>,
    active: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Line {}

impl Line {
    pub fn new() -> Line {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Line {
        static mut instance: ::protobuf::lazy::Lazy<Line> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Line,
        };
        unsafe {
            instance.get(Line::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string color_hex = 2;

    pub fn clear_color_hex(&mut self) {
        self.color_hex.clear();
    }

    pub fn has_color_hex(&self) -> bool {
        self.color_hex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color_hex(&mut self, v: ::std::string::String) {
        self.color_hex = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color_hex(&mut self) -> &mut ::std::string::String {
        if self.color_hex.is_none() {
            self.color_hex.set_default();
        }
        self.color_hex.as_mut().unwrap()
    }

    // Take field
    pub fn take_color_hex(&mut self) -> ::std::string::String {
        self.color_hex.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_color_hex(&self) -> &str {
        match self.color_hex.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_color_hex_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.color_hex
    }

    fn mut_color_hex_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.color_hex
    }

    // optional bool active = 3;

    pub fn clear_active(&mut self) {
        self.active = ::std::option::Option::None;
    }

    pub fn has_active(&self) -> bool {
        self.active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: bool) {
        self.active = ::std::option::Option::Some(v);
    }

    pub fn get_active(&self) -> bool {
        self.active.unwrap_or(false)
    }

    fn get_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.active
    }
}

impl ::protobuf::Message for Line {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.color_hex)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.active = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.color_hex.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.active {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.color_hex.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.active {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for Line {
    fn new() -> Line {
        Line::new()
    }

    fn descriptor_static(_: ::std::option::Option<Line>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Line::get_name_for_reflect,
                    Line::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "color_hex",
                    Line::get_color_hex_for_reflect,
                    Line::mut_color_hex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "active",
                    Line::get_active_for_reflect,
                    Line::mut_active_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Line>(
                    "Line",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Line {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_color_hex();
        self.clear_active();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Line {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Line {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LineList {
    // message fields
    line: ::protobuf::RepeatedField<Line>,
    debug_info: ::protobuf::SingularPtrField<DebugInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LineList {}

impl LineList {
    pub fn new() -> LineList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LineList {
        static mut instance: ::protobuf::lazy::Lazy<LineList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LineList,
        };
        unsafe {
            instance.get(LineList::new)
        }
    }

    // repeated .Line line = 1;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::protobuf::RepeatedField<Line>) {
        self.line = v;
    }

    // Mutable pointer to the field.
    pub fn mut_line(&mut self) -> &mut ::protobuf::RepeatedField<Line> {
        &mut self.line
    }

    // Take field
    pub fn take_line(&mut self) -> ::protobuf::RepeatedField<Line> {
        ::std::mem::replace(&mut self.line, ::protobuf::RepeatedField::new())
    }

    pub fn get_line(&self) -> &[Line] {
        &self.line
    }

    fn get_line_for_reflect(&self) -> &::protobuf::RepeatedField<Line> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Line> {
        &mut self.line
    }

    // optional .DebugInfo debug_info = 4;

    pub fn clear_debug_info(&mut self) {
        self.debug_info.clear();
    }

    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_info(&mut self, v: DebugInfo) {
        self.debug_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_info(&mut self) -> &mut DebugInfo {
        if self.debug_info.is_none() {
            self.debug_info.set_default();
        }
        self.debug_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_info(&mut self) -> DebugInfo {
        self.debug_info.take().unwrap_or_else(|| DebugInfo::new())
    }

    pub fn get_debug_info(&self) -> &DebugInfo {
        self.debug_info.as_ref().unwrap_or_else(|| DebugInfo::default_instance())
    }

    fn get_debug_info_for_reflect(&self) -> &::protobuf::SingularPtrField<DebugInfo> {
        &self.debug_info
    }

    fn mut_debug_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DebugInfo> {
        &mut self.debug_info
    }
}

impl ::protobuf::Message for LineList {
    fn is_initialized(&self) -> bool {
        for v in &self.line {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.debug_info {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.line)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_info)?;
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
        for value in &self.line {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.line {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.debug_info.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LineList {
    fn new() -> LineList {
        LineList::new()
    }

    fn descriptor_static(_: ::std::option::Option<LineList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Line>>(
                    "line",
                    LineList::get_line_for_reflect,
                    LineList::mut_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugInfo>>(
                    "debug_info",
                    LineList::get_debug_info_for_reflect,
                    LineList::mut_debug_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LineList>(
                    "LineList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LineList {
    fn clear(&mut self) {
        self.clear_line();
        self.clear_debug_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LineList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Direction {
    UPTOWN = 0,
    DOWNTOWN = 1,
}

impl ::protobuf::ProtobufEnum for Direction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Direction> {
        match value {
            0 => ::std::option::Option::Some(Direction::UPTOWN),
            1 => ::std::option::Option::Some(Direction::DOWNTOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Direction] = &[
            Direction::UPTOWN,
            Direction::DOWNTOWN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Direction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Direction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Direction {
}

impl ::protobuf::reflect::ProtobufValue for Direction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13webclient_api.proto\"\x87\x01\n\tDebugInfo\x12,\n\x12processing_ti\
    me_ms\x18\x01\x20\x01(\x03R\x10processingTimeMs\x12#\n\rbuild_version\
    \x18\x02\x20\x01(\tR\x0cbuildVersion\x12'\n\x0fbuild_timestamp\x18\x03\
    \x20\x01(\x03R\x0ebuildTimestamp\"`\n\x0bLineArrival\x12\x1c\n\ttimestam\
    p\x18\x01\x20\x01(\x03R\ttimestamp\x12\x17\n\x07trip_id\x18\x02\x20\x01(\
    \tR\x06tripId\x12\x1a\n\x08headsign\x18\x03\x20\x01(\tR\x08headsign\"\
    \xcd\x01\n\x0cLineArrivals\x12\x12\n\x04line\x18\x01\x20\x01(\tR\x04line\
    \x12(\n\tdirection\x18\x02\x20\x01(\x0e2\n.DirectionR\tdirection\x12$\n\
    \x0eline_color_hex\x18\x04\x20\x01(\tR\x0clineColorHex\x12(\n\x08arrival\
    s\x18\x06\x20\x03(\x0b2\x0c.LineArrivalR\x08arrivals\x12)\n\ndebug_info\
    \x18\x05\x20\x01(\x0b2\n.DebugInfoR\tdebugInfoJ\x04\x08\x03\x10\x04\"Y\n\
    \x15TrainItineraryArrival\x12\x1c\n\ttimestamp\x18\x01\x20\x01(\x03R\tti\
    mestamp\x12\"\n\x07station\x18\x02\x20\x01(\x0b2\x08.StationR\x07station\
    \"\xf8\x01\n\x0eTrainItinerary\x12\x12\n\x04line\x18\x01\x20\x01(\tR\x04\
    line\x12(\n\tdirection\x18\x02\x20\x01(\x0e2\n.DirectionR\tdirection\x12\
    $\n\x0eline_color_hex\x18\x03\x20\x01(\tR\x0clineColorHex\x120\n\x07arri\
    val\x18\x04\x20\x03(\x0b2\x16.TrainItineraryArrivalR\x07arrival\x12%\n\
    \x0edata_timestamp\x18\x06\x20\x01(\x03R\rdataTimestamp\x12)\n\ndebug_in\
    fo\x18\x05\x20\x01(\x0b2\n.DebugInfoR\tdebugInfo\"\xa8\x01\n\rStationSta\
    tus\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\
    \x05\x20\x01(\tR\x02id\x12!\n\x04line\x18\x02\x20\x03(\x0b2\r.LineArriva\
    lsR\x04line\x12%\n\x0edata_timestamp\x18\x03\x20\x01(\x03R\rdataTimestam\
    p\x12)\n\ndebug_info\x18\x04\x20\x01(\x0b2\n.DebugInfoR\tdebugInfo\"C\n\
    \x07Station\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05lines\x18\x03\x20\x03(\tR\x05l\
    ines\"\\\n\x0bStationList\x12\"\n\x07station\x18\x01\x20\x03(\x0b2\x08.S\
    tationR\x07station\x12)\n\ndebug_info\x18\x05\x20\x01(\x0b2\n.DebugInfoR\
    \tdebugInfo\"O\n\x04Line\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x1b\n\tcolor_hex\x18\x02\x20\x01(\tR\x08colorHex\x12\x16\n\x06activ\
    e\x18\x03\x20\x01(\x08R\x06active\"P\n\x08LineList\x12\x19\n\x04line\x18\
    \x01\x20\x03(\x0b2\x05.LineR\x04line\x12)\n\ndebug_info\x18\x04\x20\x01(\
    \x0b2\n.DebugInfoR\tdebugInfo*%\n\tDirection\x12\n\n\x06UPTOWN\x10\0\x12\
    \x0c\n\x08DOWNTOWN\x10\x01J\xcb\x15\n\x06\x12\x04\0\0L\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x05\
    \0\x01\x12\x03\x02\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\x03\x02\r\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x02\x08\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x03\x0b\x0c\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x04\x02\x0f\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x02\n\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03\x04\r\x0e\n\n\n\x02\x04\0\x12\x04\x07\0\x0b\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x07\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\x02(\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x08\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\x11#\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08&'\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\t\x02$\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\t\x02\n\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\t\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\t\x12\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\t\"#\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\n\x02%\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\n\
    \x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\n\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\n\x11\x20\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\n#$\
    \n\n\n\x02\x04\x01\x12\x04\r\0\x11\x01\n\n\n\x03\x04\x01\x01\x12\x03\r\
    \x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0e\x02\x1f\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x0e\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0e\x11\x1a\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x0e\x1d\x1e\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x0f\x02\x1e\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x0f\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0f\x0b\x11\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x0f\x12\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x0f\x1c\x1d\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x10\x02\x1f\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x10\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x10\x12\x1a\
    \n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x10\x1d\x1e\n\n\n\x02\x04\x02\
    \x12\x04\x13\0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\x03\x13\x08\x14\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03\x14\x02\x1b\n\x0c\n\x05\x04\x02\x02\0\x04\x12\
    \x03\x14\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x14\x0b\x11\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x14\x12\x16\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x14\x19\x1a\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x15\x02#\n\x0c\
    \n\x05\x04\x02\x02\x01\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x06\x12\x03\x15\x0b\x14\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x15\x15\
    \x1e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x15!\"\n\x0b\n\x04\x04\x02\
    \x02\x02\x12\x03\x16\x02%\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x16\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x16\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\x02\x01\x12\x03\x16\x12\x20\n\x0c\n\x05\x04\x02\x02\x02\x03\
    \x12\x03\x16#$\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x17\x02$\n\x0c\n\x05\
    \x04\x02\x02\x03\x04\x12\x03\x17\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\
    \x12\x03\x17\x0b\x16\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x17\x17\x1f\
    \n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\x17\"#\n\x0b\n\x04\x04\x02\x02\
    \x04\x12\x03\x19\x02$\n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03\x19\x02\n\
    \n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03\x19\x0b\x14\n\x0c\n\x05\x04\x02\
    \x02\x04\x01\x12\x03\x19\x15\x1f\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\
    \x19\"#\n\n\n\x03\x04\x02\t\x12\x03\x1b\x0b\r\n\x0b\n\x04\x04\x02\t\0\
    \x12\x03\x1b\x0b\x0c\n\x0c\n\x05\x04\x02\t\0\x01\x12\x03\x1b\x0b\x0c\n\
    \x0c\n\x05\x04\x02\t\0\x02\x12\x03\x1b\x0b\x0c\n\n\n\x02\x04\x03\x12\x04\
    \x1e\0!\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1e\x08\x1d\n\x0b\n\x04\x04\
    \x03\x02\0\x12\x03\x1f\x02\x1f\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x1f\
    \x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1f\x0b\x10\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03\x1f\x11\x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\
    \x1f\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x20\x02\x1f\n\x0c\n\x05\
    \x04\x03\x02\x01\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x06\
    \x12\x03\x20\x0b\x12\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x20\x13\x1a\
    \n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x20\x1d\x1e\n\n\n\x02\x04\x04\
    \x12\x04#\0+\x01\n\n\n\x03\x04\x04\x01\x12\x03#\x08\x16\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03$\x02\x1b\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03$\x02\n\
    \n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03$\x0b\x11\n\x0c\n\x05\x04\x04\x02\
    \0\x01\x12\x03$\x12\x16\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03$\x19\x1a\n\
    \x0b\n\x04\x04\x04\x02\x01\x12\x03%\x02#\n\x0c\n\x05\x04\x04\x02\x01\x04\
    \x12\x03%\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03%\x0b\x14\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x03%\x15\x1e\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x03%!\"\n\x0b\n\x04\x04\x04\x02\x02\x12\x03&\x02%\n\x0c\n\x05\
    \x04\x04\x02\x02\x04\x12\x03&\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\
    \x03&\x0b\x11\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03&\x12\x20\n\x0c\n\
    \x05\x04\x04\x02\x02\x03\x12\x03&#$\n\x0b\n\x04\x04\x04\x02\x03\x12\x03'\
    \x02-\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\x04\
    \x02\x03\x06\x12\x03'\x0b\x20\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03'!(\
    \n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03'+,\n\x0b\n\x04\x04\x04\x02\x04\
    \x12\x03(\x02$\n\x0c\n\x05\x04\x04\x02\x04\x04\x12\x03(\x02\n\n\x0c\n\
    \x05\x04\x04\x02\x04\x05\x12\x03(\x0b\x10\n\x0c\n\x05\x04\x04\x02\x04\
    \x01\x12\x03(\x11\x1f\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03(\"#\n\x0b\
    \n\x04\x04\x04\x02\x05\x12\x03*\x02$\n\x0c\n\x05\x04\x04\x02\x05\x04\x12\
    \x03*\x02\n\n\x0c\n\x05\x04\x04\x02\x05\x06\x12\x03*\x0b\x14\n\x0c\n\x05\
    \x04\x04\x02\x05\x01\x12\x03*\x15\x1f\n\x0c\n\x05\x04\x04\x02\x05\x03\
    \x12\x03*\"#\n\n\n\x02\x04\x05\x12\x04-\04\x01\n\n\n\x03\x04\x05\x01\x12\
    \x03-\x08\x15\n\x0b\n\x04\x04\x05\x02\0\x12\x03.\x02\x1b\n\x0c\n\x05\x04\
    \x05\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03.\x0b\
    \x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03.\x12\x16\n\x0c\n\x05\x04\x05\
    \x02\0\x03\x12\x03.\x19\x1a\n\x0b\n\x04\x04\x05\x02\x01\x12\x03/\x02\x19\
    \n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x01\x05\x12\x03/\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03/\x12\
    \x14\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03/\x17\x18\n\x0b\n\x04\x04\
    \x05\x02\x02\x12\x030\x02!\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x030\x02\
    \n\n\x0c\n\x05\x04\x05\x02\x02\x06\x12\x030\x0b\x17\n\x0c\n\x05\x04\x05\
    \x02\x02\x01\x12\x030\x18\x1c\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x030\
    \x1f\x20\n\x0b\n\x04\x04\x05\x02\x03\x12\x031\x02$\n\x0c\n\x05\x04\x05\
    \x02\x03\x04\x12\x031\x02\n\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x031\x0b\
    \x10\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x031\x11\x1f\n\x0c\n\x05\x04\
    \x05\x02\x03\x03\x12\x031\"#\n\x0b\n\x04\x04\x05\x02\x04\x12\x033\x02$\n\
    \x0c\n\x05\x04\x05\x02\x04\x04\x12\x033\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x04\x06\x12\x033\x0b\x14\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\x033\x15\
    \x1f\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x033\"#\n\n\n\x02\x04\x06\x12\
    \x046\0:\x01\n\n\n\x03\x04\x06\x01\x12\x036\x08\x0f\n\x0b\n\x04\x04\x06\
    \x02\0\x12\x037\x02\x19\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x037\x02\n\n\
    \x0c\n\x05\x04\x06\x02\0\x05\x12\x037\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x037\x12\x14\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x037\x17\x18\n\
    \x0b\n\x04\x04\x06\x02\x01\x12\x038\x02\x1b\n\x0c\n\x05\x04\x06\x02\x01\
    \x04\x12\x038\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\x038\x0b\x11\n\
    \x0c\n\x05\x04\x06\x02\x01\x01\x12\x038\x12\x16\n\x0c\n\x05\x04\x06\x02\
    \x01\x03\x12\x038\x19\x1a\n\x0b\n\x04\x04\x06\x02\x02\x12\x039\x02\x1c\n\
    \x0c\n\x05\x04\x06\x02\x02\x04\x12\x039\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x02\x05\x12\x039\x0b\x11\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x039\x12\
    \x17\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x039\x1a\x1b\n\n\n\x02\x04\x07\
    \x12\x04<\0@\x01\n\n\n\x03\x04\x07\x01\x12\x03<\x08\x13\n\x0b\n\x04\x04\
    \x07\x02\0\x12\x03=\x02\x1f\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03=\x02\n\
    \n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03=\x0b\x12\n\x0c\n\x05\x04\x07\x02\
    \0\x01\x12\x03=\x13\x1a\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03=\x1d\x1e\n\
    \x0b\n\x04\x04\x07\x02\x01\x12\x03?\x02$\n\x0c\n\x05\x04\x07\x02\x01\x04\
    \x12\x03?\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x06\x12\x03?\x0b\x14\n\x0c\n\
    \x05\x04\x07\x02\x01\x01\x12\x03?\x15\x1f\n\x0c\n\x05\x04\x07\x02\x01\
    \x03\x12\x03?\"#\n\n\n\x02\x04\x08\x12\x04B\0F\x01\n\n\n\x03\x04\x08\x01\
    \x12\x03B\x08\x0c\n\x0b\n\x04\x04\x08\x02\0\x12\x03C\x02\x1b\n\x0c\n\x05\
    \x04\x08\x02\0\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03C\
    \x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03C\x12\x16\n\x0c\n\x05\x04\
    \x08\x02\0\x03\x12\x03C\x19\x1a\n\x0b\n\x04\x04\x08\x02\x01\x12\x03D\x02\
    \x20\n\x0c\n\x05\x04\x08\x02\x01\x04\x12\x03D\x02\n\n\x0c\n\x05\x04\x08\
    \x02\x01\x05\x12\x03D\x0b\x11\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03D\
    \x12\x1b\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03D\x1e\x1f\n\x0b\n\x04\
    \x04\x08\x02\x02\x12\x03E\x02\x1b\n\x0c\n\x05\x04\x08\x02\x02\x04\x12\
    \x03E\x02\n\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\x03E\x0b\x0f\n\x0c\n\x05\
    \x04\x08\x02\x02\x01\x12\x03E\x10\x16\n\x0c\n\x05\x04\x08\x02\x02\x03\
    \x12\x03E\x19\x1a\n\n\n\x02\x04\t\x12\x04H\0L\x01\n\n\n\x03\x04\t\x01\
    \x12\x03H\x08\x10\n\x0b\n\x04\x04\t\x02\0\x12\x03I\x02\x19\n\x0c\n\x05\
    \x04\t\x02\0\x04\x12\x03I\x02\n\n\x0c\n\x05\x04\t\x02\0\x06\x12\x03I\x0b\
    \x0f\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03I\x10\x14\n\x0c\n\x05\x04\t\x02\
    \0\x03\x12\x03I\x17\x18\n\x0b\n\x04\x04\t\x02\x01\x12\x03K\x02$\n\x0c\n\
    \x05\x04\t\x02\x01\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\t\x02\x01\x06\x12\
    \x03K\x0b\x14\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03K\x15\x1f\n\x0c\n\x05\
    \x04\t\x02\x01\x03\x12\x03K\"#\
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
