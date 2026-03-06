fn main() {
    // Let tree-sitter-rust handle its own build
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

