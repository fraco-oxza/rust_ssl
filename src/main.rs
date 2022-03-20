use openssl::rand::rand_bytes;
use openssl::rsa::Rsa;

fn main() {
    let key = Rsa::generate(1024).unwrap();
    let mut buf = [0u8; 32];
    rand_bytes(&mut buf).unwrap();
    let private = key.private_key_to_pem().unwrap();
    println!("{}", String::from_utf8(private.to_vec()).unwrap());
    // println!("{}", hex::encode(&buf));
}
