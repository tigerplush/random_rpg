use core::panic;

use clap::{value_parser, Arg, ArgAction, Command};
use random_rpg::{generate, init};

fn main() {
    let matches = Command::new("rrpg")
        .subcommand(
            Command::new("init")
                .about("initialises a new world")
                .arg(Arg::new("name").short('n').long("name"))
                .arg(
                    Arg::new("seed")
                        .short('s')
                        .long("seed")
                        .value_parser(value_parser!(u32)),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Takes settings files and generates a world from them")
                .arg(Arg::new("name").short('n').long("name"))
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .long("debug")
                        .action(ArgAction::Count),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let name = init_matches.get_one::<String>("name");
            let seed = init_matches.get_one::<u32>("seed");
            match init(name, seed) {
                Ok(_) => (),
                Err(error) => panic!("Problem creating new world: {error:?}"),
            };
        }
        Some(("generate", generate_matches)) => {
            let name = generate_matches.get_one::<String>("name");
            let debug = generate_matches.get_count("debug");
            match generate(name, debug > 0) {
                Ok(_) => (),
                Err(error) => panic!("Problem generating world: {error:?}"),
            }
        }
        _ => unreachable!(),
    }
}
