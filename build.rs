use std::env;
use std::path::PathBuf;

use coin_build_tools::coinbuilder;

const HEADER: &str = "Cbc_C_Interface.h";

fn main() {
    bindgen_lib();
}


fn bindgen_lib() {
    let (include, _) = coinbuilder::get_metadata_from("Cbc");

    let mut header_path = String::new();

    for i in &include {
        if PathBuf::from(i).join(HEADER).exists() {
            header_path = i.to_string() +"\\" +  HEADER;
            break;
        }
    }

    println!("{:?}", include);

    let clang_args = include
        .iter()
        .map(|dir| format!("-I{}", dir))
        .collect::<Vec<String>>();

    let bindings = bindgen::Builder::default()
        .header(&header_path)
        .clang_args(clang_args.iter())
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
