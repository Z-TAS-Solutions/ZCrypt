use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_out = "src/zproto";
    fs::create_dir_all(proto_out)?;

    tonic_build::configure()
        .out_dir(proto_out)
        .compile(&["proto/ZProto.proto"], &["proto"])?;
    Ok(())
}
