// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protobuf-codegen-pure=3.0.0-pre
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `Tile.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_PRE;

#[derive(PartialEq,Clone,Default)]
#[derive(Serialize, Deserialize)]
pub struct Tile {
    // message oneof groups
    pub payload: ::std::option::Option<tile::Payload>,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Tile {
    fn default() -> &'a Tile {
        <Tile as ::protobuf::Message>::default_instance()
    }
}

impl Tile {
    pub fn new() -> Tile {
        ::std::default::Default::default()
    }

    // bytes tile_data = 1;

    pub fn get_tile_data(&self) -> &[u8] {
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_data(ref v)) => v,
            _ => &[],
        }
    }

    pub fn clear_tile_data(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_tile_data(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tile_data(&mut self, v: ::bytes::Bytes) {
        self.payload = ::std::option::Option::Some(tile::Payload::tile_data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tile_data(&mut self) -> &mut ::bytes::Bytes {
        if let ::std::option::Option::Some(tile::Payload::tile_data(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(tile::Payload::tile_data(::bytes::Bytes::new()));
        }
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tile_data(&mut self) -> ::bytes::Bytes {
        if self.has_tile_data() {
            match self.payload.take() {
                ::std::option::Option::Some(tile::Payload::tile_data(v)) => v,
                _ => panic!(),
            }
        } else {
            ::bytes::Bytes::new()
        }
    }

    // .smmdb.Tile.TileDetails tile_details = 2;

    pub fn get_tile_details(&self) -> &tile::TileDetails {
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_details(ref v)) => v,
            _ => <tile::TileDetails as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_tile_details(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_tile_details(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_details(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tile_details(&mut self, v: tile::TileDetails) {
        self.payload = ::std::option::Option::Some(tile::Payload::tile_details(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tile_details(&mut self) -> &mut tile::TileDetails {
        if let ::std::option::Option::Some(tile::Payload::tile_details(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(tile::Payload::tile_details(tile::TileDetails::new()));
        }
        match self.payload {
            ::std::option::Option::Some(tile::Payload::tile_details(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tile_details(&mut self) -> tile::TileDetails {
        if self.has_tile_details() {
            match self.payload.take() {
                ::std::option::Option::Some(tile::Payload::tile_details(v)) => v,
                _ => panic!(),
            }
        } else {
            tile::TileDetails::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "tile_data",
            Tile::has_tile_data,
            Tile::get_tile_data,
            Tile::set_tile_data,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, tile::TileDetails>(
            "tile_details",
            Tile::has_tile_details,
            Tile::get_tile_details,
            Tile::mut_tile_details,
            Tile::set_tile_details,
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Tile>(
            "Tile",
            0,
            fields,
        )
    }
}

impl ::protobuf::Message for Tile {
    fn is_initialized(&self) -> bool {
        if let Some(tile::Payload::tile_details(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(tile::Payload::tile_data(is.read_carllerche_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(tile::Payload::tile_details(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &tile::Payload::tile_data(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &tile::Payload::tile_details(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &tile::Payload::tile_data(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &tile::Payload::tile_details(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> Tile {
        Tile::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }

    fn default_instance() -> &'static Tile {
        static instance: Tile = Tile {
            payload: ::std::option::Option::None,
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for Tile {
    fn clear(&mut self) {
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tile {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Tile`
pub mod tile {

    #[derive(Clone,PartialEq,Debug)]
    #[derive(Serialize, Deserialize)]
    pub enum Payload {
        tile_data(::bytes::Bytes),
        tile_details(TileDetails),
    }

