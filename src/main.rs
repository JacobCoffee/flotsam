extern crate digitalocean;
use digitalocean::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    match Droplet::list().execute(&client) {
        Ok(droplets) => {
            println!("Droplets:");
            for droplet in droplets {
                println!("ID: {}, Name: {}, Status: {}", droplet.id(), droplet.name(), droplet.status());
            }
        },
        Err(e) => println!("Error listing droplets: {:?}", e),
    }
}
