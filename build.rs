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

    // To find libc++ on macOS, We must find the path to XCode, which apple enjoys
    // changing each macOS release.
    // Thus, we do this in a robust fashion by querying `xcrun`.
    if cfg!(target_os = "macos") {
        let xcrun_output = Command::new("xcrun")
          .args(["--show-sdk-path"])
          .output()
          .expect("failed to execute `xcrun --show-sdk-path`, which is used to find the location of MacOS platform libraries. Please ensure that XCode is installed.");
        let libcpp_path = PathBuf::from(String::from_utf8(xcrun_output.stdout)
            .expect("Path returned by `xcrun --show-sdk-path` is invalid UTF-8. This must never happen.").trim())
          .join("usr/lib");
        println!("cargo:rustc-link-search={}", libcpp_path.display());
    }


    // if cfg!(feature = "static") {
    //     // Step 2: check libleanshared.so/libleanshared.dylib/libleanshared.dll is actually there, just for cleaner error messages
    //     let mut shared_lib = lib_dir.clone();
    //     let exists = if cfg!(target_os = "windows") {
    //         shared_lib.push("libleanshared.dll");
    //         shared_lib.exists()
    //     } else if cfg!(target_os = "macos") {
    //         shared_lib.push("libleanshared.dylib");
    //         shared_lib.exists()
    //     } else if cfg!(unix) {
    //         shared_lib.push("libleanshared.so");
    //         shared_lib.exists()
    //     } else {
    //         true
    //     };
    //     if !exists {
    //         panic!(
    //             "{} was not found. We errored, as this would probably cause a linking failure later",
    //             shared_lib.display()
    //         );
    //     }

    //     // Step 3: actually link with the library
    //     println!("cargo:rustc-link-search={}", lib_dir.display());
    //     println!("cargo:rustc-link-lib=leanshared");
    //     println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    // } else if cfg!(feature = "extern") {
    if cfg!(feature = "extern") {
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

        if cfg!(target_os = "macos") {
            // Static linking against libc++(abi) on MacOS is a total shitshow, so just don't.
            for lib in ["Lake"] {
                println!("cargo:rustc-link-lib=static={lib}");
            }
            for lib in ["c++", "c++abi"] {
                println!("cargo:rustc-link-lib=dylib={lib}");
            }

        } else {
            for lib in ["Lake", "c++", "c++abi"] {
                println!("cargo:rustc-link-lib=static={lib}");
            }
        }
        for lib in ["m", "dl", "gmp"] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }
}
