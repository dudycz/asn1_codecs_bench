use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asn_path = "./asn/sample.asn";

    // RASN code generation
    use rasn_compiler::prelude::*;
    let out_dir = env::var("OUT_DIR")?;
    Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path([asn_path].iter())
        .set_output_path(PathBuf::from(&out_dir).join("sample_rasn.rs"))
        .compile()
        .expect("Error during compilation");

    // asn1-codecs code generation
    use asn1_compiler::{
        generator::{Codec, Derive, Visibility},
        Asn1Compiler,
    };
    let rs_module_path = PathBuf::from(&out_dir).join("sample_hampi.rs");
    let mut compiler = Asn1Compiler::new(
        rs_module_path
            .to_str()
            .expect("Failed to convert path to string"),
        &Visibility::Public,
        vec![Codec::Uper],
        vec![Derive::Debug],
    );
    compiler.compile_files(&[asn_path])?;

    // asn1rs code generation
    use asn1rs::converter::Converter;
    use asn1rs::gen::rust::RustCodeGenerator;
    let mut converter = Converter::default();

    if let Err(e) = converter.load_file(asn_path) {
        panic!("Loading of .asn1 file failed {:?}", e);
    }

    if let Err(e) = converter.to_rust(out_dir, |_generator: &mut RustCodeGenerator| {}) {
        panic!("Conversion to rust failed: {:?}", e);
    }

    Ok(())
}
