pub mod sample_rasn;
pub mod sample_hampi;

use asn1_codecs::{uper::UperCodec, PerCodecData};   // HAMPI codec
use rasn::uper; // RASN codec

pub fn build_sample_rasn() -> sample_rasn::world3d::World {
    use crate::sample_rasn::world3d::*;

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

pub fn build_sample_hampi() -> sample_hampi::World {
    use crate::sample_hampi::*;

    World {
        depth: WorldDepth((0..10).map(|_| {
                Plane {
                    rows: PlaneRows((0..10).map(|_| {
                            Column {
                                elements: ColumnElements((0..10).map(|_| Color {
                                        r: ColorR(42),
                                        g: ColorG(128),
                                        b: ColorB(77),
                                        a: ColorA(12312),
                                    }).collect()),
                            }
                        }).collect()),
                }
            }).collect())
    }
}

pub fn encode_hampi(w: &sample_hampi::World) -> Vec<u8> {
    let mut data = PerCodecData::new_uper();
    w.uper_encode(&mut data).unwrap();
    data.into_bytes()
}

pub fn decode_hampi(encoded: &[u8]) -> sample_hampi::World {
    let mut d = PerCodecData::from_slice_uper(encoded);
    sample_hampi::World::uper_decode(&mut d).unwrap()
}