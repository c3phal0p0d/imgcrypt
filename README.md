# imgcrypt

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
