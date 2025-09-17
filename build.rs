use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    // Generate prost code into src/pb so the crate compiles without an external codegen step
    config.out_dir("src/pb");
    config.compile_protos(
        &["proto/common.proto", "proto/dex_trade_event.proto"],
        &["proto"],
    )?;
    println!("cargo:rerun-if-changed=proto/common.proto");
    println!("cargo:rerun-if-changed=proto/dex_trade_event.proto");
    Ok(())
}