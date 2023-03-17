use std::io::Write;
use ring::digest::{Context, SHA512};

fn main() {
    let message = "The quick brown fox jumps over the lazy dog";
    
    // create a new context for SHA-512 hashing
    let mut context = Context::new(&SHA512);
    
    // update the context with the message bytes
    context.update(message.as_bytes());
    
    // get the hash digest as a byte array
    let digest = context.finish();
    
    // print the digest as hexadecimal string
    let hex_digest = digest.as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    
    println!("{}", hex_digest);
}
