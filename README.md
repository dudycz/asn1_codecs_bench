# Rust ASN.1 Codecs Benchmark

[![CI](https://github.com/dudycz/asn1_codecs_bench/actions/workflows/rust.yml/badge.svg)](https://github.com/dudycz/asn1_codecs_bench/actions/workflows/rust.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust%20Edition-2024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

This project aims to benchmark existing ASN1 PER (Packed Encoding Rules) codecs in Rust.

## Codecs used in comparision

The project currently includes the following UPER codecs:

1. [rasn](https://github.com/librasn) v0.28
2. [asn1-codecs](https://github.com/ystero-dev/hampi) v0.7.2
3. [asn1rs](https://github.com/kellerkindt/asn1rs) v0.3.1

## Benchmark Results

System configuration: Ubuntu 24.04, Rustc: 1.95.0, AMD Ryzen 5850U 32GB RAM.

### Encoding and decoding sample telecom protocol message

For more details regarding ASN1 definition, refer to [telco_sample.asn](asn/telco_sample.asn)

This benchmark uses a generic telecom-inspired protocol with realistic features (~377 bytes encoded):
- CHOICE types (PDU with Request variant)
- ENUMERATED types (PriorityLevel, RecordType)
- OPTIONAL fields (timestamps, addresses, metadata)
- Variable-length SEQUENCE-OF (20 records)
- OCTET STRING fields (identifiers, payloads)
- Nested structures with BOOLEAN and INTEGER fields

| Codec        | Encoding (µs) | Decoding (µs) |
|--------------|--------------:|--------------:|
| rasn         | 38.3          | 6.5           |
| asn1-codecs  | 5.4           | 5.4           |
| asn1rs       | 2.5           | 2.8           |

### Encoding and decoding 3D array (10×10×10 Color structs, 5 bytes each)

For more details regarding ASN1 definition, refer to [sample.asn](asn/sample.asn)

| Codec        | Encoding (µs) | Decoding (µs) |
|--------------|--------------:|--------------:|
| rasn         | 1001.0        | 109.7         |
| asn1-codecs  | 156.5         | 49.0          |
| asn1rs       | 69.2          | 74.2          |


### Encoding flat SEQUENCE-OF 1000 integers (u64)

| Codec        | Encoding (µs) |
|--------------|--------------:|
| rasn         | 442.3         |
| asn1-codecs  | 49.9          |
| asn1rs       | 37.9          |

## Usage

To run the benchmarks, follow these steps:

```
cargo build
cargo bench
```

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

## License

This project is licensed under the [Apache Licence, version 2.0](LICENSE).
