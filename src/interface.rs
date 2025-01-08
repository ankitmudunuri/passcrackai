use prettytable::{Table, row, cell};
use std::collections::HashMap;

pub fn create_view(inp_table: &HashMap<String, Vec<String>>) -> prettytable::Table {
    // Format of table:
    // | Account Domain | Username | Password | Password Strength |

    let mut table = Table::new();

    table.set_titles(row!["Account Domain", "Username", "Password", "Password Strength"]);

    for (k, v) in inp_table {
        table.add_row(row![k, v[0], v[1], v[2]]);
    }


    return table;
}