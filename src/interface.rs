use prettytable::{Table, row, cell};
use std::collections::HashMap;

pub struct ITable <'a>{
    table: prettytable::Table,
    passmap: &'a mut HashMap<String, Vec<String>>,
    size: i64
}

impl ITable <'_>{

    pub fn init(inp_data: &mut HashMap<String, Vec<String>>) -> ITable{
        let mut newtable = Table::new();
        let mut initsize: i64 = 0;

        newtable.set_titles(row!["Account Domain", "Username", "Password", "Password Strength"]);

        for (k, v) in &mut *inp_data {
            newtable.add_row(row![k, v[0], v[1], v[2]]);
            initsize += 1;
        }

        ITable{ table: newtable, passmap: inp_data, size: initsize}

    }
    
    pub fn print(&self){
        print!("{esc}c", esc = 27 as char);
        &self.table.printstd();
        return;
    }

    pub fn get_size(&self) -> i64 { return self.size; }

    pub fn update(&mut self, key: String, value: Vec<String>) -> (){
        self.passmap.insert(key.clone(), value.clone());
        self.table.add_row(row![key, value[0], value[1], value[2]]);
        self.size += 1;
        return;
    }

    pub fn remove(&mut self, key: String) -> (){
        let mut idx = self.passmap[&key][3].parse::<i64>();
        self.passmap.remove(&key.clone());
        self.table.remove_row(idx.expect("Not int").try_into().unwrap());
        self.size -= 1;
        return;
    }
}