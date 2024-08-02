use digitalocean::prelude::*;
use strsim::jaro_winkler;

pub fn list_droplets(client: &DigitalOcean) {
    match Droplet::list().execute(client) {
        Ok(droplets) => {
            println!("Droplets:");
            for droplet in droplets {
                println!(
                    "ID: {}, Name: {}, Status: {}",
                    droplet.id(),
                    droplet.name(),
                    droplet.status()
                );
            }
        }
        Err(e) => println!("Error listing droplets: {:?}", e),
    }
}

pub fn find_droplet(client: &DigitalOcean, name: &str) {
    match Droplet::list().execute(client) {
        Ok(droplets) => {
            let mut best_match = None;
            let mut best_score = 0.0;

            for droplet in droplets {
                let score = jaro_winkler(name, droplet.name());
                if score > best_score {
                    best_score = score;
                    best_match = Some(droplet);
                }
            }

            if let Some(droplet) = best_match {
                println!("Dropet found:");
                println!("ID: {}", droplet.id());
                println!("Name: {}", droplet.name());
                println!("Status: {}", droplet.status());
                println!("Memory: {} MB", droplet.memory());
                println!("VCPUs: {}", droplet.vcpus());
                println!("Disk: {} GB", droplet.disk());
                println!("Region: {}", droplet.region().slug());
                println!("Image: {}", droplet.image().name());
            } else {
                println!("Droplet with name '{}' not found.", name);
            }
        }
        Err(e) => println!("Error finding droplet: {:?}", e),
    }
}
