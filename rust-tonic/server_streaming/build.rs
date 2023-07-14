fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_grpc()?;
    Ok(())
}

fn build_grpc() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile_well_known_types(true)
        .compile(&["../../protos/server_streaming.proto"], &["../../protos"])?;
    Ok(())
}
