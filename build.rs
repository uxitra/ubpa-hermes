use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=static/html/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");


    // Run tailwind build command
    let status = Command::new("npx")
        .args([
            "@tailwindcss/cli",
            "-i", "./static/html/input.css",
            "-o", "./static/html/output.css",
            "--minify",
        ])
        .status()
        .expect("Failed to run Tailwind build command");

    if !status.success() {
        panic!("Tailwind build failed!");
    }
}
