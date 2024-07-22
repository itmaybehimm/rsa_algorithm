# RSA Rust

RSA Rust is a command-line interface (CLI) tool for generating RSA key pairs and encrypting/decrypting messages using the RSA algorithm. This project is ongoing, and future updates will include support for larger numbers.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
  - [GenKeys](#genkeys)
  - [EncryptMessage](#encryptmessage)
  - [DecryptMessage](#decryptmessage)
- [License](#license)

## Installation

To use this tool, you need to have Rust installed on your system. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

Clone this repository and build the project:

```sh
git clone https://github.com/yourusername/rsa_rust.git
cd rsa_rust
cargo build --release
```

## Usage

Run the CLI tool with the desired command:

```sh
cargo run -- <COMMAND>
```

## Commands

### GenKeys

Generates an RSA key pair.

Usage:

```sh
cargo run -- gen-keys
```

### EncryptMessage

Encrypts a message using a public key.

Usage:

```sh
cargo run -- encrypt-message <MESSAGE> <PUBLIC_KEY_FILE>
```

Example:

```sh
cargo run -- encrypt-message "Hello, World!" public_key.txt
```

### DecryptMessage

Decrypts a message using a private key and prime numbers.

Usage:

```sh
cargo run -- decrypt-message <ENCODED_MESSAGE_FILE> <PRIVATE_KEY_FILE> <PRIME_FILE>
```

Example:

```sh
cargo run -- decrypt-message encoded.txt private_key.txt primes.txt
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## File Descriptions

### `src/main.rs`

The entry point for the CLI tool. It defines the `SubCommand` enum for the available commands and their arguments. The `main` function parses the command-line arguments and calls the appropriate functions in the `rsa_algorithm` module.

### `src/lib.rs`

The main library file that implements the RSA key generation, encryption, and decryption functions. It defines the following functions:

- `gen_keys`: Generates RSA key pairs.
- `encrpyt`: Encrypts a message using a public key.
- `decrpyt`: Decrypts a message using a private key and primes.

### `src/encrypt.rs`

Contains the implementation of the `encrypt` function, which encrypts a message using the RSA algorithm.

### `src/decrypt.rs`

Contains the implementation of the `decrpyt` function, which decrypts a message using the RSA algorithm and the Chinese Remainder Theorem (CRT).

## Notes

- Ensure that the public key, private key, and prime files are in the correct format for the encryption and decryption functions to work correctly.
- The `encrpyt` and `decrpyt` functions create `encoded.txt` and `decoded.txt` files respectively, which contain the encrypted and decrypted messages.

Feel free to contribute to this project by submitting issues or pull requests on the [GitHub repository](https://github.com/yourusername/rsa_rust).
