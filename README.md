# Rust ASN.1 Codecs Benchmark

This project aims to benchmark two ASN1 PER (Packed Encoding Rules) codecs in Rust.

## Codecs used in comparision

The project currently includes the following two PER codecs:

1. [rasn, (https://github.com/librasn)] v0.14.0
2. [asn1-codecs, (https://github.com/ystero-dev/hampi)] v0.6.1

## Benchmark Results

The following tables present the benchmark results for various test cases:

### Encoding SEQUENCE-OF 100 integers (u16)

| Codec        | Encoding (µs)  |
|--------------|----------------|
| rasn         | 39.6           |
| asn1-codecs  | 7.5            |

### Encoding and decoding nested SEQUENCE-OF containing SEQUENCE type
For more details regarding ASN1 definition, refer to  [sample.asn](src/sample.asn)

| Codec        | Encoding (µs)  | Decoding (µs)  |
|--------------|----------------|----------------|
| rasn         | 1916.7         | 826.8          |
| asn1-codecs  | 208            | 145.5          |

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