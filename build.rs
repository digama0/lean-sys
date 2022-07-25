fn main() {
    //TODO: fix this... this is not good
    println!("cargo:rustc-link-search=/home/tekne/.elan/toolchains/leanprover--lean4---nightly/lib/lean");
    println!("cargo:rustc-link-lib=leanshared");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/tekne/.elan/toolchains/leanprover--lean4---nightly/lib/lean");
}