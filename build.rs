
/**
 * This is the main build function to build proto files.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}