use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    print!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    // printing login, id from response
    println!("{:#?}", users);

    /*
    User {
            login: "k0pernicus",
            id: 3605451,
        },
        User {
            login: "jaxx",
            id: 723258,
        },
        User {
            login: "dhharris",
            id: 9009622,
        },
        User {
            login: "zhangsoledad",
            id: 3198439,
        },
        User {
            login: "ssebastianj",
            id: 309535,
        },
        User {
            login: "oclbdk",
            id: 136982,
        },
        User {
            login: "Latrasis",
            id: 4656227,
        },
        User {
            login: "narendasan",
            id: 1790613,
        },
        User {
            login: "rishabh92",
            id: 13951936,
        },
        User {
            login: "hueftl",
            id: 11706301,
        },
        User {
            login: "seeekr",
            id: 302886,
        },
        User {
            login: "krzyk",
            id: 105730,
        },

        */
    Ok(())
}
