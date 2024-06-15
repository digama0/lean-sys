use std::path::PathBuf;
use std::process::Command;

fn cargo_add_ld_options() {
    // Run the command 'leanc --print-ldflags'
    let output = Command::new("leanc")
        .arg("--print-ldflags")
        .output()
        .expect("Failed to execute leanc command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the output to a string
        let output_str = String::from_utf8(output.stdout).expect("Invalid UTF-8 in output");

        // Split the output by '-L' and get the part after '-L'
        let parts: Vec<&str> = output_str.split("-L").collect();
        if parts.len() < 2 {
            eprintln!("Error: No ldflags found after -L");
            return;
        }
        let ld_search_path = parts[1].split_whitespace().next().unwrap_or_default();

        // Pass search_path to Cargo as arguments
        println!("cargo:rustc-link-search=native={}", ld_search_path);

        let lib_names : Vec<&str> = parts[1].split(" ").skip(2).map(|part| {
            // drop the '-l' part.
            part.split_whitespace().next().unwrap_or_default().strip_prefix("-l").unwrap_or_default()

        }).collect();

        for lib_name in lib_names.iter() {
            println!("cargo:rustc-link-lib={}", lib_name);
        }
        println!("cargo::warning=error: lib names: {:?}", lib_names);
        
    } else {
        // Print an error message if the command failed
        println!("cargo::warning='Error: leanc command failed with status {:?}", output.status);
    }
}


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

    println!("cargo:rustc-link-search={}/lib", lean_dir.display());
    println!("cargo:rustc-link-search={}/lib/lean", lean_dir.display());

    cargo_add_ld_options();
    
}
