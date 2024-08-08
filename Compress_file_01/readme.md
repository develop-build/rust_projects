````markdown
# PDF Compressor

This Rust project compresses a PDF file using the `flate2` crate. The compressed file is generated in Gzip format.

## Usage

To run the project, use the following command:

```sh
cargo run <input_file> <output_file>
```
````

Replace `<input_file>` with the path to the PDF file you want to compress, and `<output_file>` with the desired output path for the compressed file.

### Example

```sh
cargo run book.pdf compressed.gz
```

This will compress the `book.pdf` file and save the compressed output as `compressed.gz`.

## Output

After running the program, you will see output similar to the following:

```
Source len: 3767427
Target len: 3224294
Time Elapsed: 2.6149774s
```

- **Source len**: The size of the original PDF file in bytes.
- **Target len**: The size of the compressed file in bytes.
- **Time Elapsed**: The time taken to compress the file.

## Dependencies

This project relies on the following dependencies:

- `flate2`: For compressing the file.
- `std::env`: For handling command-line arguments.
- `std::fs`: For file operations.
- `std::io`: For I/O operations.
- `std::time`: For measuring elapsed time.

To add the `flate2` dependency, include the following in your `Cargo.toml`:

```toml
[dependencies]
flate2 = "1.0.31"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

```
This `README.md` provides clear instructions on how to use the project, an example of how to run it, and a summary of the expected output and dependencies.
```
