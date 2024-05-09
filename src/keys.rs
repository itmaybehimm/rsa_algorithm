use crate::prime;
use num_bigint::{BigUint, ToBigUint};
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};
// use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Primes {
    p: BigUint,
    q: BigUint,
}
struct Keys {
    private_key: (BigUint, BigUint),
    public_key: (BigUint, BigUint),
}

impl Primes {
    fn new(p: BigUint, q: BigUint) -> Primes {
        Primes { p, q }
    }

    fn gen_keys(&self) -> Keys {
        let n = &self.p * &self.q;
        let one: BigUint = 1u32.to_biguint().unwrap();
        let r = (&self.p - &one) * (&self.q - &one);
        let e = 65537u32.to_biguint().unwrap();
        let d = e.modinv(&r).unwrap();
        Keys::new((n.clone(), d), (n, e))
    }
}

impl Keys {
    fn new(private_key: (BigUint, BigUint), public_key: (BigUint, BigUint)) -> Keys {
        Keys {
            private_key,
            public_key,
        }
    }
}

pub fn get_keys() -> Result<(), Box<dyn Error>> {
    let n: u64 = 100000;
    let prime_list = prime::generate_primes(n)?;
    let biguint_list: Vec<BigUint> = prime_list
        .into_iter()
        .map(|prime| BigUint::from_u64(prime).expect("Failed to convert u64 to BigUint"))
        .collect();
    let a = thread_rng().gen_range(0..biguint_list.len());
    let b = thread_rng().gen_range(0..biguint_list.len());
    let primes = Primes::new(biguint_list[a].clone(), biguint_list[b].clone());
    let keys = primes.gen_keys();

    let mut file = File::create("./primes.txt")?;
    let to_write = format!("{:?}, {:?}", primes.p, primes.q);
    file.write_all(to_write.as_bytes())?;
    println!("Created a primes.txt file");

    let mut file = File::create("./private_key.txt")?;
    let to_write = format!("{}, {}", keys.private_key.0, keys.private_key.1);
    file.write_all(to_write.as_bytes())?;
    println!("Created a private_key.txt file");

    let mut file = File::create("./public_key.txt")?;
    let to_write = format!("{}, {}", keys.public_key.0, keys.public_key.1);
    file.write_all(to_write.as_bytes())?;
    println!("Created a public_key.txt file");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_keys() {
        let primes = Primes::new(11u32.to_biguint().unwrap(), 5u32.to_biguint().unwrap());
        let keys = primes.gen_keys();
        let expec_keys = Keys::new(
            (55u32.to_biguint().unwrap(), 33u32.to_biguint().unwrap()),
            (55u32.to_biguint().unwrap(), 65537u32.to_biguint().unwrap()),
        );
        assert_eq!(keys.private_key, expec_keys.private_key);
        assert_eq!(keys.public_key, expec_keys.public_key);

        let primes = Primes::new(
            8956097u32.to_biguint().unwrap(),
            3255247u32.to_biguint().unwrap(),
        );
        let keys = primes.gen_keys();
        let expec_keys = Keys::new(
            (
                29154307890959u64.to_biguint().unwrap(),
                25752508916993u64.to_biguint().unwrap(),
            ),
            (
                29154307890959u64.to_biguint().unwrap(),
                65537u32.to_biguint().unwrap(),
            ),
        );
        assert_eq!(keys.private_key, expec_keys.private_key);
        assert_eq!(keys.public_key, expec_keys.public_key);
    }
}

// fn modular_inverse(e: i64, phi_n: i64) -> i64 {
//     let (gcd, d, _) = gcd_extended(e, phi_n);
//     if gcd != 1 {
//         panic!("No modular inverse exists for the given parameters");
//     }
//     // Ensure d is positive
//     let d = if d < 0 { d + phi_n } else { d };
//     d
// }

// fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
//     // Base Case
//     if a == 0 {
//         return (b, 0, 1);
//     }

//     // Recursive call
//     let (gcd, x1, y1) = gcd_extended(b % a, a);

//     // Update x and y
//     let x = y1 - (b / a) * x1;
//     let y = x1;

//     (gcd, x, y)
// }
