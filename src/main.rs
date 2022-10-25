use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env::args, fs};

mod args;
use args::AppArgs;

use crate::args::{AddContact, MainActions};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    name: String,
    phone: String,
}
// pub struct Contacts [Contact]

fn main() {
    // let mut hello: Vec<i32> = (0..10).collect();
    // let mut hello: [i32; 5] = [1, 2, 3, 4, 5];
    // let args: Vec<String> = args().collect();
    let args = AppArgs::parse();

    let contacts_json_string = fs::read_to_string("./src/contacts.json").unwrap();
    let mut contacts: Vec<Contact> = serde_json::from_str(&contacts_json_string).unwrap();

    let contacts_json_string_towrite = serde_json::to_string(&contacts).unwrap();
    // fs::write("./src/contacts.json", &contacts_json_string_towrite).unwrap();

    println!("{:?}", args);

    let mut addContactToJson = |name: &String, phone: &String| {
        contacts.push(Contact {
            name: name.to_string(),
            phone: phone.to_string(),
        });
        saveJson("./src/contacts.json", &contacts);
        println!("added: {} {}", name, phone);
    };
    fn showContactsAll(contacts: Vec<Contact>) {
        println!("{:?}", contacts)
    }

    match args.action {
        MainActions::Add(info) => addContactToJson(&info.name, &info.phone),
        MainActions::Show => showContactsAll(contacts),
        _ => println!("none :("),
    }
}

fn saveJson(path: &str, contacts: &Vec<Contact>) {
    let contacts_json_string_towrite = serde_json::to_string(contacts).unwrap();
    fs::write(path, contacts_json_string_towrite).unwrap();
}
fn fetchJson(path: &str, contacts: &mut Vec<Contact>) {
    let contacts_json_string = fs::read_to_string(path).unwrap();
    *contacts = serde_json::from_str(&contacts_json_string).unwrap();
}
