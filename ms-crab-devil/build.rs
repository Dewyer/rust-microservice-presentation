fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("interface/watermark_service.proto")?;
    Ok(())
}
