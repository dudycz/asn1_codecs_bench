# Rust ASN.1 Codecs Benchmark

This project aims to benchmark existing ASN1 PER (Packed Encoding Rules) codecs in Rust.

## Codecs used in comparision

The project currently includes the following UPER codecs:

1. [rasn](https://github.com/librasn) v0.28
2. [asn1-codecs](https://github.com/ystero-dev/hampi) v0.7.2
3. [asn1rs](https://github.com/kellerkindt/asn1rs) v0.3.1

## Benchmark Results

System configuration: Ubuntu 24.04, Rustc: 1.92.0, AMD Ryzen 5850U 32GB RAM.

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
| rasn         | 37.1          | 6.6           |
| asn1-codecs  | 5.7           | 4.9           |
| asn1rs       | 2.5           | 2.9           |

### Encoding and decoding 3D array (10×10×10 Color structs, 5 bytes each)

For more details regarding ASN1 definition, refer to [sample.asn](asn/sample.asn)

| Codec        | Encoding (µs) | Decoding (µs) |
|--------------|--------------:|--------------:|
| rasn         | 958           | 103           |
| asn1-codecs  | 180           | 49            |
| asn1rs       | 69            | 74            |


### Encoding flat SEQUENCE-OF 1000 integers (u64)

| Codec        | Encoding (µs) |
|--------------|--------------:|
| rasn         | 459           |
| asn1-codecs  | 52            |
| asn1rs       | 37            |

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