    impl ::protobuf::Oneof for Payload {
    }
    #[derive(PartialEq,Clone,Default)]
    #[derive(Serialize, Deserialize)]
    pub struct TileDetails {
        // message fields
        pub x: f32,
        pub y: f32,
        pub dim_x: u32,
        pub dim_y: u32,
        pub orientation: u32,
        pub z_index: u32,
        pub tile_type: ::protobuf::ProtobufEnumOrUnknown<tile_details::TileType>,
        pub entity_type: ::protobuf::ProtobufEnumOrUnknown<tile_details::EntityType>,
        pub link: u32,
        pub id: u32,
        pub costume: u32,
        pub container: u32,
        pub inknown_0: u32,
        pub unknown_1: u32,
        pub unknown_2: u32,
        // special fields
        #[serde(skip)]
        pub unknown_fields: ::protobuf::UnknownFields,
        #[serde(skip)]
        pub cached_size: ::protobuf::rt::CachedSize,
    }

    impl<'a> ::std::default::Default for &'a TileDetails {
        fn default() -> &'a TileDetails {
            <TileDetails as ::protobuf::Message>::default_instance()
        }
    }

    impl TileDetails {
        pub fn new() -> TileDetails {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "x",
                |m: &TileDetails| { &m.x },
                |m: &mut TileDetails| { &mut m.x },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "y",
                |m: &TileDetails| { &m.y },
                |m: &mut TileDetails| { &mut m.y },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "dim_x",
                |m: &TileDetails| { &m.dim_x },
                |m: &mut TileDetails| { &mut m.dim_x },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "dim_y",
                |m: &TileDetails| { &m.dim_y },
                |m: &mut TileDetails| { &mut m.dim_y },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "orientation",
                |m: &TileDetails| { &m.orientation },
                |m: &mut TileDetails| { &mut m.orientation },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "z_index",
                |m: &TileDetails| { &m.z_index },
                |m: &mut TileDetails| { &mut m.z_index },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "tile_type",
                |m: &TileDetails| { &m.tile_type },
                |m: &mut TileDetails| { &mut m.tile_type },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "entity_type",
                |m: &TileDetails| { &m.entity_type },
                |m: &mut TileDetails| { &mut m.entity_type },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "link",
                |m: &TileDetails| { &m.link },
                |m: &mut TileDetails| { &mut m.link },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "id",
                |m: &TileDetails| { &m.id },
                |m: &mut TileDetails| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "costume",
                |m: &TileDetails| { &m.costume },
                |m: &mut TileDetails| { &mut m.costume },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "container",
                |m: &TileDetails| { &m.container },
                |m: &mut TileDetails| { &mut m.container },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "inknown_0",
                |m: &TileDetails| { &m.inknown_0 },
                |m: &mut TileDetails| { &mut m.inknown_0 },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "unknown_1",
                |m: &TileDetails| { &m.unknown_1 },
                |m: &mut TileDetails| { &mut m.unknown_1 },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "unknown_2",
                |m: &TileDetails| { &m.unknown_2 },
                |m: &mut TileDetails| { &mut m.unknown_2 },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TileDetails>(
                "Tile.TileDetails",
                1,
                fields,
            )
        }
    }

    impl ::protobuf::Message for TileDetails {
        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
            while !is.eof()? {
                let (field_number, wire_type) = is.read_tag_unpack()?;
                match field_number {
                    1 => {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.x = is.read_float()?;
                    },
                    2 => {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.y = is.read_float()?;
                    },
                    3 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.dim_x = is.read_uint32()?;
                    },
                    4 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.dim_y = is.read_uint32()?;
                    },
                    5 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.orientation = is.read_uint32()?;
                    },
                    6 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.z_index = is.read_uint32()?;
                    },
                    7 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.tile_type = is.read_enum_or_unknown()?;
                    },
                    8 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.entity_type = is.read_enum_or_unknown()?;
                    },
                    9 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.link = is.read_uint32()?;
                    },
                    10 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.id = is.read_uint32()?;
                    },
                    11 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.costume = is.read_uint32()?;
                    },
                    12 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.container = is.read_uint32()?;
                    },
                    13 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.inknown_0 = is.read_uint32()?;
                    },
                    14 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.unknown_1 = is.read_uint32()?;
                    },
                    15 => {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                        }
                        self.unknown_2 = is.read_uint32()?;
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
            if self.x != 0. {
                my_size += 5;
            }
            if self.y != 0. {
                my_size += 5;
            }
            if self.dim_x != 0 {
                my_size += ::protobuf::rt::value_size(3, self.dim_x, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.dim_y != 0 {
                my_size += ::protobuf::rt::value_size(4, self.dim_y, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.orientation != 0 {
                my_size += ::protobuf::rt::value_size(5, self.orientation, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.z_index != 0 {
                my_size += ::protobuf::rt::value_size(6, self.z_index, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.tile_type != ::protobuf::ProtobufEnumOrUnknown::new(tile_details::TileType::UNKNOWN) {
                my_size += ::protobuf::rt::enum_or_unknown_size(7, self.tile_type);
            }
            if self.entity_type != ::protobuf::ProtobufEnumOrUnknown::new(tile_details::EntityType::STATIC) {
                my_size += ::protobuf::rt::enum_or_unknown_size(8, self.entity_type);
            }
            if self.link != 0 {
                my_size += ::protobuf::rt::value_size(9, self.link, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.id != 0 {
                my_size += ::protobuf::rt::value_size(10, self.id, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.costume != 0 {
                my_size += ::protobuf::rt::value_size(11, self.costume, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.container != 0 {
                my_size += ::protobuf::rt::value_size(12, self.container, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.inknown_0 != 0 {
                my_size += ::protobuf::rt::value_size(13, self.inknown_0, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.unknown_1 != 0 {
                my_size += ::protobuf::rt::value_size(14, self.unknown_1, ::protobuf::wire_format::WireTypeVarint);
            }
            if self.unknown_2 != 0 {
                my_size += ::protobuf::rt::value_size(15, self.unknown_2, ::protobuf::wire_format::WireTypeVarint);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
            self.cached_size.set(my_size);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
            if self.x != 0. {
                os.write_float(1, self.x)?;
            }
            if self.y != 0. {
                os.write_float(2, self.y)?;
            }
            if self.dim_x != 0 {
                os.write_uint32(3, self.dim_x)?;
            }
            if self.dim_y != 0 {
                os.write_uint32(4, self.dim_y)?;
            }
            if self.orientation != 0 {
                os.write_uint32(5, self.orientation)?;
            }
            if self.z_index != 0 {
                os.write_uint32(6, self.z_index)?;
            }
            if self.tile_type != ::protobuf::ProtobufEnumOrUnknown::new(tile_details::TileType::UNKNOWN) {
                os.write_enum(7, ::protobuf::ProtobufEnumOrUnknown::value(&self.tile_type))?;
            }
            if self.entity_type != ::protobuf::ProtobufEnumOrUnknown::new(tile_details::EntityType::STATIC) {
                os.write_enum(8, ::protobuf::ProtobufEnumOrUnknown::value(&self.entity_type))?;
            }
            if self.link != 0 {
                os.write_uint32(9, self.link)?;
            }
            if self.id != 0 {
                os.write_uint32(10, self.id)?;
            }
            if self.costume != 0 {
                os.write_uint32(11, self.costume)?;
            }
            if self.container != 0 {
                os.write_uint32(12, self.container)?;
            }
            if self.inknown_0 != 0 {
                os.write_uint32(13, self.inknown_0)?;
            }
            if self.unknown_1 != 0 {
                os.write_uint32(14, self.unknown_1)?;
            }
            if self.unknown_2 != 0 {
                os.write_uint32(15, self.unknown_2)?;
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

        fn new() -> TileDetails {
            TileDetails::new()
        }

        fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
            ::protobuf::reflect::MessageDescriptor::new_generated_2(super::file_descriptor(), 1)
        }

        fn default_instance() -> &'static TileDetails {
            static instance: TileDetails = TileDetails {
                x: 0.,
                y: 0.,
                dim_x: 0,
                dim_y: 0,
                orientation: 0,
                z_index: 0,
                tile_type: ::protobuf::ProtobufEnumOrUnknown::from_i32(0),
                entity_type: ::protobuf::ProtobufEnumOrUnknown::from_i32(0),
                link: 0,
                id: 0,
                costume: 0,
                container: 0,
                inknown_0: 0,
                unknown_1: 0,
                unknown_2: 0,
                unknown_fields: ::protobuf::UnknownFields::new(),
                cached_size: ::protobuf::rt::CachedSize::new(),
            };
            &instance
        }
    }

    impl ::protobuf::Clear for TileDetails {
        fn clear(&mut self) {
            self.x = 0.;
            self.y = 0.;
            self.dim_x = 0;
            self.dim_y = 0;
            self.orientation = 0;
            self.z_index = 0;
            self.tile_type = ::protobuf::ProtobufEnumOrUnknown::new(tile_details::TileType::UNKNOWN);
            self.entity_type = ::protobuf::ProtobufEnumOrUnknown::new(tile_details::EntityType::STATIC);
            self.link = 0;
            self.id = 0;
            self.costume = 0;
            self.container = 0;
            self.inknown_0 = 0;
            self.unknown_1 = 0;
            self.unknown_2 = 0;
            self.unknown_fields.clear();
        }
    }

    impl ::std::fmt::Debug for TileDetails {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for TileDetails {
        type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
    }

    /// Nested message and enums of message `TileDetails`
    pub mod tile_details {
        #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
        #[derive(Serialize, Deserialize)]
        pub enum TileType {
            UNKNOWN = 0,
            NORMAL = 4,
            QUESTIONMARK = 5,
            HARD = 6,
            GROUND = 7,
            COIN = 8,
            PIPE = 9,
            BACKGROUND = 16,
            MUSHROOM = 20,
            FALLING = 21,
            CLOUD = 22,
            NOTE = 23,
            FINISH_PLATFORM = 26,
            FINISH = 27,
            KAIZO = 29,
            STAR = 35,
            START_PLATFORM = 37,
            WOODEN_ARROW = 38,
            BOO = 41,
            SPIKE = 43,
            SPECIAL = 44,
            FINISH_LINE = 49,
            TRACK = 59,
            ICE = 63,
            START = 69,
        }

        impl ::protobuf::ProtobufEnum for TileType {
            fn value(&self) -> i32 {
                *self as i32
            }

            fn from_i32(value: i32) -> ::std::option::Option<TileType> {
                match value {
                    0 => ::std::option::Option::Some(TileType::UNKNOWN),
                    4 => ::std::option::Option::Some(TileType::NORMAL),
                    5 => ::std::option::Option::Some(TileType::QUESTIONMARK),
                    6 => ::std::option::Option::Some(TileType::HARD),
                    7 => ::std::option::Option::Some(TileType::GROUND),
                    8 => ::std::option::Option::Some(TileType::COIN),
                    9 => ::std::option::Option::Some(TileType::PIPE),
                    16 => ::std::option::Option::Some(TileType::BACKGROUND),
                    20 => ::std::option::Option::Some(TileType::MUSHROOM),
                    21 => ::std::option::Option::Some(TileType::FALLING),
                    22 => ::std::option::Option::Some(TileType::CLOUD),
                    23 => ::std::option::Option::Some(TileType::NOTE),
                    26 => ::std::option::Option::Some(TileType::FINISH_PLATFORM),
                    27 => ::std::option::Option::Some(TileType::FINISH),
                    29 => ::std::option::Option::Some(TileType::KAIZO),
                    35 => ::std::option::Option::Some(TileType::STAR),
                    37 => ::std::option::Option::Some(TileType::START_PLATFORM),
                    38 => ::std::option::Option::Some(TileType::WOODEN_ARROW),
                    41 => ::std::option::Option::Some(TileType::BOO),
                    43 => ::std::option::Option::Some(TileType::SPIKE),
                    44 => ::std::option::Option::Some(TileType::SPECIAL),
                    49 => ::std::option::Option::Some(TileType::FINISH_LINE),
                    59 => ::std::option::Option::Some(TileType::TRACK),
                    63 => ::std::option::Option::Some(TileType::ICE),
                    69 => ::std::option::Option::Some(TileType::START),
                    _ => ::std::option::Option::None
                }
            }

            fn values() -> &'static [Self] {
                static values: &'static [TileType] = &[
                    TileType::UNKNOWN,
                    TileType::NORMAL,
                    TileType::QUESTIONMARK,
                    TileType::HARD,
                    TileType::GROUND,
                    TileType::COIN,
                    TileType::PIPE,
                    TileType::BACKGROUND,
                    TileType::MUSHROOM,
                    TileType::FALLING,
                    TileType::CLOUD,
                    TileType::NOTE,
                    TileType::FINISH_PLATFORM,
                    TileType::FINISH,
                    TileType::KAIZO,
                    TileType::STAR,
                    TileType::START_PLATFORM,
                    TileType::WOODEN_ARROW,
                    TileType::BOO,
                    TileType::SPIKE,
                    TileType::SPECIAL,
                    TileType::FINISH_LINE,
                    TileType::TRACK,
                    TileType::ICE,
                    TileType::START,
                ];
                values
            }

            fn enum_descriptor_static() -> ::protobuf::reflect::EnumDescriptor {
                ::protobuf::reflect::EnumDescriptor::new_generated_2(super::super::file_descriptor(), 0)
            }
        }

        impl ::std::default::Default for TileType {
            fn default() -> Self {
                TileType::UNKNOWN
            }
        }

        impl ::protobuf::reflect::ProtobufValue for TileType {
            type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeEnum<Self>;
        }

        impl TileType {
            pub(in super::super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
                ::protobuf::reflect::GeneratedEnumDescriptorData::new_2::<TileType>("Tile.TileDetails.TileType", 0)
            }
        }

        #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
        #[derive(Serialize, Deserialize)]
        pub enum EntityType {
            STATIC = 0,
            LIVING = 1,
            PLATFORM = 65535,
        }

        impl ::protobuf::ProtobufEnum for EntityType {
            fn value(&self) -> i32 {
                *self as i32
            }

            fn from_i32(value: i32) -> ::std::option::Option<EntityType> {
                match value {
                    0 => ::std::option::Option::Some(EntityType::STATIC),
                    1 => ::std::option::Option::Some(EntityType::LIVING),
                    65535 => ::std::option::Option::Some(EntityType::PLATFORM),
                    _ => ::std::option::Option::None
                }
            }

            fn values() -> &'static [Self] {
                static values: &'static [EntityType] = &[
                    EntityType::STATIC,
                    EntityType::LIVING,
                    EntityType::PLATFORM,
                ];
                values
            }

            fn enum_descriptor_static() -> ::protobuf::reflect::EnumDescriptor {
                ::protobuf::reflect::EnumDescriptor::new_generated_2(super::super::file_descriptor(), 1)
            }
        }

        impl ::std::default::Default for EntityType {
            fn default() -> Self {
                EntityType::STATIC
            }
        }

        impl ::protobuf::reflect::ProtobufValue for EntityType {
            type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeEnum<Self>;
        }

        impl EntityType {
            pub(in super::super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
                ::protobuf::reflect::GeneratedEnumDescriptorData::new_2::<EntityType>("Tile.TileDetails.EntityType", 1)
            }
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nTile.proto\x12\x05smmdb\"\x9f\x08\n\x04Tile\x12\x1f\n\ttile_data\x18\
    \x01\x20\x01(\x0cH\0R\x08tileDataB\0\x12>\n\x0ctile_details\x18\x02\x20\
    \x01(\x0b2\x17.smmdb.Tile.TileDetailsH\0R\x0btileDetailsB\0\x1a\xa6\x07\
    \n\x0bTileDetails\x12\x0e\n\x01x\x18\x01\x20\x01(\x02R\x01xB\0\x12\x0e\n\
    \x01y\x18\x02\x20\x01(\x02R\x01yB\0\x12\x15\n\x05dim_x\x18\x03\x20\x01(\
    \rR\x04dimXB\0\x12\x15\n\x05dim_y\x18\x04\x20\x01(\rR\x04dimYB\0\x12\"\n\
    \x0borientation\x18\x05\x20\x01(\rR\x0borientationB\0\x12\x19\n\x07z_ind\
    ex\x18\x06\x20\x01(\rR\x06zIndexB\0\x12?\n\ttile_type\x18\x07\x20\x01(\
    \x0e2\x20.smmdb.Tile.TileDetails.TileTypeR\x08tileTypeB\0\x12E\n\x0benti\
    ty_type\x18\x08\x20\x01(\x0e2\".smmdb.Tile.TileDetails.EntityTypeR\nenti\
    tyTypeB\0\x12\x14\n\x04link\x18\t\x20\x01(\rR\x04linkB\0\x12\x10\n\x02id\
    \x18\n\x20\x01(\rR\x02idB\0\x12\x1a\n\x07costume\x18\x0b\x20\x01(\rR\x07\
    costumeB\0\x12\x1e\n\tcontainer\x18\x0c\x20\x01(\rR\tcontainerB\0\x12\
    \x1d\n\tinknown_0\x18\r\x20\x01(\rR\x08inknown0B\0\x12\x1d\n\tunknown_1\
    \x18\x0e\x20\x01(\rR\x08unknown1B\0\x12\x1d\n\tunknown_2\x18\x0f\x20\x01\
    (\rR\x08unknown2B\0\"\x80\x03\n\x08TileType\x12\r\n\x07UNKNOWN\x10\0\x1a\
    \0\x12\x0c\n\x06NORMAL\x10\x04\x1a\0\x12\x12\n\x0cQUESTIONMARK\x10\x05\
    \x1a\0\x12\n\n\x04HARD\x10\x06\x1a\0\x12\x0c\n\x06GROUND\x10\x07\x1a\0\
    \x12\n\n\x04COIN\x10\x08\x1a\0\x12\n\n\x04PIPE\x10\t\x1a\0\x12\x10\n\nBA\
    CKGROUND\x10\x10\x1a\0\x12\x0e\n\x08MUSHROOM\x10\x14\x1a\0\x12\r\n\x07FA\
    LLING\x10\x15\x1a\0\x12\x0b\n\x05CLOUD\x10\x16\x1a\0\x12\n\n\x04NOTE\x10\
    \x17\x1a\0\x12\x15\n\x0fFINISH_PLATFORM\x10\x1a\x1a\0\x12\x0c\n\x06FINIS\
    H\x10\x1b\x1a\0\x12\x0b\n\x05KAIZO\x10\x1d\x1a\0\x12\n\n\x04STAR\x10#\
    \x1a\0\x12\x14\n\x0eSTART_PLATFORM\x10%\x1a\0\x12\x12\n\x0cWOODEN_ARROW\
    \x10&\x1a\0\x12\t\n\x03BOO\x10)\x1a\0\x12\x0b\n\x05SPIKE\x10+\x1a\0\x12\
    \r\n\x07SPECIAL\x10,\x1a\0\x12\x11\n\x0bFINISH_LINE\x101\x1a\0\x12\x0b\n\
    \x05TRACK\x10;\x1a\0\x12\t\n\x03ICE\x10?\x1a\0\x12\x0b\n\x05START\x10E\
    \x1a\0\x1a\0\"<\n\nEntityType\x12\x0c\n\x06STATIC\x10\0\x1a\0\x12\x0c\n\
    \x06LIVING\x10\x01\x1a\0\x12\x10\n\x08PLATFORM\x10\xff\xff\x03\x1a\0\x1a\
    \0:\0B\x0b\n\x07payload\x12\0:\0B\0b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> ::protobuf::reflect::FileDescriptor {
    static file_descriptor_lazy: ::protobuf::rt::LazyV2<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::LazyV2::INIT;
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(Tile::generated_message_descriptor_data());
        messages.push(tile::TileDetails::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        enums.push(tile::tile_details::TileType::generated_enum_descriptor_data());
        enums.push(tile::tile_details::EntityType::generated_enum_descriptor_data());
        ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    ::protobuf::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
