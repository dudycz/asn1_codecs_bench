# Rust ASN.1 Codecs Benchmark

This project aims to benchmark existing ASN1 PER (Packed Encoding Rules) codecs in Rust.

## Codecs used in comparision

The project currently includes the following UPER codecs:

1. [rasn](https://github.com/librasn) v0.22.0
2. [asn1-codecs](https://github.com/ystero-dev/hampi) v0.7.0
3. [asn1rs](https://github.com/kellerkindt/asn1rs) v0.3.1

## Benchmark Results
System configuration: Ubuntu 24.04, Rustc: 1.81, AMD Ryzen 5850U 32GB RAM.
### Encoding and decoding nested SEQUENCE-OF (1000 elements, 5 bytes each)
For more details regarding ASN1 definition, refer to  [sample.asn](src/sample.asn)

| Codec        | Encoding (µs)  | Decoding (µs)  |
|--------------|----------------|----------------|
| rasn         | 1043           | 123            |
| asn1-codecs  | 150            | 52             |
| asn1rs       | 83             | 73             |

### Encoding flat SEQUENCE-OF 1000 integers (u64)

| Codec        | Encoding (µs)  |
|--------------|----------------|
| rasn         | 440            |
| asn1-codecs  | 59             |
| asn1rs       | 34             |

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
