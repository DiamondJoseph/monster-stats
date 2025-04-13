use std::io;

#[tokio::main]
async fn main() {
    let exit = String::from("exit\n");
    let rustemon_client: rustemon::client::RustemonClient =
        rustemon::client::RustemonClient::default();

    loop {
        println!("Enter a species name (or `exit`):");
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
            Err(err) => println!(
                "Unable to fetch {} ({}): is the species name correct?",
                input, err
            ),
        }
        println!()
    }
}
