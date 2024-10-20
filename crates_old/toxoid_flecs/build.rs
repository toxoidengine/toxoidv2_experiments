// use std::path::{PathBuf, Path};

fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=flecs.h");
    println!("cargo:rerun-if-changed=flecs.c");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("emscripten") {
        // Get rid of the warning about unused command line arguments from emcc
        std::env::set_var("CFLAGS", "-Wno-unused-command-line-argument");
    };

    // Bindgen
    // let wasi_sdk_path = "C:/Users/troye/dev/toxoid/toxoidv2_experiments/crates/toxoid_flecs/wasi-sdk-24.0-x86_64-windows";
    // let clang_path = format!("{}/bin/clang", wasi_sdk_path);
    // let sysroot = format!("{}/share/wasi-sysroot", wasi_sdk_path);

    // Set environment variables for cc and cxx
    // std::env::set_var("CC", &clang_path);
    // std::env::set_var("CXX", &clang_path);
    
    // let bindings = bindgen::Builder::default()
    //     // .clang_arg("-DFLECS_CUSTOM_BUILD")
    //     // .clang_arg(format!("--sysroot={}", sysroot))
    //     // .clang_arg("--target=wasm32-wasi")
    //     // .clang_arg("-fno-exceptions")
    //     .header(Path::new("flecs.h").to_str().unwrap())
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from("./src");
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // Set the target to WASI
    // let target = "wasm32-wasi";
    // env::set_var("TARGET", target);

    // Set the compiler to use the WASI SDK
    // let wasi_sdk_path = "/path/to/wasi-sdk"; // Update this path
    // env::set_var("CC", format!("{}/bin/clang", wasi_sdk_path));
    // env::set_var("CXX", format!("{}/bin/clang++", wasi_sdk_path));
    // env::set_var("WASI_SDK_PATH", wasi_sdk_path);

    // Compile Flecs
    cc::Build::new()
        .define("FLECS_CUSTOM_BUILD", None)
        .include("flecs.h")
        .file("flecs.c")
        .compile("flecs_core");

    // cc::Build::new()
    //     .compiler(&clang_path)
    //     .define("FLECS_CUSTOM_BUILD", None)
    //     .include("flecs.h")
    //     .file("flecs.c")
    //     .flag(&format!("--sysroot={}", sysroot))
    //     .flag("--target=wasm32-wasi")
    //     .compile("flecs_core");
}
