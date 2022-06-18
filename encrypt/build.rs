use cc::Build;

fn main() {
    let dyn_libs = ["crypt32"];
    dyn_libs.map(|lib| println!("cargo:rustc-link-lib={}", lib));

    Build::new()
        // .include("D:/Windows Kits/10/Include/10.0.19041.0/shared")
        .file("src/windows.cc")
        .static_crt(true)
        .compile("encrypt");
}
