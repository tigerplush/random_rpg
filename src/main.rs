use clap::{Arg, Command};
use random_rpg::init;

fn main() {
    let matches = Command::new("rrpg")
        .subcommand(
            Command::new("init")
                .about("initialises a new world")
                .arg(Arg::new("name").short('n').long("name")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let name = init_matches.get_one::<String>("name");
            match init(name) {
                Ok(_) => (),
                Err(error) => panic!("Problem creating new world: {error:?}"),
            };
        }
        _ => unreachable!(),
    }
}
