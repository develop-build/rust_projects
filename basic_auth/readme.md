````markdown
# Rust HTTP GET Request with Basic Authentication

This Rust project demonstrates how to make a simple HTTP GET request using the `reqwest` crate with optional Basic Authentication.

## Overview

The project sends a GET request to the `https://httpbin.org/get` endpoint using the `reqwest::blocking::Client`. The request optionally includes Basic Authentication headers, which are constructed using a username and an optional password.

If the request is successful, the program prints the response details, including the URL, status code, and headers.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## Getting Started

1. _Clone the repository_:
   git clone <repository-url>
   cd <repository-directory>

   ```

   ```

2. _Build the project_:
   cargo build

3. _Run the project_:
   cargo run

### Key Components

- _reqwest::blocking::Client_: This is a synchronous HTTP client used to send the request.

- _basic_auth_: This method attaches Basic Authentication credentials to the request.

- _Option<String> for Password_: The password is optional, meaning the request can be sent without it.

Upon running the code, you should see an output similar to this:

Got response: Ok(
Response {
url: Url {
scheme: "https",
cannot_be_a_base: false,
username: "",
password: None,
host: Some(
Domain(
"httpbin.org",
),
),
port: None,
path: "/get",
query: None,
fragment: None,
},
status: 200,
headers: {
"date": "Mon, 12 Aug 2024 01:07:52 GMT",
"content-type": "application/json",
"content-length": "268",
"connection": "keep-alive",
"server": "gunicorn/19.9.0",
"access-control-allow-origin": "\*",
"access-control-allow-credentials": "true",
},
},
)

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): A powerful, ergonomic HTTP Client for Rust.

## License

This project is licensed under the MIT License.

# Troubleshooting

If you encounter any issues or have questions about this project, please feel free to open an issue on the repository.
````
