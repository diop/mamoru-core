use cosmrs::bip32::{Mnemonic, XPrv};
use cosmrs::crypto::secp256k1;
use rand_core::OsRng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate random Mnemonic using the default language (English)
    let mnemonic = Mnemonic::random(OsRng, Default::default());
    let seed = mnemonic.to_seed("");

    // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/44'/118'/0'/0/0";
    let xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;
    let private_key = secp256k1::SigningKey::from(&xprv);
    let public_key = private_key.public_key();

    println!("Address: {}", public_key.account_id("cosmos")?);
    println!("Pub key: {}", public_key.to_string());
    println!("Private key (base64): {}", base64::encode(xprv.to_bytes()));

    Ok(())
}
