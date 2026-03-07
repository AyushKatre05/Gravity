use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Build tree-sitter-rust grammar only
    // (tree-sitter crate handles its own compilation)
    let ts_rust_dir = out_dir.join("tree-sitter-rust");
    if !ts_rust_dir.exists() {
        Command::new("git")
            .args(&["clone", "https://github.com/tree-sitter/tree-sitter-rust.git"])
            .arg(&ts_rust_dir)
            .output()
            .expect("Failed to clone tree-sitter-rust");
    }
    
    // Get tree-sitter include path from the tree-sitter crate
    let ts_include = PathBuf::from(env::var("DEP_TREE_SITTER_INCLUDE").unwrap_or_else(|_| "/usr/include".to_string()));
    
    // Compile tree-sitter-rust parser and scanner
    cc::Build::new()
        .file(ts_rust_dir.join("src/parser.c"))
        .file(ts_rust_dir.join("src/scanner.c"))
        .include(ts_rust_dir.join("src"))
        .include(&ts_include)
        .compile("tree_sitter_rust");
    
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=tree_sitter_rust");
}

