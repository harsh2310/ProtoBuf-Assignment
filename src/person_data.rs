// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `person_data.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Person {
    // message fields
    pub last_name: ::std::string::String,
    pub first_name: ::std::string::String,
    pub date_of_birth: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Person {
    fn default() -> &'a Person {
        <Person as ::protobuf::Message>::default_instance()
    }
}

impl Person {
    pub fn new() -> Person {
        ::std::default::Default::default()
    }

    // string last_name = 1;


    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }
    pub fn clear_last_name(&mut self) {
        self.last_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_name(&mut self, v: ::std::string::String) {
        self.last_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_name(&mut self) -> &mut ::std::string::String {
        &mut self.last_name
    }

    // Take field
    pub fn take_last_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.last_name, ::std::string::String::new())
    }

    // string first_name = 2;


    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }
    pub fn clear_first_name(&mut self) {
        self.first_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_first_name(&mut self, v: ::std::string::String) {
        self.first_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first_name(&mut self) -> &mut ::std::string::String {
        &mut self.first_name
    }

    // Take field
    pub fn take_first_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.first_name, ::std::string::String::new())
    }

    // string date_of_birth = 3;


    pub fn get_date_of_birth(&self) -> &str {
        &self.date_of_birth
    }
    pub fn clear_date_of_birth(&mut self) {
        self.date_of_birth.clear();
    }

    // Param is passed by value, moved
    pub fn set_date_of_birth(&mut self, v: ::std::string::String) {
        self.date_of_birth = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_date_of_birth(&mut self) -> &mut ::std::string::String {
        &mut self.date_of_birth
    }

    // Take field
    pub fn take_date_of_birth(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.date_of_birth, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Person {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.last_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.first_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.date_of_birth)?;
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
        if !self.last_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.last_name);
        }
        if !self.first_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.first_name);
        }
        if !self.date_of_birth.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.date_of_birth);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.last_name.is_empty() {
            os.write_string(1, &self.last_name)?;
        }
        if !self.first_name.is_empty() {
            os.write_string(2, &self.first_name)?;
        }
        if !self.date_of_birth.is_empty() {
            os.write_string(3, &self.date_of_birth)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Person {
        Person::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "last_name",
                |m: &Person| { &m.last_name },
                |m: &mut Person| { &mut m.last_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "first_name",
                |m: &Person| { &m.first_name },
                |m: &mut Person| { &mut m.first_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "date_of_birth",
                |m: &Person| { &m.date_of_birth },
                |m: &mut Person| { &mut m.date_of_birth },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Person>(
                "Person",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Person {
        static instance: ::protobuf::rt::LazyV2<Person> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Person::new)
    }
}

impl ::protobuf::Clear for Person {
    fn clear(&mut self) {
        self.last_name.clear();
        self.first_name.clear();
        self.date_of_birth.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Person {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Person {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11person_data.proto\"h\n\x06Person\x12\x1b\n\tlast_name\x18\x01\x20\
    \x01(\tR\x08lastName\x12\x1d\n\nfirst_name\x18\x02\x20\x01(\tR\tfirstNam\
    e\x12\"\n\rdate_of_birth\x18\x03\x20\x01(\tR\x0bdateOfBirthJ\xfc\x01\n\
    \x06\x12\x04\0\0\x05\x02\n\x08\n\x01\x0c\x12\x03\0\0\x10\n\n\n\x02\x04\0\
    \x12\x04\x01\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x01\x08\x0e\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x02\x04\x17\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x02\x04\x01\x0f\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x02\x04\n\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x02\x0b\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x02\x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x03\x04\x18\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x03\x04\x02\x17\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x03\x0b\x15\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x03\x16\x17\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x04\x04\x1b\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x04\x04\x03\
    \x18\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\x04\x0b\x18\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x04\x19\x1ab\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}