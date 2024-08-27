use asn1_codecs_bench::*;

fn check_sample_rasn() {
    let w = build_sample_rasn();
    let buf = encode_rasn(&w);
    println!("rasn_encoded: {:?}", buf.len());
}

fn check_sample_hampi() {
    let w = build_sample_hampi();
    let buf = encode_hampi(&w);
    println!("hampi_encoded: {:?}", buf.len())
}

fn check_simple_rasn(i: u16) {
    let data: Vec<u16> = vec![i; 100];
    let buf = rasn::uper::encode(&data).unwrap();
    println!("rasn_encoded: {:?}", buf.len());
}

// HAMPI
#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct Elem(pub u16);

#[derive(asn1_codecs_derive::UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct BenchSequence(pub Vec<Elem>);

fn check_simple_hampi(i: u16) {
    use asn1_codecs::{uper::UperCodec, PerCodecData};

    let w = BenchSequence((0..100).map(|_| Elem(i)).collect());
    let mut data = PerCodecData::new_uper();
    w.uper_encode(&mut data).unwrap();
    println!("hampi_encoded: {:?}", data.into_bytes().len());
}

fn main() {
    for i in 0..256 {
        check_sample_rasn();
        check_sample_hampi();
        check_simple_rasn(i);
        check_simple_hampi(i);
    }
}
