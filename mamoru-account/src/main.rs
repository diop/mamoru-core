use std::error::Error;

use argh::FromArgs;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use cosmrs::{
    bip32::{Mnemonic, XPrv},
    crypto::secp256k1,
};
use rand_core::OsRng;

/// Mamoru Account generator
#[derive(FromArgs, PartialEq, Debug)]
struct Cli {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Generate(Generate),
    Info(Info),
    Migrate(Migrate),
}

/// Generates new account
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
struct Generate {}

/// Prints account info
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "info")]
struct Info {
    /// the private key in base64 format
    #[argh(option)]
    private_key: String,
}

/// Shows an address with an old and a new address
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "migrate")]
struct Migrate {
    /// the old account prefix
    #[argh(positional)]
    old_prefix: String,

    /// the new account prefix
    #[argh(positional)]
    new_prefix: String,

    /// the private key in base64 format
    #[argh(positional)]
    private_key: String,
}

const ACCOUNT_PREFIX: &str = "mamoru";
const DERIVE_PATH: &str = "m/44'/118'/0'/0/0";

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    match cli.command {
        Command::Generate(_options) => {
            // Generate random Mnemonic using the default language (English)
            let mnemonic = Mnemonic::random(OsRng, Default::default());
            let seed = mnemonic.to_seed("");

            // Derive a child `XPrv` using the provided BIP32 derivation path
            let xprv = XPrv::derive_from_path(&seed, &DERIVE_PATH.parse()?)?;
            let private_key = secp256k1::SigningKey::from(&xprv);
            let public_key = private_key.public_key();

            println!("Address: {}", public_key.account_id(ACCOUNT_PREFIX)?);
            println!("Pub key: {}", public_key.to_string());
            println!("Private key (base64): {}", STANDARD.encode(xprv.to_bytes()));
            println!("Mnemonic: {}", mnemonic.phrase());
        }
        Command::Info(options) => {
            let key_bytes = STANDARD.decode(options.private_key)?;
            let private_key = secp256k1::SigningKey::try_from(key_bytes.as_slice())?;
            let public_key = private_key.public_key();

            println!("Address: {}", public_key.account_id(ACCOUNT_PREFIX)?);
            println!("Pub key: {}", public_key.to_string());
        }
        Command::Migrate(options) => {
            let key_bytes = STANDARD.decode(options.private_key)?;
            let private_key = secp256k1::SigningKey::try_from(key_bytes.as_slice())?;
            let public_key = private_key.public_key();

            println!(
                "Old address: {}",
                public_key.account_id(&options.old_prefix)?
            );
            println!("Address: {}", public_key.account_id(&options.new_prefix)?);
        }
    }

    Ok(())
}
