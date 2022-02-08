use clap::{Parser, Subcommand};

mod data;
mod args;
mod commands;

/// LegacyConverter is a command line tool that allows people to convert their legacy qosmetics files to the new formats
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler")]
struct Opts {
    #[clap(subcommand)]
    subcmd: MainCommand,
}

#[derive(Subcommand, Debug, Clone)]
enum MainCommand {
    /// Convert a Qbloq to a Cyoob
    Qbloq(args::cyoob::CyoobArgs),
    /// Convert a Qbloq to a Cyoob
    Cyoob(args::cyoob::CyoobArgs),
    /// Convert a Qsaber to a Whacker
    Qsaber,
    /// Convert a Qsaber to a Whacker
    Whacker,
    /// Convert a Qwall to a Dodgy
    Qwall,
    /// Convert a Qwall to a Dodgy
    Dodgy,
}

fn main() {
    match (Opts::parse() as Opts).subcmd {
        MainCommand::Qbloq(c) => commands::cyoob::convert(c),
        MainCommand::Cyoob(c) => commands::cyoob::convert(c),
        MainCommand::Qsaber => todo!("Not implemented!"),
        MainCommand::Whacker => todo!("Not implemented!"),
        MainCommand::Qwall => todo!("Not implemented!"),
        MainCommand::Dodgy => todo!("Not implemented!")
    }
}