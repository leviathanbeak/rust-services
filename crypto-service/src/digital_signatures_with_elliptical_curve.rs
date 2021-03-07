use openssl::{ecdsa::EcdsaSig, sha::sha256};
use openssl::ec::*;
use openssl::nid::Nid;

fn main() {
    let data = "I am a top secret document";

    // the key is generated on the secp256k1 elliptical curve
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let key = EcKey::generate(&group).unwrap();

    // messages that need to be signed are hashed
    let hash = sha256(data.as_bytes());
    
    // signed by the owner
    let signature = EcdsaSig::sign(&hash, &key).unwrap();

    // verify
    let verified = signature.verify(&hash, &key).unwrap();

    if verified {
        println!("signature is fine");
    } else {
        panic!("wrong signature")
    }
}