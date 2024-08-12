use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = String::from("test_user");
    let password: Option<String> = None;
    let response = client
        .get("https://httpbin.org/get")
        .basic_auth(user, password)
        .send();

    println!("Got response: {:#?}", response);
    Ok(())
}
