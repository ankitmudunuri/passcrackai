use std::collections::HashMap;
use std::io;
use std::io::Write;
use prettytable::{Table, row, cell};
use crate::interface::ITable;
use crate::strengthestimation::estimate;

pub fn view_password(passmap: &HashMap<String, Vec<String>>){

    print!("What field do you want to search the viewable username by (1 = Domain, 2 = Username, 3 = Password)?: ");
    io::stdout().flush().unwrap();

    let mut newtable = Table::new();

    newtable.set_titles(row!["Account Domain", "Username", "Password", "Password Strength"]);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read I/O");
    let inpnum: u32 = input.trim().parse().expect("Couldn't parse input");
    if inpnum == 1 {
        input = "".to_string();
        print!("Please enter the domain that you want to search for: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read I/O");

        for (k, v) in passmap {
            if k.contains(&input.trim()){
                newtable.add_row(row![k, v[0], v[1], v[2]]);
            }
        }
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
        newtable.printstd();
    }
    else if inpnum == 2 {
        input = "".to_string();
        print!("Please enter the password that you want to search for: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read I/O");

        for (k, v) in passmap {
            if v[0].contains(&input.trim()){
                newtable.add_row(row![k, v[0], v[1], v[2]]);
            }
        }
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
        newtable.printstd();
    }
    else if inpnum == 3 {
        input = "".to_string();
        print!("Please enter the password that you want to search for: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read I/O");

        for (k, v) in passmap {
            if v[1].contains(&input.trim()){
                newtable.add_row(row![k, v[0], v[1], v[2]]);
            }
        }
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
        newtable.printstd();
    }
    else {
        print!("That isn't a valid input.");
        io::stdout().flush().unwrap();
    }
    println!("\n(Press ENTER to continue)");
    io::stdin().read_line(&mut input).expect("Failed to read I/O");
}

pub fn add_password(table: &mut ITable){

    let mut input = String::new();

    let mut tempvect: Vec<String> = Vec::new();
    
    print!("Enter the domain of the credentials that you want to add: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    let domain = input.clone();
    input = "".to_string();

    print!("Enter the username of the credentials that you want to add: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    tempvect.push(input.clone());
    input = "".to_string();

    print!("Enter the password of the credentials that you want to add: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    tempvect.push(input.clone());

    let strength = estimate(input);
    tempvect.push(strength);

    input = "".to_string();

    tempvect.push(table.get_size().to_string());

    table.add(domain, tempvect);
}

pub fn remove_password(table: &mut ITable){
    let mut input = String::new();
    let mut domain = String::new();

    print!("Enter the domain that you would like to remove: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut domain).expect("Failed to read I/O");

    print!("Are you sure? (1 = yes, 2 = no): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    let inpnum: u32 = input.trim().parse().expect("Couldn't parse input");
    if inpnum == 1 {
        table.remove(domain.clone());
        print!("Credentials for the domain {} has been successfully removed.", domain.trim());
        io::stdout().flush().unwrap();
    }
    else if inpnum == 2 {
        print!("Removal process for the {} domain has been aborted.", domain.trim());
        io::stdout().flush().unwrap();
    }
    else {
        print!("That is not a valid input");
        io::stdout().flush().unwrap();
    }

    println!("\n(Press ENTER to continue)");
    io::stdin().read_line(&mut input).expect("Failed to read I/O");
}

pub fn update_password(table: &mut ITable){
    let mut input = String::new();

    let mut tempvect: Vec<String> = Vec::new();
    
    print!("Enter the domain of the credentials that you want to update: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    let domain = input.clone();
    input = "".to_string();

    print!("Enter the new username: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    tempvect.push(input.clone());
    input = "".to_string();

    print!("Enter the new password: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read I/O");

    tempvect.push(input.clone());

    let strength = estimate(input);
    tempvect.push(strength);

    input = "".to_string();

    tempvect.push(table.get_size().to_string());

    table.update(domain, tempvect);
}