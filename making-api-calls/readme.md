# GitHub Stargazers Fetcher

This Rust project demonstrates how to make an asynchronous HTTP request to the GitHub API to fetch the list of stargazers for a specific repository. The response is then deserialized into a vector of user structs using the `serde` crate.

## Features

- Makes an asynchronous HTTP GET request using the `reqwest` crate.
- Sets a custom User-Agent header.
- Deserializes JSON response into Rust structs using `serde`.
- Prints out the list of users (stargazers) with their `login` and `id` fields.

## Prerequisites

Before running the project, ensure you have the following installed:

- Rust (with `cargo`)
- A GitHub account (for accessing public GitHub APIs)

## Getting Started

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/your-username/your-repo.git
   cd your-repo
   ```

2. **Add Dependencies:**

   Ensure your `Cargo.toml` file includes the following dependencies:

   ```toml
   [dependencies]
   serde_json = "1.0.122"
   serde = { version = "1.0.205", features = ["derive"] }
   reqwest = { version = "0.12.5", features = ["json"] }
   tokio = { version = "1.39.2", features = ["full"] }
   ```

3. **Run the Code:**

   Execute the program using Cargo:

   ```sh
   cargo run
   ```

   The program fetches the list of stargazers for the `rust-cookbook` repository under the `rust-lang-nursery` organization.

## Code Explanation

- **HTTP Client Setup:**

  - The program uses `reqwest::Client` to create a new HTTP client.
  - A GET request is made to the GitHub API using the repository owner and name.

- **User-Agent Header:**

  - A custom `User-Agent` header is set to identify the client making the request (`rust web-api-client demo`).

- **JSON Deserialization:**

  - The JSON response from the API is deserialized into a `Vec<User>` where `User` is a struct defined with `serde` for automatic JSON deserialization.

- **Printing the Result:**
  - The deserialized user data, including `login` and `id`, is printed to the console.

## Example Output

When you run the program, you'll see the output like this:

```plaintext
User {
    login: "k0pernicus",
    id: 3605451,
},
User {
    login: "jaxx",
    id: 723258,
},
```

This output shows the `login` and `id` of each stargazer for the specified repository.

## License

This project is licensed under the MIT License.
