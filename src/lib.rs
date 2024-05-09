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
