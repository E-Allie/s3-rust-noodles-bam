use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    println!("Compiling protos...");

    //TODO: Add type_attributes during this phase, vs by hand annotation.

    prost_build::compile_protos(&["protos/reads.proto"], &["src/", "protos/"])?;

    Ok(())
}
