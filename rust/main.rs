use std::io;

#[tokio::main]
async fn main() {
    println!("Enter a Pokémon species name:");
    let mut species = String::new();

    io::stdin()
        .read_line(&mut species)
        .expect("Failed to read line");

    let rustemon_client = rustemon::client::RustemonClient::default();
    let species_stats = rustemon::pokemon::pokemon::get_by_name(&species, &rustemon_client).await;
    match species_stats {
        Ok(species) => {
            for stat in species.stats.into_iter() {
                println!("{}: {}", stat.stat.name, stat.base_stat);
            }
        }
        Err(_) => println!("Not a valid species name"),
    }
}
