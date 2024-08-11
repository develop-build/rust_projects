# Rust HTTP Client Example

This Rust project demonstrates how to make HTTP requests using two different methods: synchronous and asynchronous, both using the `reqwest` crate. Additionally, it showcases error handling using the `error_chain` crate.

## Overview

The project includes two main functions:

1. `using_reqwest_crate`: Makes a synchronous HTTP GET request using `reqwest::blocking`.
2. `using_async_await`: Makes an asynchronous HTTP GET request using `reqwest` with async/await syntax.

## Dependencies

The following crates are used in this project:

- `reqwest`: A convenient HTTP client for Rust.
- `error_chain`: A crate for easy error handling and error propagation.

## Project Structure

- **`main.rs`**: The main file that contains the implementation of the two HTTP request methods and the error handling logic.

## How to Run

### Prerequisites

Ensure you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

### Running the Synchronous Example

To run the synchronous HTTP request example:

```sh
cargo run --release
```

The output will display the status, headers, and body of the response received from the `https://httpbin.org/get` endpoint using the synchronous method.

### Running the Asynchronous Example

The asynchronous example is included in the same program. The program automatically runs both the synchronous and asynchronous examples.

The output will display the status, headers, and body of the response received from the `https://httpbin.org/get` endpoint using the asynchronous method.

### Example Output

Running the program will generate output similar to the following:

```
Status: 200 OK
Headers:
{
    "date": "Sun, 11 Aug 2024 05:14:30 GMT",
    "content-type": "application/json",
    "content-length": "220",
    "connection": "keep-alive",
    "server": "gunicorn/19.9.0",
    "access-control-allow-origin": "*",
    "access-control-allow-credentials": "true",
}
Body: {
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-66b848b6-4cfc76072bd58e3174b7c9d7"
  },
  "origin": "49.37.75.185",
  "url": "https://httpbin.org/get"
}

Status: 200 OK
Headers:
{
    "date": "Sun, 11 Aug 2024 05:14:31 GMT",
    "content-type": "application/json",
    "content-length": "220",
    "connection": "keep-alive",
    "server": "gunicorn/19.9.0",
    "access-control-allow-origin": "*",
    "access-control-allow-credentials": "true",
}
Body:
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-66b848b7-359fbd5f6d9c753804277a0e"
  },
  "origin": "49.37.75.185",
  "url": "https://httpbin.org/get"
}


## Error Handling

This project uses the `error_chain` crate to handle errors. The `Result` type returned by the functions uses the `error_chain!` macro to define custom error types and handle foreign links to standard library errors (like `std::io::Error` and `reqwest::Error`).

## License

This project is licensed under the MIT License.
```
