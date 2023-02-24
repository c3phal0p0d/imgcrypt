use clap::Parser;
use commands::{encode, decode, remove, print};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = args::ImgcryptArgs::parse();

    match args.command {
        args::Command::Encode(cmd_args) => encode(cmd_args),
        args::Command::Decode(cmd_args) => decode(cmd_args),
        args::Command::Remove(cmd_args) => remove(cmd_args),
        args::Command::Print(cmd_args) => print(cmd_args),
    };

}