use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::path::Path;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::RngCore;
use rand::rngs::OsRng;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{KeyInit, Aead};
use serde_json;
use base64;
use std::process;
mod interface;

fn encrypt_password(pass: &str) -> String {
    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let passhash = argon2.hash_password(pass.as_bytes(), &salt).expect("Failed to hash password");

    return passhash.to_string();

}

fn login(phc: &str) {
    let mut inputpass = String::new();
    
    let parsed_hash = PasswordHash::new(phc).expect("Stored hash is invalid");

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

    return;
}

fn create_password() {
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

    let phc = encrypt_password(&input);

    if Path::new("files/entrypass.json").exists() {
        
        let f = File::create("files/entrypass.json").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);

        filebuf.write(phc.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }
    else {
        let f = File::create("files/entrypass.json").expect("Unable to create file");
        let mut filebuf = io::BufWriter::new(f);
        
        filebuf.write(phc.as_bytes()).expect("Couldn't write");
        
        filebuf.flush().expect("Unable to flush buffer");
    }

}

fn main() {

    let mut passfile = match File::open("files/entrypass.json") {
        Err(why) => File::create("files/entrypass.json").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut passwords = String::new();
    match passfile.read_to_string(&mut passwords) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if passwords.is_empty() {
            println!("Welcome! This is your first time, so you must create a password.");
            create_password();
        }
        else {
            println!("Welcome back!");
            let phctext = fs::read_to_string("files/entrypass.json").expect("Unable to read file");
            
            login(&phctext);

        }

    }
    
    let mut passmap: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    println!("Works past creating hash map");

    let mut vistable: prettytable::Table = interface::create_view(&passmap);

    println!("Creates table");

    interface::print_table(&vistable);

    let password = "Test1234";

    

}