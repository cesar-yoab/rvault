use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "rvault", version = "0.1.0", author = "cyoab", about = "A CLI Password manager")]
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
    updated_key: String
}

#[derive(Debug, Args)]
struct DeleteArgs {
    /// Key to delete
    delete_key: String 
}

impl Rvault {
    pub fn exec(self) {
        println!("RVault CLI");
    }
}
