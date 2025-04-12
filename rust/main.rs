use std::io;

#[tokio::main]
async fn main() {
    println!("Enter a Pokémon species name (or `exit`):");
    let exit = String::from("exit\n");
    let rustemon_client: rustemon::client::RustemonClient =
        rustemon::client::RustemonClient::default();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.eq_ignore_ascii_case(&exit) {
            break;
        }

        let result = rustemon::pokemon::pokemon::get_by_name(&input, &rustemon_client).await;
        match result {
            Ok(species) => {
                for stat in species.stats.into_iter() {
                    println!("{}: {}", stat.stat.name, stat.base_stat);
                }
            }
            Err(_) => println!("Not a valid species name"),
        }
    }
}
