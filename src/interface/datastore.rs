use bincode;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::HashMap;
use argon2::{password_hash, Argon2};
use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, OsRng}, aes::{cipher, Aes256}, Aes256Gcm, Nonce
};
use rand::Rng;
use base64;

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut key)
        .expect("Argon2 key derivation failed");
    key
}

pub fn store_data(data: Vec<u8>, password: String, salt: String) {

    let savedata = encrypt(data, password, salt);

    if Path::new("files/passwordbank.txt").exists() {
        
        let f = File::create("files/passwordbank.txt").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);

        filebuf.write_all(savedata.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }
    else {
        let f = File::create("files/passwordbank.txt").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);

        filebuf.write_all(savedata.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }
}

pub fn load_data(inp_map: &mut HashMap<String, Vec<String>>, password: String, salt: String) {
    let mut bankfile = match File::open("files/passwordbank.txt") {
        Err(_why) => File::create("files/passwordbank.txt").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut bank: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut bin: String = String::new();
    match bankfile.read_to_string(&mut bin) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if bin.is_empty() {
        }
        else {
            bin = decrypt(bin, password, salt);
            bank = deserialize(bin);
            *inp_map = bank;

        }

    }
}

pub fn serialize(data: &HashMap<String, Vec<String>>) -> Vec<u8>{
    return bincode::serialize(&data).expect("Failed to serialize data");
}

fn encrypt(data: Vec<u8>, password: String, salt: String) -> String{
    let saltbytes = salt.as_bytes();
    let key = derive_key(password.as_bytes(), saltbytes);

    let cipher = Aes256Gcm::new_from_slice(&key).expect("Invalid key length");
    let mut noncebytes = [0u8; 12];
    OsRng.fill_bytes(&mut noncebytes);

    let ciphertext = cipher.encrypt(Nonce::from_slice(&noncebytes), data.as_ref()).expect("Encryption failed");

    return format!("{}:{}", base64::encode(noncebytes), base64::encode(ciphertext));

}

fn deserialize(data: String) -> HashMap<String, Vec<String>>{
    return bincode::deserialize(data.as_bytes()).expect("Failed to deserialize data");
}

fn decrypt(data: String, password:String, salt: String) -> String{
    let parts: Vec<&str> = data.split(':').collect();
    // Error format checking
    if parts.len() != 2 {
        panic!("Invalid data format. Expected format: nonce:ciphertext");
    }
    else {
        
    }

    let (nonceb64, cipherb64) = (parts[0], parts[1]);

    let noncebytes = base64::decode(nonceb64).expect("Failed to decode nonce");
    let ciphertext = base64::decode(cipherb64).expect("Failed to decode ciphertext");

    let key = derive_key(password.as_bytes(), salt.as_bytes());

    let cipher = Aes256Gcm::new_from_slice(&key).expect("Invalid key");
    let plaintext = cipher.decrypt(Nonce::from_slice(&noncebytes), ciphertext.as_ref()).expect("Decryption failed");


    return String::from_utf8(plaintext).expect("Decryption bytes are not UTF-8");
}

