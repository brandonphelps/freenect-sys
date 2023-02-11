use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("libfreenect");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=freenect");
    println!("cargo:rustc-link-lib=static=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=udev");

    let bindings = bindgen::Builder::default()
        .header("libfreenect/include/libfreenect.h")
        .clang_arg("-I./libfreenect/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
