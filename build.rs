/* @file

    Build configuration script of RUCI.
    SPDX-License-Identifier: WTFPL

*/
fn main() {
    // Rust toolchain bug
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap()=="musl" {
        println!("cargo::rustc-link-arg=-Wl,--dynamic-linker=/lib/ld-musl-{}.so.1",
                 std::env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    }
    // LibUCI C libraries
    println!("cargo::rustc-link-arg=-luci");
}