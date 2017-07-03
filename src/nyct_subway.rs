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
pub struct TripReplacementPeriod {
    // message fields
    route_id: ::protobuf::SingularField<::std::string::String>,
    replacement_period: ::protobuf::SingularPtrField<super::gtfs_realtime::TimeRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TripReplacementPeriod {}

impl TripReplacementPeriod {
    pub fn new() -> TripReplacementPeriod {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TripReplacementPeriod {
        static mut instance: ::protobuf::lazy::Lazy<TripReplacementPeriod> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TripReplacementPeriod,
        };
        unsafe {
            instance.get(TripReplacementPeriod::new)
        }
    }

    // optional string route_id = 1;

    pub fn clear_route_id(&mut self) {
        self.route_id.clear();
    }

    pub fn has_route_id(&self) -> bool {
        self.route_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_route_id(&mut self, v: ::std::string::String) {
        self.route_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_route_id(&mut self) -> &mut ::std::string::String {
        if self.route_id.is_none() {
            self.route_id.set_default();
        }
        self.route_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_route_id(&mut self) -> ::std::string::String {
        self.route_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_route_id(&self) -> &str {
        match self.route_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_route_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.route_id
    }

    fn mut_route_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.route_id
    }

    // optional .transit_realtime.TimeRange replacement_period = 2;

    pub fn clear_replacement_period(&mut self) {
        self.replacement_period.clear();
    }

    pub fn has_replacement_period(&self) -> bool {
        self.replacement_period.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replacement_period(&mut self, v: super::gtfs_realtime::TimeRange) {
        self.replacement_period = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replacement_period(&mut self) -> &mut super::gtfs_realtime::TimeRange {
        if self.replacement_period.is_none() {
            self.replacement_period.set_default();
        }
        self.replacement_period.as_mut().unwrap()
    }

    // Take field
    pub fn take_replacement_period(&mut self) -> super::gtfs_realtime::TimeRange {
        self.replacement_period.take().unwrap_or_else(|| super::gtfs_realtime::TimeRange::new())
    }

    pub fn get_replacement_period(&self) -> &super::gtfs_realtime::TimeRange {
        self.replacement_period.as_ref().unwrap_or_else(|| super::gtfs_realtime::TimeRange::default_instance())
    }

    fn get_replacement_period_for_reflect(&self) -> &::protobuf::SingularPtrField<super::gtfs_realtime::TimeRange> {
        &self.replacement_period
    }

    fn mut_replacement_period_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::gtfs_realtime::TimeRange> {
        &mut self.replacement_period
    }
}

impl ::protobuf::Message for TripReplacementPeriod {
    fn is_initialized(&self) -> bool {
        for v in &self.replacement_period {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.route_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.replacement_period)?;
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
        if let Some(ref v) = self.route_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.replacement_period.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.route_id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.replacement_period.as_ref() {
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

impl ::protobuf::MessageStatic for TripReplacementPeriod {
    fn new() -> TripReplacementPeriod {
        TripReplacementPeriod::new()
    }

    fn descriptor_static(_: ::std::option::Option<TripReplacementPeriod>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "route_id",
                    TripReplacementPeriod::get_route_id_for_reflect,
                    TripReplacementPeriod::mut_route_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::gtfs_realtime::TimeRange>>(
                    "replacement_period",
                    TripReplacementPeriod::get_replacement_period_for_reflect,
                    TripReplacementPeriod::mut_replacement_period_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TripReplacementPeriod>(
                    "TripReplacementPeriod",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TripReplacementPeriod {
    fn clear(&mut self) {
        self.clear_route_id();
        self.clear_replacement_period();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TripReplacementPeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TripReplacementPeriod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NyctFeedHeader {
    // message fields
    nyct_subway_version: ::protobuf::SingularField<::std::string::String>,
    trip_replacement_period: ::protobuf::RepeatedField<TripReplacementPeriod>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NyctFeedHeader {}

impl NyctFeedHeader {
    pub fn new() -> NyctFeedHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NyctFeedHeader {
        static mut instance: ::protobuf::lazy::Lazy<NyctFeedHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NyctFeedHeader,
        };
        unsafe {
            instance.get(NyctFeedHeader::new)
        }
    }

    // required string nyct_subway_version = 1;

    pub fn clear_nyct_subway_version(&mut self) {
        self.nyct_subway_version.clear();
    }

    pub fn has_nyct_subway_version(&self) -> bool {
        self.nyct_subway_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nyct_subway_version(&mut self, v: ::std::string::String) {
        self.nyct_subway_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nyct_subway_version(&mut self) -> &mut ::std::string::String {
        if self.nyct_subway_version.is_none() {
            self.nyct_subway_version.set_default();
        }
        self.nyct_subway_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_nyct_subway_version(&mut self) -> ::std::string::String {
        self.nyct_subway_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nyct_subway_version(&self) -> &str {
        match self.nyct_subway_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_nyct_subway_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.nyct_subway_version
    }

    fn mut_nyct_subway_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.nyct_subway_version
    }

    // repeated .TripReplacementPeriod trip_replacement_period = 2;

    pub fn clear_trip_replacement_period(&mut self) {
        self.trip_replacement_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_trip_replacement_period(&mut self, v: ::protobuf::RepeatedField<TripReplacementPeriod>) {
        self.trip_replacement_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_trip_replacement_period(&mut self) -> &mut ::protobuf::RepeatedField<TripReplacementPeriod> {
        &mut self.trip_replacement_period
    }

    // Take field
    pub fn take_trip_replacement_period(&mut self) -> ::protobuf::RepeatedField<TripReplacementPeriod> {
        ::std::mem::replace(&mut self.trip_replacement_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_trip_replacement_period(&self) -> &[TripReplacementPeriod] {
        &self.trip_replacement_period
    }

    fn get_trip_replacement_period_for_reflect(&self) -> &::protobuf::RepeatedField<TripReplacementPeriod> {
        &self.trip_replacement_period
    }

    fn mut_trip_replacement_period_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TripReplacementPeriod> {
        &mut self.trip_replacement_period
    }
}

impl ::protobuf::Message for NyctFeedHeader {
    fn is_initialized(&self) -> bool {
        if self.nyct_subway_version.is_none() {
            return false;
        }
        for v in &self.trip_replacement_period {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nyct_subway_version)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.trip_replacement_period)?;
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
        if let Some(ref v) = self.nyct_subway_version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.trip_replacement_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.nyct_subway_version.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.trip_replacement_period {
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

impl ::protobuf::MessageStatic for NyctFeedHeader {
    fn new() -> NyctFeedHeader {
        NyctFeedHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<NyctFeedHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nyct_subway_version",
                    NyctFeedHeader::get_nyct_subway_version_for_reflect,
                    NyctFeedHeader::mut_nyct_subway_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TripReplacementPeriod>>(
                    "trip_replacement_period",
                    NyctFeedHeader::get_trip_replacement_period_for_reflect,
                    NyctFeedHeader::mut_trip_replacement_period_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NyctFeedHeader>(
                    "NyctFeedHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NyctFeedHeader {
    fn clear(&mut self) {
        self.clear_nyct_subway_version();
        self.clear_trip_replacement_period();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NyctFeedHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctFeedHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NyctTripDescriptor {
    // message fields
    train_id: ::protobuf::SingularField<::std::string::String>,
    is_assigned: ::std::option::Option<bool>,
    direction: ::std::option::Option<NyctTripDescriptor_Direction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NyctTripDescriptor {}

impl NyctTripDescriptor {
    pub fn new() -> NyctTripDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NyctTripDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<NyctTripDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NyctTripDescriptor,
        };
        unsafe {
            instance.get(NyctTripDescriptor::new)
        }
    }

    // optional string train_id = 1;

    pub fn clear_train_id(&mut self) {
        self.train_id.clear();
    }

    pub fn has_train_id(&self) -> bool {
        self.train_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_train_id(&mut self, v: ::std::string::String) {
        self.train_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_train_id(&mut self) -> &mut ::std::string::String {
        if self.train_id.is_none() {
            self.train_id.set_default();
        }
        self.train_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_train_id(&mut self) -> ::std::string::String {
        self.train_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_train_id(&self) -> &str {
        match self.train_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_train_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.train_id
    }

    fn mut_train_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.train_id
    }

    // optional bool is_assigned = 2;

    pub fn clear_is_assigned(&mut self) {
        self.is_assigned = ::std::option::Option::None;
    }

    pub fn has_is_assigned(&self) -> bool {
        self.is_assigned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_assigned(&mut self, v: bool) {
        self.is_assigned = ::std::option::Option::Some(v);
    }

    pub fn get_is_assigned(&self) -> bool {
        self.is_assigned.unwrap_or(false)
    }

    fn get_is_assigned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_assigned
    }

    fn mut_is_assigned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_assigned
    }

    // optional .NyctTripDescriptor.Direction direction = 3;

    pub fn clear_direction(&mut self) {
        self.direction = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: NyctTripDescriptor_Direction) {
        self.direction = ::std::option::Option::Some(v);
    }

    pub fn get_direction(&self) -> NyctTripDescriptor_Direction {
        self.direction.unwrap_or(NyctTripDescriptor_Direction::NORTH)
    }

    fn get_direction_for_reflect(&self) -> &::std::option::Option<NyctTripDescriptor_Direction> {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut ::std::option::Option<NyctTripDescriptor_Direction> {
        &mut self.direction
    }
}

impl ::protobuf::Message for NyctTripDescriptor {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.train_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_assigned = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.direction = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.train_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.is_assigned {
            my_size += 2;
        }
        if let Some(v) = self.direction {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.train_id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.is_assigned {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.direction {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for NyctTripDescriptor {
    fn new() -> NyctTripDescriptor {
        NyctTripDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<NyctTripDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "train_id",
                    NyctTripDescriptor::get_train_id_for_reflect,
                    NyctTripDescriptor::mut_train_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_assigned",
                    NyctTripDescriptor::get_is_assigned_for_reflect,
                    NyctTripDescriptor::mut_is_assigned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NyctTripDescriptor_Direction>>(
                    "direction",
                    NyctTripDescriptor::get_direction_for_reflect,
                    NyctTripDescriptor::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NyctTripDescriptor>(
                    "NyctTripDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NyctTripDescriptor {
    fn clear(&mut self) {
        self.clear_train_id();
        self.clear_is_assigned();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NyctTripDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctTripDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NyctTripDescriptor_Direction {
    NORTH = 1,
    EAST = 2,
    SOUTH = 3,
    WEST = 4,
}

impl ::protobuf::ProtobufEnum for NyctTripDescriptor_Direction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NyctTripDescriptor_Direction> {
        match value {
            1 => ::std::option::Option::Some(NyctTripDescriptor_Direction::NORTH),
            2 => ::std::option::Option::Some(NyctTripDescriptor_Direction::EAST),
            3 => ::std::option::Option::Some(NyctTripDescriptor_Direction::SOUTH),
            4 => ::std::option::Option::Some(NyctTripDescriptor_Direction::WEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NyctTripDescriptor_Direction] = &[
            NyctTripDescriptor_Direction::NORTH,
            NyctTripDescriptor_Direction::EAST,
            NyctTripDescriptor_Direction::SOUTH,
            NyctTripDescriptor_Direction::WEST,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NyctTripDescriptor_Direction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NyctTripDescriptor_Direction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NyctTripDescriptor_Direction {
}

impl ::protobuf::reflect::ProtobufValue for NyctTripDescriptor_Direction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NyctStopTimeUpdate {
    // message fields
    scheduled_track: ::protobuf::SingularField<::std::string::String>,
    actual_track: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NyctStopTimeUpdate {}

impl NyctStopTimeUpdate {
    pub fn new() -> NyctStopTimeUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NyctStopTimeUpdate {
        static mut instance: ::protobuf::lazy::Lazy<NyctStopTimeUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NyctStopTimeUpdate,
        };
        unsafe {
            instance.get(NyctStopTimeUpdate::new)
        }
    }

    // optional string scheduled_track = 1;

    pub fn clear_scheduled_track(&mut self) {
        self.scheduled_track.clear();
    }

    pub fn has_scheduled_track(&self) -> bool {
        self.scheduled_track.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scheduled_track(&mut self, v: ::std::string::String) {
        self.scheduled_track = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scheduled_track(&mut self) -> &mut ::std::string::String {
        if self.scheduled_track.is_none() {
            self.scheduled_track.set_default();
        }
        self.scheduled_track.as_mut().unwrap()
    }

    // Take field
    pub fn take_scheduled_track(&mut self) -> ::std::string::String {
        self.scheduled_track.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_scheduled_track(&self) -> &str {
        match self.scheduled_track.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_scheduled_track_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.scheduled_track
    }

    fn mut_scheduled_track_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.scheduled_track
    }

    // optional string actual_track = 2;

    pub fn clear_actual_track(&mut self) {
        self.actual_track.clear();
    }

    pub fn has_actual_track(&self) -> bool {
        self.actual_track.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actual_track(&mut self, v: ::std::string::String) {
        self.actual_track = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_actual_track(&mut self) -> &mut ::std::string::String {
        if self.actual_track.is_none() {
            self.actual_track.set_default();
        }
        self.actual_track.as_mut().unwrap()
    }

    // Take field
    pub fn take_actual_track(&mut self) -> ::std::string::String {
        self.actual_track.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_actual_track(&self) -> &str {
        match self.actual_track.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_actual_track_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.actual_track
    }

    fn mut_actual_track_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.actual_track
    }
}

impl ::protobuf::Message for NyctStopTimeUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.scheduled_track)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.actual_track)?;
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
        if let Some(ref v) = self.scheduled_track.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.actual_track.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.scheduled_track.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.actual_track.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for NyctStopTimeUpdate {
    fn new() -> NyctStopTimeUpdate {
        NyctStopTimeUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<NyctStopTimeUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "scheduled_track",
                    NyctStopTimeUpdate::get_scheduled_track_for_reflect,
                    NyctStopTimeUpdate::mut_scheduled_track_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "actual_track",
                    NyctStopTimeUpdate::get_actual_track_for_reflect,
                    NyctStopTimeUpdate::mut_actual_track_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NyctStopTimeUpdate>(
                    "NyctStopTimeUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NyctStopTimeUpdate {
    fn clear(&mut self) {
        self.clear_scheduled_track();
        self.clear_actual_track();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NyctStopTimeUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctStopTimeUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const nyct_feed_header: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::FeedHeader, ::protobuf::types::ProtobufTypeMessage<super::NyctFeedHeader>> = ::protobuf::ext::ExtFieldOptional { field_number: 1001, phantom: ::std::marker::PhantomData };

    pub const nyct_trip_descriptor: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::TripDescriptor, ::protobuf::types::ProtobufTypeMessage<super::NyctTripDescriptor>> = ::protobuf::ext::ExtFieldOptional { field_number: 1001, phantom: ::std::marker::PhantomData };

    pub const nyct_stop_time_update: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::TripUpdate_StopTimeUpdate, ::protobuf::types::ProtobufTypeMessage<super::NyctStopTimeUpdate>> = ::protobuf::ext::ExtFieldOptional { field_number: 1001, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11nyct-subway.proto\x1a\x13gtfs-realtime.proto\"~\n\x15TripReplaceme\
    ntPeriod\x12\x19\n\x08route_id\x18\x01\x20\x01(\tR\x07routeId\x12J\n\x12\
    replacement_period\x18\x02\x20\x01(\x0b2\x1b.transit_realtime.TimeRangeR\
    \x11replacementPeriod\"\x90\x01\n\x0eNyctFeedHeader\x12.\n\x13nyct_subwa\
    y_version\x18\x01\x20\x02(\tR\x11nyctSubwayVersion\x12N\n\x17trip_replac\
    ement_period\x18\x02\x20\x03(\x0b2\x16.TripReplacementPeriodR\x15tripRep\
    lacementPeriod\"\xc4\x01\n\x12NyctTripDescriptor\x12\x19\n\x08train_id\
    \x18\x01\x20\x01(\tR\x07trainId\x12\x1f\n\x0bis_assigned\x18\x02\x20\x01\
    (\x08R\nisAssigned\x12;\n\tdirection\x18\x03\x20\x01(\x0e2\x1d.NyctTripD\
    escriptor.DirectionR\tdirection\"5\n\tDirection\x12\t\n\x05NORTH\x10\x01\
    \x12\x08\n\x04EAST\x10\x02\x12\t\n\x05SOUTH\x10\x03\x12\x08\n\x04WEST\
    \x10\x04\"`\n\x12NyctStopTimeUpdate\x12'\n\x0fscheduled_track\x18\x01\
    \x20\x01(\tR\x0escheduledTrack\x12!\n\x0cactual_track\x18\x02\x20\x01(\t\
    R\x0bactualTrack:X\n\x10nyct_feed_header\x18\xe9\x07\x20\x01(\x0b2\x0f.N\
    yctFeedHeader\x12\x1c.transit_realtime.FeedHeaderR\x0enyctFeedHeader:h\n\
    \x14nyct_trip_descriptor\x18\xe9\x07\x20\x01(\x0b2\x13.NyctTripDescripto\
    r\x12\x20.transit_realtime.TripDescriptorR\x12nyctTripDescriptor:t\n\x15\
    nyct_stop_time_update\x18\xe9\x07\x20\x01(\x0b2\x13.NyctStopTimeUpdate\
    \x12+.transit_realtime.TripUpdate.StopTimeUpdateR\x12nyctStopTimeUpdateB\
    \x1d\n\x1bcom.google.transit.realtimeJ\xe5*\n\x07\x12\x05\x03\0\x86\x01\
    \x01\nG\n\x01\x0c\x12\x03\x03\0\x12\x1a=\r\n\x20NYCT\x20Subway\x20extens\
    ions\x20for\x20the\x20GTFS-realtime\x20protocol.\r\n\r\n\n\x08\n\x01\x08\
    \x12\x03\x04\04\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x04\04\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x04\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x04\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x04\x163\n\t\n\x02\x03\0\x12\x03\
    \x06\x07\x1c\n\n\n\x02\x04\0\x12\x04\x08\0\x0e\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x08\x08\x1d\n8\n\x04\x04\0\x02\0\x12\x03\n\x02\x1f\x1a+\x20The\
    \x20replacement\x20period\x20is\x20for\x20this\x20route\r\n\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x12\x1a\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\n\x1d\x1e\nx\n\x04\x04\0\x02\x01\x12\x03\r\x02=\x1ak\
    \x20The\x20start\x20time\x20is\x20omitted,\x20the\x20end\x20time\x20is\
    \x20currently\x20now\x20+\x2030\x20minutes\x20for\r\n\x20all\x20routes\
    \x20of\x20the\x20A\x20division\r\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \r\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\r\x0b%\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\r&8\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\r;<\n9\n\
    \x02\x04\x01\x12\x04\x11\0\x1e\x01\x1a-\x20NYCT\x20Subway\x20extensions\
    \x20for\x20the\x20feed\x20header\r\n\n\n\n\x03\x04\x01\x01\x12\x03\x11\
    \x08\x16\nR\n\x04\x04\x01\x02\0\x12\x03\x14\x02*\x1aE\x20Version\x20of\
    \x20the\x20NYCT\x20Subway\x20extensions\r\n\x20The\x20current\x20version\
    \x20is\x201.0\r\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x14\x02\n\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03\x14\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x14\x12%\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x14()\n\x82\
    \x04\n\x04\x04\x01\x02\x01\x12\x03\x1d\x02=\x1a\xf4\x03\x20For\x20the\
    \x20NYCT\x20Subway,\x20the\x20GTFS-realtime\x20feed\x20replaces\x20any\
    \x20scheduled\r\n\x20trip\x20within\x20the\x20trip_replacement_period.\r\
    \n\x20This\x20feed\x20is\x20a\x20full\x20dataset,\x20it\x20contains\x20a\
    ll\x20trips\x20starting\r\n\x20in\x20the\x20trip_replacement_period.\x20\
    If\x20a\x20trip\x20from\x20the\x20static\x20GTFS\x20is\x20not\r\n\x20fou\
    nd\x20in\x20the\x20GTFS-realtime\x20feed,\x20it\x20should\x20be\x20consi\
    dered\x20as\x20cancelled.\r\n\x20The\x20replacement\x20period\x20can\x20\
    be\x20different\x20for\x20each\x20route,\x20so\x20here\x20is\r\n\x20a\
    \x20list\x20of\x20the\x20routes\x20where\x20the\x20trips\x20in\x20the\
    \x20feed\x20replace\x20all\r\n\x20scheduled\x20trips\x20within\x20the\
    \x20replacement\x20period.\r\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\
    \x1d\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x1d\x0b\x20\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x1d!8\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03\x1d;<\n\t\n\x01\x07\x12\x04\x20\0\"\x01\n\t\n\x02\x07\0\x12\x03\
    !\x011\n\n\n\x03\x07\0\x02\x12\x03\x20\x07\"\n\n\n\x03\x07\0\x04\x12\x03\
    !\x01\t\n\n\n\x03\x07\0\x06\x12\x03!\n\x18\n\n\n\x03\x07\0\x01\x12\x03!\
    \x19)\n\n\n\x03\x07\0\x03\x12\x03!,0\n=\n\x02\x04\x02\x12\x04%\0[\x01\
    \x1a1\x20NYCT\x20Subway\x20extensions\x20for\x20the\x20trip\x20descripto\
    r\r\n\n\n\n\x03\x04\x02\x01\x12\x03%\x08\x1a\n\x9c\x08\n\x04\x04\x02\x02\
    \0\x12\x03;\x02\x1f\x1a\x8e\x08\x20The\x20nyct_train_id\x20is\x20meant\
    \x20for\x20internal\x20use\x20only.\x20It\x20provides\x20an\r\n\x20easy\
    \x20way\x20to\x20associated\x20GTFS-realtime\x20trip\x20identifiers\x20w\
    ith\x20NYCT\x20rail\r\n\x20operations\x20identifier\r\n\r\n\x20The\x20AT\
    S\x20office\x20system\x20assigns\x20unique\x20train\x20identification\
    \x20(Train\x20ID)\x20to\r\n\x20each\x20train\x20operating\x20within\x20o\
    r\x20ready\x20to\x20enter\x20the\x20mainline\x20of\x20the\r\n\x20monitor\
    ed\x20territory.\x20An\x20example\x20of\x20this\x20is\x2006\x200123+\x20\
    PEL/BBR\x20and\x20is\x20decoded\r\n\x20as\x20follows:\r\n\r\n\x20The\x20\
    first\x20character\x20represents\x20the\x20trip\x20type\x20designator.\
    \x200\x20identifies\x20a\r\n\x20scheduled\x20revenue\x20trip.\x20Other\
    \x20revenue\x20trip\x20values\x20that\x20are\x20a\x20result\x20of\x20a\r\
    \n\x20change\x20to\x20the\x20base\x20schedule\x20include;\x20[=\x20rerou\
    te],\x20[/\x20skip\x20stop],\x20[$\x20turn\r\n\x20train]\x20also\x20know\
    n\x20as\x20shortly\x20lined\x20service.\r\n\r\n\x20The\x20second\x20char\
    acter\x206\x20represents\x20the\x20trip\x20line\x20i.e.\x20number\x206\
    \x20train\x20The\r\n\x20third\x20set\x20of\x20characters\x20identify\x20\
    the\x20decoded\x20origin\x20time.\x20The\x20last\r\n\x20character\x20may\
    \x20be\x20blank\x20\"on\x20the\x20whole\x20minute\"\x20or\x20+\x20\"30\
    \x20seconds\"\r\n\r\n\x20Note:\x20Origin\x20times\x20will\x20not\x20chan\
    ge\x20when\x20there\x20is\x20a\x20trip\x20type\x20change.\x20\x20This\r\
    \n\x20is\x20followed\x20by\x20a\x20three\x20character\x20\"Origin\x20Loc\
    ation\"\x20/\x20\"Destination\r\n\x20Location\"\r\n\n\x0c\n\x05\x04\x02\
    \x02\0\x04\x12\x03;\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03;\x0b\x11\
    \n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03;\x12\x1a\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03;\x1d\x1e\n\xea\x05\n\x04\x04\x02\x02\x01\x12\x03J\x02\x20\
    \x1a\xdc\x05\x20This\x20trip\x20has\x20been\x20assigned\x20to\x20a\x20ph\
    ysical\x20train.\x20If\x20true,\x20this\x20trip\x20is\r\n\x20already\x20\
    underway\x20or\x20most\x20likely\x20will\x20depart\x20shortly.\r\n\r\n\
    \x20Train\x20Assignment\x20is\x20a\x20function\x20of\x20the\x20Automatic\
    \x20Train\x20Supervision\x20(ATS)\r\n\x20office\x20system\x20used\x20by\
    \x20NYCT\x20Rail\x20Operations\x20to\x20monitor\x20and\x20track\x20train\
    \r\n\x20movements.\x20ATS\x20provides\x20the\x20ability\x20to\x20\"assig\
    n\"\x20the\x20nyct_train_id\r\n\x20attribute\x20when\x20a\x20physical\
    \x20train\x20is\x20at\x20its\x20origin\x20terminal.\x20These\x20assigned\
    \r\n\x20trips\x20have\x20the\x20is_assigned\x20field\x20set\x20in\x20the\
    \x20TripDescriptor.\r\n\r\n\x20When\x20a\x20train\x20is\x20at\x20a\x20te\
    rminal\x20but\x20has\x20not\x20been\x20given\x20a\x20work\x20program\x20\
    it\x20is\r\n\x20declared\x20unassigned\x20and\x20is\x20tagged\x20as\x20s\
    uch.\x20Unassigned\x20trains\x20can\x20be\x20moved\r\n\x20to\x20a\x20sto\
    rage\x20location\x20or\x20assigned\x20a\x20nyct_train_id\x20when\x20a\
    \x20determination\x20for\r\n\x20service\x20is\x20made.\r\n\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03J\x0b\x0f\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03J\x10\x1b\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03J\x1e\x1f\n3\n\x04\x04\x02\x04\0\x12\x04\
    M\x02R\x03\x1a%\x20The\x20direction\x20the\x20train\x20is\x20moving.\r\n\
    \n\x0c\n\x05\x04\x02\x04\0\x01\x12\x03M\x07\x10\n\r\n\x06\x04\x02\x04\0\
    \x02\0\x12\x03N\x04\x0e\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03N\x04\
    \t\n\x0e\n\x07\x04\x02\x04\0\x02\0\x02\x12\x03N\x0c\r\n\r\n\x06\x04\x02\
    \x04\0\x02\x01\x12\x03O\x04\r\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x01\x12\
    \x03O\x04\x08\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03O\x0b\x0c\n\r\
    \n\x06\x04\x02\x04\0\x02\x02\x12\x03P\x04\x0e\n\x0e\n\x07\x04\x02\x04\0\
    \x02\x02\x01\x12\x03P\x04\t\n\x0e\n\x07\x04\x02\x04\0\x02\x02\x02\x12\
    \x03P\x0c\r\n\r\n\x06\x04\x02\x04\0\x02\x03\x12\x03Q\x04\r\n\x0e\n\x07\
    \x04\x02\x04\0\x02\x03\x01\x12\x03Q\x04\x08\n\x0e\n\x07\x04\x02\x04\0\
    \x02\x03\x02\x12\x03Q\x0b\x0c\n\x9a\x02\n\x04\x04\x02\x02\x02\x12\x03Z\
    \x02#\x1a\x8c\x02\x20Uptown\x20and\x20Bronx-bound\x20trains\x20are\x20mo\
    ving\x20NORTH.\r\n\x20Times\x20Square\x20Shuttle\x20to\x20Grand\x20Centr\
    al\x20is\x20also\x20northbound.\r\n\r\n\x20Downtown\x20and\x20Brooklyn-b\
    ound\x20trains\x20are\x20moving\x20SOUTH.\r\n\x20Times\x20Square\x20Shut\
    tle\x20to\x20Times\x20Square\x20is\x20also\x20southbound.\r\n\r\n\x20EAS\
    T\x20and\x20WEST\x20are\x20not\x20used\x20currently.\r\n\n\x0c\n\x05\x04\
    \x02\x02\x02\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03Z\
    \x0b\x14\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03Z\x15\x1e\n\x0c\n\x05\
    \x04\x02\x02\x02\x03\x12\x03Z!\"\n\t\n\x01\x07\x12\x04]\0_\x01\n\t\n\x02\
    \x07\x01\x12\x03^\x02:\n\n\n\x03\x07\x01\x02\x12\x03]\x07&\n\n\n\x03\x07\
    \x01\x04\x12\x03^\x02\n\n\n\n\x03\x07\x01\x06\x12\x03^\x0b\x1d\n\n\n\x03\
    \x07\x01\x01\x12\x03^\x1e2\n\n\n\x03\x07\x01\x03\x12\x03^59\n?\n\x02\x04\
    \x03\x12\x05b\0\x82\x01\x01\x1a2\x20NYCT\x20Subway\x20extensions\x20for\
    \x20the\x20stop\x20time\x20update\r\n\n\n\n\x03\x04\x03\x01\x12\x03b\x08\
    \x1a\n\xa8\x03\n\x04\x04\x03\x02\0\x12\x03r\x02&\x1a\x9a\x03\x20Provides\
    \x20the\x20planned\x20station\x20arrival\x20track.\x20The\x20following\
    \x20is\x20the\x20Manhattan\r\n\x20track\x20configurations:\r\n\x201:\x20\
    southbound\x20local\r\n\x202:\x20southbound\x20express\r\n\x203:\x20nort\
    hbound\x20express\r\n\x204:\x20northbound\x20local\r\n\r\n\x20In\x20the\
    \x20Bronx\x20(except\x20Dyre\x20Ave\x20line)\r\n\x20M:\x20bi-directional\
    \x20express\x20(in\x20the\x20AM\x20express\x20to\x20Manhattan,\x20in\x20\
    the\x20PM\r\n\x20express\x20away).\r\n\r\n\x20The\x20Dyre\x20Ave\x20line\
    \x20is\x20configured:\r\n\x201:\x20southbound\r\n\x202:\x20northbound\r\
    \n\x203:\x20bi-directional\r\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03r\
    \x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03r\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03r\x12!\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03r$%\n\
    \xe2\x05\n\x04\x04\x03\x02\x01\x12\x04\x81\x01\x02#\x1a\xd3\x05\x20This\
    \x20is\x20the\x20actual\x20track\x20that\x20the\x20train\x20is\x20operat\
    ing\x20on\x20and\x20can\x20be\x20used\x20to\r\n\x20determine\x20if\x20a\
    \x20train\x20is\x20operating\x20according\x20to\x20its\x20current\x20sch\
    edule\r\n\x20(plan).\r\n\r\n\x20The\x20actual\x20track\x20is\x20known\
    \x20only\x20shortly\x20before\x20the\x20train\x20reaches\x20a\x20station\
    ,\r\n\x20typically\x20not\x20before\x20it\x20leaves\x20the\x20previous\
    \x20station.\x20Therefore,\x20the\x20NYCT\r\n\x20feed\x20sets\x20this\
    \x20field\x20only\x20for\x20the\x20first\x20station\x20of\x20the\x20rema\
    ining\x20trip.\r\n\r\n\x20Different\x20actual\x20and\x20scheduled\x20tra\
    ck\x20is\x20the\x20result\x20of\x20manually\x20rerouting\x20a\r\n\x20tra\
    in\x20off\x20it\x20scheduled\x20path.\x20\x20When\x20this\x20occurs,\x20\
    prediction\x20data\x20may\x20become\r\n\x20unreliable\x20since\x20the\
    \x20train\x20is\x20no\x20longer\x20operating\x20in\x20accordance\x20to\
    \x20its\r\n\x20schedule.\x20\x20The\x20rules\x20engine\x20for\x20the\x20\
    'countdown'\x20clocks\x20will\x20remove\x20this\r\n\x20train\x20from\x20\
    all\x20schedule\x20stations.\r\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\
    \x81\x01\x02\n\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\x81\x01\x0b\x11\n\r\
    \n\x05\x04\x03\x02\x01\x01\x12\x04\x81\x01\x12\x1e\n\r\n\x05\x04\x03\x02\
    \x01\x03\x12\x04\x81\x01!\"\n\x0b\n\x01\x07\x12\x06\x84\x01\0\x86\x01\
    \x01\n\n\n\x02\x07\x02\x12\x04\x85\x01\x02;\n\x0b\n\x03\x07\x02\x02\x12\
    \x04\x84\x01\x071\n\x0b\n\x03\x07\x02\x04\x12\x04\x85\x01\x02\n\n\x0b\n\
    \x03\x07\x02\x06\x12\x04\x85\x01\x0b\x1d\n\x0b\n\x03\x07\x02\x01\x12\x04\
    \x85\x01\x1e3\n\x0b\n\x03\x07\x02\x03\x12\x04\x85\x016:\
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
