use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let libnftables = pkg_config::Config::new()
        .probe("libnftables")
        .expect("Didn't find libnftables!");

    let header_path = libnftables
        .include_paths
        .iter()
        .map(|path| {
            let mut header_path = path.clone();
            header_path.push("nftables/libnftables.h");
            header_path
        })
        .find(|path| {
            path.try_exists()
                .expect("Unable to test include header existence")
        })
        .expect("Did not find include header")
        .into_os_string()
        .into_string()
        .expect("Found header path is not a valid UTF-8 string!");

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
