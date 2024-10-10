fn main() {
    println!("cargo:rerun-if-changed=migrations"); // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#stable-rust-cargo-build-script
    tauri_build::build()
}
