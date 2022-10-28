use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AppArgs {
    #[clap(subcommand)]
    pub action: MainActions,
}

#[derive(Debug, Subcommand)]
pub enum MainActions {
    /// Add a contact
    Add(AddContact),
    /// Delete a contact
    Delete(DeleteContact),
    /// Edit a contact
    Edit(EditContact),
    /// Search a contact
    Search(SearchContact),
    /// Shows all contacts
    Show,
}

#[derive(Debug, Args)]
pub struct AddContact {
    /// Name of the contact
    pub name: String,
    /// Phone number of the contact
    pub phone: String,
}
#[derive(Debug, Args)]
pub struct DeleteContact {
    /// Serial no. of the contact. Found in table in "show" command
    pub serial: String,
}
#[derive(Debug, Args)]
pub struct SearchContact {
    /// Keyword can be either name or phone number
    pub keyword: String,
}
#[derive(Debug, Args)]
pub struct EditContact {
    /// Serial no. of the contact. Found in table in "show" command
    pub serial: String,
}

// #[derive(Debug, Subcommand)]
// pub enum AddContactSub {
//     Name(Vadfasd),
//     Phone(String),
// }
