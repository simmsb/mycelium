fn main() {
    println!("cargo::rustc-check-cfg=cfg(trace_macros)");
    println!("cargo::rustc-check-cfg=cfg(loom)");
}
