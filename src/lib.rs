pub mod sample_asn1rs;
pub mod sample_hampi;
pub mod sample_rasn;

use asn1_codecs::{uper::UperCodec, PerCodecData}; // HAMPI codec
use asn1rs::prelude::*;
use rasn::uper; // RASN codec

// RASN codec
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

// Hampi codec
pub fn build_sample_hampi() -> sample_hampi::World {
    use crate::sample_hampi::*;

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

// asn1rs codec
pub fn build_sample_asn1rs() -> sample_asn1rs::World {
    use crate::sample_asn1rs::*;

    World {
        depth: (0..10)
            .map(|_| {
                let plane = Plane {
                    rows: (0..10)
                        .map(|_| {
                            let column = Column {
                                elements: (0..10)
                                    .map(|_| Color {
                                        r: 42,
                                        g: 128,
                                        b: 77,
                                        a: 12312,
                                    })
                                    .collect(),
                            };
                            column
                        })
                        .collect(),
                };
                plane
            })
            .collect(),
    }
}

pub fn encode_asn1rs(w: &sample_asn1rs::World) -> Vec<u8> {
    let mut writer = UperWriter::default();
    writer.write(w).unwrap();
    writer.into_bytes_vec()
}

pub fn decode_asn1rs(encoded: &[u8]) -> sample_asn1rs::World {
    let mut reader = UperReader::from(encoded);
    reader.read::<sample_asn1rs::World>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilers() {
        let w = build_sample_rasn();
        let encoded = encode_rasn(&w);

        let dec_hampi = decode_hampi(&encoded);
        let enc_hampi = encode_hampi(&dec_hampi);

        let dec_asn1rs = decode_asn1rs(&encoded);
        let enc_asn1rs = encode_asn1rs(&dec_asn1rs);

        // Compare encoded with enc_hampi and enc_asn1rs
        assert_eq!(encoded, enc_hampi);
        assert_eq!(encoded, enc_asn1rs);
    }
}
