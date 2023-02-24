use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use std::str::FromStr;

use crate::Error;
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

// Create PNG struct from file
pub fn read_png(file_path: PathBuf) -> Result<Png, Error> {
    Ok(Png::from_file(file_path).unwrap())
}

// Encode secret message within PNG and save the output as a file
pub fn encode(args: EncodeArgs) -> Result<(), Error>{
    let mut png = read_png(args.file_path).unwrap();
    let chunk = Chunk::new(ChunkType::from_str(&args.chunk_type).unwrap(), args.message.into_bytes());
    png.append_chunk(chunk);

    let mut png_file = File::create(args.output_file_path)?;
    png_file.write_all(&png.as_bytes())?;

    println!("Successfully encoded message within PNG");

    Ok(())

}

// Extract secret message from PNG if it exists
pub fn decode(args: DecodeArgs) -> Result<(), Error> {
    let png = read_png(args.file_path).unwrap();
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("Hidden message: {}", chunk.data_as_string().unwrap());
    } else {
        println!("No hidden messages found");
    }
    
    Ok(())
}

// Remove chunk from PNG and save the output as a file
pub fn remove(args: RemoveArgs) -> Result<(), Error>{
    let mut png = read_png(args.file_path.clone()).unwrap();
    png.remove_chunk(&args.chunk_type)?;

    let mut png_file = File::create(args.file_path)?;
    png_file.write_all(&png.as_bytes())?;

    println!("Successfully removed chunk {} from PNG", args.chunk_type);

    Ok(())
}

// Print PNG chunks
pub fn print(args: PrintArgs) -> Result<(), Error>{
    let png = read_png(args.file_path).unwrap();
    println!("PNG: {}", png.to_string());

    for chunk in png.chunks().iter() {
        println!("{}", chunk)
    }

    Ok(())
}