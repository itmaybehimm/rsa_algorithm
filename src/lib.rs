mod decrpyt;
mod encrypt;
mod keys;
mod prime;
use std::error::Error;

pub fn gen_keys() -> Result<(), Box<dyn Error>> {
    keys::get_keys()?;
    Ok(())
}

pub fn encrpyt(message: String, public_key: String) -> Result<(), Box<dyn Error>> {
    encrypt::encrypt(message, public_key)?;
    Ok(())
}

pub fn decrpyt(
    message: String,
    private_key_path: String,
    primes_path: String,
) -> Result<(), Box<dyn Error>> {
    decrpyt::decrpyt(message, private_key_path, primes_path)?;
    Ok(())
}
