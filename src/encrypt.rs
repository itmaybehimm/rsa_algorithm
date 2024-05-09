use num_bigint::BigUint;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};
pub fn encrypt(message: String, public_key: String) -> Result<String, Box<dyn Error>> {
    // Parse the modulus (n) and public exponent (e) from the public key string
    let mut public_key_str = String::new();
    File::open(public_key)?.read_to_string(&mut public_key_str)?;
    let parts: Vec<&str> = public_key_str.split(',').collect();
    let n = parts[0].trim().parse::<BigUint>()?;
    let e = parts[1].trim().parse::<u32>()?;
    let message_bytes = message.as_bytes();
    let mut encoded_message = Vec::new();

    for byte in message_bytes.iter() {
        let byte_big_uint = BigUint::new(vec![*byte as u32]);
        let encoded_byte = byte_big_uint.pow(e) % &n;
        encoded_message.push(encoded_byte);
    }

    let mut file = File::create("encoded.txt")?;
    let encoded_message: String = encoded_message
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let buf = format!("{}", encoded_message);
    file.write_all(buf.as_bytes())?;
    println!("Created encoded.txt:");
    Ok(encoded_message)
}
