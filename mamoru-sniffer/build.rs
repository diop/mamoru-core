static SERVICES: &[&str] = &[
    "./proto/validation-chain/proto/validationchain/validationchain/tx.proto",
    "./proto/validation-chain/proto/validationchain/validationchain/query.proto",
];

static INCLUDES: &[&str] = &["./proto/validation-chain/proto/", "./proto/"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .extern_path(".cosmos", "::cosmrs::proto::cosmos")
        .include_file("includes.rs");

    builder = allow_deserialize_chain_from_str(builder);

    builder.compile(SERVICES, INCLUDES)?;

    Ok(())
}

fn allow_deserialize_chain_from_str(builder: tonic_build::Builder) -> tonic_build::Builder {
    builder
        .type_attribute(
            "validationchain.validationchain.Chain.ChainType",
            "#[derive(strum_macros::EnumString)]",
        )
        .type_attribute(
            "validationchain.validationchain.Chain.ChainType",
            "#[strum(serialize_all = \"SCREAMING_SNAKE_CASE\")]",
        )
}
