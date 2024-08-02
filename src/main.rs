use clap::{value_parser, Arg, Command};
use random_rpg::init;

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
        _ => unreachable!(),
    }
}
