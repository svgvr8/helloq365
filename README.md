## HelloQ365
HelloQ365 is a Python library by Quantum365 that provides encryption and decryption functionality using a simple substitution cipher. It is built using Rust for high performance.

## Installation
You can install HelloQ365 using pip:
`` pip install helloq365 ``

## Usage
Here's an example of how to use the library:

```
{
    import helloq365 

plaintext = "Hello, world!"
key = 3

ciphertext = helloq365.encrypt(plaintext, key)
print(ciphertext)

decrypted = helloq365.decrypt(ciphertext, key)
print(decrypted) 

}
```
This will output:

``Khoor, zruog!
Hello, world! ``

## Development
If you want to build the library from source, you'll need to have Rust and Cargo installed. You can build the library using the following command:

`` cargo build --release ``
This will create a *.whl file in the target/wheels directory, which you can install using pip.

## License
HelloQ365 is licensed under the MIT License.

## Project Links
[GitHub Repository](https://github.com/svgvr8/helloq365)
[PyPI Package](https://pypi.org/project/helloq365/0.1.0/)