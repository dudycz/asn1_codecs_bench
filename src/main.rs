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

// HAMPI
#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct Elem(pub u16);

#[derive(asn1_codecs_derive::UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct BenchSequence(pub Vec<Elem>);

fn main() {
    // Check telco sample message sizes
    println!("=== Telco Sample Message Sizes ===");

    let pdu_asn1rs = build_telco_sample_asn1rs();
    let encoded_asn1rs = encode_telco_sample_asn1rs(&pdu_asn1rs);
    println!("asn1rs encoded size: {} bytes", encoded_asn1rs.len());

    let pdu_rasn = build_telco_sample_rasn();
    let encoded_rasn = encode_telco_sample_rasn(&pdu_rasn);
    println!("rasn encoded size: {} bytes", encoded_rasn.len());

    let pdu_hampi = build_telco_sample_hampi();
    let encoded_hampi = encode_telco_sample_hampi(&pdu_hampi);
    println!("asn1-codecs encoded size: {} bytes", encoded_hampi.len());

    println!("\n=== Sample Message Sizes ===");
    check_sample_rasn();
    check_sample_hampi();
}
