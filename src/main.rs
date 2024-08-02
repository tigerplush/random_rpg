use clap::{Arg, Command};
use random_rpg::init;

fn main() {
    let matches = Command::new("rrpg")
        .subcommand(
            Command::new("init")
                .about("initialises a new world")
                .arg(Arg::new("name").short('n')),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _init_matches)) => {
            match init() {
                Ok(_) => (),
                Err(error) => panic!("Problem creating new world: {error:?}"),
            };
        }
        _ => unreachable!(),
    }
}
