fn main() {
    if let Err(error) = rsa_algorithm::gen_keys() {
        println!("Error: {}", error);
        std::process::exit(1);
    }
}
