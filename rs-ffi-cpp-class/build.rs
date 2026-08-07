use std::env;

fn main() {
    let dst = cmake::Config::new(".").build();

    println!("cargo::rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo::rustc-link-lib=static=wrapper");
    println!("cargo::rustc-link-lib=static=counter");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => println!("cargo::rustc-link-lib=dylib=c++"),
        "linux" => println!("cargo::rustc-link-lib=dylib=stdc++"),
        _ => {}, // MSVC links its C++ runtime automatically
    }

    println!("cargo::rerun-if-changed=ext");
    println!("cargo::rerun-if-changed=wrapper");
    println!("cargo::rerun-if-changed=CMakeLists.txt");
    println!("cargo::rerun-if-changed=build.rs");
}

