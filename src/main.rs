mod cli;

use clap::{Parser, Subcommand};
use digitalocean::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all droplets
    ListDroplets,
    /// Find a droplet by name.
    FindDroplet {
        /// The name of the droplet to find (Uses Fuzzy search w/ Jaro-Winkler similarity)
        name: String,
    },
}

fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ListDroplets) => cli::list_droplets(&client),
        Some(Commands::FindDroplet { name }) => cli::find_droplet(&client, &name),
        None => println!("No command specified. Use --help for usage information."),
    }
}
