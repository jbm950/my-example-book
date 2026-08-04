use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("cmake-build");
    let install_dir = out_dir.join("cmake-install");

    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let build_type = if profile == "release" { "Release" } else { "Debug" };

    let status = Command::new("cmake")
        .args(["-S", "cpp", "-B"])
        .arg(&build_dir)
        .arg(format!("-DCMAKE_BUILD_TYPE={build_type}"))
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .status()
        .expect("failed to run cmake - is it installed and on PATH?");
    assert!(status.success(), "failed to configure CMake");

    let status = Command::new("cmake")
        .args(["--build"])
        .arg(&build_dir)
        .args(["--config", build_type])
        .status()
        .expect("failed to run cmake --build");
    assert!(status.success(), "failed to build using CMake");

    let status = Command::new("cmake")
        .args(["--install"])
        .arg(&build_dir)
        .args(["--config", build_type])
        .status()
        .expect("failed to run cmake --install");
    assert!(status.success(), "failed to install using CMake");

    println!("cargo::rustc-link-search=native={}", install_dir.join("lib").display());
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

