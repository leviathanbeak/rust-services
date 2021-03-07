use openssl::rand::rand_bytes;
use openssl::symm::{decrypt, encrypt, Cipher};

fn main() {
    // desired data to be encrypted
    let data = b"leviathan appeared from the depths";

    // use AES of 128 bits
    let cipher = Cipher::aes_128_cbc();

    // generate random key and iv 
    let mut key_buff = [0; 16];
    rand_bytes(&mut key_buff).unwrap();
    
    let mut iv = [0; 16];
    rand_bytes(&mut iv).unwrap();

    // encrypt the data
    let encrypted = encrypt(cipher, &key_buff, Some(&iv), data).unwrap();
    println!("{:?}", encrypted);

    // decrypt the data
    let result = decrypt(cipher, &key_buff, Some(&iv), &encrypted).unwrap();
    let decrypted = std::str::from_utf8(&result).unwrap();
    println!("{}", decrypted);
}
