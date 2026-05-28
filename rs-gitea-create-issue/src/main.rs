use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Issue {
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::fs::read_to_string("creds")?;

    let client = Client::new();
    let issue = Issue {
            title: "Issue Title!".into(),
            body: "I made an issue from commandline!".into(),
        };
    let response = client
        .post("http://gitea.odin.orlfl.milamnet.io/api/v1/repos/jmilam/test-repo/issues")
        .bearer_auth(token.trim())
        .json(&issue)
        .send()
        .await?
        .error_for_status()?;

    println!("{}", response.text().await?);

    Ok(())
}
