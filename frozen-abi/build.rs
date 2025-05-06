extern crate rustc_version;
//use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo:rustc-cfg=RUSTC_WITH_SPECIALIZATION");
//            println!("cargo:rustc-cfg=RUSTC_NEEDS_PROC_MACRO_HYGIENE");

}
