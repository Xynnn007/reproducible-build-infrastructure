fn main() -> shadow_rs::SdResult<()> {
    tonic_build::compile_protos("../../proto/rbiservice.proto")?;
    shadow_rs::new()
}