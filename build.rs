use std::env;
use std::path::{Path, PathBuf};

fn generate_rasn(asn_path: &str, out_dir: &Path, out_name: &str) {
    use rasn_compiler::OutputMode;
    use rasn_compiler::prelude::*;

    Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path([asn_path].iter())
        .set_output_mode(OutputMode::SingleFile(out_dir.join(out_name)))
        .compile()
        .unwrap_or_else(|_| panic!("rasn compilation failed for {asn_path}"));
}

fn generate_hampi(asn_path: &str, out_dir: &Path, out_name: &str) {
    use asn1_compiler::{
        Asn1Compiler,
        generator::{Codec, Derive, Visibility},
    };

    let rs_module_path = out_dir.join(out_name);
    let mut compiler = Asn1Compiler::new(
        rs_module_path
            .to_str()
            .expect("Failed to convert path to string"),
        &Visibility::Public,
        vec![Codec::Uper],
        vec![Derive::Debug],
    );
    compiler
        .compile_files(&[asn_path])
        .unwrap_or_else(|e| panic!("asn1-codecs compilation failed for {asn_path}: {e:?}"));
}

fn generate_asn1rs(asn_path: &str, out_dir: &str) {
    use asn1rs::converter::Converter;
    use asn1rs::r#gen::rust::RustCodeGenerator;

    let mut converter = Converter::default();
    if let Err(e) = converter.load_file(asn_path) {
        panic!("Loading of {asn_path} failed {:?}", e);
    }
    if let Err(e) = converter.to_rust(out_dir, |_generator: &mut RustCodeGenerator| {}) {
        panic!("Conversion of {asn_path} to rust failed: {:?}", e);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asn_files = ["./asn/sample.asn", "./asn/telco_sample.asn"];

    println!("cargo:rerun-if-changed=build.rs");
    for f in &asn_files {
        println!("cargo:rerun-if-changed={f}");
    }

    let out_dir = env::var("OUT_DIR")?;
    let out_dir_path = PathBuf::from(&out_dir);

    generate_rasn(asn_files[0], &out_dir_path, "sample_rasn.rs");
    generate_rasn(asn_files[1], &out_dir_path, "telco_sample_rasn.rs");

    generate_hampi(asn_files[0], &out_dir_path, "sample_hampi.rs");
    generate_hampi(asn_files[1], &out_dir_path, "telco_sample_hampi.rs");

    for f in &asn_files {
        generate_asn1rs(f, &out_dir);
    }

    Ok(())
}
