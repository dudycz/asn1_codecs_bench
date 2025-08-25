// Hampi codec
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
pub mod sample_hampi {
    include!(concat!(env!("OUT_DIR"), "/sample_hampi.rs"));
}

use asn1_codecs::{PerCodecData, uper::UperCodec}; // HAMPI codec

pub fn build_sample_hampi() -> sample_hampi::World {
    use crate::hampi_helpers::sample_hampi::*;
    World {
        depth: WorldDepth(
            (0..10)
                .map(|_| Plane {
                    rows: PlaneRows(
                        (0..10)
                            .map(|_| Column {
                                elements: ColumnElements(
                                    (0..10)
                                        .map(|_| Color {
                                            r: ColorR(42),
                                            g: ColorG(128),
                                            b: ColorB(77),
                                            a: ColorA(12312),
                                        })
                                        .collect(),
                                ),
                            })
                            .collect(),
                    ),
                })
                .collect(),
        ),
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
