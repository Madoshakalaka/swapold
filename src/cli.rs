use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command, ValueHint};


pub fn build_cli() -> Command {
command!() 
        .arg(arg!(<FILE_NAME> "The base of the file, without .old")
            .value_hint(ValueHint::FilePath)
            .value_parser(clap::value_parser!(PathBuf))
            )

}
