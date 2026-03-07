fn main() {
    // Link against the tree-sitter libraries
    // They are built and installed in the Docker image
    println!("cargo:rustc-link-lib=dylib=tree_sitter");
    println!("cargo:rustc-link-lib=dylib=tree_sitter_rust");
    
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

