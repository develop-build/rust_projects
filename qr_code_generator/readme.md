# QR Code Generator

This Rust project generates a QR code for a given URL and saves it as a PNG image. The project uses the `qrcode` crate to generate the QR code and the `image` crate to save the QR code as an image file.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/develop-build/qr_code_generator.git
cd qr_code_generator
```

### 2. Add Dependencies

Ensure your `Cargo.toml` includes the following dependencies:

```toml
[dependencies]
image = "0.25.2"
qrcode = "0.14.1"
```

### 3. Build and Run the Project

To generate the QR code, run the following command:

```bash
cargo run
```

### 4. Output

After running the project, a QR code image named `qrcode.png` will be generated in the project directory. This image represents the QR code for the URL `https://www.rust-lang.org/`.

# Output:

The program outputs _"QR code generated"_ to the console when the image has been successfully saved.

## License

This project is licensed under the MIT License.
