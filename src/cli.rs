use clap::{Arg, ArgMatches, Command};

pub fn setup_cli(args: &Vec<String>) -> clap::error::Result<ArgMatches> {
    Command::new("Blockchain Simulator")
        .subcommand(
            Command::new("sim")
                .about("Simulates mining blocks")
                .arg(
                    Arg::new("difficulty")
                        .short('d')
                        .long("difficulty")
                        .value_name("NUMBER")
                        .help("Difficulty level for mining")
                        .default_value("1"),
                )
                .arg(
                    Arg::new("blocks")
                        .short('b')
                        .long("blocks")
                        .value_name("NUMBER")
                        .help("Number of blocks to mine")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("miners")
                .about("Spawning miners")
                .subcommand(
                    Command::new("add")
                        .about("Adds a miner to the network")
                        .arg(
                            Arg::new("number")
                                .help("Number of miners to add")
                                .value_name("NUMBER")
                                .default_value("1") // Defaults to 1 if no value is provided
                                .required(false), // It is optional because we have a default value
                        ),
                )
                .subcommand(
                    Command::new("remove").about("Removes a miner").arg(
                        Arg::new("id")
                            .help("Miner id to remove")
                            .value_name("NUMBER")
                            .default_value("last") // Defaults to 1 if no value is provided
                            .required(false), // It is optional because we have a default value
                    ),
                ),
        )
        .subcommand(Command::new("status").about("Simulation status"))
        .try_get_matches_from(args)
}
