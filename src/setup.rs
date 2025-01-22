use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main(){
    if !Path::new("files").exists(){
        match fs::create_dir("files") {
            Err(_why) => eprintln!("Failed to create directory: {}", _why),
            Ok(_) => {},
        };
    }

    let mut passfile = match File::open("files/entrypass.json") {
        Err(_why) => File::create("files/entrypass.json").expect("Unable to create file"),
        Ok(file) => file,
    };

    let mut bankfile = match File::open("files/passwordbank.txt") {
        Err(_why) => File::create("files/passwordbank.txt").expect("Unable to create file"),
        Ok(file) => file,
    };

    println!("Setup complete!");
}