use asn1rs::prelude::*;

pub mod sample_asn1rs {
    include!(concat!(env!("OUT_DIR"), "/world3d.rs"));
}

pub fn build_sample_asn1rs() -> sample_asn1rs::World {
    use crate::asn1rs_helpers::sample_asn1rs::*;

    let color = Color {
        r: 42,
        g: 128,
        b: 77,
        a: 12312,
    };

    let column = Column {
        elements: vec![color; 10],
    };

    let plane = Plane {
        rows: vec![column; 10],
    };

    World {
        depth: vec![plane; 10],
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
