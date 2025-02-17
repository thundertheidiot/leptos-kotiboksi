use std::fs::write;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src");

    let _ = Command::new("/bin/sh")
        .args(["-c", "which leptosfmt &>/dev/null && leptosfmt ."])
        .spawn();

    let out_dir = std::env::var_os("OUT_DIR").unwrap(); // should be safe
    let path = Path::new(&out_dir).join("secret.rs");

    let secret = Command::new("/bin/sh")
        .arg("-c")
        // unfortunately the build is run twice, once for server, once for wasm, which means that something like this cannot be used
        // .arg(r"dd if=/dev/random count=64 bs=1 | od -An -td1 | tr -d ' \n' | sed 's/[^0-9]//g' | cut -c -16")
        .arg(r"date +%H%Y%I%u%V | sha256sum | awk '{print $1}'")
        .output()
        .unwrap()
        .stdout;
    let _ = write(
        &path,
        format!(
            "pub const SECRET: &str = \"{}\";",
            std::str::from_utf8(&secret).unwrap()
        ),
    );
}
