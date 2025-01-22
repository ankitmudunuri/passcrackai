mod interface;
mod login;
mod appio;

use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;



fn main(){

    let mut passfile = match File::open("files/entrypass.json") {
        Err(_why) => File::create("files/entrypass.json").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut passwords = String::new();
    let mut password = String::new();
    let mut salt: String = String::new();
    match passfile.read_to_string(&mut passwords) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if passwords.is_empty() {
            println!("Welcome! This is your first time, so you must create a password.");
            (password, salt) = login::create_password();
        }
        else {
            println!("Welcome back!");
            let phctext = fs::read_to_string("files/entrypass.json").expect("Unable to read file");
            
            (password, salt) = login::login(&phctext);

        }

    }

    let mut bankfile = match File::open("files/passwordbank.txt") {
        Err(_why) => File::create("files/passwordbank.txt").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut passbank = String::new();
    let mut test: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    match bankfile.read_to_string(&mut passbank) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if passbank.is_empty() {
            
        }
        else {


        }

    }
    
    let mut tabletest = interface::ITable::init(&mut test);

/*

    let valtest: Vec<String> = vec!["amudunuri22".to_string(), "Test1234".to_string(), "1".to_string(), "0".to_string()];
    let valdom: String = "Yahoo".to_string();

    tabletest.add(valdom, valtest);

    let valtest2: Vec<String> = vec!["amudunuri".to_string(), "Test5678".to_string(), "3".to_string(), "1".to_string()];
    let valdom2: String = "Gmail".to_string();

    tabletest.add(valdom2, valtest2);

    tabletest.store(password.clone(), salt.clone());
    */

    tabletest.load(password, salt);
    tabletest.print();
}