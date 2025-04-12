use std::io;

#[tokio::main]
async fn main() {
    println!("Enter a Pokémon species name:");
    let mut species = String::new();

    io::stdin()
        .read_line(&mut species)
        .expect("Failed to read line");

    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = rustemon::pokemon::pokemon::get_by_name(&species, &rustemon_client).await;
    println!("{}", pokemon.as_ref().unwrap().name);
    for stat in pokemon.unwrap().stats.into_iter() {
        println!("{}: {}", stat.stat.name, stat.base_stat);
    }
    
}
