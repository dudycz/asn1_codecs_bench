mod simple_asn1rs;
use asn1_codecs::{PerCodecData, uper::UperCodec};
use criterion::{Criterion, criterion_group, criterion_main};
use rasn::uper;

fn rasn_sequence(c: &mut Criterion) {
    let data: Vec<u64> = vec![42; 1000];
    c.bench_function("rasn/encode - simple sequence", |b| {
        b.iter(|| {
            let _ = uper::encode(&data).unwrap();
        })
    });
}

// asn1-codecs
#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER")]
pub struct Elem(pub u64);

#[derive(asn1_codecs_derive::UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF")]
pub struct BenchSequence(pub Vec<Elem>);

fn hampi_sequence(c: &mut Criterion) {
    let w = BenchSequence((0..1000).map(|_| Elem(42)).collect());

    c.bench_function("asn1-codecs/encode - simple sequence", |b| {
        b.iter(|| {
            let mut data = PerCodecData::new_uper();
            w.uper_encode(&mut data).unwrap();
        })
    });
}

fn asn1rs_sequence(c: &mut Criterion) {
    use asn1rs::prelude::*;
    use simple_asn1rs::{SequenceOfBasicInteger, SimpleInteger};
    let data = SequenceOfBasicInteger((0..1000).map(|_| SimpleInteger(42)).collect());
    let mut writer = UperWriter::default();
    c.bench_function("asn1rs/encode - simple sequence", |b| {
        b.iter(|| {
            writer.write(&data).unwrap();
        })
    });
}

criterion_group!(benches, rasn_sequence, hampi_sequence, asn1rs_sequence);
criterion_main!(benches);
