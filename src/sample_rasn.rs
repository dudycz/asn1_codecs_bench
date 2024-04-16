#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]
pub mod world3d {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Color {
        #[rasn(value("0..=255"))]
        pub r: u8,
        #[rasn(value("0..=255"))]
        pub g: u8,
        #[rasn(value("0..=255"))]
        pub b: u8,
        #[rasn(value("0..=65335"))]
        pub a: u16,
    }
    impl Color {
        pub fn new(r: u8, g: u8, b: u8, a: u16) -> Self {
            Self { r, g, b, a }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Column {
        #[rasn(size("10"))]
        pub elements: SequenceOf<Color>,
    }
    impl Column {
        pub fn new(elements: SequenceOf<Color>) -> Self {
            Self { elements }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Plane {
        #[rasn(size("10"))]
        pub rows: SequenceOf<Column>,
    }
    impl Plane {
        pub fn new(rows: SequenceOf<Column>) -> Self {
            Self { rows }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct World {
        #[rasn(size("10"))]
        pub depth: SequenceOf<Plane>,
    }
    impl World {
        pub fn new(depth: SequenceOf<Plane>) -> Self {
            Self { depth }
        }
    }
}
