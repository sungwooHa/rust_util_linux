use std::fs::File;
use clap::{Parser, Subcommand};



#[derive(Subcommand)]
pub enum Command {
    more,
}

pub fn analysis_command(str : &str) -> Result<Command, &str> {
    match str{
        "more" => {
            Ok(Command::more)
        },
        _ =>{
            Err("wrong command")
        }
    }
}



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file : File,

    #[command(Subcommand)]
    command : Option<Command>,
}
