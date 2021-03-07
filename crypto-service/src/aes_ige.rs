use openssl::{aes::{AesKey, aes_ige}, symm::Mode};
use openssl::rand::rand_bytes;

fn main() {
    // plaintext we want to encrypt
    let plaintext = "levitathan comes from the depths";
    let size = plaintext.len();    

    // random bytes for key generation
    let mut key_buff = [0; 16];
    rand_bytes(&mut key_buff).unwrap();    
    
    let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    // AES Key from random bytes
    let key = AesKey::new_encrypt(&key_buff).unwrap();

    // output to store the encryption
    let mut output: Vec<u8> = vec![0; size];
    aes_ige(plaintext.as_bytes(), &mut output, &key, &mut iv, Mode::Encrypt);

    // format to hex and print
    let hex_value = format!("{:x?}", output);
    println!("{}", hex_value);

    // decrypt
    let mut decrypted = vec![0; size];
    aes_ige(&output, &mut decrypted, &key, &mut iv, Mode::Decrypt);    
    println!("{:?}", decrypted);

}