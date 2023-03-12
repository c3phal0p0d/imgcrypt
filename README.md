# imgcrypt
A command line steganography tool for encrypting/decrypting secret messages within PNG images.  

## Usage
### Installation
- Clone the repository with the commmand ```git clone https://github.com/c3phal0p0d/imgcrypt.git```
- Then run ```cd imgcrypt``` to navigate into the program directory and ```cargo build``` to build the program.
- Finally run ```cargo run``` with the command line arguments below to run the program.
### Command line arguments
* ```encode```: Encode a secret message within the specified PNG file and save the output as a file
    * ```-f```: Path to PNG file
    * ```-c```: Chunk type
    * ```-m```: Secret message to be hidden in PNG
    * ```-o```: Optional path to output PNG file (default: output.png)
* ```decode```: Search for a secret message in the specified PNG file and extract it if it exists
    * ```-f```: Path to PNG file
    * ```-c```: Chunk type
* ```remove```: Remove a chunk from the PNG file and save the output as a file
    * ```-f```: Path to PNG file
    * ```-c```: Chunk type
* ```print```: Print the contents of the PNG file
    * ```-f```: Path to PNG file
* ```--help```: Display program usage information

## Resources used
- https://picklenerd.github.io/pngme_book/
- http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
