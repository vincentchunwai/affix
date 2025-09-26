use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=headers/wrapper.h"); // rebuild if wrapper.h changes

    // Rustc link targets
    pkg_config::Config::new()
        .probe("wayland-client")
        .expect("wayland-client not found");
    pkg_config::Config::new()
        .probe("wayland-server")
        .expect("wayland-server not found");
    pkg_config::Config::new()
        .probe("wayland-egl")
        .expect("wayland-egl not found");

    let bindings = bindgen::Builder::default()
        .header("headers/wrapper.h")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/wrapper/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
