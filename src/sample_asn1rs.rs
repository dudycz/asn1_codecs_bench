use asn1rs::prelude::*;

#[asn(sequence)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct Color {
    #[asn(integer(0..255))]
    pub r: u8,
    #[asn(integer(0..255))]
    pub g: u8,
    #[asn(integer(0..255))]
    pub b: u8,
    #[asn(integer(0..65335))]
    pub a: u16,
}

impl Color {
    pub const fn r_min() -> u8 {
        0
    }

    pub const fn r_max() -> u8 {
        255
    }

    pub const fn g_min() -> u8 {
        0
    }

    pub const fn g_max() -> u8 {
        255
    }

    pub const fn b_min() -> u8 {
        0
    }

    pub const fn b_max() -> u8 {
        255
    }

    pub const fn a_min() -> u16 {
        0
    }

    pub const fn a_max() -> u16 {
        65_335
    }
}

#[asn(sequence)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct Column {
    #[asn(sequence_of(size(10), complex(Color, tag(UNIVERSAL(16)))))]
    pub elements: Vec<Color>,
}

impl Column {}

#[asn(sequence)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct Plane {
    #[asn(sequence_of(size(10), complex(Column, tag(UNIVERSAL(16)))))]
    pub rows: Vec<Column>,
}

impl Plane {}

#[asn(sequence)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct World {
    #[asn(sequence_of(size(10), complex(Plane, tag(UNIVERSAL(16)))))]
    pub depth: Vec<Plane>,
}

impl World {}
