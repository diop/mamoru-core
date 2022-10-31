static SERVICES: &[&str] = &[
    "./proto/validation-chain/proto/validationchain/tx.proto",
    "./proto/validation-chain/proto/validationchain/query.proto",
];

static INCLUDES: &[&str] = &["./proto/validation-chain/proto/", "./proto/"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .extern_path(".cosmos", "::cosmrs::proto::cosmos")
        .include_file("includes.rs")
        .compile(SERVICES, INCLUDES)?;

    Ok(())
}
