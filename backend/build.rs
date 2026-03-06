fn main() {
    // Ensure that tree-sitter-rust builds properly
    // by letting its build script run
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // tree-sitter-rust build script should handle linking automatically
    // but we ensure the compiler can find the necessary headers
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-search=native=/usr/lib");
    }
}
