//! # Set of prime numbers
//! This module generates and fills a vector with prime numbers for rsa

///Generate an vector of prime numbers and return the vector
pub fn generate_primes(n: i32) -> Result<Vec<i32>, String> {
    let mut primes = Vec::new();
    if n <= 0 {
        return Err(String::from("The end of range must be a postive integer"));
    }
    for i in 2..n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    Ok(primes)
}

///Check if number is prime
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let end = (n as f64).sqrt() as i32 + 1;
    for i in 2..end {
        if n % i == 0 {
            return false;
        }
    }
    true
}
