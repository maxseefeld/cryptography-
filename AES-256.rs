use openssl::symm::{Cipher, Crypter, Mode};
use std::io::{self, Read, Write};

fn aes_256_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).unwrap();
    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];
    let count = encrypter.update(plaintext, &mut ciphertext).unwrap();
    let rest = encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count + rest);
    ciphertext
}

fn main() {
    let key = b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let iv = b"0123456789abcdef0123456789abcdef";
    let plaintext = b"Hello, world!";
    let ciphertext = aes_256_encrypt(key, iv, plaintext);
    io::stdout().write_all(&ciphertext).unwrap();
}
