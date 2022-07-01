extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;


fn main() {
    // link the system ecdsautils shared library.
   println!("cargo:rustc-link-lib=ecdsautil");

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .allowlist_function("ecdsa_.*")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    /* Write the bindings to the $OUT_DIR/bindings.rs file.
     *
     * Contains a hack to get rid of __uint8_t and __uint32_t:
     *
     * While bindgen is able to replace the C uint types with the correct Rust types,
     * it still generates typedefs for __uint{8,32}_t [0].  We don’t need these
     * typedefs, so we remove them manually.
     *
     * [0] https://github.com/rust-lang/rust-bindgen/issues/1663
     */
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let raw_file = out_path.join("bindings.rs.raw");
    bindings
        .write_to_file(&raw_file)
        .expect("Couldn't write bindings!");

    let raw = BufReader::new(File::open(&raw_file).unwrap());

    let mut finalized = File::create(out_path.join("bindings.rs")).unwrap();


    let droplines = vec!["pub type __uint8_t = ::std::os::raw::c_uchar;", "pub type __uint32_t = ::std::os::raw::c_uint;"];

    for line in raw.lines() {
        if let Ok(ip) = line {
            if !droplines.contains(&ip.as_str()) {
                writeln!(&mut finalized, "{}", ip).unwrap();
            }
        }
    }

}
