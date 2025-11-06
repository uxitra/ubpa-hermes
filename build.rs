use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=static/html/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");

    let npx_available = Command::new("npx")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !npx_available {
        println!("cargo:warning=`npx` not found in PATH — skipping Tailwind build.");
        return;
    }

    // Run tailwind build command
    let status = Command::new("npx")
        .args([
            "@tailwindcss/cli",
            "-i",
            "./static/html/input.css",
            "-o",
            "./static/html/output.css",
            "--minify",
        ])
        .status()
        .expect("Failed to run Tailwind build command");

    if !status.success() {
        panic!("Tailwind build failed!");
    }
}
