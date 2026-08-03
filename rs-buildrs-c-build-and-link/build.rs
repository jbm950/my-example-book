use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("gcc")
        .args(["-c", "c/hello.c", "-o"])
        .arg(format!("{out_dir}/hello.o"))
        .status()
        .unwrap();

    assert!(status.success(), "failed to compile c/hello.c");

    println!("cargo::rustc-link-arg={out_dir}/hello.o");
    println!("cargo::rerun-if-changed=c/hello.c");
}
