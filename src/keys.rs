use crate::prime;

use rand::{thread_rng, Rng};
// use std::env;
use std::error::Error;

#[derive(Debug)]
struct Primes {
    p: i64,
    q: i64,
}
struct Keys {
    private_key: (i64, i64),
    public_key: (i64, i64),
}

impl Primes {
    fn new(p: i64, q: i64) -> Primes {
        Primes { p, q }
    }

    fn gen_keys(&self) -> Keys {
        let n = self.p * self.q;
        let r = (self.p - 1) * (self.q - 1);
        let e = 7;
        let d = modular_inverse(e, r);
        Keys::new((n, d), (n, e))
    }
}

impl Keys {
    fn new(private_key: (i64, i64), public_key: (i64, i64)) -> Keys {
        Keys {
            private_key,
            public_key,
        }
    }
}

fn modular_inverse(e: i64, phi_n: i64) -> i64 {
    let (gcd, d, _) = gcd_extended(e, phi_n);
    if gcd != 1 {
        panic!("No modular inverse exists for the given parameters");
    }
    // Ensure d is positive
    let d = if d < 0 { d + phi_n } else { d };
    d
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    // Base Case
    if a == 0 {
        return (b, 0, 1);
    }

    // Recursive call
    let (gcd, x1, y1) = gcd_extended(b % a, a);

    // Update x and y
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}
pub fn get_keys() -> Result<(), Box<dyn Error>> {
    // let args: Vec<String> = env::args().collect();
    // let n = args[1].parse::<i64>()?;
    let n: i64 = 1000000;
    let prime_list = prime::generate_primes(n)?;
    let a = thread_rng().gen_range(0..prime_list.len());
    let b = thread_rng().gen_range(0..prime_list.len());
    let primes = Primes::new(prime_list[a], prime_list[b]);
    let keys = primes.gen_keys();
    println!("Primes: {:?}", primes);
    println!("private key: {:?}", keys.private_key);
    println!("public key: {:?}", keys.public_key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_keys() {
        let primes = Primes::new(11, 5);
        let keys = primes.gen_keys();
        let expec_keys = Keys::new((55, 23), (55, 7));
        // let expec_keys = Keys::new((55, 33), (55, 65537));
        assert_eq!(keys.private_key, expec_keys.private_key);
        assert_eq!(keys.public_key, expec_keys.public_key);

        // let primes = Primes::new(8956097, 3255247);
        // let keys = primes.gen_keys();
        // let expec_keys = Keys::new((29154307890959, 25752508916993), (29154307890959, 65537));
        // assert_eq!(keys.private_key, expec_keys.private_key);
        // assert_eq!(keys.public_key, expec_keys.public_key);
    }
}
