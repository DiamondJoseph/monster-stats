use std::collections::HashMap;
use std::io;

use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use strum_macros::EnumString;

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
enum Caught {
    Y,
    N,
}

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
enum Funcs {
    Encounter,
    Withdraw,
    Exit,
}

#[tokio::main]
async fn main() {
    let rustemon_client: rustemon::client::RustemonClient =
        rustemon::client::RustemonClient::default();
    let mut pokemon_box = HashMap::new();

    loop {
        let function = read_in("Encounter, Withdraw or Exit?");

        match function.parse() {
            Ok(Funcs::Encounter) => encounter(&rustemon_client, &mut pokemon_box).await,
            Ok(Funcs::Withdraw) => withdraw(&pokemon_box),
            Ok(Funcs::Exit) => break,
            Err(err) => println!("Unrecognised command: {err}"),
        }
    }
}

async fn encounter(client: &RustemonClient, pokemon_box: &mut HashMap<String, Pokemon>) {
    let species_name = read_in("Enter a species name:");

    let result = rustemon::pokemon::pokemon::get_by_name(&species_name, client).await;
    match result {
        Ok(species) => {
            print_species(&species);
            let caught = read_in("Did you catch it? (Y/N):");

            match caught.parse() {
                Ok(Caught::Y) => catch(pokemon_box, species),
                Ok(Caught::N) => println!("Better luck next time!"),
                Err(err) => println!("Unrecognised statement: {err}"),
            }
        }
        Err(err) => println!(
            "Unable to fetch {} ({}): is the species name correct?",
            species_name, err
        ),
    }
}

fn catch(pokemon_box: &mut HashMap<String, Pokemon>, species: Pokemon) {
    let name = read_in(&format!("Enter a unique name for {}:", species.name));
    pokemon_box.insert(name, species);
}

fn withdraw(pokemon_box: &HashMap<String, Pokemon>) {
    let name = read_in("Withdraw who?");

    match pokemon_box.get(&name) {
        Some(species) => print_species(species),
        None => println!("Not found"),
    }
}

fn print_species(species: &Pokemon) {
    for stat in species.stats.iter() {
        println!("{}: {}", stat.stat.name, stat.base_stat);
    }
}

fn read_in(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{prompt}");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned()
}
