fn main() {
    let crates = [
        valdi_rust_ir::CRATE_ID,
        valdi_rust_backend::CRATE_ID,
        valdi_rust_runtime::CRATE_ID,
        valdi_rust_codegen::CRATE_ID,
    ];
    println!("valdi_rust foundation crates: {}", crates.join(","));
}
