mod interface;
mod login;
mod appio;
mod strengthestimation;

use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::process;
use std::thread;
use std::time::Duration;



fn main(){

    if !Path::new("files").exists(){
        match fs::create_dir("files") {
            Err(_why) => eprintln!("Failed to create directory: {}", _why),
            Ok(_) => {},
        };
    }

    thread::sleep(Duration::from_millis(100));

    let mut passfile = match File::open("files/entrypass.json") {
        Err(_why) => File::create("files/entrypass.json").expect("Unable to create file"),
        Ok(file) => file,
    };

    thread::sleep(Duration::from_millis(100));

    let mut passwords = String::new();
    let mut password = String::new();
    let mut salt: String = String::new();
    match passfile.read_to_string(&mut passwords) {
        Err(why) => panic!("Error reading JSON: {}", why),
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

    thread::sleep(Duration::from_millis(100));

    let mut passbank = String::new();
    let mut initmap: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut table = interface::ITable::init(&mut initmap);
    match bankfile.read_to_string(&mut passbank) {
        Err(why) => panic!("Error reading: {}", why),
        Ok(_) => 
        if passbank.is_empty() {
        }
        else {
            table.load(password.clone(), salt.clone());
        }

    }
    let mut inputstr = String::new();

    loop {
        inputstr = "".to_string();

        table.print();
        print!("Please enter what action you want to do (1 = Search/View Password, 2 = Add Password, 3 = Remove Password, 4 = Update Password, 5 = Quit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inputstr).expect("Failed to read I/O");
        let inpnum: u32 = match inputstr.trim().parse() {
            Ok(num) => num,
            Err(why) => {
                println!("That isn't a valid input");
                io::stdout().flush().unwrap();
                println!("\n(Press ENTER to continue)");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut inputstr).expect("Failed to read I/O");
                continue;
            }
        };

        if inpnum == 1 {
            appio::view_password(table.get_passmap());
        }
        else if inpnum == 2 {
            appio::add_password(&mut table);
        }
        else if inpnum == 3 {
            appio::remove_password(&mut table);
        }

        else if inpnum == 4 {
            appio::update_password(&mut table);
        }

        else if inpnum == 5 {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush().unwrap();
            print!("Goodbye!");
            io::stdout().flush().unwrap();
            table.store(password, salt);

            println!("\n(Press ENTER to continue)");
            io::stdin().read_line(&mut inputstr).expect("Failed to read I/O");

            process::exit(0);
        }

        else {
            println!("That isn't a valid input");
            io::stdout().flush().unwrap();
            println!("\n(Press ENTER to continue)");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut inputstr).expect("Failed to read I/O");
        }
    }

/*

    let valtest: Vec<String> = vec!["amudunuri22".to_string(), "Test1234".to_string(), "1".to_string(), "0".to_string()];
    let valdom: String = "Yahoo".to_string();

    tabletest.add(valdom, valtest);

    let valtest2: Vec<String> = vec!["amudunuri".to_string(), "Test5678".to_string(), "3".to_string(), "1".to_string()];
    let valdom2: String = "Gmail".to_string();

    tabletest.add(valdom2, valtest2);

    tabletest.store(password.clone(), salt.clone());
    */
    table.print();
}