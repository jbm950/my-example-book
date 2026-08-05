use std::env;

fn main() {
    let dst = cmake::Config::new("cpp").build();

    println!("cargo::rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo::rustc-link-lib=static=hello");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => println!("cargo::rustc-link-lib=dylib=c++"),
        "linux" => println!("cargo::rustc-link-lib=dylib=stdc++"),
        _ => {}, // MSVC links its C++ runtime automatically
    }

    println!("cargo::rerun-if-changed=cpp");
    println!("cargo::rerun-if-changed=build.rs");
}

