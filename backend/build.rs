fn main() {
    // For now, we're using regex-based parsing instead of tree-sitter
    // This eliminates FFI complexity
    println!("cargo:rerun-if-changed=build.rs");
}

