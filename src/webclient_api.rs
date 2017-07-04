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
pub struct LineArrivals {
    // message fields
    line: ::protobuf::SingularField<::std::string::String>,
    direction: ::std::option::Option<Direction>,
    timestamp: ::std::vec::Vec<i64>,
    line_color_hex: ::protobuf::SingularField<::std::string::String>,
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

    // repeated int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: ::std::vec::Vec<i64>) {
        self.timestamp = v;
    }

    // Mutable pointer to the field.
    pub fn mut_timestamp(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.timestamp
    }

    // Take field
    pub fn take_timestamp(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.timestamp, ::std::vec::Vec::new())
    }

    pub fn get_timestamp(&self) -> &[i64] {
        &self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.timestamp
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
}

impl ::protobuf::Message for LineArrivals {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.timestamp)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line_color_hex)?;
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
        for value in &self.timestamp {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(ref v) = self.line_color_hex.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
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
        for v in &self.timestamp {
            os.write_int64(3, *v)?;
        };
        if let Some(ref v) = self.line_color_hex.as_ref() {
            os.write_string(4, &v)?;
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    LineArrivals::get_timestamp_for_reflect,
                    LineArrivals::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line_color_hex",
                    LineArrivals::get_line_color_hex_for_reflect,
                    LineArrivals::mut_line_color_hex_for_reflect,
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
        self.clear_timestamp();
        self.clear_line_color_hex();
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
pub struct StationStatus {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    line: ::protobuf::RepeatedField<LineArrivals>,
    data_timestamp: ::std::option::Option<i64>,
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
}

impl ::protobuf::Message for StationStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.line {
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
        for value in &self.line {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.data_timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.line {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.data_timestamp {
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
        self.clear_line();
        self.clear_data_timestamp();
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
}

impl ::protobuf::Message for StationList {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.station)?;
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
}

impl ::protobuf::Message for LineList {
    fn is_initialized(&self) -> bool {
        for v in &self.line {
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
    UNKNOWN = 2,
}

impl ::protobuf::ProtobufEnum for Direction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Direction> {
        match value {
            0 => ::std::option::Option::Some(Direction::UPTOWN),
            1 => ::std::option::Option::Some(Direction::DOWNTOWN),
            2 => ::std::option::Option::Some(Direction::UNKNOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Direction] = &[
            Direction::UPTOWN,
            Direction::DOWNTOWN,
            Direction::UNKNOWN,
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
    \n\x13webclient_api.proto\"\x90\x01\n\x0cLineArrivals\x12\x12\n\x04line\
    \x18\x01\x20\x01(\tR\x04line\x12(\n\tdirection\x18\x02\x20\x01(\x0e2\n.D\
    irectionR\tdirection\x12\x1c\n\ttimestamp\x18\x03\x20\x03(\x03R\ttimesta\
    mp\x12$\n\x0eline_color_hex\x18\x04\x20\x01(\tR\x0clineColorHex\"m\n\rSt\
    ationStatus\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12!\n\x04line\
    \x18\x02\x20\x03(\x0b2\r.LineArrivalsR\x04line\x12%\n\x0edata_timestamp\
    \x18\x03\x20\x01(\x03R\rdataTimestamp\"-\n\x07Station\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\
    \"1\n\x0bStationList\x12\"\n\x07station\x18\x01\x20\x03(\x0b2\x08.Statio\
    nR\x07station\"O\n\x04Line\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x1b\n\tcolor_hex\x18\x02\x20\x01(\tR\x08colorHex\x12\x16\n\x06activ\
    e\x18\x03\x20\x01(\x08R\x06active\"%\n\x08LineList\x12\x19\n\x04line\x18\
    \x01\x20\x03(\x0b2\x05.LineR\x04line*2\n\tDirection\x12\n\n\x06UPTOWN\
    \x10\0\x12\x0c\n\x08DOWNTOWN\x10\x01\x12\x0b\n\x07UNKNOWN\x10\x02J\xfb\t\
    \n\x06\x12\x04\0\0&\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\
    \x12\x04\x02\0\x06\x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x0e\n\x0b\n\
    \x04\x05\0\x02\0\x12\x03\x03\x02\r\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\
    \x03\x02\x08\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x03\x0b\x0c\n\x0b\n\x04\
    \x05\0\x02\x01\x12\x03\x04\x02\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\
    \x04\x02\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x04\r\x0e\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\x05\x02\x0e\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \x05\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\x0c\r\n\n\n\x02\x04\
    \0\x12\x04\x08\0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x14\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\t\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\t\
    \x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\t\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\t\x12\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x19\
    \x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x02#\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\n\x0b\x14\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\x15\x1e\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\n!\"\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x02\x1f\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x0b\x0b\x10\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\x11\x1a\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\x1d\x1e\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x0c\x02%\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x0c\x02\n\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x0c\x12\x20\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c#$\
    \n\n\n\x02\x04\x01\x12\x04\x0f\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \x0f\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x10\x02\x1b\n\x0c\n\x05\
    \x04\x01\x02\0\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03\x10\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x10\x12\x16\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\x10\x19\x1a\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x11\x02!\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x11\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x11\x0b\x17\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x11\x18\x1c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x11\x1f\x20\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x12\x02$\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03\x12\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x12\x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x12\x11\x1f\
    \n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x12\"#\n\n\n\x02\x04\x02\x12\
    \x04\x15\0\x18\x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\x08\x0f\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\x16\x02\x19\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\
    \x16\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x16\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x16\x12\x14\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x16\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x17\x02\x1b\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03\x17\x02\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03\x17\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x17\x12\
    \x16\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x17\x19\x1a\n\n\n\x02\x04\
    \x03\x12\x04\x1a\0\x1c\x01\n\n\n\x03\x04\x03\x01\x12\x03\x1a\x08\x13\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03\x1b\x02\x1f\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03\x1b\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x1b\x0b\x12\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1b\x13\x1a\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03\x1b\x1d\x1e\n\n\n\x02\x04\x04\x12\x04\x1e\0\"\x01\n\n\
    \n\x03\x04\x04\x01\x12\x03\x1e\x08\x0c\n\x0b\n\x04\x04\x04\x02\0\x12\x03\
    \x1f\x02\x1b\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03\x1f\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03\x1f\x12\x16\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1f\x19\x1a\n\x0b\
    \n\x04\x04\x04\x02\x01\x12\x03\x20\x02\x20\n\x0c\n\x05\x04\x04\x02\x01\
    \x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x20\x0b\
    \x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x20\x12\x1b\n\x0c\n\x05\x04\
    \x04\x02\x01\x03\x12\x03\x20\x1e\x1f\n\x0b\n\x04\x04\x04\x02\x02\x12\x03\
    !\x02\x1b\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03!\x02\n\n\x0c\n\x05\x04\
    \x04\x02\x02\x05\x12\x03!\x0b\x0f\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\
    \x03!\x10\x16\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03!\x19\x1a\n\n\n\x02\
    \x04\x05\x12\x04$\0&\x01\n\n\n\x03\x04\x05\x01\x12\x03$\x08\x10\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x03%\x02\x19\n\x0c\n\x05\x04\x05\x02\0\x04\x12\
    \x03%\x02\n\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03%\x0b\x0f\n\x0c\n\x05\
    \x04\x05\x02\0\x01\x12\x03%\x10\x14\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03%\x17\x18\
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
