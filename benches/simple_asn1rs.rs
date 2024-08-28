use asn1rs::prelude::*;

#[asn(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct SimpleInteger(#[asn(integer(min..max))] pub u64);

impl SimpleInteger {
    pub const fn value_min() -> u64 {
        0
    }

    pub const fn value_max() -> u64 {
        9_223_372_036_854_775_807
    }
}

impl SimpleInteger {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for SimpleInteger {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl ::core::ops::DerefMut for SimpleInteger {
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl ::core::convert::From<u64> for SimpleInteger {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ::core::convert::From<SimpleInteger> for u64 {
    fn from(value: SimpleInteger) -> Self {
        value.0
    }
}

#[asn(transparent)]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct SequenceOfBasicInteger(
    #[asn(sequence_of(complex(SimpleInteger, tag(UNIVERSAL(2)))))] pub Vec<SimpleInteger>,
);

impl SequenceOfBasicInteger {}

impl SequenceOfBasicInteger {
    pub const fn new(value: Vec<SimpleInteger>) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for SequenceOfBasicInteger {
    type Target = Vec<SimpleInteger>;

    fn deref(&self) -> &Vec<SimpleInteger> {
        &self.0
    }
}

impl ::core::ops::DerefMut for SequenceOfBasicInteger {
    fn deref_mut(&mut self) -> &mut Vec<SimpleInteger> {
        &mut self.0
    }
}

impl ::core::convert::From<Vec<SimpleInteger>> for SequenceOfBasicInteger {
    fn from(value: Vec<SimpleInteger>) -> Self {
        Self(value)
    }
}

impl ::core::convert::From<SequenceOfBasicInteger> for Vec<SimpleInteger> {
    fn from(value: SequenceOfBasicInteger) -> Self {
        value.0
    }
}
