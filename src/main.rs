// In src/main.rs or a separate file if you prefer
use std::error::Error;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "my_cli",
    about = "A CLI tool for key generation and encryption/decryption"
)]
enum SubCommand {
    #[structopt(about = "Generate RSA key pair")]
    GenKeys,
    #[structopt(about = "Encrypt message")]
    EncryptMessage { message: String, public_key: String },
    #[structopt(about = "Encrypt message")]
    DecryptMessage {
        message: String,
        private_key_path: String,
        prime_path: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = SubCommand::from_args();
    match opt {
        SubCommand::GenKeys => {
            println!("Generating RSA key pair...");
            rsa_algorithm::gen_keys()?;
            Ok(())
        }
        SubCommand::EncryptMessage {
            message,
            public_key,
        } => {
            println!("Encrpyting...");
            rsa_algorithm::encrpyt(message, public_key)?;
            Ok(())
        }
        SubCommand::DecryptMessage {
            message,
            private_key_path,
            prime_path,
        } => {
            println!("Decrypting...");
            rsa_algorithm::decrpyt(message, private_key_path, prime_path)?;

            Ok(())
        }
    }
}
