use std::fs;
use std::path::Path;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};
use rsa::sha2::Sha256;

pub fn get_keys() -> anyhow::Result<(RsaPrivateKey, RsaPublicKey)> {
    let base_dir = home::home_dir().unwrap().join(crate::NAME);
    fs::create_dir_all(&base_dir)?;
    let priv_key_file = base_dir.join("private.key");
    if !Path::exists(&priv_key_file) {
        return generate_new_keys();
    }
    let priv_key = RsaPrivateKey::read_pkcs8_pem_file(priv_key_file)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((priv_key, pub_key))
}

fn generate_new_keys() -> anyhow::Result<(RsaPrivateKey, RsaPublicKey)> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let base_dir = home::home_dir().unwrap().join(crate::NAME);
    priv_key.write_pkcs8_pem_file(base_dir.join("private.key"), LineEnding::LF)?;
    pub_key.write_public_key_pem_file(base_dir.join("public.key"), LineEnding::LF)?;
    Ok((priv_key, pub_key))
}

pub fn decrypt(private_key:RsaPrivateKey,enc_data:&[u8])->anyhow::Result<String>{
    let padding = Oaep::new::<Sha256>();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    Ok(std::str::from_utf8(dec_data.as_slice())?.to_owned())
}