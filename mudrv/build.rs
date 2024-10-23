fn main() {
    use build_script_cfg::Cfg;
    use search_musa_tools::find_musa_home;
    use std::{env, path::PathBuf};

    println!("cargo:rereun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=MUSA_INSTALL_PATH");

    let musa = Cfg::new("detected_musa");
    let Some(musa_path) = find_musa_home() else {
        return;
    };
    musa.define();

    println!(
        "cargo:rustc-link-search=native={}",
        musa_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=musa");
    println!("cargo:rustc-link-lib=dylib=musart");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", musa_path.join("include").display()))
        .clang_arg("-include")
        .clang_arg("stdbool.h")
        .must_use_type("MUresult")
        .must_use_type("musaError_t")
        .allowlist_function("mu.*")
        .allowlist_item("MU.*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bingdings!");
}
