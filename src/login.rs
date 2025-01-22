use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::Deserialize;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::process;
use std::path::Path;

#[derive(Deserialize)]
struct HashSalt {
    hash: String,
    salt: String,
    banksalt: String,
}

pub fn encrypt_password(pass: &str) -> (String, argon2::password_hash::SaltString){
    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let passhash = argon2.hash_password(pass.as_bytes(), &salt).expect("Failed to hash password");

    return (passhash.to_string(), salt);

}

pub fn login(phc: &str) -> (String, String){

    let parsed: HashSalt = serde_json::from_str(phc).expect("Failed to parse JSON");

    let mut inputpass = String::new();
    
    let parsed_hash = PasswordHash::new(&parsed.hash).expect("Stored hash is invalid");

    let mut counter = 0;
    let argon2obj = Argon2::default();

    loop {

        print!("Enter the password to access the manager: ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut inputpass).expect("Failed to read input I/O");

        if argon2obj.verify_password(inputpass.as_bytes(), &parsed_hash).is_ok() {
            break;
        } else {
            counter += 1;
            if counter == 5{
                println!("Too many failed attempts!");
                println!("[Press ENTER to exit]");
                io::stdin().read_line(&mut inputpass).expect("Failed to pause");
                process::exit(0);
            }
            else{ 
                println!("The password is incorrect, please try again!");
                continue;
            }
        }
        
    }

    return (inputpass, parsed.banksalt);
}

pub fn create_password() -> (String, String) {
    let mut input = String::new();
    let mut conf = String::new();

    loop{
        print!("Enter the password you want to use: ");

        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        print!("Confirm your password: ");

        _ = io::stdout().flush();
        io::stdin().read_line(&mut conf).expect("Failed to read input");

        if input != conf{
            print!("Passwords do not match! Please try again.\n");
            input.clear();
            conf.clear();
            continue;
        }
        else{
            break;
        }
    }

    let (phc, salt) = encrypt_password(&input);
    let banksalt = argon2::password_hash::SaltString::generate(&mut OsRng);


    // Note to self: Clean this shit up and remove redundancy when you get the chance (were you sleep deprived when you wrote this??)
    if Path::new("files/entrypass.json").exists() {
        
        let f = File::create("files/entrypass.json").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);
        let retstr = format!("{{\"hash\": \"{}\", \"salt\": \"{}\", \"banksalt\": \"{}\"}}", phc, salt.as_str(), banksalt.as_str());

        filebuf.write(retstr.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }
    else {
        let f = File::create("files/entrypass.json").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);

        let retstr = format!("{{\"hash\": \"{}\", \"salt\": \"{}\", \"banksalt\": \"{}\"}}", phc, salt.as_str(), banksalt.as_str());

        filebuf.write(retstr.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }

    return (conf, banksalt.as_str().to_string());

}