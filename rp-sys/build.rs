#[cfg(feature = "mock")]
fn main() {}

#[cfg(not(feature = "mock"))]
fn main() {
    let out_dir = format!("{}/RedPitaya", std::env::var("OUT_DIR").unwrap());

    build_rp(&out_dir);
    bindgen(&out_dir);
}

#[cfg(not(feature = "mock"))]
fn build_rp(out_dir: &str) {
    std::process::Command::new("git")
        .arg("clone")
        .arg("https://github.com/RedPitaya/RedPitaya.git")
        .arg(out_dir)
        .status()
        .unwrap();

    let version = if cfg!(feature = "v1_04") {
        "v1.04-25"
    } else if cfg!(feature = "v1_03") {
        "v1.03-701"
    } else if cfg!(feature = "v1_00") {
        "v1.00-26"
    } else if cfg!(feature = "v0_98") {
        "v0.98"
    } else {
        panic!("Invalid version");
    };

    std::process::Command::new("git")
        .current_dir(out_dir)
        .arg("checkout")
        .arg(version)
        .status()
        .unwrap();

    let target = std::env::var("TARGET").unwrap();
    let prefix = match &target[..] {
        "aarch64-unknown-linux-gnu" => "aarch64-linux-gnu",
        "arm-unknown-linux-gnueabi" => "arm-linux-gnueabi",
        "arm-unknown-linux-gnueabihf" => "arm-linux-gnueabihf",
        "armv7-unknown-linux-gnueabihf" => "arm-linux-gnueabihf",
        "arm-unknown-linux-musleabi" => "arm-linux-musleabi",
        "arm-unknown-linux-musleabihf" => "arm-linux-musleabihf",
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

    std::process::Command::new("make")
        .arg(&format!("CROSS_COMPILE={prefix}-"))
        .arg("api")
        .current_dir(out_dir)
        .status()
        .unwrap();

    println!("cargo:rustc-link-search={out_dir}/api/lib");
    println!("cargo:rustc-link-lib=rp");
}

#[cfg(not(feature = "mock"))]
fn bindgen(out_dir: &str) {
    #[cfg(not(feature = "v1_04"))]
    let header = "api/include/redpitaya/rp.h";
    #[cfg(feature = "v1_04")]
    let header = "rp-api/api/include/redpitaya/rp.h";

    let contents = format!("#include \"{out_dir}/{header}\"");

    let bindings = bindgen::Builder::default()
        .rustified_enum(".*")
        .header_contents("wrapper.h", &contents)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
