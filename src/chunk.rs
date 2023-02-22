use std::convert::TryFrom;
use std::io::{BufReader, Read};
use std::fmt;
use crc::crc32::checksum_ieee;

use crate::Result;
use crate::chunk_type::ChunkType;

// PNG chunk
pub struct Chunk {
    length: u32,    // number of bytes in chunk's data field
    chunk_type: ChunkType,  // chunk type code
    data: Vec<u8>,  // chunk data
    crc: u32    // CRC calculated on preceding bytes in chunk, including chunk type & data, but not length
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = checksum_ieee(&crc_data);

        Chunk { 
            length: data.len() as u32,
            chunk_type,
            data,
            crc
        }
    }

    // Length of chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    // Chunk type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // Chunk data in bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    // Convert chunk data to String, returning error if data is not valid UTF-8
    pub fn data_as_string(&self) -> Result<String> {
        let data_string = String::from_utf8(self.data.clone()).expect("Error converting data to string");
        Ok(data_string)
    }

    // Convert chunk to a byte array containing in order: length, chunk type, chunk data & crc
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0; 4];

        // read data length
        reader.read_exact(&mut buffer).unwrap();
        //let length = u32::from_be_bytes(buffer);
        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        println!("Length: {}", length);

        // read chunk type
        reader.read_exact(&mut buffer).unwrap();
        let chunk_type: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type).unwrap();
        println!("Chunk type: {}", chunk_type);

        // make sure input chunk type is valid
        if !chunk_type.is_valid(){
            return Err("Invalid chunk type")
        }

        // read data
        let data = Vec::from(&bytes[8..bytes.len()-4]);
        println!("Data: {:?}", data);

        // make sure input length is equal to calculated length
        if length!=data.len().try_into().unwrap() {
            return Err("Invalid length")
        }

        // read CRC input
        let crc = &bytes[bytes.len()-4..];
        let crc = u32::from_be_bytes(crc.try_into().unwrap());
        println!("CRC input: {}", crc);

        // calculate CRC from input chunk type and data
        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let calculated_crc = checksum_ieee(&crc_data);
        println!("CRC calculated: {}", calculated_crc);

        // make sure input CRC is equal to calculated CRC
        if crc!=calculated_crc  {
              return Err("Invalid CRC")
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Length: {}", self.length())?;
        writeln!(f, "Type: {}", self.chunk_type())?;
        writeln!(f, "Data size: {} bytes", self.data.len())?;
        writeln!(f, "Crc: {}", self.crc());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}