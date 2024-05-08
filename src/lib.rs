mod prime;

use rand::{thread_rng, Rng};
use std::env;
use std::error::Error;
pub fn gen_keys() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<i32>()?;
    let prime_list = prime::generate_primes(n)?;
    let a = thread_rng().gen_range(0..prime_list.len());
    let b = thread_rng().gen_range(0..prime_list.len());
    let _tup = (prime_list[a], prime_list[b]);
    Ok(())
}
