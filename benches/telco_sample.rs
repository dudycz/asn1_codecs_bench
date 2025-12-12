use asn1_codecs_bench::*;
use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! benchmark_encode {
    ($c:expr, $name:expr, $build_sample:expr, $encode:expr) => {
        $c.bench_function($name, |b| {
            let w = $build_sample();
            b.iter(|| {
                let _ = $encode(&w);
            })
        });
    };
}

macro_rules! benchmark_decode {
    ($c:expr, $name:expr, $build_sample:expr, $encode:expr, $decode:expr) => {
        let w = $build_sample();
        let encoded = $encode(&w);

        $c.bench_function($name, |b| {
            b.iter(|| {
                let _ = $decode(&encoded);
            })
        });
    };
}

fn run_benchmarks(c: &mut Criterion) {
    benchmark_encode!(
        c,
        "asn1rs/encode - telco_sample.asn",
        build_telco_sample_asn1rs,
        encode_telco_sample_asn1rs
    );
    benchmark_decode!(
        c,
        "asn1rs/decode - telco_sample.asn",
        build_telco_sample_asn1rs,
        encode_telco_sample_asn1rs,
        decode_telco_sample_asn1rs
    );
    benchmark_encode!(
        c,
        "rasn/encode - telco_sample.asn",
        build_telco_sample_rasn,
        encode_telco_sample_rasn
    );
    benchmark_decode!(
        c,
        "rasn/decode - telco_sample.asn",
        build_telco_sample_rasn,
        encode_telco_sample_rasn,
        decode_telco_sample_rasn
    );
    benchmark_encode!(
        c,
        "asn1-codecs/encode - telco_sample.asn",
        build_telco_sample_hampi,
        encode_telco_sample_hampi
    );
    benchmark_decode!(
        c,
        "asn1-codecs/decode - telco_sample.asn",
        build_telco_sample_hampi,
        encode_telco_sample_hampi,
        decode_telco_sample_hampi
    );
}

criterion_group!(benches, run_benchmarks);
criterion_main!(benches);
