/** TODO:
*    - better search
*    - <add> and <edit> both have to have two method of input
*        i. directly from cli e.g: conman edit 3 name 00phone00
*        ii. servey like what "conman edit 3" does now
*    - closure problem
*    - right way to cancel <edit>
*    - empty string problem in "add" and "edit" command
*/
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    io::{self, stdin, Write},
};

mod args;
use crate::args::{AppArgs, MainActions};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    name: String,
    phone: String,
}

fn main() {
    let args = AppArgs::parse();

    let contacts_json_string = fs::read_to_string("./src/contacts.json").unwrap();
    let mut contacts: Vec<Contact> = serde_json::from_str(&contacts_json_string).unwrap();
    // println!("{:?}", args);
    // println!("__________________________________________\n\n");

    //* conman add <name> <phone>
    let mut add_contact_to_json = |name: &String, phone: &String| {
        contacts.push(Contact {
            name: name.to_string(),
            phone: phone.to_string(),
        });
        save_json("./src/contacts.json", &contacts);
        println!("added: {} {}", name, phone);
    };
    //* conman show
    fn show_contacts_all(mut contacts: Vec<Contact>) {
        // println!("{:?}", contacts)
        fetch_json("./src/contacts.json", &mut contacts);
        print_table_all(contacts);
    }
    //* conman search <keyword>
    fn search_from_conacts(mut contacts: Vec<Contact>, key: &str) {
        fetch_json("./src/contacts.json", &mut contacts);
        let search_result: Vec<(usize, &Contact)> = contacts
            .iter()
            .enumerate()
            .map(|(i, con)| (i, con))
            .filter(|&con| {
                con.1.name.to_lowercase().contains(&key.to_lowercase()) || con.1.phone.contains(key)
            })
            .collect();

        // println!("{:?}", search_result);
        print_table_from_tuple(search_result);
    }
    //* conman delete <serial>
    fn delete_contact(mut contacts: Vec<Contact>, serial: usize) {
        if serial < 1 || serial > contacts.len() {
            println!("Provided value is not a valid number");
            return;
        }
        contacts.remove(serial - 1);
        save_json("./src/contacts.json", &contacts);
        println!("Deletion done.");
    }
    //* conman edit <serial>
    fn edit_contact(mut contacts: Vec<Contact>, serial: usize) {
        if serial < 1 || serial > contacts.len() {
            println!("Provided value is not a valid number");
            return;
        }
        fetch_json("./src/contacts.json", &mut contacts);
        let mut name: String = String::new();
        let mut phone: String = String::new();

        print!("Name({})    :", contacts[serial - 1].name);
        io::stdout().flush().unwrap();
        stdin().read_line(&mut name).unwrap();
        contacts[serial - 1].name = name.trim().to_string();

        print!("Phone({})   :", contacts[serial - 1].phone);
        io::stdout().flush().unwrap();
        stdin().read_line(&mut phone).unwrap();
        contacts[serial - 1].phone = phone.trim().to_string();

        save_json("./src/contacts.json", &contacts);
        println!("Changes saved.");
    }

    match args.action {
        MainActions::Add(info) => add_contact_to_json(&info.name, &info.phone),
        MainActions::Show => show_contacts_all(contacts),
        MainActions::Search(info) => search_from_conacts(contacts, &info.keyword),
        MainActions::Delete(info) => {
            delete_contact(contacts, info.serial.parse::<usize>().unwrap_or(0))
        }
        MainActions::Edit(info) => {
            edit_contact(contacts, info.serial.parse::<usize>().unwrap_or(0))
        }
    }
}

fn save_json(path: &str, contacts: &Vec<Contact>) {
    let contacts_json_string_towrite = serde_json::to_string(contacts).unwrap();
    fs::write(path, contacts_json_string_towrite).unwrap();
}
fn fetch_json(path: &str, contacts: &mut Vec<Contact>) {
    let contacts_json_string = fs::read_to_string(path).unwrap();
    *contacts = serde_json::from_str(&contacts_json_string).unwrap();
}
fn print_table_all(contacts: Vec<Contact>) {
    let mut table = Table::new();
    table
        .set_header(vec!["Sl", "Name", "Phone"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    for (i, contact) in contacts.iter().enumerate() {
        table.add_row(vec![&format!("{}", i + 1), &contact.name, &contact.phone]);
    }

    println!("{}", table);
}
fn print_table_from_tuple(contacts: Vec<(usize, &Contact)>) {
    let mut table = Table::new();
    table
        .set_header(vec!["Sl", "Name", "Phone"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    for (i, contact) in contacts {
        table.add_row(vec![&format!("{}", i + 1), &contact.name, &contact.phone]);
    }

    println!("{}", table);
}
