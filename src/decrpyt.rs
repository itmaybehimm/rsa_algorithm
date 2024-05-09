use num_bigint::BigUint;
use num_traits::{FromPrimitive, One, Zero};
use std::error::Error;
use std::ops::Rem;
pub fn decrpyt(message: String, private_key: String) -> Result<(), Box<dyn Error>> {
    let message: Vec<BigUint> = message
        .split(",")
        .map(|s| s.trim().parse::<BigUint>().unwrap())
        .collect();

    let parts: Vec<&str> = private_key.split(',').collect();
    let n = parts[0].parse::<BigUint>()?;
    let d = parts[1].parse::<BigUint>()?;
    let p = BigUint::from_i64(944857).unwrap();
    let q = BigUint::from_i64(263101).unwrap();

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
        println!("{}", x);
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
