// Part of https://github.com/rust-lang/rust/issues/97708 workaround
fn main() {
    println!("cargo::rustc-check-cfg=cfg(loom)");
    println!("cargo::rustc-check-cfg=cfg(maitake_ultraverbose)");
}
