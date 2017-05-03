use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = format!("{}/RedPitaya", env::var("OUT_DIR").unwrap());

    Command::new("git")
        .arg(&"clone")
        .arg(&"https://github.com/RedPitaya/RedPitaya.git")
        .arg(&out_dir)
        .status().unwrap();

    let target = env::var("TARGET").unwrap();
    let prefix = match &target[..] {
        "aarch64-unknown-linux-gnu" => "aarch64-linux-gnu",
        "arm-unknown-linux-gnueabi" => "arm-linux-gnueabi",
        "arm-unknown-linux-gnueabihf"  => "arm-linux-gnueabihf",
        "armv7-unknown-linux-gnueabihf" => "arm-linux-gnueabihf",
        "arm-unknown-linux-musleabi" => "arm-linux-musleabi",
        "arm-unknown-linux-musleabihf"  => "arm-linux-musleabihf",
        "armv7-unknown-linux-musleabihf" => "arm-linux-musleabihf",
        "powerpc-unknown-linux-gnu" => "powerpc-linux-gnu",
        "powerpc64-unknown-linux-gnu" => "powerpc-linux-gnu",
        "powerpc64le-unknown-linux-gnu" => "powerpc64le-linux-gnu",
        "mips-unknown-linux-gnu" => "mips-linux-gnu",
        "mipsel-unknown-linux-gnu" => "mipsel-linux-gnu",
        "i686-pc-windows-gnu" => "i686-w64-mingw32",
        "x86_64-pc-windows-gnu" => "x86_64-w64-mingw32",
        "x86_64-unknown-linux-musl" => "musl",
        "x86_64-rumprun-netbsd" => "x86_64-rumprun-netbsd",
        _ => "",
    };

    Command::new("make")
        .arg(&format!("CROSS_COMPILE={}-", prefix))
        .arg("api")
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search={}/api/lib", out_dir);
    println!("cargo:rustc-link-lib=rp");
}
