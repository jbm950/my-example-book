use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ptx_file = out_dir.join("hello_world.ptx");

    let status = Command::new("nvcc")
        .args(["-ptx", "kernels/hello_world.cu", "-o"])
        .arg(&ptx_file)
        .status()
        .expect("Failed to execute nvcc");

    assert!(status.success(), "nvcc failed to compile hello_world.cu");

    println!("cargo:rerun-if-changed=kernels");
    println!("cargo:rerun-if-changed=build.rs");
}
