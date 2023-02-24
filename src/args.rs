use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
pub struct ImgcryptArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode a secret message within the specified PNG file and save the output as a file
    Encode(EncodeArgs),

    /// Search for a secret message in the specified PNG file and extract it if it exists
    Decode(DecodeArgs),

    /// Remove a chunk from the PNG file and save the output as a file
    Remove(RemoveArgs),

    /// Print the contents of the PNG file
    Print(PrintArgs)
}

#[derive(Debug, Args)]
pub struct EncodeArgs {

    /// Path to PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,

    /// Secret message to be hidden in PNG
    #[arg(short, long)]
    pub message: String,

    /// Optional path to output PNG file
    #[arg(short, long, default_value = "output.png")]
    pub output_file_path: PathBuf
}

#[derive(Debug, Args)]
pub struct DecodeArgs {

    /// Path to PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String
}

#[derive(Debug, Args)]
pub struct RemoveArgs {

    /// Path to PNG file
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String
}

#[derive(Debug, Args)]
pub struct PrintArgs {

    /// Path to PNG file
    #[arg(short, long)]
    pub file_path: PathBuf
}