use std::collections::HashMap;
use std::io;

#[tokio::main]
async fn main() {
    let exit = String::from("exit\n");
    let caught = String::from("y\n");
    let rustemon_client: rustemon::client::RustemonClient =
        rustemon::client::RustemonClient::default();
    let mut pokemon_box = HashMap::new();

    loop {
        println!("Enter a species name (or `exit`):");
        let mut species_input = String::new();

        io::stdin()
            .read_line(&mut species_input)
            .expect("Failed to read line");

        if species_input.eq_ignore_ascii_case(&exit) {
            break;
        }

        let result =
            rustemon::pokemon::pokemon::get_by_name(&species_input, &rustemon_client).await;
        match result {
            Ok(species) => {
                for stat in species.stats.iter() {
                    println!("{}: {}", stat.stat.name, stat.base_stat);
                }

                println!("Did you catch it? (Y/N):");
                let mut caught_input = String::new();

                io::stdin()
                    .read_line(&mut caught_input)
                    .expect("Failed to read line");

                if !caught_input.eq_ignore_ascii_case(&caught) {
                    println!("Better luck next time");
                } else {
                    println!("Enter a name for your new {}", species.name);
                    let mut name_input = String::new();

                    io::stdin()
                        .read_line(&mut name_input)
                        .expect("Failed to read line");

                    pokemon_box.insert(name_input, species);
                }
            }
            Err(err) => println!(
                "Unable to fetch {} ({}): is the species name correct?",
                species_input, err
            ),
        }
        println!()
    }
}
