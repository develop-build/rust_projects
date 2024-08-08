#[macro_use]
extern crate error_chain;
extern crate reqwest;

use reqwest::blocking;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {
    // using reqwest crate
    using_reqwest_crate().unwrap();

    // using tokio
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(using_async_await())
        .unwrap();
}

async fn using_async_await() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body: \n{}", body);
    Ok(())
}

fn using_reqwest_crate() -> Result<()> {
    let mut res = blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: {}", body);
    Ok(())
}
