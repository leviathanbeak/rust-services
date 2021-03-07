use openssl::{rsa::{Rsa, Padding}};

fn main() {
    let data = "I am a top secret document";

    // A 2048-bit RSA key is created using the generate() method.
    let rsa = Rsa::generate(2048).unwrap();

    let mut encrypted_buff = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut encrypted_buff, Padding::PKCS1).unwrap();
    println!("Encrypted: {:?}", encrypted_buff);

    let mut decrypted_buff = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(&encrypted_buff, &mut decrypted_buff, Padding::PKCS1).unwrap();

    let decrypted = String::from_utf8(decrypted_buff).unwrap();
    println!("{}", decrypted);
}