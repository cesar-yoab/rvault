use std::process::exit;

use clap::{Args, Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Password};

#[derive(Debug, Parser)]
#[clap(
    name = "rvault",
    version = "0.1.0",
    author = "cyoab",
    about = "A CLI Password manager"
)]
pub struct Rvault {
    // #[clap(flatten)]
    // global_opts: GlobalOpts,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initializes a vault.
    Init,

    /// Search for a given key in the vault.
    Search(SearchArgs),

    /// Adds a new key to the vault.
    Add(AddArgs),

    /// Updates a key in the vault.
    Update(UpdateArgs),

    /// Deletes a key in the vault.
    Delete(DeleteArgs),
}

#[derive(Debug, Args)]
struct SearchArgs {
    /// Search key.
    search_key: String,
}

#[derive(Debug, Args)]
struct AddArgs {
    /// Key to add.
    add_key: String,
}

#[derive(Debug, Args)]
struct UpdateArgs {
    /// Key to update
    updated_key: String,
}

#[derive(Debug, Args)]
struct DeleteArgs {
    /// Key to delete
    delete_key: String,
}

impl Rvault {
    pub fn exec(self) {
        match self.command {
            Command::Init => {
                // Your logic for initializing a vault goes here
                self.init();
            }
            Command::Search(args) => {
                // Use args.search_key for searching within the vault
                println!("Searching for key: {}", args.search_key);
            }
            Command::Add(args) => {
                // Use args.add_key for adding a new key to the vault
                println!("Adding key: {}", args.add_key);
            }
            Command::Update(args) => {
                // Use args.updated_key for updating a key in the vault
                println!("Updating key: {}", args.updated_key);
            }
            Command::Delete(args) => {
                // Use args.delete_key for deleting a key from the vault
                println!("Deleting key: {}", args.delete_key);
            }
        }
    }

    fn init(self) {
        println!(
            "{}",
            "WARNING: This command will reset configurations and vault if it exists".yellow()
        );

        let continue_init = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to continue?")
            .default(true)
            .interact()
        {
            Ok(choice) => choice,
            Err(error) => {
                println!("{}: {}", "Error handling user input".red(), error);
                exit(1)
            }
        };

        if !continue_init {
            println!("{}", "Exiting...".blue());
            exit(0)
        }

        let _password = match Password::new()
            .with_prompt("Type your vault password")
            .with_confirmation("Confirm password", "Passwords mismatching")
            .interact()
        {
            Ok(password) => password,
            Err(error) => {
                println!("{}: {}", "Error handling user input".red(), error);
                exit(1)
            }
        };
    }
}
