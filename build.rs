extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc");
    }

    // println!("cargo:rustc-link-search=native=./bind");
    // println!("cargo:rustc-link-lib=dylib=odin_lib")
}
