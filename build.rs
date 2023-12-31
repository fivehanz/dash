use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("api_descriptor.bin"),
        )
        .compile(&["proto/api.proto"], &["proto"])?;
    Ok(())
}
