use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tinykv_descriptor.bin"))
        .compile_protos(&["proto/tinykv.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/tinykv.proto")?;

    Ok(())
}
