use std::collections::HashMap;
use std::io;

use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString)]
enum Caught {
    Y,
    N,
}

#[derive(EnumString)]
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
        let mut function_input = String::new();
        println!("Encounter, Withdraw or Exit?");
        read_in(&mut function_input);

        match function_input.trim().parse() {
            Ok(func) => match func {
                Funcs::Encounter => encounter(&rustemon_client, &mut pokemon_box).await,
                Funcs::Withdraw => withdraw(&pokemon_box),
                Funcs::Exit => {
                    break;
                }
            },
            Err(err) => println!("Unrecognised command: {err}"),
        }
    }
}

async fn encounter(client: &RustemonClient, pokemon_box: &mut HashMap<String, Pokemon>) {
    let mut species_input = String::new();
    println!("Enter a species name:");
    read_in(&mut species_input);

    let result = rustemon::pokemon::pokemon::get_by_name(&species_input, client).await;
    match result {
        Ok(species) => {
            print_species(&species);

            let mut caught_input = String::new();
            println!("Did you catch it? (Y/N):");
            read_in(&mut caught_input);

            match caught_input.trim().parse() {
                Ok(caught) => {
                    match caught {
                        Caught::Y => catch(pokemon_box, species),
                        Caught::N => {
                            println!("Better luck next time!")
                        }
                    };
                }
                Err(err) => println!(
                    "Unable to fetch {} ({}): is the species name correct?",
                    species_input, err
                ),
            }
        }
        Err(err) => println!(
            "Unable to fetch {} ({}): is the species name correct?",
            species_input, err
        ),
    }
    println!()
}

fn catch(pokemon_box: &mut HashMap<String, Pokemon>, species: Pokemon) {
    let mut name_input = String::new();
    println!("Enter a unique name for {}:", species.name);
    read_in(&mut name_input);
    pokemon_box.insert(name_input, species);
}

fn withdraw(pokemon_box: &HashMap<String, Pokemon>) {
    let mut name_input = String::new();
    println!("Withdraw who?");
    read_in(&mut name_input);

    match pokemon_box.get(&name_input) {
        Some(species) => {
            print_species(species);
        }
        None => {
            println!("Not found");
        }
    }
}

fn print_species(species: &Pokemon) {
    for stat in species.stats.iter() {
        println!("{}: {}", stat.stat.name, stat.base_stat);
    }
}

fn read_in(buffer: &mut String) {
    io::stdin().read_line(buffer).expect("Failed to read line");
}
