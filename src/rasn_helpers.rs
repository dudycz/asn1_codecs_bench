// RASN codec
#[allow(clippy::all)]
pub mod sample_rasn {
    include!(concat!(env!("OUT_DIR"), "/sample_rasn.rs"));
}
use rasn::uper; // RASN codec

pub fn build_sample_rasn() -> sample_rasn::world3d::World {
    use crate::rasn_helpers::sample_rasn::world3d::*;

    let color = Color::new(42, 128, 77, 12312);
    let elements = (0..10).map(|_| color.clone()).collect::<Vec<_>>();
    let column = Column { elements };
    let rows = (0..10).map(|_| column.clone()).collect::<Vec<_>>();
    let plane = Plane { rows };
    let depth = (0..10).map(|_| plane.clone()).collect::<Vec<_>>();

    World { depth }
}

pub fn encode_rasn(w: &sample_rasn::world3d::World) -> Vec<u8> {
    uper::encode(&w).unwrap()
}

pub fn decode_rasn(encoded: &[u8]) -> sample_rasn::world3d::World {
    uper::decode::<sample_rasn::world3d::World>(encoded).unwrap()
}
