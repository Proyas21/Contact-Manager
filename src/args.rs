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
    // /// Delete a contact
    // Delete(DeleteContact),
    // /// Edit a contact
    // Edit(EditContact),
    // /// Search a contact
    // Search(SearchContact),
    // /// Shows all contacts
    Show,
}

#[derive(Debug, Args)]
pub struct AddContact {
    /// Name of the contact
    pub name: String,
    /// Phone of the contact
    pub phone: String,
}

// #[derive(Debug, Subcommand)]
// pub enum AddContactSub {
//     Name(Vadfasd),
//     Phone(String),
// }
