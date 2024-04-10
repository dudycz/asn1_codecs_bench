use criterion::{criterion_group, criterion_main, Criterion};
use asn1_codecs_bench::*;

fn rasn_enc(c: &mut Criterion) {    
    c.bench_function("RASN/encode - sample.asn", |b| {
        b.iter(|| {
            let w = build_sample_rasn();
            let _ = encode_rasn(&w);
        })
    });
}

fn rasn_dec(c: &mut Criterion) {
    let w = build_sample_rasn();
    let encoded = encode_rasn(&w);

    c.bench_function("RASN/decode - sample.asn", |b| {
        b.iter(|| {
            let _ = decode_rasn(&encoded);
        })
    });
}

fn hampi_enc(c: &mut Criterion) {    
    c.bench_function("HAMPI/encode - sample.asn", |b| {
        b.iter(|| {
            let w = build_sample_hampi();
            let _ = encode_hampi(&w);
        })
    });
}

fn hampi_dec(c: &mut Criterion) {
    let w = build_sample_hampi();
    let encoded = encode_hampi(&w);

    c.bench_function("HAMPI/decode - sample.asn", |b| {
        b.iter(|| {
            let _ = decode_hampi(&encoded);
        })
    });
}

criterion_group!(benches, rasn_enc, rasn_dec, hampi_enc, hampi_dec);
criterion_main!(benches);