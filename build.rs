use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asn_path = "./asn/sample.asn";
    let telco_sample_asn_path = "./asn/telco_sample.asn";

    // RASN code generation
    use rasn_compiler::OutputMode;
    use rasn_compiler::prelude::*;
    let out_dir = env::var("OUT_DIR")?;

    // Generate code for sample.asn
    Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path([asn_path].iter())
        .set_output_mode(OutputMode::SingleFile(
            PathBuf::from(&out_dir).join("sample_rasn.rs"),
        ))
        .compile()
        .expect("Error during compilation");

    // Generate code for telco_sample.asn
    Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path([telco_sample_asn_path].iter())
        .set_output_mode(OutputMode::SingleFile(
            PathBuf::from(&out_dir).join("telco_sample_rasn.rs"),
        ))
        .compile()
        .expect("Error during compilation of telco_sample.asn");

    // asn1-codecs code generation
    use asn1_compiler::{
        Asn1Compiler,
        generator::{Codec, Derive, Visibility},
    };

    // Generate code for sample.asn
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

    // Generate code for telco_sample.asn
    let telco_sample_rs_module_path = PathBuf::from(&out_dir).join("telco_sample_hampi.rs");
    let mut telco_sample_compiler = Asn1Compiler::new(
        telco_sample_rs_module_path
            .to_str()
            .expect("Failed to convert path to string"),
        &Visibility::Public,
        vec![Codec::Uper],
        vec![Derive::Debug],
    );
    telco_sample_compiler.compile_files(&[telco_sample_asn_path])?;

    // asn1rs code generation
    use asn1rs::converter::Converter;
    use asn1rs::r#gen::rust::RustCodeGenerator;

    // Generate code for sample.asn
    let mut converter = Converter::default();

    if let Err(e) = converter.load_file(asn_path) {
        panic!("Loading of .asn1 file failed {:?}", e);
    }

    if let Err(e) = converter.to_rust(&out_dir, |_generator: &mut RustCodeGenerator| {}) {
        panic!("Conversion to rust failed: {:?}", e);
    }

    // Generate code for telco_sample.asn
    let mut telco_sample_converter = Converter::default();

    if let Err(e) = telco_sample_converter.load_file(telco_sample_asn_path) {
        panic!("Loading of telco_sample.asn file failed {:?}", e);
    }

    if let Err(e) =
        telco_sample_converter.to_rust(&out_dir, |_generator: &mut RustCodeGenerator| {})
    {
        panic!("Conversion of telco_sample.asn to rust failed: {:?}", e);
    }

    Ok(())
}
