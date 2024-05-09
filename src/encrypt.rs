use std::error::Error;

pub fn encrypt(message: String, public_key: String) -> Result<String, Box<dyn Error>> {
    // Parse the modulus (n) and public exponent (e) from the public key string
    let parts: Vec<&str> = public_key.split(',').collect();
    let n = parts[0].parse::<i64>()?;
    let e = parts[1].parse::<i64>()?;

    let message_bytes = message.as_bytes();
    let mut encoded_message = String::new();

    for byte in message_bytes.iter() {
        let byte_i64 = *byte as i64;
        let encoded_byte = byte_i64.pow(e as u32) % n;
        println!("{} {} {:x}", byte, encoded_byte, encoded_byte);
        encoded_message.push_str(&format!("{:x}", encoded_byte)); // Encode as hexadecimal
    }

    println!("{:?}", encoded_message);
    Ok(encoded_message)
}
