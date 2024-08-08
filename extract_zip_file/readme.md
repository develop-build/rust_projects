# ZIP File Extractor

This Rust project is a command-line tool that extracts files from a ZIP archive. It handles directories, files, and associated metadata like comments and permissions (on Unix systems).

## Features

- Extracts all files and directories from a ZIP archive.
- Preserves directory structure.
- Displays comments associated with individual files (if any).
- Handles Unix file permissions.
- Simple and efficient, leveraging the `zip` crate.

## Usage

### Building the Project

To build the project, use the following command:

#### Example

If you run the command with a file named `1.zip`, the output might look like this:

```sh
cargo run 1.zip
```

**Output:**

```
File 0 extracted to "1/"
File 1 extracted to "1/project ideas/"
File 2 extracted to "1/project ideas/Project Ideas.txt" (1002 bytes)
File 3 extracted to "1/reactHooks.JPG" (33564 bytes)
```

This output shows that:

- A directory named `1/` was created.
- The directory `1/project ideas/` was also created.
- The file `1/project ideas/Project Ideas.txt` was extracted with a size of 1002 bytes.
- The file `1/reactHooks.JPG` was extracted with a size of 33564 bytes.

### Unix Permissions

On Unix systems, the tool preserves the original file permissions stored in the ZIP archive.

## Dependencies

- [zip](https://crates.io/crates/zip) - Rust library for working with ZIP archives.

## License

This project is licensed under the MIT License.
