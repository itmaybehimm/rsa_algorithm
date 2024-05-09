use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use std::error::Error;
use std::fs::File;
use std::io::Read;

use std::ops::Rem;
pub fn decrpyt(
    message: String,
    private_key_path: String,
    primes_path: String,
) -> Result<(), Box<dyn Error>> {
    // Open the file
    let mut file = File::open(message)?;

    // Read the file line by line and parse each line into a vector of BigUint
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let message: Vec<BigUint> = data
        .split(",")
        .map(|s| s.trim().parse::<BigUint>().unwrap())
        .collect(); // Propagate parse errors if any

    let mut private_key_file = File::open(private_key_path)?;
    let mut private_key_content = String::new();
    private_key_file.read_to_string(&mut private_key_content)?;
    let private_key_parts: Vec<BigUint> = private_key_content
        .trim()
        .split(',')
        .map(|s| BigUint::from_str_radix(s.trim(), 10).unwrap())
        .collect();
    let n = private_key_parts[0].clone();
    let d = private_key_parts[1].clone();

    let mut prime_file = File::open(primes_path)?;
    let mut prime_content = String::new();
    prime_file.read_to_string(&mut prime_content)?;
    let prime_parts: Vec<BigUint> = prime_content
        .trim()
        .split(',')
        .map(|s| BigUint::from_str_radix(s.trim(), 10).unwrap())
        .collect();
    let p = prime_parts[0].clone();
    let q = prime_parts[1].clone();

    let mut decoded_message = Vec::new();
    for number in message.iter() {
        let decoded_byte = crt(number, &d, &n, &p, &q);
        decoded_message.push(decoded_byte);
    }

    for byte in decoded_message {
        let parsed_byte = byte.to_string().parse::<u8>()?;
        let mut vec = Vec::new();
        vec.push(parsed_byte);
        let x = String::from_utf8(vec)?;
        print!("{}", x);
    }
    Ok(())
}

fn crt(c: &BigUint, d: &BigUint, n: &BigUint, p: &BigUint, q: &BigUint) -> BigUint {
    let dp = d % (p - BigUint::one());
    let dq = d % (q - BigUint::one());
    let qinv = q
        .modinv(&p)
        .expect("q does not have a modular inverse modulo p");
    let m1 = c.clone().modpow(&dp, &p);
    let m2 = c.modpow(&dq, &q);
    let mut del = if m1 >= m2 { m1 - &m2 } else { m1 + p - &m2 };
    if del.is_zero() {
        del = BigUint::zero();
    }
    let h = (qinv * del).rem(p);
    let m = (m2 + h * q).rem(n);
    m
}
