fn main() {
    // Ensure that tree-sitter-rust builds properly
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

