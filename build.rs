fn main() {
    if std::env::var("TARGET").unwrap().contains("x86_64-pc-windows") {
        println!("cargo:rustc-link-search=native={}/lib64", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    if std::env::var("TARGET").unwrap().contains("86-pc-windows") {
        println!("cargo:rustc-link-search=native={}/lib32", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    }
}