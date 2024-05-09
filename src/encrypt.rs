use num_bigint::BigUint;
use std::error::Error;
pub fn encrypt(message: String, public_key: String) -> Result<Vec<BigUint>, Box<dyn Error>> {
    // Parse the modulus (n) and public exponent (e) from the public key string
    let parts: Vec<&str> = public_key.split(',').collect();
    let n = parts[0].parse::<BigUint>()?;
    let e = parts[1].parse::<u32>()?;
    let message_bytes = message.as_bytes();
    let mut encoded_message = Vec::new();

    for byte in message_bytes.iter() {
        let byte_big_uint = BigUint::new(vec![*byte as u32]);
        let encoded_byte = byte_big_uint.pow(e) % &n;
        println!("{} {}", byte, encoded_byte);
        encoded_message.push(encoded_byte);
    }

    println!("{:?}", encoded_message);
    Ok(encoded_message)
}
