use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_out = "src/zproto";
    fs::create_dir_all(proto_out)?;

    tonic_build::configure()
        .out_dir(proto_out)
        .compile(&["proto/ZProto.proto"], &["proto"])?;

    // Build the C++ matching engine
    cc::Build::new()
        .cpp(true) // Switch to C++ compiler
        .file("src/cpp/match_engine.cpp")
        .compile("match_engine"); // Links -> libmatch_engine.a

    // Re-run if C++ code changes
    println!("cargo:rerun-if-changed=src/cpp/match_engine.cpp");

    Ok(())
}
