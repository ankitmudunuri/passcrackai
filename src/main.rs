mod interface;
mod login;

use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::process;
use std::io::prelude::*;
use std::path::Path;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode}
};



fn main(){

    let mut passfile = match File::open("files/entrypass.json") {
        Err(_why) => File::create("files/entrypass.json").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut passwords = String::new();
    match passfile.read_to_string(&mut passwords) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if passwords.is_empty() {
            println!("Welcome! This is your first time, so you must create a password.");
            login::create_password();
        }
        else {
            println!("Welcome back!");
            let phctext = fs::read_to_string("files/entrypass.json").expect("Unable to read file");
            
            login::login(&phctext);

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
            //test = deserialize(test);


        }

    }

    let mut tabletest = interface::ITable::init(&mut test);

    let mut valtest: Vec<String> = vec!["amudunuri22".to_string(), "Test1234".to_string(), "1".to_string(), "0".to_string()];
    let mut valdom: String = "Yahoo".to_string();

    tabletest.update(valdom, valtest);
    tabletest.print();

    tabletest.remove("Yahoo".to_string());

    tabletest.print();
}