use criterion::{criterion_group, criterion_main, Criterion};
use asn1_codecs::{uper::UperCodec, PerCodecData};
use rasn::uper;

fn rasn_sequence(c: &mut Criterion) {
    let data: Vec<u16> = vec![42; 100];
    c.bench_function("RASN/encode - simple sequence", |b| {
        b.iter(|| {
            let _ = uper::encode(&data).unwrap();
        })
    });
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct Elem(pub u16);

#[derive(asn1_codecs_derive::UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct BenchSequence(pub Vec<Elem>);

fn hampi_sequence(c: &mut Criterion) {
    let w = BenchSequence((0..100).map(|_| Elem(42)).collect());

    c.bench_function("HAMPI/encode - simple sequence", |b| {
        b.iter(|| {
            let mut data = PerCodecData::new_uper();
            w.uper_encode(&mut data).unwrap();
        })
    });
}

criterion_group!(benches, rasn_sequence, hampi_sequence);
criterion_main!(benches);