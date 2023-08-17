use std::path::PathBuf;
use std::process::Command;

fn main() {
    if let Ok(docs_rs) = std::env::var("DOCS_RS") {
        // Detected build on `docs.rs`, so skip trying to link in Lean and just build docs
        if docs_rs == "1" {
            return;
        }
    }

    // Step 1: Get lean directory
    // TODO: support overriding lean command and/or directory via environment variables?
    let lean_output = Command::new("lean")
        .args(["--print-prefix"])
        .output()
        .expect("Failed to execute lean");
    if !lean_output.status.success() {
        panic!(
            "Command \"lean --print-prefix\" exited unsuccessfully: error {}",
            lean_output.status
        );
    }
    let lean_dir = PathBuf::from(String::from_utf8(lean_output.stdout)
        .expect("Path returned by \"lean --print-prefix\" is invalid UTF-8; this is currently not supported")
        .trim());
    let lib_dir = if cfg!(target_os = "windows") {
        lean_dir.join("bin")
    } else {
        // Default to Unix-like, of course
        lean_dir.join("lib/lean")
    };

    if !cfg!(feature = "static") {
        // Step 2: check libleanshared.so/libleanshared.dylib/libleanshared.dll is actually there, just for cleaner error messages
        let mut shared_lib = lib_dir.clone();
        let exists = if cfg!(target_os = "windows") {
            shared_lib.push("libleanshared.dll");
            shared_lib.exists()
        } else if cfg!(target_os = "macos") {
            shared_lib.push("libleanshared.dylib");
            shared_lib.exists()
        } else if cfg!(unix) {
            shared_lib.push("libleanshared.so");
            shared_lib.exists()
        } else {
            true
        };
        if !exists {
            panic!(
                "{} was not found. We errored, as this would probably cause a linking failure later",
                shared_lib.display()
            );
        }

        // Step 3: actually link with the library
        println!("cargo:rustc-link-search={}", lib_dir.display());
        println!("cargo:rustc-link-lib=leanshared");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    } else if cfg!(feature = "extern") {
        println!("cargo:rustc-link-search={}/lib/lean", lean_dir.display());
        println!("cargo:rustc-link-search={}/lib", lean_dir.display());
        for lib in ["Lean", "Init", "leanrt", "leancpp", "gmp", "c++", "c++abi"] {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    } else {
        println!("cargo:rustc-link-search={}/lib", lean_dir.display());
        println!("cargo:rustc-link-search={}/lib/lean", lean_dir.display());
        for libs in [["Lean", "leancpp"], ["Init", "leanrt"]] {
            println!("cargo:rustc-link-arg=-Wl,--start-group");
            for lib in libs {
                println!("cargo:rustc-link-lib=static={lib}");
            }
            println!("cargo:rustc-link-arg=-Wl,--end-group");
        }
        for lib in ["Lake", "c++", "c++abi"] {
            println!("cargo:rustc-link-lib=static={lib}");
        }
        for lib in ["m", "dl", "gmp"] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }
}
