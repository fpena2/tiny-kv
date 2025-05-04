use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Compile .proto files with descriptor file generation
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("storage_descriptor.bin"))
        .compile_protos(&["proto/storage.proto"], &["proto"])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("raft_descriptor.bin"))
        .compile_protos(&["proto/raft.proto"], &["proto"])?;

    Ok(())
}
